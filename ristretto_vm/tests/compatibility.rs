#![cfg_attr(target_family = "wasm", allow(unused_imports, dead_code))]
#![expect(
    clippy::expect_used,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "compatibility harness uses assertions and setup expects in Result-returning tests"
)]

#[cfg(not(target_family = "wasm"))]
use rayon::ThreadPoolBuilder;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use regex::Regex;
use ristretto_classloader::{DEFAULT_JAVA_VERSION, runtime};
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_vm::Error::InternalError;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
#[cfg(not(target_family = "wasm"))]
use std::{collections::HashMap, fs};
use tokio::sync::Mutex as AsyncMutex;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{EnvFilter, fmt};

/// Regex to filter which tests to run. Defaults to matching all tests.
/// Set the `TEST_FILTER` environment variable to a regex pattern to filter tests.
static TEST_FILTER: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = std::env::var("TEST_FILTER").unwrap_or_else(|_| ".*".to_string());
    Regex::new(&pattern).expect("valid regex")
});
const TEST_CLASS_NAME: &str = "Test";
const TEST_FILE: &str = "Test.java";
const IGNORE_FILE: &str = "ignore.txt";
const RUNTIME_PROPERTIES_FILE: &str = "runtime.properties";
const TEST_TIMEOUT_PREFIX: &str = "Test timed out after ";

/// A single failure observed while running the compatibility test suite.
#[derive(Debug, Clone)]
struct Failure {
    test_name: String,
    test_type: String,
    message: String,
}

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone)]
struct TestCase {
    relative_dir: PathBuf,
    source_dir: PathBuf,
    class_dir: PathBuf,
    java_version: String,
    java_home: PathBuf,
    system_properties: Vec<(String, String)>,
}

#[cfg(not(target_family = "wasm"))]
impl TestCase {
    fn name(&self) -> String {
        let base = self.relative_dir.to_string_lossy().replace('\\', "/");
        if self.source_dir == self.class_dir {
            base
        } else {
            format!("{base}@java{}", self.java_version)
        }
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn compatibility_tests() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = cargo_manifest
        .parent()
        .ok_or_else(|| InternalError("crate directory has no parent".to_string()))?;
    // CARGO_MANIFEST_DIR is already absolute. Avoid canonicalizing this path on Windows because
    // that produces a verbatim `\\?\` prefix, which older JDK tools reject in command arguments.
    let tests_root_dir = workspace_dir.join("tests");

    initialize_tracing();

    let test_dirs = collect_test_dirs(&tests_root_dir)?;
    let test_cases = build_test_cases(&tests_root_dir, &test_dirs)?;
    compile_tests(&test_cases)?;

    let passed = AtomicUsize::new(0);
    let failures: Arc<Mutex<Vec<Failure>>> = Arc::new(Mutex::new(Vec::new()));

    let available_threads = std::thread::available_parallelism().map_or(1, NonZero::get);
    // Each compatibility case occupies a Rayon worker, a test thread, a Tokio worker, and a GC
    // worker. Account for all four so concurrent VMs do not starve long-running tests.
    // Windows CI runners report substantially more logical processors than they can sustain
    // while several full VMs perform mapped-file and native socket operations. Running those VMs
    // concurrently can starve otherwise healthy cases past their two-minute timeout.
    let default_threads = if cfg!(target_os = "windows") {
        1
    } else {
        (available_threads / 4).max(1)
    };
    let num_threads = std::env::var("RISTRETTO_COMPATIBILITY_THREADS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&value| value > 0)
        .map_or(default_threads, |value| value.min(available_threads));
    info!("Running compatibility tests with {num_threads} threads");
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|error| InternalError(error.to_string()))?;

    pool.install(|| {
        test_cases
            .par_iter()
            .filter(|test_case| !test_case.relative_dir.starts_with("socket"))
            .for_each(|test_case| {
                run_compatibility_test(test_case, &passed, &failures);
            });
    });

    // Socket tests can bind shared ports and depend on tight client/server timing. Run them after
    // the parallel suite so unrelated VMs cannot starve or interfere with their network threads.
    test_cases
        .iter()
        .filter(|test_case| test_case.relative_dir.starts_with("socket"))
        .for_each(|test_case| {
            run_compatibility_test(test_case, &passed, &failures);
        });

    let passed_count = passed.load(Ordering::Relaxed);
    let failures = std::mem::take(&mut *failures.lock().expect("failures lock"));
    let failed_count = failures.len();

    info!("Tests: {}", passed_count + failed_count);
    if failed_count > 0 {
        error!("Tests failed: {failed_count}");
        error!("=== Compatibility test failures ===");
        for (index, failure) in failures.iter().enumerate() {
            error!(
                "[{}/{}] ({}) {}: {}",
                index + 1,
                failed_count,
                failure.test_type,
                failure.test_name,
                failure.message
            );
        }
        error!("=== End of failures ({failed_count} total) ===");
    } else {
        info!("All tests passed");
    }
    assert_eq!(
        failed_count, 0,
        "{failed_count} compatibility test(s) failed"
    );
    Ok(())
}

/// Runs a single compatibility test (both interpreted and JIT) and records the outcome.
#[cfg(not(target_family = "wasm"))]
fn run_compatibility_test(
    test_case: &TestCase,
    passed: &AtomicUsize,
    failures: &Mutex<Vec<Failure>>,
) {
    let test_name = test_case.name();

    if !TEST_FILTER.is_match(&test_name) {
        return;
    }

    // Determine if the test should be ignored
    let ignore_file = test_case.source_dir.join(IGNORE_FILE);
    if ignore_file.exists() {
        debug!("Ignoring test {test_name}");
        return;
    }

    info!("Running test {test_name}");

    let record = |result: Result<(), Failure>| match result {
        Ok(()) => {
            passed.fetch_add(1, Ordering::Relaxed);
        }
        Err(failure) => {
            failures.lock().expect("failures lock").push(failure);
        }
    };

    match expected_output(test_case) {
        Ok((expected_duration, expected_output)) => {
            record(test_vm(
                &test_case.java_version,
                &test_name,
                &test_case.class_dir,
                &test_case.source_dir,
                &test_case.system_properties,
                true,
                &expected_duration,
                &expected_output,
            ));
            record(test_vm(
                &test_case.java_version,
                &test_name,
                &test_case.class_dir,
                &test_case.source_dir,
                &test_case.system_properties,
                false,
                &expected_duration,
                &expected_output,
            ));
        }
        Err(error) => {
            let message = format!("Failed to obtain expected output: {error:?}");
            error!("{test_name}: {message}");
            let mut guard = failures.lock().expect("failures lock");
            guard.push(Failure {
                test_name: test_name.clone(),
                test_type: "int".to_string(),
                message: message.clone(),
            });
            guard.push(Failure {
                test_name,
                test_type: "jit".to_string(),
                message,
            });
        }
    }
}

/// Initializes the tracing subscriber for logging.
#[cfg(not(target_family = "wasm"))]
fn initialize_tracing() {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_timer(fmt::time::uptime())
        .compact();

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("cranelift=warn".parse().expect("directive"))
        .add_directive("ristretto_classloader=warn".parse().expect("directive"))
        .add_directive("ristretto_vm=error".parse().expect("directive"));
    fmt()
        .with_env_filter(filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

/// Collects directories of all tests to run. A test is a directory that contains a
/// `Test.java` file.
#[cfg(not(target_family = "wasm"))]
fn collect_test_dirs(tests_root_dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut test_paths = Vec::new();
    for entry in walkdir::WalkDir::new(tests_root_dir) {
        let entry = entry.map_err(|error| InternalError(error.to_string()))?;
        if entry.file_name() == TEST_FILE
            && let Some(test_dir) = entry.path().parent()
        {
            let test_dir = test_dir.strip_prefix(tests_root_dir).unwrap_or(test_dir);
            test_paths.push(test_dir.to_path_buf());
        }
    }
    Ok(test_paths)
}

/// Gets the VM directory for the Java runtime.
#[cfg(not(target_family = "wasm"))]
fn java_home(java_version: &str) -> Result<PathBuf> {
    let runtime =
        tokio::runtime::Runtime::new().map_err(|error| InternalError(error.to_string()))?;
    let (java_home, _java_version, _class_loader) =
        runtime.block_on(runtime::version_class_loader(java_version))?;
    Ok(java_home)
}

#[cfg(not(target_family = "wasm"))]
fn build_test_cases(tests_root_dir: &Path, test_dirs: &[PathBuf]) -> Result<Vec<TestCase>> {
    let workspace_dir = tests_root_dir
        .parent()
        .ok_or_else(|| InternalError("tests directory has no parent".to_string()))?;
    let output_root = workspace_dir
        .join("target")
        .join("compatibility")
        .join(format!(
            "{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ));
    let mut java_homes: HashMap<String, PathBuf> = HashMap::new();
    let mut cases = Vec::new();
    for relative_dir in test_dirs {
        let source_dir = tests_root_dir.join(relative_dir);
        let properties_file = source_dir.join(RUNTIME_PROPERTIES_FILE);
        if !properties_file.exists() {
            let version = DEFAULT_JAVA_VERSION.to_string();
            let home = if let Some(home) = java_homes.get(&version) {
                home.clone()
            } else {
                let home = java_home(&version)?;
                java_homes.insert(version.clone(), home.clone());
                home
            };
            cases.push(TestCase {
                relative_dir: relative_dir.clone(),
                source_dir: source_dir.clone(),
                class_dir: source_dir,
                java_version: version,
                java_home: home,
                system_properties: Vec::new(),
            });
            continue;
        }
        let properties = parse_runtime_properties(&properties_file)?;
        let versions = properties
            .get("java.versions")
            .ok_or_else(|| {
                InternalError(format!(
                    "{} must define java.versions",
                    properties_file.display()
                ))
            })?
            .split(',')
            .map(str::trim)
            .filter(|version| !version.is_empty());
        let system_properties = properties
            .iter()
            .filter_map(|(key, value)| {
                key.strip_prefix("system.")
                    .map(|key| (key.to_string(), value.clone()))
            })
            .collect::<Vec<_>>();
        for version in versions {
            let home = if let Some(home) = java_homes.get(version) {
                home.clone()
            } else {
                let home = java_home(version)?;
                java_homes.insert(version.to_string(), home.clone());
                home
            };
            cases.push(TestCase {
                relative_dir: relative_dir.clone(),
                source_dir: source_dir.clone(),
                class_dir: output_root.join(version).join(relative_dir),
                java_version: version.to_string(),
                java_home: home,
                system_properties: system_properties.clone(),
            });
        }
    }
    Ok(cases)
}

#[cfg(not(target_family = "wasm"))]
fn parse_runtime_properties(path: &Path) -> Result<HashMap<String, String>> {
    let contents = fs::read_to_string(path)?;
    let mut properties = HashMap::new();
    for (line_number, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(InternalError(format!(
                "invalid property at {}:{}",
                path.display(),
                line_number + 1
            )));
        };
        properties.insert(key.trim().to_string(), value.trim().to_string());
    }
    Ok(properties)
}

/// Compiles the tests in the test directories.
#[cfg(not(target_family = "wasm"))]
fn compile_tests(test_cases: &[TestCase]) -> Result<()> {
    test_cases.par_iter().try_for_each(compile_test)?;
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
#[cfg(not(target_family = "wasm"))]
fn compile_test(test_case: &TestCase) -> Result<()> {
    let test_name = test_case.name();
    let source_dir = &test_case.source_dir;
    let class_dir = &test_case.class_dir;
    fs::create_dir_all(class_dir)?;

    // Check the data of the .class file to see if it is newer than the .java file and skip
    // compilation if it is.
    let class_file = class_dir.join(format!("{TEST_CLASS_NAME}.class"));
    if test_case.source_dir == test_case.class_dir && class_file.exists() {
        let java_file = source_dir.join(TEST_FILE);
        let java_file_modified = java_file.metadata()?.modified()?;
        let class_file_modified = class_file.metadata()?.modified()?;
        if class_file_modified >= java_file_modified {
            trace!("Skipping compilation for {test_name} as .class file is up to date.");
            return Ok(());
        }
    }

    let java_file = source_dir.join(TEST_FILE);
    let mut arguments = vec!["-parameters"];
    if test_case.source_dir != test_case.class_dir {
        // Cross-runtime fixtures exercise the selected JDK's class library but
        // intentionally use Java 8 bytecode. This keeps the test focused on the
        // native surface instead of newer invokedynamic bootstrap machinery.
        arguments.extend(["-source", "8", "-target", "8"]);
    }
    arguments.extend([
        "-d",
        class_dir.to_str().unwrap_or_default(),
        "-cp",
        source_dir.to_str().unwrap_or_default(),
        java_file.to_str().unwrap_or_default(),
    ]);
    let javac = test_case.java_home.join("bin").join("javac");
    let output = std::process::Command::new(javac)
        .args(&arguments)
        .current_dir(source_dir)
        .output()
        .map_err(|error| InternalError(error.to_string()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    info!("Compiling test {test_name}: {stdout}");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("Compilation failed for {test_name}: {stderr}");
        error!(message);
        return Err(InternalError(message));
    }
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
#[cfg(not(target_family = "wasm"))]
fn expected_output(test_case: &TestCase) -> Result<(Duration, String)> {
    let test_name = test_case.name();

    let start_time = Instant::now();
    let mut arguments = vec![
        "-Dstdout.encoding=UTF-8".to_string(),
        "-Dstderr.encoding=UTF-8".to_string(),
    ];
    arguments.extend(
        test_case
            .system_properties
            .iter()
            .map(|(key, value)| format!("-D{key}={value}")),
    );
    arguments.extend([
        "-cp".to_string(),
        test_case.class_dir.to_string_lossy().into_owned(),
        TEST_CLASS_NAME.to_string(),
    ]);
    let java = test_case.java_home.join("bin").join("java");
    let output = std::process::Command::new(java)
        .args(&arguments)
        .current_dir(&test_case.source_dir)
        .output()
        .map_err(|error| InternalError(error.to_string()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("Execution failed for {test_name}: {stderr}");
        error!(message);
        return Err(InternalError(message));
    }
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok((start_time.elapsed(), stdout))
}

/// Tests the VM by running the `Test` class in the specified test directory.
#[cfg(not(target_family = "wasm"))]
fn test_vm(
    java_version: &str,
    test_name: &str,
    class_dir: &Path,
    source_dir: &Path,
    system_properties: &[(String, String)],
    interpreted: bool,
    expected_duration: &Duration,
    expected_output: &str,
) -> Result<(), Failure> {
    let test_type = if interpreted { "int" } else { "jit" };
    let is_socket_test = test_name.starts_with("socket/");
    let test_timeout = if is_socket_test {
        // Socket cases normally finish in seconds. A longer wait only delays recovery from a
        // client/server scheduling deadlock.
        Duration::from_secs(30)
    } else {
        Duration::from_mins(2)
    };
    let worker_threads = if is_socket_test { 2 } else { 1 };
    let timeout_message = format!("{TEST_TIMEOUT_PREFIX}{} seconds", test_timeout.as_secs());
    let stack_size = 8 * 1024 * 1024; // 8 MB stack
    let run_once = || {
        // Spawn a thread with a larger stack to handle deeply nested async calls that occur during
        // method handle invocations and invokedynamic resolution.
        let java_version = java_version.to_string();
        let class_dir = class_dir.to_path_buf();
        let source_dir = source_dir.to_path_buf();
        let system_properties = system_properties.to_vec();
        let timeout_message = timeout_message.clone();
        std::thread::Builder::new()
            .stack_size(stack_size)
            .spawn(move || {
                std::panic::catch_unwind(|| {
                    let runtime = tokio::runtime::Builder::new_multi_thread()
                        // Rayon already runs multiple compatibility tests in parallel. A single
                        // worker avoids multiplying the runner's thread count for most VMs, while
                        // socket cases need a second worker so their Java server and client threads
                        // can make progress independently.
                        .worker_threads(worker_threads)
                        .enable_all()
                        .thread_stack_size(stack_size)
                        .build()
                        .map_err(|error| InternalError(error.to_string()))?;
                    runtime.block_on(async {
                        let stdout = Arc::new(AsyncMutex::new(Vec::new()));
                        let stderr = Arc::new(AsyncMutex::new(Vec::new()));
                        match tokio::time::timeout(
                            test_timeout,
                            run_test(
                                &java_version,
                                &class_dir,
                                &source_dir,
                                &system_properties,
                                interpreted,
                                stdout.clone(),
                                stderr.clone(),
                            ),
                        )
                        .await
                        {
                            Ok(result) => result,
                            Err(_elapsed) => {
                                let stdout = stdout.lock().await;
                                let stdout = String::from_utf8_lossy(&stdout);
                                let stderr = stderr.lock().await;
                                let stderr = String::from_utf8_lossy(&stderr);
                                Err(InternalError(format!(
                                    "{timeout_message}\nstdout: {stdout}\nstderr: {stderr}"
                                )))
                            }
                        }
                    })
                })
            })
            .expect("Failed to spawn test thread")
            .join()
            .expect("Test thread panicked")
    };

    let max_attempts = if is_socket_test { 3 } else { 1 };
    let mut result = run_once();
    for attempt in 2..=max_attempts {
        let should_retry =
            matches!(&result, Ok(Err(error)) if is_retryable_socket_timeout(test_name, error));
        if !should_retry {
            break;
        }
        warn!("Retrying ({test_type}) {test_name} after timeout ({attempt}/{max_attempts})");
        result = run_once();
    }

    let make_failure = |message: String| Failure {
        test_name: test_name.to_string(),
        test_type: test_type.to_string(),
        message,
    };

    match result {
        Ok(Ok((duration, output))) => {
            if expected_output == output {
                info!(
                    "Passed ({test_type}) {test_name} in {duration:.2?} (target: {expected_duration:.2?})"
                );
                Ok(())
            } else {
                let error_message =
                    format!("Output mismatch: expected {expected_output}, actual {output}");
                error!("Failed ({test_type}) {test_name}: {error_message}");
                Err(make_failure(error_message))
            }
        }
        Ok(Err(error)) => {
            let error_message = match error {
                ristretto_vm::Error::Throwable(_) => "Java invocation failed".to_string(),
                error => format!("{error:?}"),
            };
            error!("Failed ({test_type}) {test_name}: {error_message}");
            Err(make_failure(error_message))
        }
        Err(error) => {
            error!("Panic ({test_type}) {test_name}: {error:?}");
            Err(make_failure(format!("panic: {error:?}")))
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn is_retryable_socket_timeout(test_name: &str, error: &ristretto_vm::Error) -> bool {
    test_name.starts_with("socket/")
        && matches!(error, InternalError(message) if message.starts_with(TEST_TIMEOUT_PREFIX))
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn socket_timeout_retry_policy() {
    let timeout = InternalError(format!("{TEST_TIMEOUT_PREFIX}30 seconds"));
    assert!(is_retryable_socket_timeout("socket/tcp_streams", &timeout));
    assert!(!is_retryable_socket_timeout("module/services", &timeout));
    assert!(!is_retryable_socket_timeout(
        "socket/tcp_streams",
        &InternalError("different failure".to_string())
    ));
}

/// Runs the test by creating a VM and invoking the `Test` class.
#[cfg(not(target_family = "wasm"))]
async fn run_test(
    java_version: &str,
    class_dir: &Path,
    source_dir: &Path,
    system_properties: &[(String, String)],
    interpreted: bool,
    stdout: Arc<AsyncMutex<Vec<u8>>>,
    stderr: Arc<AsyncMutex<Vec<u8>>>,
) -> Result<(Duration, String)> {
    let start_time = Instant::now();
    let class_path = ClassPath::from(&[class_dir]);

    // Create a garbage collector configured to use only one thread
    let gc_config = GcConfigurationBuilder::new().threads(1).build();
    let garbage_collector = GarbageCollector::with_config(gc_config);
    let user_dir = {
        let s = source_dir.to_string_lossy().to_string();
        s.strip_prefix(r"\\?\").map_or(s.clone(), str::to_string)
    };
    let mut configuration_builder = ConfigurationBuilder::new()
        .class_path(class_path)
        .java_version(java_version)
        .main_class(TEST_CLASS_NAME)
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .garbage_collector(garbage_collector)
        .add_system_property("user.dir", user_dir);
    for (key, value) in system_properties {
        configuration_builder = configuration_builder.add_system_property(key, value);
    }

    configuration_builder = configuration_builder.interpreted(interpreted);
    if !interpreted {
        // Disable batch compilation for JIT tests so that we can test the JIT compilation
        configuration_builder = configuration_builder.batch_compilation(false);
    }

    let configuration = configuration_builder.build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    // A thrown Java object is only rooted by the VM while it is executing. Do not retain or
    // format that unrooted GC reference across the thread-shutdown await.
    let invocation_failed = vm.invoke_main(&parameters).await.is_err();

    // Wait for all spawned threads to complete before collecting output
    let _ = vm.wait_for_non_daemon_threads().await;

    let stdout_lock = stdout.lock().await;
    let stdout = String::from_utf8_lossy(&stdout_lock).to_string();
    let stderr_lock = stderr.lock().await;
    let stderr = String::from_utf8_lossy(&stderr_lock).to_string();
    if invocation_failed {
        return Err(InternalError(format!(
            "main invocation failed:\nstdout: {stdout}\nstderr: {stderr}"
        )));
    }
    Ok((start_time.elapsed(), stdout))
}

#[cfg(target_family = "wasm")]
#[test]
fn compatibility_tests() {
    // The compatibility suite drives `javac`/`java` via std::process::Command and
    // dispatches per-test work through rayon's thread pool. Neither subprocess
    // creation nor multi-threaded execution is supported by wasmtime/WASI today,
    // so on wasm we simply exercise the imports and exit successfully.
    let _ = (
        TEST_CLASS_NAME,
        TEST_FILE,
        IGNORE_FILE,
        &*TEST_FILTER,
        DEFAULT_JAVA_VERSION,
    );
    info!("compatibility_tests skipped on wasm: subprocess/threading unsupported");
}

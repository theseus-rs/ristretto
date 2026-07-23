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
const TEST_TIMEOUT_PREFIX: &str = "Test timed out after ";

/// A single failure observed while running the compatibility test suite.
#[derive(Debug, Clone)]
struct Failure {
    test_name: String,
    test_type: String,
    message: String,
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn compatibility_tests() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests_root_dir = cargo_manifest.join("..").join("tests").canonicalize()?;

    initialize_tracing();

    let java_version = DEFAULT_JAVA_VERSION.to_string();
    let java_home = java_home(&java_version)?;
    let test_dirs = collect_test_dirs(&tests_root_dir)?;
    compile_tests(&java_home, &tests_root_dir, &test_dirs)?;

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
        test_dirs
            .par_iter()
            .filter(|test_dir| !test_dir.starts_with("socket"))
            .for_each(|test_dir| {
                run_compatibility_test(
                    &java_version,
                    &java_home,
                    &tests_root_dir,
                    test_dir,
                    &passed,
                    &failures,
                );
            });
    });

    // Socket tests can bind shared ports and depend on tight client/server timing. Run them after
    // the parallel suite so unrelated VMs cannot starve or interfere with their network threads.
    test_dirs
        .iter()
        .filter(|test_dir| test_dir.starts_with("socket"))
        .for_each(|test_dir| {
            run_compatibility_test(
                &java_version,
                &java_home,
                &tests_root_dir,
                test_dir,
                &passed,
                &failures,
            );
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
    java_version: &str,
    java_home: &Path,
    tests_root_dir: &Path,
    test_dir: &Path,
    passed: &AtomicUsize,
    failures: &Mutex<Vec<Failure>>,
) {
    // Normalize path separators so test names use '/' on all platforms.
    let test_name = test_dir.to_string_lossy().replace('\\', "/");
    let test_dir = tests_root_dir.join(test_dir);

    if !TEST_FILTER.is_match(&test_name) {
        return;
    }

    // Determine if the test should be ignored
    let ignore_file = test_dir.join(IGNORE_FILE);
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

    match expected_output(java_home, tests_root_dir, &test_dir) {
        Ok((expected_duration, expected_output)) => {
            record(test_vm(
                java_version,
                &test_name,
                &test_dir,
                true,
                &expected_duration,
                &expected_output,
            ));
            record(test_vm(
                java_version,
                &test_name,
                &test_dir,
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

/// Compiles the tests in the test directories.
#[cfg(not(target_family = "wasm"))]
fn compile_tests(java_home: &Path, tests_root_dir: &Path, test_dirs: &[PathBuf]) -> Result<()> {
    test_dirs
        .par_iter()
        .try_for_each(|test_dir| compile_test(java_home, tests_root_dir, test_dir))?;
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
#[cfg(not(target_family = "wasm"))]
fn compile_test(java_home: &Path, tests_root_dir: &Path, test_dir: &PathBuf) -> Result<()> {
    let test_name = test_dir.to_string_lossy().to_string();
    let test_dir = tests_root_dir.join(test_dir);

    // Check the data of the .class file to see if it is newer than the .java file and skip
    // compilation if it is.
    let class_file = test_dir.join(format!("{TEST_CLASS_NAME}.class"));
    if class_file.exists() {
        let java_file = test_dir.join(TEST_FILE);
        let java_file_modified = java_file.metadata()?.modified()?;
        let class_file_modified = class_file.metadata()?.modified()?;
        if class_file_modified >= java_file_modified {
            trace!("Skipping compilation for {test_name} as .class file is up to date.");
            return Ok(());
        }
    }

    let arguments = vec![
        "-parameters",
        "-cp",
        test_dir.to_str().unwrap_or_default(),
        TEST_FILE,
    ];
    let javac = java_home.join("bin").join("javac");
    let output = std::process::Command::new(javac)
        .args(&arguments)
        .current_dir(test_dir)
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
fn expected_output(
    java_home: &Path,
    tests_root_dir: &Path,
    test_dir: &PathBuf,
) -> Result<(Duration, String)> {
    let test_name = test_dir.to_string_lossy().to_string();
    let test_dir = tests_root_dir.join(test_dir);

    let start_time = Instant::now();
    let arguments = vec![
        "-Dstdout.encoding=UTF-8",
        "-Dstderr.encoding=UTF-8",
        "-cp",
        test_dir.to_str().unwrap_or_default(),
        TEST_CLASS_NAME,
    ];
    let javac = java_home.join("bin").join("java");
    let output = std::process::Command::new(javac)
        .args(&arguments)
        .current_dir(test_dir)
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
    test_dir: &Path,
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
        let test_dir = test_dir.to_path_buf();
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
                                &test_dir,
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
    test_dir: &Path,
    interpreted: bool,
    stdout: Arc<AsyncMutex<Vec<u8>>>,
    stderr: Arc<AsyncMutex<Vec<u8>>>,
) -> Result<(Duration, String)> {
    let start_time = Instant::now();
    let class_path = ClassPath::from(&[test_dir]);

    // Create a garbage collector configured to use only one thread
    let gc_config = GcConfigurationBuilder::new().threads(1).build();
    let garbage_collector = GarbageCollector::with_config(gc_config);
    let user_dir = {
        let s = test_dir.to_string_lossy().to_string();
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

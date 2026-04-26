#![cfg_attr(target_family = "wasm", allow(unused_imports, dead_code))]

#[cfg(not(target_family = "wasm"))]
use rayon::ThreadPoolBuilder;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use regex::Regex;
use ristretto_classloader::{DEFAULT_JAVA_VERSION, runtime};
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_vm::Error::InternalError;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
#[cfg(not(target_family = "wasm"))]
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Mutex as AsyncMutex;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info, trace};
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

    let num_threads = (std::thread::available_parallelism().map_or(1, NonZero::get) / 3).max(1);
    info!("Running compatibility tests with {num_threads} threads");
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|error| InternalError(error.to_string()))?;

    pool.install(|| {
        test_dirs.par_iter().for_each(|test_dir| {
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
    let format = tracing_subscriber::fmt::format()
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
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

/// Collects directories of all tests to run.  A test is a directory that contains a
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
    // Spawn a thread with a larger stack to handle deeply nested async calls that occur during
    // method handle invocations and invokedynamic resolution
    let java_version = java_version.to_string();
    let test_name_owned = test_name.to_string();
    let test_dir = test_dir.to_path_buf();
    let test_timeout = Duration::from_mins(2);
    let stack_size = 8 * 1024 * 1024; // 8 MB stack
    let result = std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || {
            std::panic::catch_unwind(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .thread_stack_size(stack_size)
                    .build()
                    .map_err(|error| InternalError(error.to_string()))?;
                runtime.block_on(async {
                    match tokio::time::timeout(
                        test_timeout,
                        run_test(&java_version, &test_name_owned, &test_dir, interpreted),
                    )
                    .await
                    {
                        Ok(result) => result,
                        Err(_elapsed) => Err(InternalError(
                            "Test timed out after 120 seconds".to_string(),
                        )),
                    }
                })
            })
        })
        .expect("Failed to spawn test thread")
        .join()
        .expect("Test thread panicked");

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
            error!("Failed ({test_type}) {test_name}: {error:?}");
            Err(make_failure(format!("{error:?}")))
        }
        Err(error) => {
            error!("Panic ({test_type}) {test_name}: {error:?}");
            Err(make_failure(format!("panic: {error:?}")))
        }
    }
}

/// Runs the test by creating a VM and invoking the `Test` class.
#[cfg(not(target_family = "wasm"))]
async fn run_test(
    java_version: &str,
    test_name: &str,
    test_dir: &Path,
    interpreted: bool,
) -> Result<(Duration, String)> {
    let start_time = Instant::now();
    let class_path = ClassPath::from(&[test_dir]);
    let stdout = Arc::new(AsyncMutex::new(Vec::new()));
    let stderr = Arc::new(AsyncMutex::new(Vec::new()));

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

    // Module tests need full JPMS resolution. The real JVM always initializes the
    // module system fully; our lightweight fast path skips it for classpath apps.
    // Adding ALL-DEFAULT ensures full resolution is triggered for module tests.
    if test_name.starts_with("module/") {
        configuration_builder = configuration_builder.add_module("ALL-DEFAULT");
    }

    configuration_builder = configuration_builder.interpreted(interpreted);
    if !interpreted {
        // Disable batch compilation for JIT tests so that we can test the JIT compilation
        configuration_builder = configuration_builder.batch_compilation(false);
    }

    let configuration = configuration_builder.build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    let result = vm.invoke_main(&parameters).await;

    // Wait for all spawned threads to complete before collecting output
    let _ = vm.wait_for_non_daemon_threads().await;

    let stdout_lock = stdout.lock().await;
    let stdout = String::from_utf8_lossy(&stdout_lock).to_string();
    let stderr_lock = stderr.lock().await;
    let stderr = String::from_utf8_lossy(&stderr_lock).to_string();
    if let Err(error) = result {
        return Err(InternalError(format!(
            "{error:?}:\nstdout: {stdout}\nstderr: {stderr}"
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

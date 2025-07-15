use rayon::prelude::*;
use ristretto_classloader::{DEFAULT_JAVA_VERSION, runtime};
use ristretto_vm::Error::InternalError;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, fmt};

const TEST_CLASS_NAME: &str = "Test";
const TEST_FILE: &str = "Test.java";
const IGNORE_FILE: &str = "ignore.txt";

#[test]
fn compatibility_tests() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests_root_dir = cargo_manifest.join("..").join("tests");
    let tests_root_dir_string = tests_root_dir.to_string_lossy().to_string();

    initialize_tracing()?;

    let java_version = DEFAULT_JAVA_VERSION.to_string();
    let java_home = java_home(&java_version)?;
    let test_dirs = collect_test_dirs(&tests_root_dir)?;
    compile_tests(&java_home, &test_dirs)?;

    let passed = AtomicUsize::new(0);
    let failed = AtomicUsize::new(0);

    test_dirs.par_iter().for_each(|test_dir| {
        let test_dir_string = test_dir.to_string_lossy().to_string();
        let test_name = test_dir_string
            .strip_prefix(&tests_root_dir_string)
            .unwrap_or(&test_dir_string)
            .strip_prefix("/")
            .unwrap_or(&test_dir_string);

        // Determine if the test should be ignored
        let ignore_file = test_dir.join(IGNORE_FILE);
        if ignore_file.exists() {
            debug!("Ignoring test: {test_name}");
            return;
        }

        info!("Running test: {test_name}");

        if let Ok(expected_output) = expected_output(&java_home, test_dir) {
            if test_vm(&java_version, test_dir, test_name, true, &expected_output).is_ok() {
                passed.fetch_add(1, Ordering::Relaxed);
            } else {
                failed.fetch_add(1, Ordering::Relaxed);
            }
            if test_vm(&java_version, test_dir, test_name, false, &expected_output).is_ok() {
                passed.fetch_add(1, Ordering::Relaxed);
            } else {
                failed.fetch_add(1, Ordering::Relaxed);
            }
        } else {
            // If we can't get expected output, count both tests as failed
            failed.fetch_add(2, Ordering::Relaxed);
        }
    });

    let passed_count = passed.load(Ordering::Relaxed);
    let failed_count = failed.load(Ordering::Relaxed);

    info!("Tests: {}", passed_count + failed_count);
    if failed_count > 0 {
        error!("Tests failed: {failed_count}");
    } else {
        info!("All tests passed");
    }
    assert_eq!(failed_count, 0);
    Ok(())
}

/// Initializes the tracing subscriber for logging.
fn initialize_tracing() -> Result<()> {
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
    Ok(())
}

/// Collects directories of all tests to run.  A test is a directory that contains a
/// `Test.java` file.
fn collect_test_dirs(tests_root_dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut test_paths = Vec::new();
    for entry in walkdir::WalkDir::new(tests_root_dir) {
        let entry = entry.map_err(|error| InternalError(error.to_string()))?;
        if entry.file_name() == TEST_FILE {
            let test_dir = entry.path().parent().unwrap();
            test_paths.push(test_dir.to_path_buf().canonicalize()?);
        }
    }
    Ok(test_paths)
}

/// Gets the VM directory for the Java runtime.
fn java_home(java_version: &str) -> Result<PathBuf> {
    let runtime =
        tokio::runtime::Runtime::new().map_err(|error| InternalError(error.to_string()))?;
    let (java_home, _java_version, _class_loader) =
        runtime.block_on(runtime::version_class_loader(java_version))?;
    Ok(java_home)
}

/// Compiles the tests in the test directories.
fn compile_tests(java_home: &Path, test_dirs: &[PathBuf]) -> Result<()> {
    test_dirs
        .par_iter()
        .try_for_each(|test_dir| compile_test(java_home, test_dir))?;
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
fn compile_test(java_home: &Path, test_dir: &PathBuf) -> Result<()> {
    // Check the data of the .class file to see if it is newer than the .java file and skip
    // compilation if it is.
    let class_file = test_dir.join(format!("{TEST_CLASS_NAME}.class"));
    if class_file.exists() {
        let java_file = test_dir.join(TEST_FILE);
        let java_file_modified = java_file.metadata()?.modified()?;
        let class_file_modified = class_file.metadata()?.modified()?;
        if class_file_modified >= java_file_modified {
            debug!("Skipping compilation for {java_file:?} as .class file is up to date.");
            return Ok(());
        }
    }

    let arguments = vec![
        "-parameters",
        "-XDstringConcat=inline", // TODO: Remove when invokedynamic string concatenation is implemented
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
    info!("Compiling test {test_dir:?}: {stdout}");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("Compilation failed for {test_dir:?}: {stderr}");
        error!(message);
        return Err(InternalError(message));
    }
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
fn expected_output(java_home: &Path, test_dir: &PathBuf) -> Result<String> {
    let arguments = vec![
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
        let message = format!("Execution failed for {test_dir:?}: {stderr}");
        error!(message);
        return Err(InternalError(message));
    }
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

/// Tests the VM by running the `Test` class in the specified test directory.
fn test_vm(
    java_version: &str,
    test_dir: &Path,
    test_name: &str,
    interpreted: bool,
    expected_output: &str,
) -> Result<()> {
    let test_type = if interpreted { "int" } else { "jit" };
    let result = std::panic::catch_unwind(|| {
        let runtime =
            tokio::runtime::Runtime::new().map_err(|error| InternalError(error.to_string()))?;
        runtime.block_on(run_test(java_version, test_dir, interpreted))
    });
    match result {
        Ok(Ok(output)) => {
            if expected_output == output {
                info!("Passed ({test_type}) {test_name}");
                Ok(())
            } else {
                let error_message =
                    format!("Output mismatch: expected {expected_output}, actual {output}");
                error!("Failed ({test_type}) {test_name}: {error_message}");
                Err(InternalError(error_message))
            }
        }
        Ok(Err(error)) => {
            error!("Failed ({test_type}) {test_name}: {error:?}");
            Err(error)
        }
        Err(error) => {
            error!("Panic ({test_type}) {test_name}: {error:?}");
            Err(InternalError(format!("{error:?}")))
        }
    }
}

/// Runs the test by creating a VM and invoking the `Test` class.
async fn run_test(java_version: &str, test_dir: &Path, interpreted: bool) -> Result<String> {
    let class_path = ClassPath::from(test_dir.to_string_lossy());
    let stdout = Arc::new(Mutex::new(Vec::new()));
    let stderr = Arc::new(Mutex::new(Vec::new()));
    let mut configuration_builder = ConfigurationBuilder::new()
        .class_path(class_path)
        .java_version(java_version)
        .main_class(TEST_CLASS_NAME)
        .stdout(stdout.clone())
        .stderr(stderr.clone());

    if interpreted {
        configuration_builder = configuration_builder.interpreted();
    }

    let configuration = configuration_builder.build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    let result = vm.invoke_main(&parameters).await;
    let stdout_lock = stdout.lock().await;
    let stdout = String::from_utf8_lossy(&stdout_lock).to_string();
    let stderr_lock = stderr.lock().await;
    let stderr = String::from_utf8_lossy(&stderr_lock).to_string();
    if let Err(error) = result {
        return Err(InternalError(format!(
            "{error:?}:\nstdout: {stdout}\nstderr: {stderr}"
        )));
    }
    Ok(stdout)
}

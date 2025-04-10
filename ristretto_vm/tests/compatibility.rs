use ristretto_classloader::{DEFAULT_JAVA_VERSION, runtime};
use ristretto_vm::Error::InternalError;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, fmt};

const TEST_CLASS_NAME: &str = "Test";
const TEST_FILE: &str = "Test.java";

#[test]
fn compatibility_tests() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests_root_dir = cargo_manifest.join("..").join("tests");
    let tests_root_dir_string = tests_root_dir.to_string_lossy().to_string();

    initialize_tracing()?;
    cleanup_test_dirs(&tests_root_dir)?;

    let java_version = DEFAULT_JAVA_VERSION.to_string();
    let java_home = java_home(&java_version)?;
    let test_dirs = collect_test_dirs(&tests_root_dir)?;
    compile_tests(&java_home, &test_dirs)?;

    let mut passed = 0;
    let mut failed = 0;

    for test_dir in &test_dirs {
        let test_dir_string = test_dir.to_string_lossy().to_string();
        let test_name = test_dir_string
            .strip_prefix(&tests_root_dir_string)
            .unwrap_or(&test_dir_string)
            .strip_prefix("/")
            .unwrap_or(&test_dir_string);
        info!("Running test: {test_name}");
        let expected_output = expected_output(&java_home, test_dir)?;
        if test_vm(&java_version, test_dir, test_name, true, &expected_output).is_ok() {
            passed += 1;
        } else {
            failed += 1;
        }
        if test_vm(&java_version, test_dir, test_name, false, &expected_output).is_ok() {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    info!("Tests: {}", passed + failed);
    if failed > 0 {
        error!("Tests failed: {failed}");
    } else {
        info!("All tests passed");
    }
    assert_eq!(failed, 0);
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

/// Cleans up the test directories by removing all `*.class` files.
fn cleanup_test_dirs(tests_root_dir: &PathBuf) -> Result<()> {
    for entry in walkdir::WalkDir::new(tests_root_dir) {
        let entry = entry.map_err(|error| InternalError(error.to_string()))?;
        let file_name = entry.file_name();
        if entry.file_type().is_file() && file_name.to_string_lossy().ends_with(".class") {
            if let Err(error) = fs::remove_file(entry.path()) {
                error!("Unable to remove file {file_name:?}: {error:?}");
            }
        }
    }
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
            test_paths.push(test_dir.to_path_buf());
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
    for test_dir in test_dirs {
        compile_test(java_home, test_dir)?;
    }
    Ok(())
}

/// Compiles a test directory by running `javac` on the `Test.java` file.
fn compile_test(java_home: &Path, test_dir: &PathBuf) -> Result<()> {
    let arguments = vec!["-cp", test_dir.to_str().unwrap_or_default(), TEST_FILE];
    let javac = java_home.join("bin").join("javac");
    let output = std::process::Command::new(javac)
        .args(&arguments)
        .current_dir(test_dir)
        .output()
        .map_err(|error| InternalError(error.to_string()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!("Compiling {test_dir:?}: {stdout}");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Compilation failed: {stderr}");
        return Err(InternalError("Compilation failed".to_string()));
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
        error!("Compilation failed: {stderr}");
        return Err(InternalError("Compilation failed".to_string()));
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
    let result = vm.invoke_main(parameters).await;
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

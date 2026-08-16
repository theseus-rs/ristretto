#![cfg(not(target_family = "wasm"))]

use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Debug)]
struct JavacResults {
    success_code: Option<i32>,
    class_exists: bool,
    class_path_code: Option<i32>,
    class_path_class_exists: bool,
    failure_code: Option<i32>,
}

#[test]
fn javac_success_and_compilation_failure_exit_codes() {
    let results = run_javac().expect("run javac success and failure cases");
    assert_eq!(Some(0), results.success_code);
    assert!(results.class_exists);
    assert_eq!(Some(0), results.class_path_code);
    assert!(results.class_path_class_exists);
    assert_eq!(Some(1), results.failure_code);
}

fn run_javac() -> Result<JavacResults, Box<dyn Error>> {
    let temporary_directory = tempfile::tempdir()?;
    let classes = temporary_directory.path().join("classes");
    fs::create_dir_all(&classes)?;

    let valid_source = temporary_directory.path().join("Valid.java");
    fs::write(&valid_source, "public class Valid {}")?;
    let success = Command::new(env!("CARGO_BIN_EXE_javac"))
        .arg("-d")
        .arg(&classes)
        .arg(&valid_source)
        .status()?;
    let success_code = success.code();
    let class_exists = classes.join("Valid.class").is_file();

    let class_path_source = temporary_directory.path().join("UsesValid.java");
    fs::write(
        &class_path_source,
        "public class UsesValid { private Valid value; }",
    )?;
    let class_path_status = Command::new(env!("CARGO_BIN_EXE_javac"))
        .env("CLASSPATH", &classes)
        .arg("-d")
        .arg(&classes)
        .arg(&class_path_source)
        .status()?;
    let class_path_code = class_path_status.code();
    let class_path_class_exists = classes.join("UsesValid.class").is_file();

    let invalid_source = temporary_directory.path().join("Invalid.java");
    fs::write(&invalid_source, "public class Invalid { not Java }")?;
    let failure = Command::new(env!("CARGO_BIN_EXE_javac"))
        .arg(&invalid_source)
        .status()?;
    Ok(JavacResults {
        success_code,
        class_exists,
        class_path_code,
        class_path_class_exists,
        failure_code: failure.code(),
    })
}

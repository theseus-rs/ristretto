#![cfg(not(target_family = "wasm"))]

use ristretto_classfile::{ClassFile, JAVA_8};
use ristretto_classloader::runtime;
use ristretto_vm::{ClassPath, Compiler, ConfigurationBuilder, Error, JavaError, VM};
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

type TestResult = Result<(), Box<dyn StdError>>;

#[tokio::test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "regression test asserts launch errors and constructor side effects"
)]
async fn test_inherited_abstract_main() -> TestResult {
    let (java_home, _, _) = runtime::version_class_loader("25").await?;
    let compiler = Compiler::default().await?;
    let temporary_directory = tempfile::tempdir()?;

    for parameters in ["String[] args", ""] {
        let class_directory = temporary_directory.path();
        let entry_point = class_directory.join("EntryPoint.java");
        let launch_class = class_directory.join("Test.java");
        fs::write(
            &entry_point,
            format!("public interface EntryPoint {{ default void main({parameters}) {{}} }}"),
        )?;
        fs::write(
            &launch_class,
            "public class Test implements EntryPoint {
                public static boolean constructed;
                public Test() { constructed = true; }
            }",
        )?;
        compiler
            .compile(&[
                OsString::from("-d"),
                class_directory.as_os_str().to_owned(),
                entry_point.as_os_str().to_owned(),
                launch_class.into_os_string(),
            ])
            .await?;

        // Update only the interface, leaving the launch class compiled against its default.
        fs::write(
            &entry_point,
            format!("public interface EntryPoint {{ void main({parameters}); }}"),
        )?;
        compiler
            .compile(&[
                OsString::from("-d"),
                class_directory.as_os_str().to_owned(),
                entry_point.into_os_string(),
            ])
            .await?;

        let reference = Command::new(java_home.join("bin").join("java"))
            .arg("-cp")
            .arg(class_directory)
            .args(["Test", "argument"])
            .output()?;
        let reference_error = String::from_utf8_lossy(&reference.stderr);
        assert!(
            !reference.status.success() && reference_error.contains("AbstractMethodError"),
            "main({parameters}): expected AbstractMethodError, got {}: {reference_error}",
            reference.status
        );

        for interpreted in [true, false] {
            let configuration = ConfigurationBuilder::new()
                .java_home(java_home.clone())
                .class_path(ClassPath::from(&[class_directory]))
                .main_class("Test")
                .interpreted(interpreted)
                .build()?;
            let vm = VM::new(configuration).await?;
            let result = vm.invoke_main(&["argument"]).await;
            assert!(
                matches!(
                    result,
                    Err(Error::JavaError(JavaError::AbstractMethodError(_)))
                ),
                "main({parameters}), interpreted={interpreted}: expected AbstractMethodError, got {result:?}"
            );
            assert_eq!(
                vm.class("Test").await?.static_value("constructed")?,
                ristretto_vm::Value::Int(1)
            );
        }
    }
    compiler.shutdown().await?;
    Ok(())
}

async fn reject_new_main_methods(java_major_version: u16) -> TestResult {
    let (java_home, _, _) = runtime::version_class_loader(&java_major_version.to_string()).await?;
    let compiler = Compiler::default().await?;
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("tests")
        .join("method")
        .join("main");
    let temporary_directory = tempfile::tempdir()?;

    // Compact source syntax requires Java 25, so use the equivalent explicit-class fixtures.
    // Compiling to Java 8 bytecode ensures rejection is caused by the launch protocol.
    for fixture in [
        "instance",
        "instance_args",
        "static_noargs",
        "static_args",
        "inherited",
        "interface_default",
        "private_args",
    ] {
        let class_directory = temporary_directory.path().join(fixture);
        fs::create_dir_all(&class_directory)?;
        compiler
            .compile(&[
                OsString::from("-source"),
                OsString::from("8"),
                OsString::from("-target"),
                OsString::from("8"),
                OsString::from("-d"),
                class_directory.as_os_str().to_owned(),
                fixtures.join(fixture).join("Test.java").into_os_string(),
            ])
            .await
            .map_err(|error| {
                format!("Java {java_major_version}, {fixture}: compilation failed: {error}")
            })?;
        let bytes = fs::read(class_directory.join("Test.class"))?;
        assert_eq!(ClassFile::from_bytes(&bytes)?.version, JAVA_8);

        let reference = Command::new(java_home.join("bin").join("java"))
            .args(["-Duser.language=en", "-cp"])
            .arg(&class_directory)
            .arg("Test")
            .output()?;
        let reference_error = String::from_utf8_lossy(&reference.stderr);
        assert!(
            !reference.status.success() && reference_error.contains("Main method"),
            "Java {java_major_version}, {fixture}: expected a main-method rejection, got {}: {reference_error}",
            reference.status
        );

        for interpreted in [true, false] {
            let configuration = ConfigurationBuilder::new()
                .java_home(java_home.clone())
                .class_path(ClassPath::from(&[&class_directory]))
                .main_class("Test")
                .interpreted(interpreted)
                .stdout(Arc::new(Mutex::new(Vec::new())))
                .stderr(Arc::new(Mutex::new(Vec::new())))
                .build()?;
            let vm = VM::new(configuration).await?;
            assert_eq!(vm.java_major_version(), java_major_version);
            let result = vm.invoke_main(&["argument"]).await;
            assert!(
                matches!(
                    &result,
                    Err(Error::InternalError(message)) if message == "No main method found for Test"
                ),
                "Java {java_major_version}, {fixture}, interpreted={interpreted}: expected main-method rejection, got {result:?}"
            );
        }
    }
    compiler.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_new_main_methods_rejected_v8() -> TestResult {
    reject_new_main_methods(8).await
}

#[tokio::test]
async fn test_new_main_methods_rejected_v11() -> TestResult {
    reject_new_main_methods(11).await
}

#[tokio::test]
async fn test_new_main_methods_rejected_v17() -> TestResult {
    reject_new_main_methods(17).await
}

#[tokio::test]
async fn test_new_main_methods_rejected_v21() -> TestResult {
    reject_new_main_methods(21).await
}

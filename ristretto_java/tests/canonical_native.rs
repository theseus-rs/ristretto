#![cfg(all(feature = "experimental-canonical-native", not(target_family = "wasm")))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "finite CLI integration assertions"
)]

use ristretto_classloader::{JAVA_25_VERSION, runtime};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

type TestResult = Result<(), Box<dyn Error>>;

async fn fixture() -> Result<(tempfile::TempDir, std::path::PathBuf), Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    let (jdk, _, _) = runtime::version_class_loader(JAVA_25_VERSION).await?;
    for (name, source) in [
        (
            "Caller.java",
            include_str!("fixtures/canonical_native/bridge/Caller.java"),
        ),
        (
            "Target.java",
            include_str!("fixtures/canonical_native/bridge/Target.java"),
        ),
        (
            "StatefulTarget.java",
            include_str!("fixtures/canonical_native/bridge/StatefulTarget.java"),
        ),
    ] {
        fs::write(directory.path().join(name), source)?;
    }
    let output = Command::new(jdk.join("bin/javac"))
        .args(["--release", "8", "-Xlint:-options", "-d"])
        .arg(directory.path())
        .arg(directory.path().join("Caller.java"))
        .arg(directory.path().join("Target.java"))
        .arg(directory.path().join("StatefulTarget.java"))
        .output()?;
    assert!(
        output.status.success(),
        "javac: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok((directory, jdk))
}

fn launch(classes: &Path, jdk: &Path, options: &[&str]) -> Result<Output, Box<dyn Error>> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_java"));
    for variable in [
        "JAVA_HOME",
        "CLASSPATH",
        "RISTRETTO_LOG",
        "JDK_JAVA_OPTIONS",
        "JAVA_TOOL_OPTIONS",
        "_JAVA_OPTIONS",
    ] {
        command.env_remove(variable);
    }
    command
        .env("JAVA_VERSION", JAVA_25_VERSION)
        .env("RISTRETTO_JDK_HOME", jdk)
        .env("RISTRETTO_JDK_25_HOME", jdk)
        .arg("-Xint")
        .arg("-cp")
        .arg(classes)
        .args(options)
        .args([
            "bridge.Caller",
            "hello",
            "--canonical-native=application-argument",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let started = Instant::now();
    let mut child = command.spawn()?;
    while child.try_wait()?.is_none() {
        if started.elapsed() > Duration::from_secs(60) {
            child.kill()?;
            let output = child.wait_with_output()?;
            return Err(
                format!("CLI watchdog: {}", String::from_utf8_lossy(&output.stderr)).into(),
            );
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(child.wait_with_output()?)
}

#[tokio::test]
async fn ordinary_main_calls_explicit_canonical_native_and_keeps_its_arguments() -> TestResult {
    let (directory, jdk) = fixture().await?;
    for options in [
        [
            "--canonical-native",
            "bridge.Caller.enter=bridge.Target.compute",
            "--canonical-class",
            "bridge.Target$Node",
        ],
        [
            "--canonical-native",
            "bridge/Caller.enter=bridge/Target.compute",
            "--canonical-class",
            "bridge/Target$Node",
        ],
    ] {
        let output = launch(directory.path(), &jdk, &options)?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
            "ordinary-before:hello\nordinary-caught:NegativeArraySizeException\nordinary-after:25,37\n"
        );
    }
    Ok(())
}

#[tokio::test]
async fn invalid_binding_is_rejected_before_ordinary_main() -> TestResult {
    let (directory, jdk) = fixture().await?;
    for binding in [
        "bridge.Caller.ordinary=bridge.Target.compute",
        "bridge.Caller.instance=bridge.Target.compute",
        "bridge.Caller.wrong=bridge.Target.compute",
        "bridge.Caller.enter=bridge.Target.instance",
        "bridge.Caller.enter=bridge.Target.missing",
        "bridge.Caller.enter=bridge.StatefulTarget.compute",
    ] {
        let output = launch(
            directory.path(),
            &jdk,
            &[
                "--canonical-native",
                binding,
                "--canonical-class",
                "bridge.Target$Node",
            ],
        )?;
        assert!(!output.status.success(), "accepted {binding}");
        assert!(!String::from_utf8_lossy(&output.stdout).contains("ordinary-before"));
        assert!(
            !output.stderr.is_empty(),
            "missing binding diagnostic for {binding}"
        );
    }
    let output = launch(
        directory.path(),
        &jdk,
        &[
            "--canonical-native",
            "bridge.Caller.enter=bridge.Target.compute",
        ],
    )?;
    assert!(
        !output.status.success(),
        "unadmitted Node companion accepted"
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains("ordinary-before"));
    Ok(())
}

#[tokio::test]
async fn feature_without_explicit_binding_preserves_ordinary_native_resolution() -> TestResult {
    let (directory, jdk) = fixture().await?;
    let output = launch(directory.path(), &jdk, &[])?;
    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
        "ordinary-before:hello\n"
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("UnsatisfiedLinkError"));
    Ok(())
}

#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration tests assert CLI behavior in Result-returning tests"
)]

use ristretto_classfile::{JAVA_11, JAVA_24, JAVA_25};
use ristretto_classloader::{
    JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION, JAVA_21_VERSION, JAVA_25_VERSION, runtime,
};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

type TestResult = Result<(), Box<dyn Error>>;

fn command(executable: &Path, directory: &Path, version: &str) -> Command {
    let mut command = Command::new(executable);
    command.current_dir(directory).env("JAVA_VERSION", version);
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
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n")
}

async fn compare_launch(
    version: &str,
    source: &str,
    options: &[&str],
    arguments: &[&str],
    expected: &str,
) -> TestResult {
    let directory = tempfile::tempdir()?;
    fs::write(directory.path().join("HelloWorld.java"), source)?;
    let (java_home, _, _) = runtime::version_class_loader(version).await?;
    for executable in [
        java_home.join("bin/java"),
        env!("CARGO_BIN_EXE_java").into(),
    ] {
        let output = command(&executable, directory.path(), version)
            .args(options)
            .arg("HelloWorld.java")
            .args(arguments)
            .output()?;
        assert!(
            output.status.success(),
            "{}: {}",
            executable.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(expected, stdout(&output), "{}", executable.display());
        assert_eq!(
            1,
            fs::read_dir(directory.path())?.count(),
            "source launch must not emit class files"
        );
    }
    Ok(())
}

#[tokio::test]
async fn compact_source_noargs_java25() -> TestResult {
    let source = r#"
String greeting = "Hello, World!";
void main() {
    IO.println(greeting);
    IO.println(List.of(1, 2, 3).size());
}
"#;
    compare_launch(JAVA_25_VERSION, source, &[], &[], "Hello, World!\n3\n").await
}

#[tokio::test]
async fn compact_source_arguments_java25() -> TestResult {
    let source = r#"
void main(String[] args) {
    IO.println(String.join("|", args));
}
static void main() {
    throw new AssertionError("main(String[]) must take precedence");
}
"#;
    compare_launch(
        JAVA_25_VERSION,
        source,
        &["-Xint", "--source", &JAVA_25.java().to_string()],
        &["--version", "first", "two words", "-version", "-cp", "last"],
        "--version|first|two words|-version|-cp|last\n",
    )
    .await
}

#[tokio::test]
async fn compact_source_preserves_unicode_arguments_java25() -> TestResult {
    // OpenJDK's Windows launcher converts arguments through the ANSI code page
    // (JDK-8337506), so verify Ristretto's Unicode handling directly on every OS.
    // Assert inside Java and print ASCII to avoid platform stdout encodings.
    let directory = tempfile::tempdir()?;
    fs::write(
        directory.path().join("HelloWorld.java"),
        r#"
void main(String[] args) {
    if (args.length != 2 || !args[0].equals("世界") || !args[1].equals("café ☕ 🦀")) {
        throw new AssertionError("Unicode arguments changed");
    }
    IO.println("Unicode arguments preserved");
}
"#,
    )?;
    let output = command(
        Path::new(env!("CARGO_BIN_EXE_java")),
        directory.path(),
        JAVA_25_VERSION,
    )
    .args(["HelloWorld.java", "世界", "café ☕ 🦀"])
    .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!("Unicode arguments preserved\n", stdout(&output));
    Ok(())
}

#[tokio::test]
async fn compact_source_rejected_on_older_java() -> TestResult {
    let directory = tempfile::tempdir()?;
    // Avoid newer APIs so the rejection is specifically about compact-source syntax.
    fs::write(
        directory.path().join("HelloWorld.java"),
        "void main() { System.out.println(\"must not run\"); }",
    )?;
    for version in [
        JAVA_8_VERSION,
        JAVA_11_VERSION,
        JAVA_17_VERSION,
        JAVA_21_VERSION,
    ] {
        let (java_home, _, _) = runtime::version_class_loader(version).await?;
        for executable in [
            java_home.join("bin/java"),
            env!("CARGO_BIN_EXE_java").into(),
        ] {
            let output = command(&executable, directory.path(), version)
                .arg("HelloWorld.java")
                .output()?;
            let error = String::from_utf8_lossy(&output.stderr);
            assert!(
                !output.status.success(),
                "Java {version}: {} unexpectedly succeeded",
                executable.display()
            );
            assert!(!stdout(&output).contains("must not run"));
            assert!(!error.is_empty(), "Java {version}: missing diagnostic");
            if executable == Path::new(env!("CARGO_BIN_EXE_java")) {
                let expected = match version {
                    JAVA_8_VERSION => {
                        format!("Source-file launch requires Java {}", JAVA_11.java())
                    }
                    JAVA_21_VERSION => "preview".to_string(),
                    _ => "class, interface".to_string(),
                };
                assert!(error.contains(&expected), "Java {version}: {error}");
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn classic_source_still_launches_on_java11() -> TestResult {
    compare_launch(JAVA_11_VERSION,
        "package example; class Entry { public static void main(String[] args) { System.out.println(args[0]); } }",
        &[], &["classic"], "classic\n",
    ).await
}

#[tokio::test]
async fn source_launch_preserves_multiple_export_targets() -> TestResult {
    compare_launch(
        JAVA_25_VERSION,
        "import jdk.internal.misc.Unsafe; void main() { IO.println(Unsafe.class.getName()); }",
        &[
            "--add-exports",
            "java.base/jdk.internal.misc=java.compiler,ALL-UNNAMED",
        ],
        &[],
        "jdk.internal.misc.Unsafe\n",
    )
    .await
}

#[tokio::test]
async fn source_release_rejects_compact_syntax() -> TestResult {
    let source_version = JAVA_24.java().to_string();
    let directory = tempfile::tempdir()?;
    fs::write(
        directory.path().join("HelloWorld.java"),
        "void main() { System.out.println(\"must not run\"); }",
    )?;
    let output = command(
        Path::new(env!("CARGO_BIN_EXE_java")),
        directory.path(),
        JAVA_25_VERSION,
    )
    .args(["--source", &source_version, "HelloWorld.java"])
    .output()?;
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains(&source_version) && error.contains("error"),
        "{error}"
    );
    assert_eq!(output.stdout, [] as [u8; 0]);
    assert!(!directory.path().join("HelloWorld.class").exists());
    Ok(())
}

#[tokio::test]
async fn source_application_exception_is_reported() -> TestResult {
    let directory = tempfile::tempdir()?;
    fs::write(
        directory.path().join("HelloWorld.java"),
        "void main() { throw new IllegalStateException(\"source failure\"); }",
    )?;
    let output = command(
        Path::new(env!("CARGO_BIN_EXE_java")),
        directory.path(),
        JAVA_25_VERSION,
    )
    .arg("HelloWorld.java")
    .output()?;
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("IllegalStateException: source failure"),
        "{error}"
    );
    assert!(!error.contains("InvocationTargetException"), "{error}");
    Ok(())
}

#[tokio::test]
async fn source_compilation_and_execution_use_classpath() -> TestResult {
    let directory = tempfile::tempdir()?;
    let libraries = directory.path().join("library classes");
    fs::create_dir(&libraries)?;
    fs::write(
        libraries.join("Helper.java"),
        "package library; public class Helper { public static String greeting() { return \"from classpath\"; } }",
    )?;
    let (java_home, _, _) = runtime::version_class_loader(JAVA_25_VERSION).await?;
    let compiled = command(
        &java_home.join("bin/javac"),
        directory.path(),
        JAVA_25_VERSION,
    )
    .arg("-d")
    .arg(&libraries)
    .arg(libraries.join("Helper.java"))
    .output()?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    fs::write(
        directory.path().join("HelloWorld.java"),
        "void main() { IO.println(library.Helper.greeting()); }",
    )?;
    for executable in [
        java_home.join("bin/java"),
        env!("CARGO_BIN_EXE_java").into(),
    ] {
        let output = command(&executable, directory.path(), JAVA_25_VERSION)
            .arg("-cp")
            .arg(&libraries)
            .arg("HelloWorld.java")
            .output()?;
        assert!(
            output.status.success(),
            "{}: {}",
            executable.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!("from classpath\n", stdout(&output));
        assert!(!directory.path().join("HelloWorld.class").exists());
    }
    Ok(())
}

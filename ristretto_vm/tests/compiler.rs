#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration tests use assertions in Result-returning async tests"
)]

use ristretto_vm::{
    ClassPath, ClassPathEntry, CompiledClasses, Compiler, CompilerError, ConfigurationBuilder,
    JavaSource, Memory, VM,
};
use std::error::Error;
use std::ffi::OsString;
use std::fs;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[tokio::test(flavor = "current_thread")]
async fn test_compiler() -> TestResult {
    let temporary_directory = tempfile::tempdir()?;
    let source_directory = temporary_directory.path().join("source");
    let class_directory = temporary_directory.path().join("classes");
    fs::create_dir_all(&source_directory)?;
    fs::create_dir_all(&class_directory)?;

    let valid_source = source_directory.join("Compiled.java");
    fs::write(
        &valid_source,
        "public class Compiled { public static int value() { return 42; } }",
    )?;
    let invalid_source = source_directory.join("Invalid.java");
    fs::write(&invalid_source, "public class Invalid { this is not Java }")?;

    let compiler = Compiler::default().await?;
    compiler
        .compile(&[
            OsString::from("-d"),
            class_directory.as_os_str().to_owned(),
            valid_source.into_os_string(),
        ])
        .await?;
    assert!(class_directory.join("Compiled.class").is_file());

    let invalid_result = compiler.compile(&[invalid_source.into_os_string()]).await;
    assert!(matches!(
        invalid_result,
        Err(CompilerError::CompilationFailed)
    ));

    let invalid_option = compiler.compile(&["--not-a-real-javac-option"]).await;
    assert!(matches!(
        invalid_option,
        Err(CompilerError::InvalidArguments)
    ));

    test_generated_classes(&compiler).await?;
    test_mutually_dependent_sources(&compiler).await?;
    test_in_memory_errors(&compiler, temporary_directory.path()).await?;
    test_java8_bridge().await?;
    Ok(())
}

async fn test_generated_classes(compiler: &Compiler) -> TestResult {
    let source = r"
        public class Generated {
            public static class Nested {}

            public static Runnable runnable() {
                return new Runnable() {
                    public void run() {}
                };
            }
        }
    ";
    let classes = compiler.compile_source("Generated", source).await?;

    assert_eq!(3, classes.len());
    assert!(classes.get("Generated").is_some());
    assert!(classes.get("Generated$Nested").is_some());
    assert!(classes.get("Generated$1").is_some());
    assert_valid_classes(&classes)?;
    Ok(())
}

async fn test_mutually_dependent_sources(compiler: &Compiler) -> TestResult {
    let sources = [
        JavaSource::new(
            "example.Main",
            r"
                package example;
                public class Main {
                    public static int value() { return Helper.answer(); }
                }
            ",
        ),
        JavaSource::new(
            "example.Helper",
            r"
                package example;
                public class Helper {
                    public static int answer() { return 42; }
                }
            ",
        ),
    ];
    let classes = compiler.compile_sources(&sources, &[] as &[&str]).await?;
    assert_eq!(2, classes.len());

    let memory = Memory::new("compiled-test");
    classes.load_into(&memory).await?;
    let class_path = ClassPath::new(vec![ClassPathEntry::Memory(memory)]);
    let configuration = ConfigurationBuilder::new().class_path(class_path).build()?;
    let vm = VM::new(configuration).await?;
    let value = vm
        .try_invoke("example.Main", "value()I", &[] as &[i32])
        .await?;
    assert_eq!(42, value.as_i32()?);
    Ok(())
}

async fn test_in_memory_errors(
    compiler: &Compiler,
    temporary_directory: &std::path::Path,
) -> TestResult {
    let invalid_source = compiler
        .compile_source("Broken", "public class Broken { not Java }")
        .await;
    assert!(matches!(
        invalid_source,
        Err(CompilerError::CompilationFailed)
    ));

    let output_directory = temporary_directory.join("memory-output");
    fs::create_dir_all(&output_directory)?;
    let options = [
        OsString::from("-d"),
        output_directory.as_os_str().to_owned(),
    ];
    let output_result = compiler
        .compile_source_with_options("Output", "public class Output {}", &options)
        .await;
    assert!(matches!(
        output_result,
        Err(CompilerError::InvalidArguments)
    ));
    assert!(fs::read_dir(output_directory)?.next().is_none());
    Ok(())
}

async fn test_java8_bridge() -> TestResult {
    let configuration = ConfigurationBuilder::new().java_version("8").build()?;
    let compiler = Compiler::new(configuration).await?;
    let classes = compiler
        .compile_source("Java8Source", "public class Java8Source {}")
        .await?;
    let bytecode = classes
        .get("Java8Source")
        .ok_or("Java 8 compiler returned no class")?;
    let class_file = ristretto_classfile::ClassFile::from_bytes(bytecode)?;
    assert_eq!(ristretto_classfile::JAVA_8, class_file.version);
    Ok(())
}

fn assert_valid_classes(classes: &CompiledClasses) -> TestResult {
    for (name, bytecode) in classes.iter() {
        let class_file = ristretto_classfile::ClassFile::from_bytes(bytecode)?;
        let expected = name.replace('.', "/");
        let actual = class_file.class_name()?.to_rust_string();
        if expected != actual {
            return Err(format!("expected class {expected}, found {actual}").into());
        }
    }
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn test_compiler_lts_versions() -> TestResult {
    for version in [8_u16, 11, 17, 21, 25] {
        eprintln!("Testing Java {version} compiler and runtime");
        let configuration = ConfigurationBuilder::new()
            .java_version(version.to_string())
            .interpreted(true)
            .build()?;
        let compiler = Compiler::new(configuration).await?;
        let classes = compiler
            .compile_source(
                "Versioned",
                r#"
            public class Versioned {
                public static String value(int n) { return "Java " + n + " ☕"; }
            }
        "#,
            )
            .await?;
        let bytes = classes
            .get("Versioned")
            .ok_or("compiler returned no class")?;
        assert_eq!(
            version,
            ristretto_classfile::ClassFile::from_bytes(bytes)?
                .version
                .java()
        );
        let memory = Memory::new("versioned");
        classes.load_into(&memory).await?;
        let configuration = ConfigurationBuilder::new()
            .java_version(version.to_string())
            .interpreted(true)
            .class_path(ClassPath::new(vec![ClassPathEntry::Memory(memory)]))
            .build()?;
        let vm = VM::new(configuration).await?;
        let value = vm
            .try_invoke(
                "Versioned",
                "value(I)Ljava/lang/String;",
                &[i32::from(version)],
            )
            .await?;
        assert_eq!(format!("Java {version} ☕"), value.as_string()?);
    }
    Ok(())
}

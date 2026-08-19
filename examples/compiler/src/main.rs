//! Compile and run a Java program using Ristretto's Rust APIs.

#![forbid(unsafe_code)]

use ristretto_vm::{ClassPath, Compiler, ConfigurationBuilder, VM, Value};
use std::ffi::OsString;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    common_main().await
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    common_main().await
}

async fn common_main() -> Result<()> {
    let _result = compile_and_run(ConfigurationBuilder::new()).await?;
    Ok(())
}

async fn compile_and_run(configuration_builder: ConfigurationBuilder) -> Result<Option<Value>> {
    let example_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source_file = example_directory.join("HelloWorld.java");
    let classes_directory = tempfile::tempdir()?;

    let compiler = Compiler::default().await?;
    let arguments = vec![
        OsString::from("-d"),
        classes_directory.path().as_os_str().to_owned(),
        source_file.as_os_str().to_owned(),
    ];
    compiler.compile(&arguments).await?;

    let class_path = ClassPath::from(&[classes_directory.path()]);
    let configuration = configuration_builder
        .class_path(class_path)
        .main_class("HelloWorld")
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = Vec::<&str>::new();
    Ok(vm.invoke_main(&parameters).await?)
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {
    use super::*;
    use std::io::Cursor;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test(flavor = "current_thread")]
    async fn test_compile_and_run() {
        let stdout = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
        let configuration_builder = ConfigurationBuilder::new().stdout(stdout.clone());

        let result = match compile_and_run(configuration_builder).await {
            Ok(result) => result,
            Err(error) => panic!("failed to compile and run example: {error}"),
        };

        assert!(result.is_none());
        let stdout = stdout.lock().await;
        let output = String::from_utf8_lossy(stdout.get_ref());
        assert_eq!("Hello from Ristretto!", output.trim());
    }
}

#![forbid(unsafe_code)]
#![allow(clippy::result_large_err)]

use ristretto_vm::{ClassPath, Configuration, ConfigurationBuilder, Result, VM, Value};
use std::path::PathBuf;

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

/// Creates a simple embedded JVM that executes a Java class named `HelloWorld`.
async fn common_main() -> Result<()> {
    let configuration = configuration_builder().build()?;
    let _result = invoke_main(configuration).await?;
    Ok(())
}

fn configuration_builder() -> ConfigurationBuilder {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let class_path = ClassPath::from(&[cargo_manifest_dir]);
    ConfigurationBuilder::new()
        .class_path(class_path)
        .main_class("HelloWorld")
}

async fn invoke_main(configuration: Configuration) -> Result<Option<Value>> {
    let vm = VM::new(configuration).await?;
    let parameters = Vec::<&str>::new();
    vm.invoke_main(&parameters).await
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test(flavor = "current_thread")]
    async fn test_invoke_main() -> Result<()> {
        let stdout = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
        let configuration = configuration_builder().stdout(stdout.clone()).build()?;

        let result = invoke_main(configuration).await?;

        assert!(result.is_none());
        let stdout = stdout.lock().await;
        let output = String::from_utf8_lossy(stdout.get_ref());
        assert_eq!("Hello, World!", output.trim());
        Ok(())
    }
}

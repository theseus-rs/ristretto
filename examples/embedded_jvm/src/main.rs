#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(clippy::result_large_err)]

use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
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
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let class_path = ClassPath::from(&[cargo_manifest_dir]);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = Vec::<&str>::new();
    let _result = vm.invoke_main(&parameters).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

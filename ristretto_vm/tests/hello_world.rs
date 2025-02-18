use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::path::PathBuf;

async fn test_helloworld(java_verison: &str) -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .java_version(java_verison)
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters: Vec<&str> = Vec::new();
    let result = vm.invoke_main(parameters).await?;
    assert!(result.is_none());
    Ok(())
}

// #[tokio::test]
// async fn test_helloworld_v8() -> Result<()> {
//     test_helloworld("8.442.06.1").await
// }

#[tokio::test]
async fn test_helloworld_v11() -> Result<()> {
    test_helloworld("11.0.26.4.1").await
}

#[tokio::test]
async fn test_helloworld_v17() -> Result<()> {
    test_helloworld("17.0.14.7.1").await
}

#[tokio::test]
async fn test_helloworld_v21() -> Result<()> {
    test_helloworld("21.0.6.7.1").await
}

#[tokio::test]
async fn test_helloworld_v24() -> Result<()> {
    test_helloworld("24.0.0.36.1").await
}

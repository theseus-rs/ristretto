use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::path::PathBuf;

async fn test_helloworld(java_verison: &str) -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest.join("../classes/classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .java_version(java_verison)
        .build()?;
    let vm = VM::new(configuration).await?;
    let arguments: Vec<&str> = Vec::new();
    let result = vm.invoke_main(arguments).await?;
    assert!(result.is_none());
    Ok(())
}

// #[tokio::test]
// async fn test_helloworld_v8() -> Result<()> {
//     test_helloworld("8.432.06.1").await
// }
//
// #[tokio::test]
// async fn test_helloworld_v11() -> Result<()> {
//     test_helloworld("11.0.25.9.1").await
// }

#[tokio::test]
async fn test_helloworld_v17() -> Result<()> {
    test_helloworld("17.0.12.7.1").await
}

#[tokio::test]
async fn test_helloworld_v18() -> Result<()> {
    test_helloworld("18.0.2").await
}

#[tokio::test]
async fn test_helloworld_v19() -> Result<()> {
    test_helloworld("19.0.2.7.1").await
}

#[tokio::test]
async fn test_helloworld_v20() -> Result<()> {
    test_helloworld("20.0.2.10.1").await
}

#[tokio::test]
async fn test_helloworld_v21() -> Result<()> {
    test_helloworld("21.0.5.11.1").await
}

#[tokio::test]
async fn test_helloworld_v22() -> Result<()> {
    test_helloworld("22.0.2.9.1").await
}

// #[tokio::test]
// async fn test_helloworld_v23() -> Result<()> {
//     test_helloworld("23.0.1.8.1").await
// }

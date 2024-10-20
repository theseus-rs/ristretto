use ristretto_classloader::{runtime, Result};

async fn test_runtime(version: &str, class_name: &str) -> Result<()> {
    let (_java_home, java_version, class_loader) = runtime::version_class_loader(version).await?;
    assert!(java_version.starts_with(version));
    let class = class_loader.load(class_name).await?;
    assert_eq!(class_name, class.name());
    Ok(())
}

#[tokio::test]
async fn test_get_runtime_v8() -> Result<()> {
    test_runtime("8.432.06.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v11() -> Result<()> {
    test_runtime("11.0.25.9.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v17() -> Result<()> {
    test_runtime("17.0.12.7.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v18() -> Result<()> {
    test_runtime("18.0.2", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v19() -> Result<()> {
    test_runtime("19.0.2.7.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v20() -> Result<()> {
    test_runtime("20.0.2.10.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v21() -> Result<()> {
    test_runtime("21.0.5.11.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v22() -> Result<()> {
    test_runtime("22.0.2.9.1", "java/lang/Object").await
}

#[tokio::test]
async fn test_get_runtime_v23() -> Result<()> {
    test_runtime("23.0.1.8.1", "java/lang/Object").await
}

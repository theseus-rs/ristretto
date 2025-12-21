use ristretto_classloader::{Result, runtime};

async fn test_runtime(version: &str, class_name: &str) -> Result<()> {
    let (_java_home, java_version, class_loader) = runtime::version_class_loader(version).await?;
    let major_version = version.split('.').next().expect("major version");
    assert!(java_version.starts_with(major_version));

    let class = class_loader.load(class_name).await?;
    let class_name = class_name.replace('.', "/");
    assert_eq!(class_name, class.name());

    let class_path = class_loader.class_path();
    let class_names = class_path.class_names().await?;
    assert!(class_names.contains(&class_name.clone()));
    Ok(())
}

#[tokio::test]
async fn test_get_runtime_v8() -> Result<()> {
    test_runtime("8.472.08.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v11() -> Result<()> {
    test_runtime("11.0.29.7.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v17() -> Result<()> {
    test_runtime("17.0.17.10.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v21() -> Result<()> {
    test_runtime("21.0.9.10.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v25() -> Result<()> {
    test_runtime("25.0.1.8.1", "java.lang.Object").await
}

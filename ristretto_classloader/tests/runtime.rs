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
    assert!(class_names.contains(&class_name.to_string()));
    Ok(())
}

#[tokio::test]
async fn test_get_runtime_v8() -> Result<()> {
    test_runtime("8.452.09.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v11() -> Result<()> {
    test_runtime("11.0.27.6.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v17() -> Result<()> {
    test_runtime("17.0.15.6.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v21() -> Result<()> {
    test_runtime("21.0.7.6.1", "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v24() -> Result<()> {
    test_runtime("24.0.1.9.1", "java.lang.Object").await
}

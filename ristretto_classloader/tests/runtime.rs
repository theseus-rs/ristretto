use ristretto_classloader::{
    JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION, JAVA_21_VERSION, JAVA_25_VERSION, Result,
    runtime,
};

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
    test_runtime(JAVA_8_VERSION, "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v11() -> Result<()> {
    test_runtime(JAVA_11_VERSION, "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v17() -> Result<()> {
    test_runtime(JAVA_17_VERSION, "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v21() -> Result<()> {
    test_runtime(JAVA_21_VERSION, "java.lang.Object").await
}

#[tokio::test]
async fn test_get_runtime_v25() -> Result<()> {
    test_runtime(JAVA_25_VERSION, "java.lang.Object").await
}

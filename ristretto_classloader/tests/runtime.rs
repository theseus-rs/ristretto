use ristretto_classloader::{runtime, Result};

async fn test_runtime(version: &str, class: &str) -> Result<()> {
    let (runtime_version, class_loader) = runtime::class_loader(version).await?;
    assert!(runtime_version.starts_with(version));
    let class_name = class.replace('.', "/");
    let class = class_loader.load(class).await?;
    let class_file = class.class_file();
    assert_eq!(&class_name, class_file.class_name()?);
    Ok(())
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v8() -> Result<()> {
    test_runtime("8.422.05.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v11() -> Result<()> {
    test_runtime("11.0.24.8.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v17() -> Result<()> {
    test_runtime("17.0.12.7.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v18() -> Result<()> {
    test_runtime("18.0.2.9.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v19() -> Result<()> {
    test_runtime("19.0.2.7.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v20() -> Result<()> {
    test_runtime("20.0.2.10.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v21() -> Result<()> {
    test_runtime("21.0.4.7.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v22() -> Result<()> {
    test_runtime("22.0.2.9.1", "java.lang.Object").await
}

#[test_log::test(tokio::test)]
async fn test_get_runtime_v23() -> Result<()> {
    test_runtime("23.0.0.35.1", "java.lang.Object").await
}

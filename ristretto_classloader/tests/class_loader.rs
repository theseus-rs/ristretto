//! Tests for the Ristretto `ClassLoader`

use ristretto_classloader::{ClassLoader, ClassPath, Result};
use std::path::PathBuf;

#[tokio::test]
async fn test_load_class_from_class_path_directory() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest.join("..").join("classes");
    let class_loader = ClassLoader::new("directory-test", ClassPath::from(&[classes_directory]));
    let class = class_loader.load("HelloWorld").await?;
    assert_eq!("HelloWorld", class.name());
    Ok(())
}

#[tokio::test]
async fn test_load_class_from_class_path_jar() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(&[classes_directory]);
    let class_loader = ClassLoader::new("jar-test", class_path);
    let class = class_loader.load("HelloWorld").await?;
    assert_eq!("HelloWorld", class.name());
    Ok(())
}

#[cfg(feature = "url")]
#[tokio::test]
async fn test_load_class_from_class_path_url() -> Result<()> {
    let class_path_url = "https//repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
    let class_path = ClassPath::from(&[class_path_url]);
    let class_loader = ClassLoader::new("url-test", class_path);
    let class = class_loader
        .load("org.springframework.boot.SpringApplication")
        .await?;
    assert_eq!("org/springframework/boot/SpringApplication", class.name());
    Ok(())
}

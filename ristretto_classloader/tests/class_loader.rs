use ristretto_classloader::Result;
use ristretto_classloader::{ClassLoader, ClassPath};
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::test]
async fn test_load_class_from_class_path_directory() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest.join("../classes");
    let class_path = classes_directory.to_string_lossy();
    let class_loader = Arc::new(ClassLoader::new(
        "directory-test",
        ClassPath::from(&class_path),
    ));
    let class = ClassLoader::load_class(&class_loader, "HelloWorld").await?;
    let class_file = class.get_class_file();
    assert_eq!("HelloWorld", class_file.class_name()?);
    Ok(())
}

#[tokio::test]
async fn test_load_class_from_class_path_jar() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest.join("../classes/classes.jar");
    let class_path = classes_directory.to_string_lossy();
    let class_loader = Arc::new(ClassLoader::new("jar-test", ClassPath::from(&class_path)));
    let class = ClassLoader::load_class(&class_loader, "HelloWorld").await?;
    let class_file = class.get_class_file();
    assert_eq!("HelloWorld", class_file.class_name()?);
    Ok(())
}

#[cfg(feature = "url")]
#[tokio::test]
async fn test_load_class_from_class_path_url() -> Result<()> {
    let class_path = "https//repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
    let class_loader = Arc::new(ClassLoader::new("url-test", ClassPath::from(class_path)));
    let class =
        ClassLoader::load_class(&class_loader, "org.springframework.boot.SpringApplication")
            .await?;
    let class_file = class.get_class_file();
    assert_eq!(
        "org/springframework/boot/SpringApplication",
        class_file.class_name()?
    );
    Ok(())
}

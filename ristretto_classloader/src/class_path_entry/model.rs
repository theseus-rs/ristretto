use crate::Result;
use crate::class_path_entry::directory::Directory;
use crate::class_path_entry::image::Image;
use crate::class_path_entry::jar::Jar;
use ristretto_classfile::ClassFile;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::PathBuf;

/// Represents a class path entry.
#[derive(Clone, Debug, PartialEq)]
pub enum ClassPathEntry {
    Directory(Directory),
    Image(Image),
    Jar(Jar),
}

/// Implementation for `ClassPathEntry`.
impl ClassPathEntry {
    /// Create a new class path entry.
    pub fn new<S: AsRef<OsStr>>(path: S) -> Self {
        let path = path.as_ref();

        #[cfg(feature = "url")]
        {
            let path_str = path.to_string_lossy();
            if path_str.starts_with("https://") || path_str.starts_with("http://") {
                return ClassPathEntry::Jar(Jar::from_url(path_str));
            }
        }

        let path_buf = PathBuf::from(path);
        if path_buf.is_file() {
            if let Some(file_name) = path_buf.file_name()
                && file_name == "modules"
                && let Ok(image) = Image::new(path)
            {
                return ClassPathEntry::Image(image);
            }
            ClassPathEntry::Jar(Jar::new(path))
        } else {
            ClassPathEntry::Directory(Directory::new(path))
        }
    }

    /// Get the name of the class path entry.
    #[must_use]
    pub fn name(&self) -> &OsString {
        match self {
            ClassPathEntry::Directory(directory) => directory.name(),
            ClassPathEntry::Image(image) => image.name(),
            ClassPathEntry::Jar(jar) => jar.name(),
        }
    }

    /// Read a class from the class path entry.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile> {
        match self {
            ClassPathEntry::Directory(directory) => directory.read_class(name).await,
            ClassPathEntry::Image(image) => image.read_class(name).await,
            ClassPathEntry::Jar(jar) => jar.read_class(name).await,
        }
    }

    /// Get the class names in the class path entry.
    ///
    /// # Errors
    ///
    /// if the class names cannot be read.
    pub async fn class_names(&self) -> Result<Vec<String>> {
        match self {
            ClassPathEntry::Directory(directory) => directory.class_names().await,
            ClassPathEntry::Image(image) => image.class_names().await,
            ClassPathEntry::Jar(jar) => jar.class_names().await,
        }
    }
}

/// Represents a Jar manifest.
///
/// # References
/// - [JAR File Specification](https://docs.oracle.com/en/java/javase/22/docs/specs/jar/jar.html)
#[derive(Debug)]
struct Manifest {
    class_path: String,
    main_class: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // Directory Tests
    //

    #[test]
    fn test_new_directory() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.clone());

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(
            class_path_entry.name().to_string_lossy(),
            classes_directory.to_string_lossy()
        );
    }

    #[tokio::test]
    async fn test_read_class_directory() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.clone());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(
            class_path_entry.name().to_string_lossy(),
            classes_directory.to_string_lossy()
        );
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_class_names_directory() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entry = ClassPathEntry::new(classes_directory);
        let class_names = class_path_entry.class_names().await?;
        assert!(class_names.contains(&"HelloWorld".to_string()));
        Ok(())
    }

    //
    // Jar Tests
    //

    #[test]
    fn test_new_jar() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.clone());

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(
            class_path_entry.name().to_string_lossy(),
            classes_jar.to_string_lossy()
        );
    }

    #[tokio::test]
    async fn test_read_class_jar() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.clone());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(
            class_path_entry.name().to_string_lossy(),
            classes_jar.to_string_lossy()
        );
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_class_names_jar() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar);
        let class_names = class_path_entry.class_names().await?;
        assert!(class_names.contains(&"HelloWorld".to_string()));
        Ok(())
    }

    //
    // Url Tests
    //

    #[cfg(feature = "url")]
    #[test]
    fn test_new_url() {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), url);
    }

    #[cfg(feature = "url")]
    #[tokio::test]
    async fn test_read_class_url() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);
        let class_file = class_path_entry
            .read_class("org/springframework/boot/SpringApplication")
            .await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), url);
        assert_eq!(
            "org/springframework/boot/SpringApplication",
            class_file.class_name()?
        );
        Ok(())
    }

    #[cfg(feature = "url")]
    #[tokio::test]
    async fn test_class_names_url() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);
        let class_names = class_path_entry.class_names().await?;
        assert!(class_names.contains(&"org/springframework/boot/SpringApplication".to_string()));
        Ok(())
    }
}

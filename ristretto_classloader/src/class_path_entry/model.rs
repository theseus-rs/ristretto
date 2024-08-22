use crate::class_path_entry::directory::Directory;
use crate::class_path_entry::jar::Jar;
use crate::Result;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::path::PathBuf;
use tracing::instrument;

/// Represents a class path entry.
#[derive(Clone, Debug, PartialEq)]
pub enum ClassPathEntry {
    Directory(Directory),
    Jar(Jar),
}

/// Implementation for `ClassPathEntry`.
impl ClassPathEntry {
    /// Create a new class path entry.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref();
        #[cfg(feature = "url")]
        if path.starts_with("https://") || path.starts_with("http://") {
            return ClassPathEntry::Jar(Jar::from_url(path));
        }

        if PathBuf::from(path).is_file() {
            ClassPathEntry::Jar(Jar::new(path))
        } else {
            ClassPathEntry::Directory(Directory::new(path))
        }
    }

    /// Get the name of the class path entry.
    #[must_use]
    pub fn name(&self) -> &String {
        match self {
            ClassPathEntry::Directory(directory) => directory.name(),
            ClassPathEntry::Jar(jar) => jar.name(),
        }
    }

    /// Read a class from the class path entry.
    ///
    /// # Errors
    /// if the class file cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile> {
        match self {
            ClassPathEntry::Directory(directory) => directory.read_class(name).await,
            ClassPathEntry::Jar(jar) => jar.read_class(name).await,
        }
    }
}

/// Represents a Jar manifest.
/// See: <https://docs.oracle.com/en/java/javase/22/docs/specs/jar/jar.html>
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

    #[test_log::test]
    fn test_new_directory() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.to_string_lossy());

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(
            class_path_entry.name().to_string(),
            classes_directory.to_string_lossy()
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class_directory() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.to_string_lossy());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(
            class_path_entry.name().to_string(),
            classes_directory.to_string_lossy()
        );
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    //
    // Jar Tests
    //

    #[test_log::test]
    fn test_new_jar() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.to_string_lossy());

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(
            class_path_entry.name().to_string(),
            classes_jar.to_string_lossy()
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class_jar() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.to_string_lossy());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(
            class_path_entry.name().to_string(),
            classes_jar.to_string_lossy()
        );
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    //
    // Url Tests
    //

    #[cfg(feature = "url")]
    #[test_log::test]
    fn test_new_url() {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), url);
    }

    #[cfg(feature = "url")]
    #[test_log::test(tokio::test)]
    async fn test_read_class_url() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);
        let class_file = class_path_entry
            .read_class("org.springframework.boot.SpringApplication")
            .await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), url);
        assert_eq!(
            "org/springframework/boot/SpringApplication",
            class_file.class_name()?
        );
        Ok(())
    }
}

use crate::class_path_entry::ClassPathEntry;
use crate::Error::ClassNotFound;
use crate::Result;
use ristretto_classfile::ClassFile;
use std::fmt::Display;
use tracing::{info, instrument};

/// Represents a class path.
///
/// The class path is a list of directories and JAR files that contain class files.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassPath {
    class_path: Vec<ClassPathEntry>,
}

/// Implementation for `ClassPath`.
impl ClassPath {
    /// Creates a new `ClassPath` with the given class path.
    #[must_use]
    pub fn new(class_path: Vec<ClassPathEntry>) -> Self {
        ClassPath { class_path }
    }

    /// Creates a new `ClassPath` from a string.
    ///
    /// # Errors
    /// if the class path is not valid.
    pub fn from<S: AsRef<str>>(class_path: S) -> Self {
        let class_path = class_path.as_ref();
        let class_paths = class_path.split(':').collect::<Vec<_>>();
        let mut class_path_entries = Vec::with_capacity(class_paths.len());
        for path in class_paths {
            #[cfg(feature = "url")]
            let path = path
                .replace("http//", "http://")
                .replace("https//", "https://");
            class_path_entries.push(ClassPathEntry::new(path));
        }
        ClassPath::new(class_path_entries)
    }

    /// Returns an iterator over the class path.
    pub fn iter(&self) -> impl Iterator<Item = &ClassPathEntry> {
        self.class_path.iter()
    }

    /// Read a class from the class path.
    ///
    /// # Errors
    /// if the class file is not found or cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile> {
        let name = name.as_ref();

        for class_path_entry in self.iter() {
            if let Ok(class_file) = class_path_entry.read_class(name).await {
                info!("load class {name} source: {}", class_path_entry.name());
                return Ok(class_file);
            }
        }

        Err(ClassNotFound(name.to_string()))
    }

    /// Get the class names in the class path.
    ///
    /// # Errors
    /// if the class names cannot be read.
    pub async fn class_names(&self) -> Result<Vec<String>> {
        let mut classes = Vec::new();
        for class_path_entry in self.iter() {
            let class_names = class_path_entry.class_names().await?;
            classes.extend(class_names);
        }
        classes.sort();
        Ok(classes)
    }
}

/// Into iterator for `ClassPath`.
impl IntoIterator for ClassPath {
    type Item = ClassPathEntry;
    type IntoIter = std::vec::IntoIter<ClassPathEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.class_path.into_iter()
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_path = self
            .class_path
            .iter()
            .map(|entry| entry.name().to_string())
            .collect::<Vec<_>>()
            .join(":");
        write!(f, "{class_path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let class_path = ClassPath::new(vec![ClassPathEntry::new("."), ClassPathEntry::new("..")]);
        assert_eq!(".:..", class_path.to_string());
    }

    #[test]
    fn test_from() {
        let class_path = ClassPath::from(".:..");
        assert_eq!(".:..", class_path.to_string());
    }

    #[test]
    fn test_iter() {
        let class_path = ClassPath::from(".:..");
        let mut iter = class_path.iter();
        assert_eq!(".", iter.next().expect("next").name());
        assert_eq!("..", iter.next().expect("next").name());
    }

    #[test]
    fn test_into_iter() {
        let class_path = ClassPath::from(".:..");
        let mut iter = class_path.into_iter();
        assert_eq!(".", iter.next().expect("next").name());
        assert_eq!("..", iter.next().expect("next").name());
    }

    #[tokio::test]
    async fn test_read_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let classes_jar = cargo_manifest.join("../classes/classes.jar");

        let class_path_entries = [
            classes_directory.to_string_lossy().to_string(),
            classes_jar.to_string_lossy().to_string(),
            #[cfg(feature = "url")]
            "https//repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar".to_string(),
        ];

        let class_path = class_path_entries.join(":");
        let class_path_entry = ClassPath::from(&class_path);

        let class_file = class_path_entry.read_class("HelloWorld").await?;
        assert_eq!("HelloWorld", class_file.class_name()?);

        #[cfg(feature = "url")]
        {
            let class_file = class_path_entry
                .read_class("org/springframework/boot/SpringApplication")
                .await?;
            assert_eq!(
                "org/springframework/boot/SpringApplication",
                class_file.class_name()?
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_class_names() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let classes_jar = cargo_manifest.join("../classes/classes.jar");

        let class_path_entries = [
            classes_directory.to_string_lossy().to_string(),
            classes_jar.to_string_lossy().to_string(),
            #[cfg(feature = "url")]
            "https//repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar".to_string(),
        ];

        let class_path = class_path_entries.join(":");
        let class_path_entry = ClassPath::from(&class_path);

        let class_names = class_path_entry.class_names().await?;
        assert!(class_names.contains(&"HelloWorld".to_string()));

        #[cfg(feature = "url")]
        {
            assert!(class_names.contains(&"org/springframework/boot/SpringApplication".to_string()));
        }
        Ok(())
    }
}

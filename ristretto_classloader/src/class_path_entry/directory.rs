use crate::Error::ClassNotFound;
use crate::Result;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::path::PathBuf;
use std::{fs, io};
use tracing::instrument;

/// A directory in the class path.
#[derive(Clone, Debug)]
pub struct Directory {
    name: String,
    path: PathBuf,
}

/// Implement the `Directory` struct.
impl Directory {
    /// Create a new directory from a path.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref();
        Self {
            name: path.to_string(),
            path: PathBuf::from(path),
        }
    }

    /// Get the name of the directory.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Read a class from the directory.
    ///
    /// # Errors
    /// if the class file is not found or cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile> {
        let name = name.as_ref();
        let parts = name.split('.').collect::<Vec<_>>();
        let path = self.path.clone();
        let path = parts.iter().fold(path, |path, part| path.join(part));
        let path = path.with_extension("class");

        if !path.is_file() {
            return Err(ClassNotFound(name.to_string()));
        }

        let bytes = fs::read(path)?;
        let mut cursor = io::Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        Ok(class_file)
    }
}

/// Implement the `PartialEq` trait for `Directory`.
impl PartialEq for Directory {
    /// Compare two directories.
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_new() {
        let directory = Directory::new("test");
        assert_eq!("test", directory.name());
    }

    #[test_log::test]
    fn test_equality() {
        let directory1 = Directory::new("test");
        let directory2 = Directory::new("test");
        assert_eq!(directory1, directory2);
    }

    #[test_log::test]
    fn test_inequality() {
        let directory1 = Directory::new("test1");
        let directory2 = Directory::new("test2");
        assert_ne!(directory1, directory2);
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class_invalid_directory() -> Result<()> {
        let directory = Directory::new("foo");
        let result = directory.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let directory = Directory::new(classes_directory.to_string_lossy());
        // Read the class file twice to test caching
        for _ in 0..2 {
            let class_file = directory.read_class("HelloWorld").await?;
            assert_eq!("HelloWorld", class_file.class_name()?);
        }
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class_invalid_class_name() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let directory = Directory::new(classes_directory.to_string_lossy());
        let result = directory.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }
}

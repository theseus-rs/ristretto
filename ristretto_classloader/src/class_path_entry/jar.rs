use crate::Error::ClassNotFound;
use crate::Result;
use dashmap::DashMap;
use rayon::prelude::*;
use ristretto_classfile::ClassFile;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, io};
use tokio::sync::Mutex;
use tracing::{debug, instrument};
use zip::ZipArchive;

/// A jar or zip in the class path.
#[derive(Debug)]
pub struct Jar {
    path: PathBuf,
    initialized: Arc<Mutex<bool>>,
    class_files: DashMap<String, Arc<ClassFile>>,
}

/// Implement the `Jar` struct.
impl Jar {
    /// Create a new jar from a path.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            initialized: Arc::new(Mutex::new(false)),
            class_files: DashMap::new(),
        }
    }

    /// Load all class files from a jar.
    ///
    /// # Errors
    /// if the jar cannot be read or the class files cannot be loaded.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    #[instrument(level = "trace", skip(bytes, class_files))]
    pub async fn load_class_files(
        bytes: &Vec<u8>,
        class_files: &DashMap<String, Arc<ClassFile>>,
    ) -> Result<()> {
        let reader = io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(reader)?;

        // Decompress all the bytes from the jar and store in a map to be converted into class files
        let mut class_bytes = HashMap::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();
            if !file_name.ends_with(".class") {
                continue;
            }

            let mut bytes = Vec::new();
            io::copy(&mut file, &mut bytes)?;
            let class_name = file_name.replace('/', ".").replace(".class", "");
            class_bytes.insert(class_name, bytes);
        }

        // Convert the bytes into class files in parallel
        class_bytes.par_iter().for_each(|(class_name, bytes)| {
            let mut bytes = io::Cursor::new(bytes.clone());
            let class_file = match ClassFile::from_bytes(&mut bytes) {
                Ok(class_file) => class_file,
                Err(error) => {
                    debug!("Failed to load class file {class_name:?}: {error:?}");
                    return;
                }
            };

            match class_file.verify() {
                Ok(()) => (),
                Err(error) => {
                    debug!("Failed to verify class file {class_name:?}: {error:?}");
                    return;
                }
            }

            class_files.insert(class_name.to_string(), Arc::new(class_file));
        });

        Ok(())
    }

    /// Get the name of the jar.
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// Read a class from the jar.
    ///
    /// # Errors
    /// if the class file is not found or cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<Arc<ClassFile>> {
        let name = name.as_ref();
        if let Some(class_file) = self.class_files.get(name) {
            return Ok(Arc::clone(class_file.value()));
        }

        let initialized = self.initialized.lock().await;
        if *initialized {
            return Err(ClassNotFound(name.to_string()));
        }

        let bytes = match fs::read(self.path.clone()) {
            Ok(bytes) => bytes,
            Err(error) => {
                debug!("Failed to read jar file: {error:?}");
                return Err(ClassNotFound(name.to_string()));
            }
        };
        let load_result = Self::load_class_files(&bytes, &self.class_files).await;
        match load_result {
            Ok(()) => (),
            Err(error) => {
                debug!("Failed to load class files: {error:?}");
                return Err(ClassNotFound(name.to_string()));
            }
        }
        match self.class_files.get(name) {
            Some(class_file) => Ok(Arc::clone(class_file.value())),
            None => Err(ClassNotFound(name.to_string())),
        }
    }
}

/// Implement the `PartialEq` trait for `Jar`.
impl PartialEq for Jar {
    /// Compare two jars by their paths.
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    #[test]
    fn test_new() {
        let jar = Jar::new("test");
        assert_eq!("test", jar.name());
    }

    #[test]
    fn test_equality() {
        let jar1 = Jar::new("test");
        let jar2 = Jar::new("test");
        assert_eq!(jar1, jar2);
    }

    #[test]
    fn test_inequality() {
        let jar1 = Jar::new("test1");
        let jar2 = Jar::new("test2");
        assert_ne!(jar1, jar2);
    }

    #[tokio::test]
    async fn test_read_class_invalid_jar() -> Result<()> {
        let jar = Jar::new("foo.jar");
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_read_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        // Read the class file twice to test caching
        for _ in 0..2 {
            let class_file = jar.read_class("HelloWorld").await?;
            assert_eq!("HelloWorld", class_file.class_name()?);
        }

        // Test class file initialization
        let result = jar.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_read_class_invalid_class_name() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        let result = jar.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_bad_class_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Create a jar with a bad class file
        let jar_path = temp_dir.path().join("invalid.jar");
        let mut archive = zip::ZipWriter::new(fs::File::create(&jar_path)?);
        archive.start_file("HelloWorld.class", SimpleFileOptions::default())?;
        archive.write_all(&[0x00, 0x01, 0x02])?;
        archive.finish()?;

        // Test reading the class file
        let jar = Jar::new(jar_path.to_string_lossy());
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_class_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Create an invalid class file
        let class_file = ClassFile {
            this_class: 42,
            ..Default::default()
        };
        let mut bytes = Vec::new();
        class_file.to_bytes(&mut bytes)?;

        // Create a jar with an invalid class file
        let jar_path = temp_dir.path().join("invalid.jar");
        let mut archive = zip::ZipWriter::new(fs::File::create(&jar_path)?);
        archive.start_file("HelloWorld.class", SimpleFileOptions::default())?;
        archive.write_all(bytes.as_slice())?;
        archive.finish()?;

        // Test reading the class file
        let jar = Jar::new(jar_path.to_string_lossy());
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }
}

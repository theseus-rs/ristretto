use crate::Error::ClassNotFound;
use crate::Result;
use dashmap::DashMap;
use rayon::prelude::*;
use ristretto_classfile::ClassFile;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::sync::Arc;
use tracing::{debug, instrument};
use zip::ZipArchive;

/// A jar or zip in the class path.
#[derive(Debug)]
pub struct Jar {
    name: String,
    class_files: DashMap<String, Arc<ClassFile>>,
    is_module: bool,
}

/// Implement the `Jar` struct.
impl Jar {
    /// Create new jar from a path.
    pub async fn new<S: AsRef<str>>(path: S) -> Result<Self> {
        let path = path.as_ref();
        #[cfg(target_arch = "wasm32")]
        let bytes = std::fs::read(path)?;
        #[cfg(not(target_arch = "wasm32"))]
        let bytes = tokio::fs::read(path).await?;
        Jar::from_bytes(path, bytes).await
    }

    /// Create new jar from url.
    #[cfg(feature = "url")]
    pub async fn from_url<S: AsRef<str>>(url: S) -> Result<Self> {
        let url = url.as_ref();
        let client = reqwest::Client::new();
        let bytes = client.get(url).send().await?.bytes().await?.to_vec();
        Jar::from_bytes(url, bytes).await
    }

    /// Create new jar from bytes.
    #[allow(clippy::explicit_iter_loop)]
    pub async fn from_bytes<S: AsRef<str>>(name: S, bytes: Vec<u8>) -> Result<Self> {
        let mut class_files = DashMap::new();
        Self::load_class_files(&bytes, &class_files).await?;
        let is_module = class_files.contains_key("classes.module-info");

        if is_module {
            let new_class_files = DashMap::new();
            for (key, value) in class_files {
                let key = key.strip_prefix("classes.").unwrap_or(key.as_str());
                new_class_files.insert(key.to_string(), value);
            }
            class_files = new_class_files;
        }

        Ok(Self {
            name: name.as_ref().to_string(),
            class_files,
            is_module,
        })
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
        self.name.clone()
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

        Err(ClassNotFound(name.to_string()))
    }
}

/// Implement the `PartialEq` trait for `Jar`.
impl PartialEq for Jar {
    /// Compare two jars by their paths.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use std::io::Write;
    use std::path::PathBuf;
    use zip::write::SimpleFileOptions;

    #[test_log::test(tokio::test)]
    async fn test_new() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy()).await?;
        assert!(jar.name().ends_with("classes.jar"));
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_equality() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar1 = Jar::new(classes_jar.to_string_lossy()).await?;
        let jar2 = Jar::new(classes_jar.to_string_lossy()).await?;
        assert_eq!(jar1, jar2);
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class_invalid_jar() -> Result<()> {
        let result = Jar::new("foo.jar").await;
        assert!(matches!(result, Err(Error::IoError(_))));
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_read_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy()).await?;
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

    #[test_log::test(tokio::test)]
    async fn test_read_class_invalid_class_name() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy()).await?;
        let result = jar.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_bad_class_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Create a jar with a bad class file
        let jar_path = temp_dir.path().join("invalid.jar");
        let mut archive = zip::ZipWriter::new(std::fs::File::create(&jar_path)?);
        archive.start_file("HelloWorld.class", SimpleFileOptions::default())?;
        archive.write_all(&[0x00, 0x01, 0x02])?;
        archive.finish()?;

        // Test reading the class file
        let jar = Jar::new(jar_path.to_string_lossy()).await?;
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[test_log::test(tokio::test)]
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
        let mut archive = zip::ZipWriter::new(std::fs::File::create(&jar_path)?);
        archive.start_file("HelloWorld.class", SimpleFileOptions::default())?;
        archive.write_all(bytes.as_slice())?;
        archive.finish()?;

        // Test reading the class file
        let jar = Jar::new(jar_path.to_string_lossy()).await?;
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[cfg(feature = "url")]
    #[test_log::test(tokio::test)]
    async fn test_from_url_invalid() -> Result<()> {
        let result = Jar::from_url("https://foo.url").await;
        assert!(matches!(result, Err(Error::RequestError(_))));
        Ok(())
    }

    #[cfg(feature = "url")]
    #[test_log::test(tokio::test)]
    async fn test_from_url_read_class() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let url = Jar::from_url(url).await?;
        // Read the class file twice to test caching
        for _ in 0..2 {
            let class_file = url
                .read_class("org.springframework.boot.SpringApplication")
                .await?;
            assert_eq!(
                "org/springframework/boot/SpringApplication",
                class_file.class_name()?
            );
        }

        // Test class file initialization
        let result = url.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }
}

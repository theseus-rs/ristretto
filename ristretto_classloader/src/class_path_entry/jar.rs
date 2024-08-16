use crate::Error::{ArchiveError, ClassNotFound};
use crate::Result;
use dashmap::DashMap;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;
use zip::ZipArchive;

/// A jar or zip in the class path.
#[derive(Debug)]
pub struct Jar {
    name: String,
    archive: Arc<RwLock<Archive>>,
    class_files: DashMap<String, Arc<ClassFile>>,
}

/// Implement the `Jar` struct.
impl Jar {
    /// Create new jar from a path.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref();
        let archive = Archive::from_path(PathBuf::from(path));

        Self {
            name: path.to_string(),
            archive: Arc::new(RwLock::new(archive)),
            class_files: DashMap::new(),
        }
    }

    /// Create new jar from url.
    #[cfg(feature = "url")]
    pub fn from_url<S: AsRef<str>>(url: S) -> Self {
        let url = url.as_ref();
        let archive = Archive::from_url(url);

        Self {
            name: url.to_string(),
            archive: Arc::new(RwLock::new(archive)),
            class_files: DashMap::new(),
        }
    }

    /// Create new jar from bytes.
    pub fn from_bytes<S: AsRef<str>>(name: S, bytes: Vec<u8>) -> Self {
        let archive = Archive::from_bytes(bytes);

        Self {
            name: name.as_ref().to_string(),
            archive: Arc::new(RwLock::new(archive)),
            class_files: DashMap::new(),
        }
    }

    /// Get the name of the jar.
    pub fn name(&self) -> &String {
        &self.name
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

        let mut archive = self.archive.write().await;
        let class_file = if archive.is_module().await? {
            let name = format!("classes.{name}");
            archive.load_class_file(name.as_str()).await?
        } else {
            archive.load_class_file(name).await?
        };
        if let Some(class_file) = class_file {
            let class_file = Arc::new(class_file);
            self.class_files
                .insert(name.to_string(), class_file.clone());
            return Ok(class_file);
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

/// The source of the archive.
#[allow(clippy::struct_field_names)]
#[derive(Debug)]
struct Archive {
    path: Option<PathBuf>,
    url: Option<String>,
    bytes: Option<Arc<Vec<u8>>>,
    zip_archive: Option<ZipArchive<io::Cursor<Vec<u8>>>>,
    is_module: Option<bool>,
}

/// Implement the `Archive` enum.
impl Archive {
    /// Create a new archive source from a path.
    fn from_path(path: PathBuf) -> Self {
        let path = Some(path);
        Self {
            path,
            url: None,
            bytes: None,
            zip_archive: None,
            is_module: None,
        }
    }

    /// Create a new archive source from a url.
    fn from_url<S: AsRef<str>>(url: S) -> Self {
        let url = url.as_ref().to_string();
        Self {
            path: None,
            url: Some(url),
            bytes: None,
            zip_archive: None,
            is_module: None,
        }
    }

    /// Create a new archive source from bytes.
    ///
    /// # Errors
    /// if the bytes cannot be read.
    fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            path: None,
            url: None,
            bytes: Some(Arc::new(bytes)),
            zip_archive: None,
            is_module: None,
        }
    }

    /// Create a new archive source from a path.
    ///
    /// # Errors
    /// if the archive cannot be read.
    async fn zip_archive(&mut self) -> Result<&mut ZipArchive<io::Cursor<Vec<u8>>>> {
        if let Some(ref mut zip_archive) = self.zip_archive {
            return Ok(zip_archive);
        }

        if let Some(path) = &self.path {
            #[cfg(target_arch = "wasm32")]
            let bytes = std::fs::read(path)?;
            #[cfg(not(target_arch = "wasm32"))]
            let bytes = tokio::fs::read(path).await?;
            let cursor = io::Cursor::new(bytes);
            let archive = ZipArchive::new(cursor)?;
            self.zip_archive = Some(archive);
        } else if let Some(url) = &self.url {
            let client = reqwest::Client::new();
            let bytes = client.get(url).send().await?.bytes().await?.to_vec();
            let cursor = io::Cursor::new(bytes);
            let archive = ZipArchive::new(cursor)?;
            self.zip_archive = Some(archive);
        } else if let Some(bytes) = &self.bytes {
            let bytes = bytes.to_vec();
            let cursor = io::Cursor::new(bytes);
            let archive = ZipArchive::new(cursor)?;
            self.bytes = None;
            self.zip_archive = Some(archive);
        }

        if let Some(ref mut zip_archive) = self.zip_archive {
            Ok(zip_archive)
        } else {
            Err(ArchiveError("No archive source provided".to_string()))
        }
    }

    /// Load class file from a jar.
    ///
    /// # Errors
    /// if the jar cannot be read or the class file cannot be loaded.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    #[instrument(level = "trace")]
    async fn load_class_file(&mut self, class_name: &str) -> Result<Option<ClassFile>> {
        let class_file_name = format!("{}.class", class_name.replace('.', "/"));
        let zip_archive = self.zip_archive().await?;
        if let Some(index) = zip_archive.index_for_name(&class_file_name) {
            let mut file = zip_archive.by_index(index)?;
            let mut bytes = Vec::new();
            io::copy(&mut file, &mut bytes)?;
            let mut cursor = io::Cursor::new(bytes);
            let class_file = ClassFile::from_bytes(&mut cursor)?;
            class_file.verify()?;
            return Ok(Some(class_file));
        }
        Ok(None)
    }

    /// Check if the archive is a module.
    ///
    /// # Errors
    /// if the module information cannot be read.
    async fn is_module(&mut self) -> Result<bool> {
        if let Some(is_module) = self.is_module {
            Ok(is_module)
        } else {
            let module_info = self.load_class_file("classes.module-info").await?;
            let is_module = module_info.is_some();
            self.is_module = Some(is_module);
            Ok(is_module)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::ClassFileError;
    use std::io::Write;
    use std::path::PathBuf;
    use zip::write::SimpleFileOptions;

    #[test_log::test]
    fn test_new() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        assert!(jar.name().ends_with("classes.jar"));
    }

    #[test_log::test(tokio::test)]
    async fn test_from_bytes() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let bytes = tokio::fs::read(classes_jar).await?;
        let jar = Jar::from_bytes("test", bytes);
        assert_eq!("test", jar.name().as_str());
        let class_file = jar.read_class("HelloWorld").await?;
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[test_log::test]
    fn test_equality() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar1 = Jar::new(classes_jar.to_string_lossy());
        let jar2 = Jar::new(classes_jar.to_string_lossy());
        assert_eq!(jar1, jar2);
    }

    #[test_log::test(tokio::test)]
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

    #[test_log::test(tokio::test)]
    async fn test_read_class_invalid_class_name() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
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
        let jar = Jar::new(jar_path.to_string_lossy());
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassFileError(_))));
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
        let jar = Jar::new(jar_path.to_string_lossy());
        let result = jar.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassFileError(_))));
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_archive_zip_archive_error() {
        let mut archive = Archive {
            path: None,
            url: None,
            bytes: None,
            zip_archive: None,
            is_module: None,
        };
        let result = archive.zip_archive().await;
        assert!(matches!(result, Err(ArchiveError(_))));
    }

    #[cfg(feature = "url")]
    #[test_log::test(tokio::test)]
    async fn test_from_url_read_class() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let url = Jar::from_url(url);
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

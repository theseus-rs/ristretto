use crate::class_path_entry::manifest::Manifest;
use crate::Error::{ArchiveError, ClassNotFound, FileNotFound, ParseError, PoisonedLock};
use crate::Result;
use reqwest::blocking::Client;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::{fs, io};
use tracing::instrument;
use zip::ZipArchive;

/// A jar or zip in the class path.
/// See: <https://docs.oracle.com/en/java/javase/22/docs/specs/jar/jar.html>
#[derive(Debug)]
pub struct Jar {
    name: String,
    archive: Arc<RwLock<Archive>>,
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
        }
    }

    /// Create new jar from bytes.
    pub fn from_bytes<S: AsRef<str>>(name: S, bytes: Vec<u8>) -> Self {
        let archive = Archive::from_bytes(bytes);

        Self {
            name: name.as_ref().to_string(),
            archive: Arc::new(RwLock::new(archive)),
        }
    }

    /// Get the name of the jar.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the manifest of the jar.
    ///
    /// # Errors
    /// if the manifest cannot be read.
    pub fn manifest(&self) -> Result<Manifest> {
        let file_name = "META-INF/MANIFEST.MF";
        let Some(file) = self.read_file(file_name)? else {
            return Err(FileNotFound(file_name.to_string()));
        };
        let file = String::from_utf8(file).map_err(|error| ParseError(error.to_string()))?;
        let manifest = Manifest::from_str(file.as_str())?;
        Ok(manifest)
    }

    /// Read a file from the jar.
    ///
    /// # Errors
    /// if the file is not found or cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub fn read_file<S: AsRef<str>>(&self, name: S) -> Result<Option<Vec<u8>>> {
        let mut archive = self
            .archive
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        archive.load_file(name.as_ref())
    }

    /// Read a class from the jar.
    ///
    /// # Errors
    /// if the class file is not found or cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile> {
        let name = name.as_ref();
        let mut archive = self
            .archive
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        let class_file = if archive.is_module()? {
            let name = format!("classes/{name}");
            archive.load_class_file(name.as_str())?
        } else {
            archive.load_class_file(name)?
        };
        let Some(class_file) = class_file else {
            return Err(ClassNotFound(name.to_string()));
        };
        Ok(class_file)
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
#[expect(clippy::struct_field_names)]
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
    fn zip_archive(&mut self) -> Result<&mut ZipArchive<io::Cursor<Vec<u8>>>> {
        if let Some(ref mut zip_archive) = self.zip_archive {
            return Ok(zip_archive);
        }

        if let Some(path) = &self.path {
            let bytes = fs::read(path)?;
            let cursor = io::Cursor::new(bytes);
            let archive = ZipArchive::new(cursor)?;
            self.zip_archive = Some(archive);
        } else if let Some(url) = &self.url {
            let client = Client::new();
            let bytes = client.get(url).send()?.bytes()?.to_vec();
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
    #[instrument(level = "trace")]
    fn load_class_file(&mut self, class_name: &str) -> Result<Option<ClassFile>> {
        let class_file_name = format!("{class_name}.class");
        let file = self.load_file(&class_file_name)?;
        if let Some(bytes) = file {
            let mut cursor = io::Cursor::new(bytes);
            let class_file = ClassFile::from_bytes(&mut cursor)?;
            class_file.verify()?;
            return Ok(Some(class_file));
        }
        Ok(None)
    }

    /// Load file from a jar.
    ///
    /// # Errors
    /// if the jar cannot be read or the class file cannot be loaded.
    #[instrument(level = "trace")]
    fn load_file(&mut self, file_name: &str) -> Result<Option<Vec<u8>>> {
        let zip_archive = self.zip_archive()?;
        if let Some(index) = zip_archive.index_for_name(file_name) {
            let mut file = zip_archive.by_index(index)?;
            let file_size = usize::try_from(file.size())?;
            let mut bytes = Vec::with_capacity(file_size);
            io::copy(&mut file, &mut bytes)?;
            return Ok(Some(bytes));
        }
        Ok(None)
    }

    /// Check if the archive is a module.
    ///
    /// # Errors
    /// if the module information cannot be read.
    fn is_module(&mut self) -> Result<bool> {
        if let Some(is_module) = self.is_module {
            Ok(is_module)
        } else {
            let module_info = self.load_class_file("classes/module-info")?;
            let is_module = module_info.is_some();
            self.is_module = Some(is_module);
            Ok(is_module)
        }
    }
}

impl Clone for Jar {
    /// Clone the jar.
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            archive: Arc::clone(&self.archive),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_path_entry::manifest::{MAIN_CLASS, MANIFEST_VERSION};
    use crate::Error::ClassFileError;
    use std::io::Write;
    use std::path::PathBuf;
    use zip::write::SimpleFileOptions;

    #[test]
    fn test_new() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        assert!(jar.name().ends_with("classes.jar"));
    }

    #[test]
    fn test_from_bytes() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let bytes = fs::read(classes_jar)?;
        let jar = Jar::from_bytes("test", bytes);
        assert_eq!("test", jar.name().as_str());
        let class_file = jar.read_class("HelloWorld")?;
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[test]
    fn test_equality() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar1 = Jar::new(classes_jar.to_string_lossy());
        let jar2 = Jar::new(classes_jar.to_string_lossy());
        assert_eq!(jar1, jar2);
    }

    #[test]
    fn test_read_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        // Read the class file twice to test caching
        for _ in 0..2 {
            let class_file = jar.read_class("HelloWorld")?;
            assert_eq!("HelloWorld", class_file.class_name()?);
        }

        // Test class file initialization
        let result = jar.read_class("Foo");
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[test]
    fn test_read_manifest() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        let manifest = jar.manifest()?;
        assert_eq!(Some("1.0"), manifest.attribute(MANIFEST_VERSION));
        assert_eq!(Some("HelloWorld"), manifest.attribute(MAIN_CLASS));
        Ok(())
    }

    #[test]
    fn test_read_class_invalid_class_name() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let jar = Jar::new(classes_jar.to_string_lossy());
        let result = jar.read_class("Foo");
        assert!(matches!(result, Err(ClassNotFound(_))));
    }

    #[test]
    fn test_bad_class_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Create a jar with a bad class file
        let jar_path = temp_dir.path().join("invalid.jar");
        let mut archive = zip::ZipWriter::new(std::fs::File::create(&jar_path)?);
        archive.start_file("HelloWorld.class", SimpleFileOptions::default())?;
        archive.write_all(&[0x00, 0x01, 0x02])?;
        archive.finish()?;

        // Test reading the class file
        let jar = Jar::new(jar_path.to_string_lossy());
        let result = jar.read_class("HelloWorld");
        assert!(matches!(result, Err(ClassFileError(_))));
        Ok(())
    }

    #[test]
    fn test_invalid_class_file() -> Result<()> {
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
        let result = jar.read_class("HelloWorld");
        assert!(matches!(result, Err(ClassFileError(_))));
        Ok(())
    }

    #[test]
    fn test_archive_zip_archive_error() {
        let mut archive = Archive {
            path: None,
            url: None,
            bytes: None,
            zip_archive: None,
            is_module: None,
        };
        let result = archive.zip_archive();
        assert!(matches!(result, Err(ArchiveError(_))));
    }

    #[cfg(feature = "url")]
    #[test]
    fn test_from_url_read_class() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let url = Jar::from_url(url);
        // Read the class file twice to test caching
        for _ in 0..2 {
            let class_file = url.read_class("org/springframework/boot/SpringApplication")?;
            assert_eq!(
                "org/springframework/boot/SpringApplication",
                class_file.class_name()?
            );
        }

        // Test class file initialization
        let result = url.read_class("Foo");
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }
}

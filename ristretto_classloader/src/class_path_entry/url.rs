use crate::class_path_entry::jar::Jar;
use crate::Error::ClassNotFound;
use crate::Result;
use dashmap::DashMap;
use reqwest::Client;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, instrument};

/// A url in the class path.
#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Url {
    url: String,
    initialized: Arc<Mutex<bool>>,
    class_files: DashMap<String, Arc<ClassFile>>,
}

/// Implement the `Url` struct.
impl Url {
    /// Create a new url.
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        let url = url.as_ref();
        Self {
            url: url.to_string(),
            initialized: Arc::new(Mutex::new(false)),
            class_files: DashMap::new(),
        }
    }

    /// Load all class files from a url.
    ///
    /// # Errors
    /// if the url cannot be read or the class files cannot be loaded.
    #[instrument(level = "trace", skip(self))]
    async fn load_class_files(&self) -> Result<()> {
        let url = self.url.as_str();
        let client = Client::new();
        let url_bytes = client.get(url).send().await?.bytes().await?.to_vec();
        Jar::load_class_files(&url_bytes, &self.class_files).await
    }

    /// Get the name of the url.
    pub fn name(&self) -> String {
        self.url.clone()
    }

    /// Read a class from the url.
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

        let load_result = self.load_class_files().await;
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

/// Implement the `PartialEq` trait for `Url`.
impl PartialEq for Url {
    /// Compare two urls.
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";

    #[test]
    fn test_new() {
        let url = Url::new(URL);
        assert_eq!(URL, url.name());
    }

    #[test]
    fn test_equality() {
        let url1 = Url::new(URL);
        let url2 = Url::new(URL);
        assert_eq!(url1, url2);
    }

    #[test]
    fn test_inequality() {
        let url1 = Url::new(URL);
        let url2 = Url::new("https://foo.url");
        assert_ne!(url1, url2);
    }

    #[tokio::test]
    async fn test_read_class_invalid_url() -> Result<()> {
        let url = Url::new("https://foo.url");
        let result = url.read_class("HelloWorld").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_read_class() -> Result<()> {
        let url = Url::new(URL);
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

    #[tokio::test]
    async fn test_read_class_invalid_class_name() -> Result<()> {
        let url = Url::new(URL);
        let result = url.read_class("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
        Ok(())
    }
}

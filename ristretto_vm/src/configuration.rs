use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{ClassPath, DEFAULT_JAVA_VERSION};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Write, stderr, stdout};
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration
pub struct Configuration {
    class_path: ClassPath,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
    system_properties: HashMap<String, String>,
    interpreted: bool,
    preview_features: bool,
    stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

/// Configuration
impl Configuration {
    /// Get the class path
    #[must_use]
    pub fn class_path(&self) -> &ClassPath {
        &self.class_path
    }

    /// Get the main class
    #[must_use]
    pub fn main_class(&self) -> Option<&String> {
        self.main_class.as_ref()
    }

    /// Get the jar
    #[must_use]
    pub fn jar(&self) -> Option<&PathBuf> {
        self.jar.as_ref()
    }

    /// Get the Java home path
    #[must_use]
    pub fn java_home(&self) -> Option<&PathBuf> {
        self.java_home.as_ref()
    }

    /// Get the Java version
    #[must_use]
    pub fn java_version(&self) -> Option<&String> {
        self.java_version.as_ref()
    }

    /// Get the system properties
    #[must_use]
    pub fn system_properties(&self) -> &HashMap<String, String> {
        &self.system_properties
    }

    /// Get the interpreted flag
    #[must_use]
    pub fn interpreted(&self) -> bool {
        self.interpreted
    }

    /// Get the preview features flag
    #[must_use]
    pub fn preview_features(&self) -> bool {
        self.preview_features
    }

    /// Get the standard output stream
    #[must_use]
    pub fn stdout(&self) -> Arc<Mutex<dyn Write>> {
        self.stdout.clone()
    }

    /// Get the standard error stream
    #[must_use]
    pub fn stderr(&self) -> Arc<Mutex<dyn Write>> {
        self.stderr.clone()
    }
}

impl Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Configuration")
            .field("class_path", &self.class_path)
            .field("main_class", &self.main_class)
            .field("jar", &self.jar)
            .field("java_home", &self.java_home)
            .field("java_version", &self.java_version)
            .field("system_properties", &self.system_properties)
            .field("interpreted", &self.interpreted)
            .field("preview_features", &self.preview_features)
            .finish()
    }
}

/// Configuration builder
pub struct ConfigurationBuilder {
    class_path: Option<ClassPath>,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
    system_properties: HashMap<String, String>,
    interpreted: bool,
    preview_features: bool,
    stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

/// Configuration builder
impl ConfigurationBuilder {
    /// Create a new configuration builder
    #[must_use]
    pub fn new() -> Self {
        ConfigurationBuilder {
            class_path: None,
            main_class: None,
            jar: None,
            java_home: None,
            java_version: None,
            system_properties: HashMap::new(),
            interpreted: false,
            preview_features: false,
            stdout: Arc::new(Mutex::new(stdout())),
            stderr: Arc::new(Mutex::new(stderr())),
        }
    }

    /// Set the VM class path
    #[must_use]
    pub fn class_path(mut self, class_path: ClassPath) -> Self {
        self.class_path = Some(class_path);
        self
    }

    /// Set the main class to run
    #[must_use]
    pub fn main_class<S: AsRef<str>>(mut self, main_class: S) -> Self {
        self.main_class = Some(main_class.as_ref().to_string());
        self
    }

    /// Set the jar
    #[must_use]
    pub fn jar(mut self, jar: PathBuf) -> Self {
        self.jar = Some(jar);
        self
    }

    /// Set the Java home path
    #[must_use]
    pub fn java_home(mut self, java_home: PathBuf) -> Self {
        self.java_home = Some(java_home);
        self
    }

    /// Set the Java version
    #[must_use]
    pub fn java_version<S: AsRef<str>>(mut self, version: S) -> Self {
        self.java_version = Some(version.as_ref().to_string());
        self
    }

    /// Set the system properties
    #[must_use]
    pub fn add_system_property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.system_properties.insert(key, value);
        self
    }

    /// Set the system properties
    #[must_use]
    pub fn system_properties(mut self, properties: HashMap<String, String>) -> Self {
        self.system_properties = properties;
        self
    }

    /// Enable interpreted mode
    #[must_use]
    pub fn interpreted(mut self) -> Self {
        self.interpreted = true;
        self
    }

    /// Enable preview features
    #[must_use]
    pub fn preview_features(mut self) -> Self {
        self.preview_features = true;
        self
    }

    /// Set the standard output stream
    #[must_use]
    pub fn stdout(mut self, stdout: Arc<Mutex<dyn Write + Send + Sync>>) -> Self {
        self.stdout = stdout;
        self
    }

    /// Set the standard error stream
    #[must_use]
    pub fn stderr(mut self, stderr: Arc<Mutex<dyn Write + Send + Sync>>) -> Self {
        self.stderr = stderr;
        self
    }

    /// Build the configuration
    ///
    /// # Errors
    /// An error will be returned if the configuration cannot be built.
    pub fn build(self) -> Result<Configuration> {
        let class_path = if let Some(class_path) = self.class_path {
            class_path
        } else {
            ClassPath::from(".")
        };

        let java_home = self.java_home;
        let java_version = if let Some(java_version) = self.java_version {
            if java_home.is_some() {
                return Err(InternalError(
                    "Cannot specify both Java home and version".to_string(),
                ));
            }
            Some(java_version)
        } else if java_home.is_none() {
            Some(DEFAULT_JAVA_VERSION.to_string())
        } else {
            None
        };

        Ok(Configuration {
            class_path,
            main_class: self.main_class,
            jar: self.jar,
            java_home,
            java_version,
            system_properties: self.system_properties,
            interpreted: self.interpreted,
            preview_features: self.preview_features,
            stdout: self.stdout,
            stderr: self.stderr,
        })
    }
}

/// Debug implementation for ConfigurationBuilder
impl Debug for ConfigurationBuilder {
    /// Format the configuration builder
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigurationBuilder")
            .field("class_path", &self.class_path)
            .field("main_class", &self.main_class)
            .field("jar", &self.jar)
            .field("java_home", &self.java_home)
            .field("java_version", &self.java_version)
            .field("system_properties", &self.system_properties)
            .field("interpreted", &self.interpreted)
            .field("preview_features", &self.preview_features)
            .finish()
    }
}

/// Default configuration builder
impl Default for ConfigurationBuilder {
    /// Create a default configuration builder
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_builder() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(ClassPath::from(".."))
            .main_class("Foo")
            .jar(PathBuf::from("test.jar"))
            .java_version("21")
            .preview_features()
            .build()?;
        assert_eq!(&ClassPath::from(".."), configuration.class_path());
        assert_eq!(Some(&"Foo".to_string()), configuration.main_class());
        assert_eq!(Some(&PathBuf::from("test.jar")), configuration.jar());
        assert_eq!(Some(&"21".to_string()), configuration.java_version());
        assert!(!configuration.interpreted());
        assert!(configuration.preview_features());
        Ok(())
    }

    #[test]
    fn test_configuration_builder_new() -> Result<()> {
        let configuration = ConfigurationBuilder::new().build()?;
        assert_eq!(&ClassPath::from("."), configuration.class_path());
        assert!(configuration.main_class().is_none());
        assert!(configuration.jar().is_none());
        assert_eq!(
            Some(&DEFAULT_JAVA_VERSION.to_string()),
            configuration.java_version()
        );
        assert!(configuration.system_properties().is_empty());
        assert!(!configuration.preview_features());
        Ok(())
    }

    #[test]
    fn test_configuration_builder_default() -> Result<()> {
        let configuration = ConfigurationBuilder::default().build()?;
        assert_eq!(&ClassPath::from("."), configuration.class_path());
        assert_eq!(
            Some(&DEFAULT_JAVA_VERSION.to_string()),
            configuration.java_version()
        );
        Ok(())
    }

    #[test]
    fn test_configuration_builder_java_home() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .java_home(PathBuf::from("."))
            .build()?;
        assert_eq!(Some(&PathBuf::from(".")), configuration.java_home());
        assert_eq!(None, configuration.java_version());
        Ok(())
    }

    #[test]
    fn test_configuration_builder_java_home_and_java_version_error() {
        let result = ConfigurationBuilder::new()
            .java_home(PathBuf::from("."))
            .java_version("21")
            .build();
        assert!(matches!(result, Err(InternalError(_))));
    }

    #[test]
    fn test_configuration_builder_system_properties() -> Result<()> {
        let mut system_properties = HashMap::new();
        system_properties.insert("a".to_string(), "1".to_string());
        let configuration = ConfigurationBuilder::new()
            .system_properties(system_properties)
            .add_system_property("b", "2")
            .build()?;

        let system_properties = configuration.system_properties();
        assert_eq!(Some(&"1".to_string()), system_properties.get("a"));
        assert_eq!(Some(&"2".to_string()), system_properties.get("b"));
        Ok(())
    }
}

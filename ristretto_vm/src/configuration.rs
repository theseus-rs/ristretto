use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{ClassPath, DEFAULT_JAVA_VERSION};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write, stderr, stdin, stdout};
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for the Ristretto Virtual Machine (VM).
///
/// This struct contains all the settings needed to configure and run
/// the VM, including classpath, main class, Java version, system properties,
/// and I/O streams.
pub struct Configuration {
    class_path: ClassPath,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
    system_properties: HashMap<String, String>,
    interpreted: bool,
    preview_features: bool,
    stdin: Arc<Mutex<dyn Read + Send + Sync>>,
    stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

/// Configuration
impl Configuration {
    /// Returns a reference to the class path configuration.
    ///
    /// The class path determines where the VM will look for classes during execution.
    #[must_use]
    pub fn class_path(&self) -> &ClassPath {
        &self.class_path
    }

    /// Returns the name of the main class to be executed, if specified.
    ///
    /// This is the class containing the `main` method that will be invoked when the VM starts.
    /// Returns `None` if no main class has been specified.
    #[must_use]
    pub fn main_class(&self) -> Option<&String> {
        self.main_class.as_ref()
    }

    /// Returns the path to the JAR file to be executed, if specified.
    ///
    /// When running a JAR file, the VM will look for the main class in the JAR's manifest.
    /// Returns `None` if no JAR file has been specified.
    #[must_use]
    pub fn jar(&self) -> Option<&PathBuf> {
        self.jar.as_ref()
    }

    /// Returns the path to the Java home directory, if specified.
    ///
    /// This is used to locate the standard Java libraries.
    /// Returns `None` if no Java home has been specified.
    #[must_use]
    pub fn java_home(&self) -> Option<&PathBuf> {
        self.java_home.as_ref()
    }

    /// Returns the Java version to be used, if specified.
    ///
    /// This determines which version of the Java standard libraries will be used.
    /// Returns `None` if no specific Java version has been requested.
    #[must_use]
    pub fn java_version(&self) -> Option<&String> {
        self.java_version.as_ref()
    }

    /// Returns a reference to the system properties map.
    ///
    /// System properties are key-value pairs accessible via `System.getProperty()` in Java code.
    /// These properties configure various aspects of the Java runtime environment.
    #[must_use]
    pub fn system_properties(&self) -> &HashMap<String, String> {
        &self.system_properties
    }

    /// Returns whether the VM should run in interpreted mode.
    ///
    /// When `true`, the VM will interpret bytecode rather than using JIT compilation.
    /// This may be slower but can be useful for debugging or testing.
    #[must_use]
    pub fn interpreted(&self) -> bool {
        self.interpreted
    }

    /// Returns whether Java preview features should be enabled.
    ///
    /// When `true`, the VM will allow the use of preview features from the Java language.
    /// Preview features are not finalized and may change in future Java releases.
    #[must_use]
    pub fn preview_features(&self) -> bool {
        self.preview_features
    }

    /// Returns a reference to the standard input stream.
    ///
    /// This stream is used for normal input from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stdin(&self) -> Arc<Mutex<dyn Read + Send + Sync>> {
        self.stdin.clone()
    }

    /// Returns a reference to the standard output stream.
    ///
    /// This stream is used for normal output from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stdout(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.stdout.clone()
    }

    /// Returns a reference to the standard error stream.
    ///
    /// This stream is used for error messages from the VM and executed Java programs.
    /// The stream is wrapped in an `Arc<Mutex>` to allow for thread-safe access.
    #[must_use]
    pub fn stderr(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.stderr.clone()
    }
}

#[expect(clippy::missing_fields_in_debug)]
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

/// Builder for creating a `Configuration` with a fluent interface.
///
/// This builder provides methods to set all the configuration options
/// and then create a `Configuration` instance with the `build()` method.
pub struct ConfigurationBuilder {
    class_path: Option<ClassPath>,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
    system_properties: HashMap<String, String>,
    interpreted: bool,
    preview_features: bool,
    stdin: Arc<Mutex<dyn Read + Send + Sync>>,
    stdout: Arc<Mutex<dyn Write + Send + Sync>>,
    stderr: Arc<Mutex<dyn Write + Send + Sync>>,
}

/// Configuration builder
impl ConfigurationBuilder {
    /// Creates a new `ConfigurationBuilder` with default values.
    ///
    /// Default values include:
    /// - No class path (will be set to "." when building)
    /// - No main class
    /// - No JAR file
    /// - No Java home
    /// - Default Java version (will be set when building if no Java home is provided)
    /// - Empty system properties
    /// - JIT mode enabled
    /// - Preview features disabled
    /// - Standard output and error streams directed to system stdout/stderr
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
            stdin: Arc::new(Mutex::new(stdin())),
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

    /// Set the standard input stream
    #[must_use]
    pub fn stdin(mut self, stdin: Arc<Mutex<dyn Read + Send + Sync>>) -> Self {
        self.stdin = stdin;
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
    ///
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
            stdin: self.stdin,
            stdout: self.stdout,
            stderr: self.stderr,
        })
    }
}

/// Debug implementation for `ConfigurationBuilder`
#[expect(clippy::missing_fields_in_debug)]
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

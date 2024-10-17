use ristretto_classloader::{ClassPath, DEFAULT_RUNTIME_VERSION};
use std::env;
use std::path::PathBuf;
use std::string::ToString;

/// Configuration
#[derive(Debug, PartialEq)]
pub struct Configuration {
    class_path: ClassPath,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    runtime_version: String,
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
    pub fn main_class(&self) -> Option<String> {
        self.main_class.clone()
    }

    /// Get the jar
    #[must_use]
    pub fn jar(&self) -> Option<&PathBuf> {
        self.jar.as_ref()
    }

    /// Get the runtime version for the VM
    #[must_use]
    pub fn runtime_version(&self) -> &String {
        &self.runtime_version
    }
}

/// Configuration builder
#[derive(Debug)]
pub struct ConfigurationBuilder {
    class_path: Option<ClassPath>,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    runtime_version: Option<String>,
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
            runtime_version: None,
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

    /// Set the VM runtime version
    #[must_use]
    pub fn runtime_version<S: AsRef<str>>(mut self, version: S) -> Self {
        self.runtime_version = Some(version.as_ref().to_string());
        self
    }

    /// Build the configuration
    #[must_use]
    pub fn build(self) -> Configuration {
        let class_path = if let Some(class_path) = self.class_path {
            class_path
        } else {
            ClassPath::from(".")
        };

        let runtime_version = if let Some(runtime_version) = self.runtime_version {
            runtime_version.to_string()
        } else if let Ok(runtime_version) = env::var("JAVA_VERSION") {
            runtime_version
        } else {
            DEFAULT_RUNTIME_VERSION.to_string()
        };

        Configuration {
            class_path,
            main_class: self.main_class,
            jar: self.jar,
            runtime_version,
        }
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
    fn test_configuration_builder() {
        let configuration = ConfigurationBuilder::new()
            .class_path(ClassPath::from(".."))
            .main_class("Foo")
            .jar(PathBuf::from("test.jar"))
            .runtime_version("21")
            .build();
        assert_eq!(&ClassPath::from(".."), configuration.class_path());
        assert_eq!(Some("Foo".to_string()), configuration.main_class());
        assert_eq!(Some(&PathBuf::from("test.jar")), configuration.jar());
        assert_eq!("21", configuration.runtime_version());
    }

    #[test]
    fn test_configuration_builder_new() {
        let configuration = ConfigurationBuilder::new().build();
        assert_eq!(&ClassPath::from("."), configuration.class_path());
        assert_eq!(&DEFAULT_RUNTIME_VERSION, configuration.runtime_version());
    }

    #[test]
    fn test_configuration_builder_default() {
        let configuration = ConfigurationBuilder::default().build();
        assert_eq!(&ClassPath::from("."), configuration.class_path());
        assert_eq!(&DEFAULT_RUNTIME_VERSION, configuration.runtime_version());
    }
}

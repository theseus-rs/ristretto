use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{ClassPath, DEFAULT_JAVA_VERSION};
use std::path::PathBuf;
use std::string::ToString;

/// Configuration
#[derive(Debug, PartialEq)]
pub struct Configuration {
    class_path: ClassPath,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
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
}

/// Configuration builder
#[derive(Debug)]
pub struct ConfigurationBuilder {
    class_path: Option<ClassPath>,
    main_class: Option<String>,
    jar: Option<PathBuf>,
    java_home: Option<PathBuf>,
    java_version: Option<String>,
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
        })
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
            .build()?;
        assert_eq!(&ClassPath::from(".."), configuration.class_path());
        assert_eq!(Some(&"Foo".to_string()), configuration.main_class());
        assert_eq!(Some(&PathBuf::from("test.jar")), configuration.jar());
        assert_eq!(Some(&"21".to_string()), configuration.java_version());
        Ok(())
    }

    #[test]
    fn test_configuration_builder_new() -> Result<()> {
        let configuration = ConfigurationBuilder::new().build()?;
        assert_eq!(&ClassPath::from("."), configuration.class_path());
        assert_eq!(
            Some(&DEFAULT_JAVA_VERSION.to_string()),
            configuration.java_version()
        );
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
}

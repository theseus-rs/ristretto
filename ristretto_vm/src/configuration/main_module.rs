//! Main module specification for JPMS.
//!
//! This module provides the `MainModule` type for representing the main module
//! specification (--module or -m).

/// Represents the main module specification (--module or -m).
///
/// The main module contains the entry point for modular applications.
/// This corresponds to the `--module` or `-m` command-line option.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::MainModule;
///
/// // Create a main module with just the module name
/// let module = MainModule::new("my.module");
/// assert_eq!("my.module", module.name);
/// assert!(module.main_class.is_none());
///
/// // Create a main module with a specific main class
/// let module = MainModule::with_main_class("my.module", "com.example.Main");
/// assert_eq!("my.module", module.name);
/// assert_eq!(Some("com.example.Main".to_string()), module.main_class);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MainModule {
    /// The name of the main module.
    pub name: String,
    /// Optional main class within the module.
    pub main_class: Option<String>,
}

impl MainModule {
    /// Creates a new main module specification.
    ///
    /// The main class will be determined from the module's `module-info.class`
    /// or JAR manifest.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::MainModule;
    ///
    /// let module = MainModule::new("my.module");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            main_class: None,
        }
    }

    /// Creates a new main module specification with a main class.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module
    /// * `main_class` - The fully qualified name of the main class
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::MainModule;
    ///
    /// let module = MainModule::with_main_class("my.module", "com.example.Main");
    /// ```
    #[must_use]
    pub fn with_main_class(name: impl Into<String>, main_class: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            main_class: Some(main_class.into()),
        }
    }

    /// Parses a main module specification from a CLI string.
    ///
    /// The format is `module[/mainclass]`, e.g., `my.module` or `my.module/com.example.Main`.
    ///
    /// # Arguments
    ///
    /// * `spec` - The CLI specification string
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::MainModule;
    ///
    /// let module = MainModule::parse("my.module/com.example.Main");
    /// assert_eq!("my.module", module.name);
    /// assert_eq!(Some("com.example.Main".to_string()), module.main_class);
    /// ```
    #[must_use]
    pub fn parse(spec: &str) -> Self {
        if let Some(idx) = spec.find('/') {
            Self {
                name: spec[..idx].to_string(),
                main_class: Some(spec[idx + 1..].to_string()),
            }
        } else {
            Self {
                name: spec.to_string(),
                main_class: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = MainModule::new("my.module");
        assert_eq!("my.module", module.name);
        assert!(module.main_class.is_none());
    }

    #[test]
    fn test_new_with_string() {
        let module = MainModule::new(String::from("my.module"));
        assert_eq!("my.module", module.name);
    }

    #[test]
    fn test_with_main_class() {
        let module = MainModule::with_main_class("my.module", "com.example.Main");
        assert_eq!("my.module", module.name);
        assert_eq!(Some("com.example.Main".to_string()), module.main_class);
    }

    #[test]
    fn test_with_main_class_strings() {
        let module = MainModule::with_main_class(
            String::from("my.module"),
            String::from("com.example.Main"),
        );
        assert_eq!("my.module", module.name);
        assert_eq!(Some("com.example.Main".to_string()), module.main_class);
    }

    #[test]
    fn test_parse_simple() {
        let module = MainModule::parse("my.module");
        assert_eq!("my.module", module.name);
        assert!(module.main_class.is_none());
    }

    #[test]
    fn test_parse_with_class() {
        let module = MainModule::parse("my.module/com.example.Main");
        assert_eq!("my.module", module.name);
        assert_eq!(Some("com.example.Main".to_string()), module.main_class);
    }

    #[test]
    fn test_parse_with_nested_class() {
        let module = MainModule::parse("my.module/com.example.Main$Inner");
        assert_eq!("my.module", module.name);
        assert_eq!(
            Some("com.example.Main$Inner".to_string()),
            module.main_class
        );
    }

    #[test]
    fn test_parse_empty_class() {
        let module = MainModule::parse("my.module/");
        assert_eq!("my.module", module.name);
        assert_eq!(Some(String::new()), module.main_class);
    }

    #[test]
    fn test_clone() {
        let module = MainModule::with_main_class("my.module", "com.example.Main");
        let cloned = module.clone();
        assert_eq!(module, cloned);
    }

    #[test]
    fn test_debug() {
        let module = MainModule::with_main_class("my.module", "com.example.Main");
        let debug_str = format!("{module:?}");
        assert!(debug_str.contains("my.module"));
        assert!(debug_str.contains("com.example.Main"));
    }

    #[test]
    fn test_default() {
        let module = MainModule::default();
        assert_eq!("", module.name);
        assert!(module.main_class.is_none());
    }

    #[test]
    fn test_equality() {
        let module1 = MainModule::with_main_class("my.module", "com.example.Main");
        let module2 = MainModule::with_main_class("my.module", "com.example.Main");
        let module3 = MainModule::new("my.module");

        assert_eq!(module1, module2);
        assert_ne!(module1, module3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(MainModule::new("my.module"));
        set.insert(MainModule::new("my.module"));
        set.insert(MainModule::new("other.module"));

        assert_eq!(2, set.len());
    }

    #[test]
    fn test_empty_name() {
        let module = MainModule::new("");
        assert_eq!("", module.name);
    }

    #[test]
    fn test_complex_module_name() {
        let module = MainModule::new("com.example.myapp.module");
        assert_eq!("com.example.myapp.module", module.name);
    }

    #[test]
    fn test_parse_multiple_slashes() {
        // Only the first slash is used as delimiter
        let module = MainModule::parse("my.module/com/example/Main");
        assert_eq!("my.module", module.name);
        assert_eq!(Some("com/example/Main".to_string()), module.main_class);
    }
}

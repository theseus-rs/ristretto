//! Module export configuration for JPMS.
//!
//! This module provides the `ModuleExport` type for representing export directives
//! between modules (--add-exports).

/// Represents an export directive (--add-exports).
///
/// An export allows `source` module's `package` to be accessed by `target` module.
/// This corresponds to the `--add-exports` command-line option.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::ModuleExport;
///
/// // Export java.lang package from java.base to ALL-UNNAMED
/// let export = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
/// assert_eq!("java.base", export.source);
/// assert_eq!("java.lang", export.package);
/// assert_eq!("ALL-UNNAMED", export.target);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleExport {
    /// The source module containing the package.
    pub source: String,
    /// The package to export.
    pub package: String,
    /// The target module that can access the package.
    pub target: String,
}

impl ModuleExport {
    /// Creates a new module export.
    ///
    /// # Arguments
    ///
    /// * `source` - The module containing the package to export
    /// * `package` - The package to export
    /// * `target` - The module that can access the package
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::ModuleExport;
    ///
    /// let export = ModuleExport::new("java.base", "java.lang", "my.module");
    /// ```
    #[must_use]
    pub fn new(
        source: impl Into<String>,
        package: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            package: package.into(),
            target: target.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let export = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        assert_eq!("java.base", export.source);
        assert_eq!("java.lang", export.package);
        assert_eq!("ALL-UNNAMED", export.target);
    }

    #[test]
    fn test_new_with_string() {
        let export = ModuleExport::new(
            String::from("java.base"),
            String::from("java.lang"),
            String::from("my.module"),
        );
        assert_eq!("java.base", export.source);
        assert_eq!("java.lang", export.package);
        assert_eq!("my.module", export.target);
    }

    #[test]
    fn test_clone() {
        let export = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        let cloned = export.clone();
        assert_eq!(export, cloned);
    }

    #[test]
    fn test_debug() {
        let export = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        let debug_str = format!("{export:?}");
        assert!(debug_str.contains("java.base"));
        assert!(debug_str.contains("java.lang"));
        assert!(debug_str.contains("ALL-UNNAMED"));
    }

    #[test]
    fn test_equality() {
        let export1 = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        let export2 = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        let export3 = ModuleExport::new("java.base", "java.util", "ALL-UNNAMED");

        assert_eq!(export1, export2);
        assert_ne!(export1, export3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"));
        set.insert(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"));
        set.insert(ModuleExport::new("java.base", "java.util", "ALL-UNNAMED"));

        assert_eq!(2, set.len());
    }

    #[test]
    fn test_all_unnamed_target() {
        let export = ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED");
        assert_eq!("ALL-UNNAMED", export.target);
    }

    #[test]
    fn test_specific_module_target() {
        let export = ModuleExport::new("java.base", "java.lang", "my.module");
        assert_eq!("my.module", export.target);
    }

    #[test]
    fn test_nested_package() {
        let export = ModuleExport::new("java.base", "java.lang.reflect", "my.module");
        assert_eq!("java.lang.reflect", export.package);
    }

    #[test]
    fn test_empty_strings() {
        let export = ModuleExport::new("", "", "");
        assert_eq!("", export.source);
        assert_eq!("", export.package);
        assert_eq!("", export.target);
    }

    #[test]
    fn test_complex_names() {
        let export = ModuleExport::new(
            "com.example.app.module",
            "com.example.app.internal.api",
            "org.library.consumer",
        );
        assert_eq!("com.example.app.module", export.source);
        assert_eq!("com.example.app.internal.api", export.package);
        assert_eq!("org.library.consumer", export.target);
    }
}

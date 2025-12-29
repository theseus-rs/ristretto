//! Module opens configuration for JPMS.
//!
//! This module provides the `ModuleOpens` type for representing opens directives
//! for deep reflection (--add-opens).

/// Represents an opens directive (--add-opens).
///
/// An opens directive allows `source` module's `package` to be accessed via
/// deep reflection by `target` module. This corresponds to the `--add-opens`
/// command-line option.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::ModuleOpens;
///
/// // Open java.lang.reflect from java.base to ALL-UNNAMED for reflection
/// let opens = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
/// assert_eq!("java.base", opens.source);
/// assert_eq!("java.lang.reflect", opens.package);
/// assert_eq!("ALL-UNNAMED", opens.target);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleOpens {
    /// The source module containing the package.
    pub source: String,
    /// The package to open for deep reflection.
    pub package: String,
    /// The target module that can reflectively access the package.
    pub target: String,
}

impl ModuleOpens {
    /// Creates a new module opens directive.
    ///
    /// # Arguments
    ///
    /// * `source` - The module containing the package to open
    /// * `package` - The package to open for reflection
    /// * `target` - The module that can reflectively access the package
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::ModuleOpens;
    ///
    /// let opens = ModuleOpens::new("java.base", "java.lang.reflect", "my.module");
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
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "my.module");
        assert_eq!("java.base", opens.source);
        assert_eq!("java.lang.reflect", opens.package);
        assert_eq!("my.module", opens.target);
    }

    #[test]
    fn test_new_with_string() {
        let opens = ModuleOpens::new(
            String::from("java.base"),
            String::from("java.lang.reflect"),
            String::from("my.module"),
        );
        assert_eq!("java.base", opens.source);
        assert_eq!("java.lang.reflect", opens.package);
        assert_eq!("my.module", opens.target);
    }

    #[test]
    fn test_clone() {
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
        let cloned = opens.clone();
        assert_eq!(opens, cloned);
    }

    #[test]
    fn test_debug() {
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
        let debug_str = format!("{opens:?}");
        assert!(debug_str.contains("java.base"));
        assert!(debug_str.contains("java.lang.reflect"));
        assert!(debug_str.contains("ALL-UNNAMED"));
    }

    #[test]
    fn test_equality() {
        let opens1 = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
        let opens2 = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
        let opens3 = ModuleOpens::new("java.base", "sun.reflect", "ALL-UNNAMED");

        assert_eq!(opens1, opens2);
        assert_ne!(opens1, opens3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ModuleOpens::new(
            "java.base",
            "java.lang.reflect",
            "ALL-UNNAMED",
        ));
        set.insert(ModuleOpens::new(
            "java.base",
            "java.lang.reflect",
            "ALL-UNNAMED",
        ));
        set.insert(ModuleOpens::new("java.base", "sun.reflect", "ALL-UNNAMED"));

        assert_eq!(2, set.len());
    }

    #[test]
    fn test_all_unnamed_target() {
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "ALL-UNNAMED");
        assert_eq!("ALL-UNNAMED", opens.target);
    }

    #[test]
    fn test_specific_module_target() {
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "my.module");
        assert_eq!("my.module", opens.target);
    }

    #[test]
    fn test_internal_package() {
        let opens = ModuleOpens::new("java.base", "sun.nio.ch", "my.module");
        assert_eq!("sun.nio.ch", opens.package);
    }

    #[test]
    fn test_empty_strings() {
        let opens = ModuleOpens::new("", "", "");
        assert_eq!("", opens.source);
        assert_eq!("", opens.package);
        assert_eq!("", opens.target);
    }

    #[test]
    fn test_complex_names() {
        let opens = ModuleOpens::new(
            "com.example.app.module",
            "com.example.app.internal.impl",
            "org.test.framework",
        );
        assert_eq!("com.example.app.module", opens.source);
        assert_eq!("com.example.app.internal.impl", opens.package);
        assert_eq!("org.test.framework", opens.target);
    }

    #[test]
    fn test_difference_from_export() {
        // Opens is for deep reflection, while export is for compile-time access
        // This test documents the semantic difference
        let opens = ModuleOpens::new("java.base", "java.lang.reflect", "my.module");
        assert_eq!("java.lang.reflect", opens.package);
        // Opens allows setAccessible(true) and other reflection operations
    }
}

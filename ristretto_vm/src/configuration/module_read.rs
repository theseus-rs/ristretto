//! Module read edge configuration for JPMS.
//!
//! This module provides the `ModuleRead` type for representing read edges
//! between modules (--add-reads).

/// Represents a read edge between modules (--add-reads).
///
/// A read edge allows `source` module to read (access) the `target` module.
/// This corresponds to the `--add-reads` command-line option.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::ModuleRead;
///
/// // Allow my.module to read java.sql
/// let read = ModuleRead::new("my.module", "java.sql");
/// assert_eq!("my.module", read.source);
/// assert_eq!("java.sql", read.target);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleRead {
    /// The source module that will read the target.
    pub source: String,
    /// The target module to be read.
    pub target: String,
}

impl ModuleRead {
    /// Creates a new module read edge.
    ///
    /// # Arguments
    ///
    /// * `source` - The module that will read the target
    /// * `target` - The module to be read
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::ModuleRead;
    ///
    /// let read = ModuleRead::new("my.module", "java.sql");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let read = ModuleRead::new("my.module", "java.sql");
        assert_eq!("my.module", read.source);
        assert_eq!("java.sql", read.target);
    }

    #[test]
    fn test_new_with_string() {
        let read = ModuleRead::new(String::from("source.module"), String::from("target.module"));
        assert_eq!("source.module", read.source);
        assert_eq!("target.module", read.target);
    }

    #[test]
    fn test_clone() {
        let read = ModuleRead::new("my.module", "java.sql");
        let cloned = read.clone();
        assert_eq!(read, cloned);
    }

    #[test]
    fn test_debug() {
        let read = ModuleRead::new("my.module", "java.sql");
        let debug_str = format!("{read:?}");
        assert!(debug_str.contains("my.module"));
        assert!(debug_str.contains("java.sql"));
    }

    #[test]
    fn test_equality() {
        let read1 = ModuleRead::new("my.module", "java.sql");
        let read2 = ModuleRead::new("my.module", "java.sql");
        let read3 = ModuleRead::new("my.module", "java.xml");

        assert_eq!(read1, read2);
        assert_ne!(read1, read3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ModuleRead::new("my.module", "java.sql"));
        set.insert(ModuleRead::new("my.module", "java.sql"));
        set.insert(ModuleRead::new("my.module", "java.xml"));

        assert_eq!(2, set.len());
    }

    #[test]
    fn test_all_unnamed_target() {
        let read = ModuleRead::new("my.module", "ALL-UNNAMED");
        assert_eq!("ALL-UNNAMED", read.target);
    }

    #[test]
    fn test_empty_strings() {
        let read = ModuleRead::new("", "");
        assert_eq!("", read.source);
        assert_eq!("", read.target);
    }

    #[test]
    fn test_complex_module_names() {
        let read = ModuleRead::new("com.example.app.module", "org.library.core");
        assert_eq!("com.example.app.module", read.source);
        assert_eq!("org.library.core", read.target);
    }
}

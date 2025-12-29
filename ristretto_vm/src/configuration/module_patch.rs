//! Module patch configuration for JPMS.
//!
//! This module provides the `ModulePatch` type for representing module patches
//! (--patch-module).

use std::path::PathBuf;

/// Represents a module patch (--patch-module).
///
/// A patch allows replacing or augmenting a module's classes with those from a path.
/// This corresponds to the `--patch-module` command-line option.
///
/// # Example
///
/// ```rust
/// use ristretto_vm::ModulePatch;
/// use std::path::PathBuf;
///
/// // Patch java.base module with classes from /path/to/patch
/// let patch = ModulePatch::new("java.base", "/path/to/patch");
/// assert_eq!("java.base", patch.module);
/// assert_eq!(PathBuf::from("/path/to/patch"), patch.path);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModulePatch {
    /// The module to patch.
    pub module: String,
    /// The path containing the patch classes.
    pub path: PathBuf,
}

impl ModulePatch {
    /// Creates a new module patch.
    ///
    /// # Arguments
    ///
    /// * `module` - The name of the module to patch
    /// * `path` - The path containing the replacement/additional classes
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_vm::ModulePatch;
    /// use std::path::PathBuf;
    ///
    /// let patch = ModulePatch::new("java.base", PathBuf::from("/patches/java.base"));
    /// ```
    #[must_use]
    pub fn new(module: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            module: module.into(),
            path: path.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let patch = ModulePatch::new("java.base", "/path/to/patch");
        assert_eq!("java.base", patch.module);
        assert_eq!(PathBuf::from("/path/to/patch"), patch.path);
    }

    #[test]
    fn test_new_with_string() {
        let patch = ModulePatch::new(String::from("java.base"), String::from("/path/to/patch"));
        assert_eq!("java.base", patch.module);
        assert_eq!(PathBuf::from("/path/to/patch"), patch.path);
    }

    #[test]
    fn test_new_with_pathbuf() {
        let patch = ModulePatch::new("java.base", PathBuf::from("/path/to/patch"));
        assert_eq!(PathBuf::from("/path/to/patch"), patch.path);
    }

    #[test]
    fn test_clone() {
        let patch = ModulePatch::new("java.base", "/path/to/patch");
        let cloned = patch.clone();
        assert_eq!(patch, cloned);
    }

    #[test]
    fn test_debug() {
        let patch = ModulePatch::new("java.base", "/path/to/patch");
        let debug_str = format!("{patch:?}");
        assert!(debug_str.contains("java.base"));
        assert!(debug_str.contains("path/to/patch"));
    }

    #[test]
    fn test_equality() {
        let patch1 = ModulePatch::new("java.base", "/path/to/patch");
        let patch2 = ModulePatch::new("java.base", "/path/to/patch");
        let patch3 = ModulePatch::new("java.base", "/other/path");
        let patch4 = ModulePatch::new("java.sql", "/path/to/patch");

        assert_eq!(patch1, patch2);
        assert_ne!(patch1, patch3);
        assert_ne!(patch1, patch4);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ModulePatch::new("java.base", "/path1"));
        set.insert(ModulePatch::new("java.base", "/path1"));
        set.insert(ModulePatch::new("java.base", "/path2"));

        assert_eq!(2, set.len());
    }

    #[test]
    fn test_relative_path() {
        let patch = ModulePatch::new("java.base", "patches/java.base");
        assert_eq!(PathBuf::from("patches/java.base"), patch.path);
    }

    #[test]
    fn test_absolute_path() {
        let patch = ModulePatch::new("java.base", "/absolute/path/to/patches");
        assert!(patch.path.is_absolute() || patch.path.to_string_lossy().starts_with('/'));
    }

    #[test]
    fn test_empty_strings() {
        let patch = ModulePatch::new("", "");
        assert_eq!("", patch.module);
        assert_eq!(PathBuf::from(""), patch.path);
    }

    #[test]
    fn test_complex_module_name() {
        let patch = ModulePatch::new("com.example.myapp", "/patches/myapp");
        assert_eq!("com.example.myapp", patch.module);
    }

    #[test]
    fn test_jar_path() {
        let patch = ModulePatch::new("java.base", "/patches/java.base-patch.jar");
        assert!(patch.path.to_string_lossy().ends_with(".jar"));
    }

    #[test]
    fn test_multiple_path_components() {
        let patch = ModulePatch::new("java.base", "/home/user/projects/patches/java.base");
        let path_str = patch.path.to_string_lossy();
        assert!(path_str.contains("home"));
        assert!(path_str.contains("java.base"));
    }
}

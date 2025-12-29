//! Error handling for the Ristretto module system
//!
//! This module provides the error types and result alias used throughout the Ristretto
//! module subsystem. The [`ModuleError`] enum covers various failure scenarios that can
//! occur during module resolution, access checking, and configuration operations.

/// Ristretto module result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`ModuleError`].
pub type Result<T, E = ModuleError> = core::result::Result<T, E>;

/// Errors that can occur during module operations
///
/// This enum represents all possible error conditions that may arise during module resolution,
/// access checking, and configuration in the Ristretto JVM implementation.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ModuleError {
    /// A required module was not found in any module finder
    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    /// A package was not found in the module graph
    #[error("Package not found: {0}")]
    PackageNotFound(String),

    /// A package is defined in multiple modules (split package violation)
    #[error("Package {package} is split between modules {module1} and {module2}")]
    SplitPackage {
        /// The package that is split
        package: String,
        /// The first module containing the package
        module1: String,
        /// The second module containing the package
        module2: String,
    },

    /// A cyclic dependency was detected during module resolution
    #[error("Cyclic dependency detected: {}", .0.join(" -> "))]
    CyclicDependency(Vec<String>),

    /// A module requires another module that was not found
    #[error("Module {requiring} requires {required} which is not found")]
    RequiredModuleNotFound {
        /// The module that has the requirement
        requiring: String,
        /// The required module that was not found
        required: String,
    },

    /// An invalid module name was encountered
    #[error("Invalid module name: {0}")]
    InvalidModuleName(String),

    /// An automatic module name could not be derived from the JAR filename
    #[error("Invalid automatic module name derived from: {0}")]
    InvalidAutomaticModuleName(String),

    /// Module resolution failed for the specified reason
    #[error("Module resolution failed: {0}")]
    ResolutionFailed(String),

    /// Access to a package was denied due to module encapsulation
    #[error("Module {from_module} cannot access package {package} in module {to_module}")]
    AccessDenied {
        /// The module attempting to access
        from_module: String,
        /// The module being accessed
        to_module: String,
        /// The package that was not exported
        package: String,
    },

    /// Reflective access to a package was denied
    #[error(
        "Module {from_module} cannot reflectively access package {package} in module {to_module}"
    )]
    ReflectionAccessDenied {
        /// The module attempting reflective access
        from_module: String,
        /// The module being accessed
        to_module: String,
        /// The package that was not opened
        package: String,
    },

    /// An error occurred while parsing a module descriptor
    #[error("Module descriptor parse error: {0}")]
    DescriptorParseError(String),

    /// An I/O error occurred during module operations
    #[error("IO error: {0}")]
    IoError(String),

    /// An internal error occurred in the module system
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Convert [`std::io::Error`] to [`ModuleError::IoError`]
impl From<std::io::Error> for ModuleError {
    fn from(error: std::io::Error) -> Self {
        ModuleError::IoError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_not_found() {
        let error = ModuleError::ModuleNotFound("com.example".to_string());
        assert_eq!(error.to_string(), "Module not found: com.example");
    }

    #[test]
    fn test_package_not_found() {
        let error = ModuleError::PackageNotFound("com.example.pkg".to_string());
        assert_eq!(error.to_string(), "Package not found: com.example.pkg");
    }

    #[test]
    fn test_split_package() {
        let error = ModuleError::SplitPackage {
            package: "com.example".to_string(),
            module1: "mod.a".to_string(),
            module2: "mod.b".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Package com.example is split between modules mod.a and mod.b"
        );
    }

    #[test]
    fn test_cyclic_dependency() {
        let error = ModuleError::CyclicDependency(vec![
            "mod.a".to_string(),
            "mod.b".to_string(),
            "mod.a".to_string(),
        ]);
        assert_eq!(
            error.to_string(),
            "Cyclic dependency detected: mod.a -> mod.b -> mod.a"
        );
    }

    #[test]
    fn test_required_module_not_found() {
        let error = ModuleError::RequiredModuleNotFound {
            requiring: "mod.a".to_string(),
            required: "mod.b".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Module mod.a requires mod.b which is not found"
        );
    }

    #[test]
    fn test_invalid_module_name() {
        let error = ModuleError::InvalidModuleName("123invalid".to_string());
        assert_eq!(error.to_string(), "Invalid module name: 123invalid");
    }

    #[test]
    fn test_invalid_automatic_module_name() {
        let error = ModuleError::InvalidAutomaticModuleName("---bad---.jar".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid automatic module name derived from: ---bad---.jar"
        );
    }

    #[test]
    fn test_resolution_failed() {
        let error = ModuleError::ResolutionFailed("cyclic dependency".to_string());
        assert_eq!(
            error.to_string(),
            "Module resolution failed: cyclic dependency"
        );
    }

    #[test]
    fn test_access_denied() {
        let error = ModuleError::AccessDenied {
            from_module: "mod.a".to_string(),
            to_module: "mod.b".to_string(),
            package: "com.example.internal".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Module mod.a cannot access package com.example.internal in module mod.b"
        );
    }

    #[test]
    fn test_reflection_access_denied() {
        let error = ModuleError::ReflectionAccessDenied {
            from_module: "mod.a".to_string(),
            to_module: "mod.b".to_string(),
            package: "com.example.internal".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Module mod.a cannot reflectively access package com.example.internal in module mod.b"
        );
    }

    #[test]
    fn test_descriptor_parse_error() {
        let error = ModuleError::DescriptorParseError("invalid attribute".to_string());
        assert_eq!(
            error.to_string(),
            "Module descriptor parse error: invalid attribute"
        );
    }

    #[test]
    fn test_io_error() {
        let error = ModuleError::IoError("file not found".to_string());
        assert_eq!(error.to_string(), "IO error: file not found");
    }

    #[test]
    fn test_internal_error() {
        let error = ModuleError::InternalError("unexpected state".to_string());
        assert_eq!(error.to_string(), "Internal error: unexpected state");
    }

    #[test]
    fn test_io_error_from_std() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = ModuleError::from(io_error);
        assert_eq!(error.to_string(), "IO error: file not found");
    }

    #[test]
    fn test_error_equality() {
        let error1 = ModuleError::ModuleNotFound("test".to_string());
        let error2 = ModuleError::ModuleNotFound("test".to_string());
        let error3 = ModuleError::ModuleNotFound("other".to_string());
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_error_clone() {
        let error = ModuleError::SplitPackage {
            package: "pkg".to_string(),
            module1: "m1".to_string(),
            module2: "m2".to_string(),
        };
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_error_debug() {
        let error = ModuleError::ModuleNotFound("test".to_string());
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("ModuleNotFound"));
        assert!(debug_str.contains("test"));
    }
}

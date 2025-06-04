//! Error handling for the Ristretto classloader
//!
//! This module provides the error types and result alias used throughout the Ristretto classloader.
//! The [`Error`] enum covers various failure scenarios that can occur during class loading
//! operations.

/// Ristretto classloader result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when loading classes
///
/// This enum represents all possible error conditions that may arise during class loading
/// operations in the Ristretto JVM implementation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while processing a runtime archive
    #[error("Archive error: {0}")]
    ArchiveError(String),
    /// An error occurred while loading a class
    #[error(transparent)]
    ClassFileError(#[from] ristretto_classfile::Error),
    /// A class was not found
    #[error("Class not found: {0}")]
    ClassNotFound(String),
    /// Specified field not found
    #[error("Field not found: {class_name}.{field_name}")]
    FieldNotFound {
        class_name: String,
        field_name: String,
    },
    /// A file was not found
    #[error("File not found: {0}")]
    FileNotFound(String),
    /// Illegal access attempt
    #[error("Illegal access: {0}")]
    IllegalAccessError(String),
    /// Invalid value type
    #[error("Invalid value type: {0}")]
    InvalidValueType(String),
    /// An error occurred while performing an IO operation
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Specified method not found
    #[error("Method not found: {class_name}.{method_name}{method_descriptor}")]
    MethodNotFound {
        class_name: String,
        method_name: String,
        method_descriptor: String,
    },
    /// Error parsing data
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Poisoned lock
    #[error("Poisoned lock: {0}")]
    PoisonedLock(String),
    /// An error occurred while performing a request
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    /// Error serializing or deserializing data
    #[error("Serde error: {0}")]
    SerdeError(String),
    /// An error occurred while trying to convert a number
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    /// The requested version is not supported
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
    /// An error while reading a jar or module
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}

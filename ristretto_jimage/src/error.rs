//! Error handling for the Ristretto Image reader.
//!
//! This module provides a comprehensive error type system for the Ristretto Image reader,
//! covering errors that might occur while reading an Image file.

/// Image result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur while reading an Image file.
///
/// This enum represents all possible error conditions that might arise during Image file reading.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid attribute data in the Image file.
    #[error("Invalid attribute data")]
    InvalidAttributeData,
    /// Invalid attribute type in the Image file.
    #[error("Invalid attribute type: {0}")]
    InvalidAttributeType(u8),
    /// Invalid Image index
    #[error("Invalid image index: {0}")]
    InvalidIndex(usize),
    /// Invalid magic bytes in the Image file.
    #[error("Invalid magic bytes: {0:?}")]
    InvalidMagicBytes(Vec<u8>),
    /// IO error
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Poisoned lock
    #[error("Poisoned lock: {0}")]
    PoisonedLock(String),
    /// Resource name does not match the expected name.
    #[error("Resource name mismatch: expected '{expected}', actual '{actual}'")]
    ResourceNameMismatch { expected: String, actual: String },
    /// An error occurred while trying to convert a number
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    /// UTF-8 conversion error
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// The Image version is not supported.
    #[error("Version not supported: {0}.{1}")]
    VersionNotSupported(u16, u16),
}

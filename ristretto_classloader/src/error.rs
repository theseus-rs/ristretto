/// Ristretto result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when loading classes
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while loading a class
    #[error(transparent)]
    ClassFileError(#[from] ristretto_classfile::Error),
    /// A class was not found
    #[error("Class not found: {0}")]
    ClassNotFound(String),
    /// An error occurred while performing an IO operation
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// An error occurred while performing a request
    #[cfg(feature = "url")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

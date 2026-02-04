//! Error types for the Ristretto POM crate.

use thiserror::Error;

/// A result type for Ristretto POM operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when parsing or manipulating a POM.
#[derive(Debug, Error)]
pub enum Error {
    /// An IO error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// An XML parsing error occurred.
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::DeError),
    /// An XML serialization error occurred.
    #[error("Serialization error: {0}")]
    Serialization(#[from] quick_xml::SeError),
    /// The coordinate is invalid.
    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),
    /// A required field is missing.
    #[error("Missing required field: {0}")]
    MissingField(String),
    /// An invalid value was encountered.
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    /// The model version is invalid.
    #[error("Invalid model version: expected '4.0.0', got '{0}'")]
    InvalidModelVersion(String),
    /// A semantic validation error occurred.
    #[error("Validation error: {0}")]
    ValidationError(String),
}

//! Error handling for the Ristretto VM.
//!
//! This module provides a comprehensive error type system for the Ristretto VM,
//! handling various error conditions that can occur during class loading,
//! verification, execution, and other VM operations.
//!
//! The central type is [`Error`], which encompasses all possible error conditions.
//! The module also provides a type alias [`Result<T>`](Result) for convenience.

use crate::java_error::JavaError;

/// Ristretto VM result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Comprehensive error type for the Ristretto VM.
///
/// This enum represents all the different error conditions that can occur within the Ristretto VM.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while loading a class file
    #[error(transparent)]
    ClassFileError(#[from] ristretto_classfile::Error),
    /// An error occurred in class loading
    #[error(transparent)]
    ClassLoaderError(#[from] ristretto_classloader::Error),
    /// An error occurred in garbage collection
    #[error(transparent)]
    GcError(#[from] ristretto_gc::Error),
    /// An error occurred while compiling
    #[error(transparent)]
    JitError(#[from] ristretto_jit::Error),
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
    /// Invalid constant
    #[error("Invalid constant; expected {expected}, found {actual}")]
    InvalidConstant { expected: String, actual: String },
    /// Invalid constant index
    #[error("Invalid instant index: {0}")]
    InvalidConstantIndex(u16),
    /// Invalid local variable
    #[error("Invalid local variable; expected {expected}, found {actual}")]
    InvalidLocalVariable { expected: String, actual: String },
    /// Invalid local variable index
    #[error("Invalid local variable index: {0}")]
    InvalidLocalVariableIndex(usize),
    /// Invalid operand for the operation
    #[error("Invalid operand; expected {expected}, found {actual}")]
    InvalidOperand { expected: String, actual: String },
    /// Invalid program counter
    #[error("Invalid program counter: {0}")]
    InvalidProgramCounter(usize),
    /// Invalid stack value
    #[error("Invalid stack value; expected {expected}, found {actual}")]
    InvalidStackValue { expected: String, actual: String },
    /// A Java error occurred
    #[error(transparent)]
    JavaError(#[from] JavaError),
    /// The operand stack overflowed
    #[error("Operand stack overflow")]
    OperandStackOverflow,
    /// The operand stack underflow
    #[error("Operand stack underflow")]
    OperandStackUnderflow,
    /// The parameters stack underflow
    #[error("Parameters stack underflow")]
    ParametersUnderflow,
    /// An error occurred while attempting to parse an integer
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    /// Poisoned lock
    #[error("Poisoned lock: {0}")]
    PoisonedLock(String),
    /// Error that represents a JVM throwable object
    #[error("java.lang.Throwable: {0}")]
    Throwable(ristretto_classloader::Object),
    /// An error occurred while converting from an integer
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    /// Unsupported class file version
    #[error("Unsupported class file version: {0}")]
    UnsupportedClassFileVersion(u16),
}

/// Convert [`std::io::Error` errors](std::io::Error) to [`InternalError`](Error::InternalError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::InternalError(error.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "Internal error: file not found");
    }
}

use std::num::TryFromIntError;
use std::string::FromUtf8Error;

/// Ristretto classfile result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur processing ristretto classes
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// `BootstrapMethods` attribute not defined in class file
    #[error("BootstrapMethods attribute not defined in class file")]
    BootstrapMethodsNotDefined,
    /// Invalid annotation element tag
    #[error("Invalid annotation element tag: {0}")]
    InvalidAnnotationElementTag(u8),
    /// Invalid array type code
    #[error("Invalid array type code {0}")]
    InvalidArrayTypeCode(u8),
    /// Invalid attribute length
    #[error("Invalid attribute length: {0}")]
    InvalidAttributeLength(u32),
    /// Invalid attribute name index
    #[error("Invalid attribute name index: {0}")]
    InvalidAttributeNameIndex(u16),
    /// Invalid base type code
    #[error("Invalid base type code {0}")]
    InvalidBaseTypeCode(char),
    /// Invalid bootstrap method index
    #[error("Invalid bootstrap method index {0}")]
    InvalidBootstrapMethodIndex(usize),
    /// Invalid class access flags
    #[error("Invalid class access flags: {0}")]
    InvalidClassAccessFlags(u16),
    /// Invalid constant pool index
    #[error("Invalid constant pool index {0}")]
    InvalidConstantPoolIndex(u16),
    /// Invalid constant pool index type
    #[error("Invalid constant pool index type {0}")]
    InvalidConstantPoolIndexType(u16),
    /// Invalid constant tag
    #[error("Invalid constant tag: {0}")]
    InvalidConstantTag(u8),
    /// Invalid field access flags
    #[error("Invalid field access flags: {0}")]
    InvalidFieldAccessFlags(u16),
    /// Invalid field type code
    #[error("Invalid field type code {0}")]
    InvalidFieldTypeCode(char),
    /// Invalid field type descriptor
    #[error("Invalid field type descriptor {0}")]
    InvalidFieldTypeDescriptor(String),
    /// Invalid instruction
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u8),
    /// Invalid instruction offset
    #[error("Invalid instruction offset: {0}")]
    InvalidInstructionOffset(u32),
    /// Invalid magic number when reading a class file
    #[error("Invalid magic number: {0}")]
    InvalidMagicNumber(u32),
    /// Invalid method access flags
    #[error("Invalid method access flags: {0}")]
    InvalidMethodAccessFlags(u16),
    /// Error when creating a reference kind
    #[error("Invalid reference kind: {0}")]
    InvalidReferenceKind(u8),
    /// Invalid stack frame type
    #[error("Invalid stack frame type: {0}")]
    InvalidStackFrameType(u8),
    /// Invalid target type code
    #[error("Invalid target type code {0}")]
    InvalidTargetTypeCode(u8),
    /// Invalid verification type tag
    #[error("Invalid verification type tag: {0}")]
    InvalidVerificationTypeTag(u8),
    /// Error when creating a Version from major and minor
    #[error("Invalid version: major={major}; minor={minor}")]
    InvalidVersion { major: u16, minor: u16 },
    /// Invalid tag in the constant pool for the class file version
    #[error("Class file version does not support constant tag {0}")]
    InvalidVersionConstant(u8),
    /// Invalid wide instruction
    #[error("Invalid wide instruction: {0}")]
    InvalidWideInstruction(u8),
    /// IO error
    #[error("IO error: {0}")]
    IoError(String),
    /// Error when attempting to create a UTF-8 string from bytes
    #[error("Invalid UTF-8 sequence: {0}")]
    FromUtf8Error(String),
    /// Error when attempting to convert a numeric value to a different type
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    /// Error when verifying a class file
    #[error("{context}: {message}")]
    VerificationError { context: String, message: String },
}

/// Convert [`FromUtf8Error` errors](FromUtf8Error) to [`FromUtf8Error`](Error::FromUtf8Error)
impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::FromUtf8Error(error.to_string())
    }
}

/// Convert [`std::io::Error` errors](std::io::Error) to [`IoError`](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_utf8_error() {
        let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
        let utf8_error = String::from_utf8(invalid_utf8).expect_err("expected FromUtf8Error");
        let error = Error::from(utf8_error);
        assert_eq!(
            error.to_string(),
            "Invalid UTF-8 sequence: invalid utf-8 sequence of 1 bytes from index 1"
        );
    }

    #[test]
    fn test_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "IO error: file not found");
    }
}

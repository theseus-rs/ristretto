//! Error handling for the Ristretto classfile verifiers
//!
//! This module provides the error types and result alias used throughout the Ristretto classfile
//! verifiers.

/// Ristretto classfile verifiers result type
///
/// This is a type alias for the standard library's [`Result`](core::result::Result) type with the
/// error type defaulting to [`VerifyError`].
pub type Result<T, E = VerifyError> = core::result::Result<T, E>;

/// Errors that can occur when verifying classes
///
/// This enum represents all possible error conditions that may arise during class verification
/// operations in the Ristretto JVM implementation.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum VerifyError {
    /// Indicates a class format error.
    #[error("ClassFormatError: {0}")]
    ClassFormatError(String),
    /// Indicates an `IllegalAccessError`.
    #[error("IllegalAccessError: {0}")]
    IllegalAccessError(String),
    /// Indicates an `IncompatibleClassChangeError`.
    #[error("IncompatibleClassChangeError: {0}")]
    IncompatibleClassChangeError(String),
    /// Indicates a `NoClassDefFoundError`.
    #[error("NoClassDefFoundError: {0}")]
    NoClassDefFoundError(String),
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
    /// Invalid method access flags
    #[error("Invalid method access flags: {0}")]
    InvalidMethodAccessFlags(u16),
    /// An error occurred while parsing a method descriptor
    #[error("Invalid method descriptor: {0}")]
    InvalidMethodDescriptor(String),
    /// Error when creating a reference kind
    #[error("Invalid reference kind: {0}")]
    InvalidReferenceKind(u8),
    /// Invalid stack frame type
    #[error("Invalid stack frame type: {0}")]
    InvalidStackFrameType(u8),
    /// Invalid stack frame offset
    #[error("Invalid stack frame offset: {0}")]
    InvalidStackFrameOffset(u16),
    /// Invalid target type code
    #[error("Invalid target type code {0}")]
    InvalidTargetTypeCode(u8),
    /// Invalid verification type tag
    #[error("Invalid verification type tag: {0}")]
    InvalidVerificationTypeTag(u8),
    /// Invalid tag in the constant pool for the class file version
    #[error("Class file version does not support constant tag {0}")]
    InvalidVersionConstant(u8),
    /// Invalid wide instruction
    #[error("Invalid wide instruction: {0}")]
    InvalidWideInstruction(u8),
    /// An error occurred while trying to convert a number
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    /// Indicates an unsupported class version error.
    #[error("UnsupportedClassVersionError: {0}")]
    UnsupportedClassVersionError(String),
    /// Indicates a verification error.
    #[error("VerifyError: {0}")]
    VerifyError(String),
    /// Indicates a verification error with context.
    #[error("VerifyError: {context}: {message}")]
    VerificationError { context: String, message: String },
}

impl From<crate::Error> for VerifyError {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::InvalidAnnotationElementTag(tag) => {
                VerifyError::InvalidAnnotationElementTag(tag)
            }
            crate::Error::InvalidArrayTypeCode(code) => VerifyError::InvalidArrayTypeCode(code),
            crate::Error::InvalidAttributeLength(len) => VerifyError::InvalidAttributeLength(len),
            crate::Error::InvalidAttributeNameIndex(idx) => {
                VerifyError::InvalidAttributeNameIndex(idx)
            }
            crate::Error::InvalidBaseTypeCode(code) => VerifyError::InvalidBaseTypeCode(code),
            crate::Error::InvalidConstantPoolIndex(idx) => {
                VerifyError::InvalidConstantPoolIndex(idx)
            }
            crate::Error::InvalidConstantPoolIndexType(idx) => {
                VerifyError::InvalidConstantPoolIndexType(idx)
            }
            crate::Error::InvalidConstantTag(tag) => VerifyError::InvalidConstantTag(tag),
            crate::Error::InvalidFieldTypeCode(code) => VerifyError::InvalidFieldTypeCode(code),
            crate::Error::InvalidFieldTypeDescriptor(desc) => {
                VerifyError::InvalidFieldTypeDescriptor(desc)
            }
            crate::Error::InvalidInstruction(code) => VerifyError::InvalidInstruction(code),
            crate::Error::InvalidInstructionOffset(offset) => {
                VerifyError::InvalidInstructionOffset(offset)
            }
            crate::Error::InvalidMethodDescriptor(desc) => {
                VerifyError::InvalidMethodDescriptor(desc)
            }
            crate::Error::InvalidReferenceKind(kind) => VerifyError::InvalidReferenceKind(kind),
            crate::Error::InvalidStackFrameType(tag) => VerifyError::InvalidStackFrameType(tag),
            crate::Error::InvalidTargetTypeCode(code) => VerifyError::InvalidTargetTypeCode(code),
            crate::Error::InvalidVerificationTypeTag(tag) => {
                VerifyError::InvalidVerificationTypeTag(tag)
            }
            crate::Error::InvalidWideInstruction(code) => VerifyError::InvalidWideInstruction(code),
            crate::Error::TryFromIntError(err) => VerifyError::TryFromIntError(err),
            crate::Error::VerificationError(error) => error,
            _ => VerifyError::VerifyError(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_classfile_error_conversion(source: crate::Error, expected: &VerifyError) {
        assert_eq!(expected, &VerifyError::from(source));
    }

    #[test]
    fn test_from_invalid_annotation_element_tag() {
        assert_classfile_error_conversion(
            crate::Error::InvalidAnnotationElementTag(b'I'),
            &VerifyError::InvalidAnnotationElementTag(b'I'),
        );
    }

    #[test]
    fn test_from_invalid_array_type_code() {
        assert_classfile_error_conversion(
            crate::Error::InvalidArrayTypeCode(99),
            &VerifyError::InvalidArrayTypeCode(99),
        );
    }

    #[test]
    fn test_from_invalid_attribute_length() {
        assert_classfile_error_conversion(
            crate::Error::InvalidAttributeLength(4),
            &VerifyError::InvalidAttributeLength(4),
        );
    }

    #[test]
    fn test_from_invalid_attribute_name_index() {
        assert_classfile_error_conversion(
            crate::Error::InvalidAttributeNameIndex(5),
            &VerifyError::InvalidAttributeNameIndex(5),
        );
    }

    #[test]
    fn test_from_invalid_base_type_code() {
        assert_classfile_error_conversion(
            crate::Error::InvalidBaseTypeCode('Q'),
            &VerifyError::InvalidBaseTypeCode('Q'),
        );
    }

    #[test]
    fn test_from_invalid_constant_pool_index() {
        assert_classfile_error_conversion(
            crate::Error::InvalidConstantPoolIndex(7),
            &VerifyError::InvalidConstantPoolIndex(7),
        );
    }

    #[test]
    fn test_from_invalid_constant_pool_index_type() {
        assert_classfile_error_conversion(
            crate::Error::InvalidConstantPoolIndexType(8),
            &VerifyError::InvalidConstantPoolIndexType(8),
        );
    }

    #[test]
    fn test_from_invalid_constant_tag() {
        assert_classfile_error_conversion(
            crate::Error::InvalidConstantTag(9),
            &VerifyError::InvalidConstantTag(9),
        );
    }

    #[test]
    fn test_from_invalid_field_type_code() {
        assert_classfile_error_conversion(
            crate::Error::InvalidFieldTypeCode('Z'),
            &VerifyError::InvalidFieldTypeCode('Z'),
        );
    }

    #[test]
    fn test_from_invalid_instruction() {
        assert_classfile_error_conversion(
            crate::Error::InvalidInstruction(0xFF),
            &VerifyError::InvalidInstruction(0xFF),
        );
    }

    #[test]
    fn test_from_invalid_instruction_offset() {
        assert_classfile_error_conversion(
            crate::Error::InvalidInstructionOffset(12),
            &VerifyError::InvalidInstructionOffset(12),
        );
    }

    #[test]
    fn test_from_invalid_reference_kind() {
        assert_classfile_error_conversion(
            crate::Error::InvalidReferenceKind(13),
            &VerifyError::InvalidReferenceKind(13),
        );
    }

    #[test]
    fn test_from_invalid_stack_frame_type() {
        assert_classfile_error_conversion(
            crate::Error::InvalidStackFrameType(14),
            &VerifyError::InvalidStackFrameType(14),
        );
    }

    #[test]
    fn test_from_invalid_target_type_code() {
        assert_classfile_error_conversion(
            crate::Error::InvalidTargetTypeCode(15),
            &VerifyError::InvalidTargetTypeCode(15),
        );
    }

    #[test]
    fn test_from_invalid_verification_type_tag() {
        assert_classfile_error_conversion(
            crate::Error::InvalidVerificationTypeTag(16),
            &VerifyError::InvalidVerificationTypeTag(16),
        );
    }

    #[test]
    fn test_from_invalid_wide_instruction() {
        assert_classfile_error_conversion(
            crate::Error::InvalidWideInstruction(17),
            &VerifyError::InvalidWideInstruction(17),
        );
    }

    #[test]
    fn test_from_classfile_error_string_variants() {
        assert_eq!(
            VerifyError::InvalidFieldTypeDescriptor("bad".to_string()),
            VerifyError::from(crate::Error::InvalidFieldTypeDescriptor("bad".to_string()))
        );
        assert_eq!(
            VerifyError::InvalidMethodDescriptor("bad".to_string()),
            VerifyError::from(crate::Error::InvalidMethodDescriptor("bad".to_string()))
        );
    }

    #[test]
    fn test_from_classfile_error_passthrough_and_fallback() {
        assert_eq!(
            VerifyError::ClassFormatError("bad class".to_string()),
            VerifyError::from(crate::Error::VerificationError(
                VerifyError::ClassFormatError("bad class".to_string())
            ))
        );

        assert!(matches!(
            VerifyError::from(crate::Error::InvalidMagicNumber(0)),
            VerifyError::VerifyError(_)
        ));
    }

    #[test]
    fn test_from_try_from_int_error() {
        let err = u8::try_from(u16::MAX).expect_err("value must not fit in u8");
        assert!(matches!(
            VerifyError::from(crate::Error::TryFromIntError(err)),
            VerifyError::TryFromIntError(_)
        ));
    }
}

use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::field::Field;
use crate::verifiers::attributes::AttributeContext;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType,
};
use crate::verifiers::{attributes, field_access_flags};

/// Verify the `ClassFile` fields.
///
/// # Errors
/// Returns `VerificationError` if the fields are invalid.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    for field in &class_file.fields {
        field_access_flags::verify(class_file, field)?;
        verify_name_index(class_file, field)?;
        verify_descriptor_index(class_file, field)?;
        attributes::verify(
            class_file,
            &field.attributes,
            AttributeContext::Field(field),
        )?;
    }
    Ok(())
}

/// Verify the field `name_index`.
///
/// # Errors
///
/// Returns `InvalidConstantPoolIndex` or `InvalidConstantPoolIndexType` if the `name_index` is
/// invalid.
fn verify_name_index(class_file: &ClassFile, field: &Field) -> Result<()> {
    let name_index = field.name_index;
    match class_file.constant_pool.get(name_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(name_index)),
        _ => return Err(InvalidConstantPoolIndexType(name_index)),
    }
    Ok(())
}

/// Verify the field `descriptor_index`.
///
/// # Errors
///
/// Returns `InvalidConstantPoolIndex` or `InvalidConstantPoolIndexType` if the `descriptor_index`
/// is invalid.
fn verify_descriptor_index(class_file: &ClassFile, field: &Field) -> Result<()> {
    let descriptor_index = field.descriptor_index;
    match class_file.constant_pool.get(descriptor_index) {
        Some(Constant::Utf8 { .. }) => {
            // valid constant
        }
        None => return Err(InvalidConstantPoolIndex(descriptor_index)),
        _ => return Err(InvalidConstantPoolIndexType(descriptor_index)),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field_access_flags::FieldAccessFlags;
    use crate::{BaseType, FieldType};

    fn get_test_class_file_and_field() -> (ClassFile, Field) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Utf8("foo".to_string()));
        constant_pool.push(Constant::Utf8("I".to_string()));
        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };
        (class_file, field)
    }

    #[test]
    fn test_invalid_access_flag_error() {
        let (mut class_file, mut field) = get_test_class_file_and_field();
        field.access_flags = FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE;
        class_file.fields.push(field);
        assert!(verify(&class_file).is_err());
    }

    #[test]
    fn test_invalid_name_index() {
        let (class_file, mut field) = get_test_class_file_and_field();
        field.name_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_name_index(&class_file, &field)
        );
    }

    #[test]
    fn test_invalid_name_index_type() -> Result<()> {
        let (mut class_file, mut field) = get_test_class_file_and_field();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Class(field.name_index));
        field.name_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(field.name_index)),
            verify_name_index(&class_file, &field)
        );
        Ok(())
    }

    #[test]
    fn test_invalid_descriptor_index() {
        let (class_file, mut field) = get_test_class_file_and_field();
        field.descriptor_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_descriptor_index(&class_file, &field)
        );
    }

    #[test]
    fn test_invalid_descriptor_index_type() -> Result<()> {
        let (mut class_file, mut field) = get_test_class_file_and_field();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.push(Constant::Class(field.descriptor_index));
        field.descriptor_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(field.descriptor_index)),
            verify_descriptor_index(&class_file, &field)
        );
        Ok(())
    }

    #[test]
    fn test_verify_success() {
        let (mut class_file, field) = get_test_class_file_and_field();
        class_file.fields.push(field);
        assert_eq!(Ok(()), verify(&class_file));
    }
}

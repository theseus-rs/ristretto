use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::field::Field;
use crate::verifiers::field_access_flags;
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::Result;

/// Verify the `ClassFile` fields.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    for field in &class_file.fields {
        field_access_flags::verify(class_file, field)?;
        verify_name_index(class_file, field)?;
        verify_descriptor_index(class_file, field)?;
        // TODO: verify attributes
    }
    Ok(())
}

fn verify_name_index(class_file: &ClassFile, field: &Field) -> Result<()> {
    let name_index = field.name_index;
    match class_file.constant_pool.get(name_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(name_index)),
        _ => return Err(InvalidConstantPoolIndexType(name_index)),
    }
    Ok(())
}

fn verify_descriptor_index(class_file: &ClassFile, field: &Field) -> Result<()> {
    let descriptor_index = field.descriptor_index;
    match class_file.constant_pool.get(descriptor_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
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

    #[test_log::test]
    fn test_success() {
        let (class_file, _field) = get_test_class_file_and_field();
        assert_eq!(Ok(()), verify(&class_file));
    }

    #[test_log::test]
    fn test_invalid_access_flag_error() {
        let (class_file, mut field) = get_test_class_file_and_field();
        field.access_flags = FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE;
        assert_eq!(Ok(()), verify(&class_file));
    }

    #[test_log::test]
    fn test_invalid_name_index() {
        let (class_file, mut field) = get_test_class_file_and_field();
        field.name_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_name_index(&class_file, &field)
        );
    }

    #[test_log::test]
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

    #[test_log::test]
    fn test_invalid_descriptor_index() {
        let (class_file, mut field) = get_test_class_file_and_field();
        field.descriptor_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_descriptor_index(&class_file, &field)
        );
    }

    #[test_log::test]
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
}

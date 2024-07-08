use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::method::Method;
use crate::verifiers::method_access_flags;
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::Result;

/// Verify the `ClassFile` methods.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    for method in &class_file.methods {
        method_access_flags::verify(class_file, method)?;
        verify_name_index(class_file, method)?;
        verify_descriptor_index(class_file, method)?;
        // TODO: verify attributes
    }
    Ok(())
}

fn verify_name_index(class_file: &ClassFile, method: &Method) -> Result<()> {
    let name_index = method.name_index;
    match class_file.constant_pool.get(name_index) {
        Some(Constant::Utf8 { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(name_index)),
        _ => return Err(InvalidConstantPoolIndexType(name_index)),
    }
    Ok(())
}

fn verify_descriptor_index(class_file: &ClassFile, method: &Method) -> Result<()> {
    let descriptor_index = method.descriptor_index;
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
    use crate::method_access_flags::MethodAccessFlags;

    fn get_test_class_file_and_method() -> (ClassFile, Method) {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.add(Constant::Utf8("foo".to_string()));
        constant_pool.add(Constant::Utf8("V".to_string()));
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![],
        };
        (class_file, method)
    }

    #[test]
    fn test_success() {
        let (class_file, _method) = get_test_class_file_and_method();
        assert_eq!(Ok(()), crate::verifiers::methods::verify(&class_file));
    }

    #[test]
    fn test_invalid_access_flag_error() {
        let (class_file, mut method) = get_test_class_file_and_method();
        method.access_flags = MethodAccessFlags::FINAL | MethodAccessFlags::SYNCHRONIZED;
        assert_eq!(Ok(()), crate::verifiers::methods::verify(&class_file));
    }

    #[test]
    fn test_invalid_name_index() {
        let (class_file, mut method) = get_test_class_file_and_method();
        method.name_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_name_index(&class_file, &method)
        );
    }

    #[test]
    fn test_invalid_name_index_type() -> Result<()> {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.add(Constant::Class {
            name_index: method.name_index,
        });
        method.name_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(method.name_index)),
            verify_name_index(&class_file, &method)
        );
        Ok(())
    }

    #[test]
    fn test_invalid_descriptor_index() {
        let (class_file, mut method) = get_test_class_file_and_method();
        method.descriptor_index = u16::MAX;
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_descriptor_index(&class_file, &method)
        );
    }

    #[test]
    fn test_invalid_descriptor_index_type() -> Result<()> {
        let (mut class_file, mut method) = get_test_class_file_and_method();
        let constant_pool = &mut class_file.constant_pool;
        constant_pool.add(Constant::Class {
            name_index: method.descriptor_index,
        });
        method.descriptor_index = u16::try_from(constant_pool.len())?;
        assert_eq!(
            Err(InvalidConstantPoolIndexType(method.descriptor_index)),
            verify_descriptor_index(&class_file, &method)
        );
        Ok(())
    }
}

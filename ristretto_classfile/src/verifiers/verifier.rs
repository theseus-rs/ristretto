use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::attributes::AttributeContext;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidConstantPoolIndex, InvalidConstantPoolIndexType,
};
use crate::verifiers::{
    attributes, class_access_flags, constant_pool, fields, interfaces, methods, nest,
    permitted_subclasses, record,
};

/// Verify the `ClassFile`.
///
/// # Errors
/// Returns `VerificationError` if the class file is invalid.
pub(crate) fn verify(class_file: &ClassFile) -> Result<()> {
    constant_pool::verify(class_file)?;
    class_access_flags::verify(class_file)?;
    verify_this_class(class_file)?;
    verify_super_class(class_file)?;
    interfaces::verify(class_file)?;
    fields::verify(class_file)?;
    methods::verify(class_file)?;
    attributes::verify(class_file, &class_file.attributes, AttributeContext::Class)?;
    nest::verify(class_file)?;
    permitted_subclasses::verify(class_file)?;
    record::verify(class_file)?;
    Ok(())
}

fn verify_this_class(class_file: &ClassFile) -> Result<()> {
    let this_class = class_file.this_class;
    let constant_pool = &class_file.constant_pool;
    match constant_pool.get(class_file.this_class) {
        Some(Constant::Class { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(this_class)),
        _ => return Err(InvalidConstantPoolIndexType(this_class)),
    }
    Ok(())
}

fn verify_super_class(class_file: &ClassFile) -> Result<()> {
    let super_class = class_file.super_class;

    if !class_file
        .access_flags
        .contains(ClassAccessFlags::INTERFACE)
        && super_class == 0
    {
        return Ok(());
    }

    let constant_pool = &class_file.constant_pool;
    match constant_pool.get(class_file.super_class) {
        Some(Constant::Class { .. }) => {} // valid constant
        None => return Err(InvalidConstantPoolIndex(super_class)),
        _ => return Err(InvalidConstantPoolIndexType(super_class)),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::class_file::ClassFile;

    #[test]
    fn test_verify_this_class_success() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        class_file.this_class = constant_pool.add_class("Foo")?;

        assert_eq!(Ok(()), verify_this_class(&class_file));
        Ok(())
    }

    #[test]
    fn test_verify_this_class_invalid_index() {
        let class_file = ClassFile {
            this_class: u16::MAX,
            ..Default::default()
        };
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_this_class(&class_file)
        );
    }

    #[test]
    fn test_verify_this_class_invalid_index_type() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        let index = constant_pool.add_integer(42)?;
        class_file.this_class = index;

        assert_eq!(
            Err(InvalidConstantPoolIndexType(index)),
            verify_this_class(&class_file)
        );
        Ok(())
    }

    #[test]
    fn test_verify_super_class_success() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        class_file.super_class = constant_pool.add_class("Foo")?;

        assert_eq!(Ok(()), verify_super_class(&class_file));
        Ok(())
    }

    #[test]
    fn test_verify_super_class_zero() {
        let class_file = ClassFile {
            super_class: 0,
            ..Default::default()
        };
        assert_eq!(Ok(()), verify_super_class(&class_file));
    }

    #[test]
    fn test_verify_super_class_invalid_index() {
        let class_file = ClassFile {
            super_class: u16::MAX,
            ..Default::default()
        };
        assert_eq!(
            Err(InvalidConstantPoolIndex(u16::MAX)),
            verify_super_class(&class_file)
        );
    }

    #[test]
    fn test_verify_super_class_invalid_index_type() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        let index = constant_pool.add_integer(42)?;
        class_file.super_class = index;

        assert_eq!(
            Err(InvalidConstantPoolIndexType(index)),
            verify_super_class(&class_file)
        );
        Ok(())
    }

    #[test]
    fn test_verify_super_class_interface_invalid() {
        let class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE,
            super_class: 0,
            ..Default::default()
        };
        assert_eq!(
            Err(InvalidConstantPoolIndex(0)),
            verify_super_class(&class_file)
        );
    }

    #[test]
    fn test_verify_success() -> Result<()> {
        let mut class_file = ClassFile {
            version: crate::JAVA_8,
            ..Default::default()
        };
        let constant_pool = &mut class_file.constant_pool;
        class_file.this_class = constant_pool.add_class("Foo")?;
        class_file.super_class = constant_pool.add_class("java/lang/Object")?;
        assert_eq!(Ok(()), verify(&class_file));
        Ok(())
    }
}

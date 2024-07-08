use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::{class_access_flags, constant_pool, fields, methods};
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::Result;

/// Verify the `ClassFile`.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    constant_pool::verify(class_file)?;
    class_access_flags::verify(class_file)?;
    verify_this_class(class_file)?;
    verify_super_class(class_file)?;
    fields::verify(class_file)?;
    methods::verify(class_file)?;
    // TODO: verify attributes
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
    match constant_pool.get(class_file.this_class) {
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
        constant_pool.add(Constant::Class { name_index: 1 });
        let index = constant_pool.len();
        class_file.this_class = u16::try_from(index)?;

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
        constant_pool.add(Constant::Integer(42));
        let index = u16::try_from(constant_pool.len())?;
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
        constant_pool.add(Constant::Class { name_index: 1 });
        let index = u16::try_from(constant_pool.len())?;
        class_file.super_class = index;

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
        constant_pool.add(Constant::Integer(42));
        let index = u16::try_from(constant_pool.len())?;
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
}

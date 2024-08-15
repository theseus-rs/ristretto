use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::Error::InvalidClassAccessFlags;
use crate::Result;

/// Verify the `ClassFile` `ClassAccessFlags`.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    let access_flags = class_file.access_flags;

    if access_flags.contains(ClassAccessFlags::ANNOTATION)
        && !access_flags.contains(ClassAccessFlags::INTERFACE)
    {
        return Err(InvalidClassAccessFlags(access_flags.bits()));
    }

    if access_flags.contains(ClassAccessFlags::INTERFACE) {
        if !access_flags.contains(ClassAccessFlags::ABSTRACT) {
            let full_class_name = class_file.class_name()?;
            let class_name = full_class_name.split('/').last().unwrap_or_default();
            if class_name != "package-info" {
                return Err(InvalidClassAccessFlags(access_flags.bits()));
            }
        }
        if access_flags.contains(ClassAccessFlags::FINAL)
            || access_flags.contains(ClassAccessFlags::SUPER)
            || access_flags.contains(ClassAccessFlags::ENUM)
            || access_flags.contains(ClassAccessFlags::MODULE)
        {
            return Err(InvalidClassAccessFlags(access_flags.bits()));
        }
    } else if access_flags.contains(ClassAccessFlags::FINAL)
        && access_flags.contains(ClassAccessFlags::ABSTRACT)
    {
        return Err(InvalidClassAccessFlags(access_flags.bits()));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::class_file::ClassFile;
    use crate::constant_pool::ConstantPool;
    use std::io::Cursor;

    #[test_log::test]
    fn test_verify_success() -> Result<()> {
        let class_bytes = include_bytes!("../../../classes/Simple.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

        assert_eq!(Ok(()), verify(&class_file));
        Ok(())
    }

    fn test_verify_error(access_flags: ClassAccessFlags) -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let this_class = constant_pool.add_class("Foo")?;
        let class_file = ClassFile {
            constant_pool,
            access_flags,
            this_class,
            ..Default::default()
        };

        assert_eq!(
            Err(InvalidClassAccessFlags(access_flags.bits())),
            verify(&class_file)
        );
        Ok(())
    }

    #[test_log::test]
    fn test_verify_annotation_not_interface_error() -> Result<()> {
        test_verify_error(ClassAccessFlags::ANNOTATION)
    }

    #[test_log::test]
    fn test_verify_interface_not_abstract_error() -> Result<()> {
        test_verify_error(ClassAccessFlags::INTERFACE)
    }

    #[test_log::test]
    fn test_verify_interface_is_final_error() -> Result<()> {
        test_verify_error(
            ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT | ClassAccessFlags::FINAL,
        )
    }

    #[test_log::test]
    fn test_verify_interface_is_super_error() -> Result<()> {
        test_verify_error(
            ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT | ClassAccessFlags::SUPER,
        )
    }

    #[test_log::test]
    fn test_verify_interface_is_enum_error() -> Result<()> {
        test_verify_error(
            ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT | ClassAccessFlags::ENUM,
        )
    }

    #[test_log::test]
    fn test_verify_interface_is_module_error() -> Result<()> {
        test_verify_error(
            ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT | ClassAccessFlags::MODULE,
        )
    }

    #[test_log::test]
    fn test_verify_not_abstract_and_finale_error() -> Result<()> {
        test_verify_error(ClassAccessFlags::ABSTRACT | ClassAccessFlags::FINAL)
    }
}

use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::Result;
use crate::class_file::ClassFile;
use crate::constant::Constant;

/// Verify the `ClassFile` interfaces.
pub fn verify(class_file: &ClassFile) -> Result<()> {
    for interface in &class_file.interfaces {
        let constant_pool = &class_file.constant_pool;
        match constant_pool.get(*interface) {
            Some(Constant::Class { .. }) => {} // valid constant
            None => return Err(InvalidConstantPoolIndex(*interface)),
            _ => return Err(InvalidConstantPoolIndexType(*interface)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_success() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        let index = constant_pool.add_class("Foo")?;
        class_file.interfaces.push(index);

        assert_eq!(Ok(()), verify(&class_file));
        Ok(())
    }

    #[test]
    fn test_verify_invalid_index() {
        let mut class_file = ClassFile::default();
        let index = 1;
        class_file.interfaces.push(index);

        assert_eq!(Err(InvalidConstantPoolIndex(index)), verify(&class_file));
    }

    #[test]
    fn test_verify_invalid_index_type() -> Result<()> {
        let mut class_file = ClassFile::default();
        let constant_pool = &mut class_file.constant_pool;
        let index = constant_pool.add_integer(42)?;
        class_file.interfaces.push(index);

        assert_eq!(
            Err(InvalidConstantPoolIndexType(index)),
            verify(&class_file)
        );
        Ok(())
    }
}

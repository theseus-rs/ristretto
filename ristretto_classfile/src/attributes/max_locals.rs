use crate::attributes::Instruction;
use crate::{ConstantPool, FieldType, Result};

/// Trait for calculating the maximum locals size.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3>
pub trait MaxLocals {
    /// Calculates the maximum locals size.
    ///
    /// # Errors
    /// if the locals size exceeds `u16::MAX`
    fn max_locals(&self, constant_pool: &ConstantPool, descriptor_index: u16) -> Result<u16>;
}

impl MaxLocals for [Instruction] {
    fn max_locals(&self, constant_pool: &ConstantPool, descriptor_index: u16) -> Result<u16> {
        let method_descriptor = constant_pool.try_get_utf8(descriptor_index)?;
        let (parameters, _return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
        let parameters = u16::try_from(parameters.len())?;
        let mut max_locals_size = parameters.saturating_sub(1);
        for instruction in self {
            if let Some(local_index) = instruction.locals_index()? {
                max_locals_size = max_locals_size.max(local_index);
            }
        }
        let max_locals_size = max_locals_size.saturating_add(1);
        Ok(max_locals_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::Instruction;

    #[test]
    #[expect(clippy::useless_vec)]
    fn test_max_locals_vec() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
        let instructions = vec![Instruction::Istore_0, Instruction::Return];
        assert_eq!(
            instructions.max_locals(&constant_pool, descriptor_index)?,
            2
        );
        Ok(())
    }

    #[test]
    fn test_max_locals_empty() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
        let instructions = [];
        assert_eq!(
            instructions.max_locals(&constant_pool, descriptor_index)?,
            2
        );
        Ok(())
    }

    #[test]
    fn test_max_locals_return() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
        let instructions = [Instruction::Return];
        assert_eq!(
            instructions.max_locals(&constant_pool, descriptor_index)?,
            2
        );
        Ok(())
    }

    #[test]
    fn test_max_locals_single_index() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
        let instructions = [
            Instruction::Istore_2,
            Instruction::Iload_2,
            Instruction::Return,
        ];
        assert_eq!(
            instructions.max_locals(&constant_pool, descriptor_index)?,
            3
        );
        Ok(())
    }

    #[test]
    fn test_max_locals_multiple_indexes() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
        let instructions = [
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Istore_1,
            Instruction::Iload_1,
            Instruction::Istore_3,
            Instruction::Iload_3,
            Instruction::Return,
        ];
        assert_eq!(
            instructions.max_locals(&constant_pool, descriptor_index)?,
            4
        );
        Ok(())
    }
}

use crate::attributes::Instruction;
use crate::{ConstantPool, Result};

/// Trait for calculating the maximum stack size.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3>
pub trait MaxStack {
    /// Calculates the maximum stack size.
    ///
    /// # Errors
    /// if the stack size exceeds `u16::MAX`
    fn max_stack(&self, constant_pool: &ConstantPool) -> Result<u16>;
}

impl MaxStack for [Instruction] {
    fn max_stack(&self, constant_pool: &ConstantPool) -> Result<u16> {
        let mut max_stack_size = 0;
        let mut stack: i32 = 0;
        for instruction in self {
            let stack_delta = i32::from(instruction.stack_delta(constant_pool)?);
            stack = stack.saturating_add(stack_delta);
            max_stack_size = max_stack_size.max(stack);
        }
        let max_stack_size = u16::try_from(max_stack_size)?;
        Ok(max_stack_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::Instruction;

    #[test]
    #[expect(clippy::useless_vec)]
    fn test_max_stack_vec() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let instructions = vec![Instruction::Iconst_0, Instruction::Return];
        assert_eq!(instructions.max_stack(&constant_pool)?, 1);
        Ok(())
    }

    #[test]
    fn test_max_stack_empty() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let instructions = [];
        assert_eq!(instructions.max_stack(&constant_pool)?, 0);
        Ok(())
    }

    #[test]
    fn test_max_stack_return() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let instructions = [Instruction::Return];
        assert_eq!(instructions.max_stack(&constant_pool)?, 0);
        Ok(())
    }

    #[test]
    fn test_max_stack_two_constants() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let instructions = [
            Instruction::Iconst_0,
            Instruction::Iconst_1,
            Instruction::Return,
        ];
        assert_eq!(instructions.max_stack(&constant_pool)?, 2);
        Ok(())
    }

    #[test]
    fn test_max_stack_pop_single_constant() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let instructions = [
            Instruction::Iconst_0,
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Pop,
            Instruction::Iconst_2,
            Instruction::Pop,
            Instruction::Return,
        ];
        assert_eq!(instructions.max_stack(&constant_pool)?, 1);
        Ok(())
    }
}

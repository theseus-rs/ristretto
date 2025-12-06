use crate::attributes::Instruction;
use crate::{ConstantPool, Result};

/// Trait for calculating the maximum stack size required by a sequence of JVM bytecode
/// instructions.
///
/// This trait analyzes bytecode instructions to determine the maximum operand stack depth that can
/// be reached during execution. The maximum stack size is a required value in the JVM class file
/// format's Code attribute for methods.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{Instruction, MaxStack};
/// use ristretto_classfile::ConstantPool;
///
/// // Create a constant pool (needed for some instructions)
/// let constant_pool = ConstantPool::new();
///
/// // Define a sequence of instructions that manipulate the stack
/// let instructions = [
///     Instruction::Iconst_0,  // Pushes 0 onto stack (+1)
///     Instruction::Iconst_1,  // Pushes 1 onto stack (+1, total: 2)
///     Instruction::Pop,       // Removes top value (-1, total: 1)
///     Instruction::Return,    // Method return (no effect on stack)
/// ];
///
/// // Calculate the maximum stack size reached
/// let max_size = instructions.max_stack(&constant_pool)?;
/// assert_eq!(max_size, 2); // Maximum depth was 2 (after Iconst_1)
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
///
/// # References
///
/// - [JVM Specification §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
pub trait MaxStack {
    /// Calculates the maximum stack size required by the instructions.
    ///
    /// # Errors
    ///
    /// if the stack size exceeds `u16::MAX`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Instruction, MaxStack};
    /// use ristretto_classfile::ConstantPool;
    ///
    /// // Create a constant pool (needed for some instructions)
    /// let constant_pool = ConstantPool::new();
    ///
    /// // Define a sequence of instructions that manipulate the stack
    /// let instructions = [
    ///     Instruction::Iconst_0,  // Pushes 0 onto stack (+1)
    ///     Instruction::Iconst_1,  // Pushes 1 onto stack (+1, total: 2)
    ///     Instruction::Pop,       // Removes top value (-1, total: 1)
    ///     Instruction::Return,    // Method return (no effect on stack)
    /// ];
    ///
    /// // Calculate the maximum stack size reached
    /// let max_size = instructions.max_stack(&constant_pool)?;
    /// assert_eq!(max_size, 2); // Maximum depth was 2 (after Iconst_1)
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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

use crate::attributes::{ExceptionTableEntry, Instruction};
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
/// - [JVMS §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
pub trait MaxStack {
    /// Calculates the maximum stack size required by the instructions.
    ///
    /// # Errors
    ///
    /// Returns an error for stack underflow/overflow, inconsistent branch merge
    /// heights, invalid control flow, or invalid instruction operands/descriptors.
    /// This checks slot counts, not JVM operand types. Use the bytecode verifier
    /// for complete verification. Exception handlers must be supplied through
    /// [`Self::max_stack_with_exception_table`].
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
    fn max_stack(&self, constant_pool: &ConstantPool<'_>) -> Result<u16>;

    /// Calculates maximum stack depth including exception handlers. Handler ranges
    /// and targets use logical instruction indices, just like branch operands.
    /// Each handler starts with one exception-reference slot. Unreachable code is
    /// excluded. Legacy `jsr`/`ret` subroutines are followed using their return
    /// addresses; this API does not enforce class-file version restrictions.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid exception tables, stack underflow/overflow,
    /// incompatible merge heights, invalid branches/subroutine returns, recursive
    /// subroutine calls, or invalid instruction operands/descriptors.
    fn max_stack_with_exception_table(
        &self,
        constant_pool: &ConstantPool<'_>,
        exception_table: &[ExceptionTableEntry],
    ) -> Result<u16>;
}

mod analysis;

impl MaxStack for [Instruction] {
    fn max_stack(&self, constant_pool: &ConstantPool<'_>) -> Result<u16> {
        self.max_stack_with_exception_table(constant_pool, &[])
    }

    fn max_stack_with_exception_table(
        &self,
        constant_pool: &ConstantPool<'_>,
        exception_table: &[ExceptionTableEntry],
    ) -> Result<u16> {
        analysis::analyze(self, constant_pool, exception_table)
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

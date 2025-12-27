use crate::attributes::Instruction;
use crate::{BaseType, ConstantPool, FieldType, Method, MethodAccessFlags, Result};

/// Trait for calculating the maximum number of local variables needed by a method.
///
/// The `max_locals` value is a crucial part of a method's `Code` attribute. It specifies the
/// number values required in the local variable array of the method's stack frame.
///
/// This count includes:
///
/// 1.  The `this` reference for instance (non-static) methods (occupies slot `0`).
/// 2.  All method parameters. `long` and `double` parameters each occupy two slots;
///     all other types occupy one slot.
/// 3.  Any additional local variables declared and used within the method body.
///     Again, `long` and `double` types will use two slots.
///
/// The JVM uses `max_locals` to determine the size of the local variable array when a new
/// stack frame is created for a method invocation.
///
/// # Calculation Logic
///
/// The calculation proceeds as follows:
/// - Initialize `max_locals` to 1 if it's an instance method (for `this`), or 0 if static.
/// - Add the number of slots required for method parameters based on their types.
/// - Iterate through all instructions in the method's code:
///   - If an instruction accesses a local variable (e.g., `iload`, `istore`, `lload`, `lstore`, `iinc`),
///     determine the highest local variable index it uses.
///   - Update `max_locals` if this instruction uses a higher index than currently recorded.
///     For `long` and `double` types use `index` and `index + 1`.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{ConstantPool, Method, MethodAccessFlags};
/// use ristretto_classfile::attributes::{Instruction, MaxLocals};
/// use std::collections::HashMap;
///
/// // Create a constant pool and add a method descriptor for a method like:
/// // public void exampleMethod(int p1, long p2)
/// let mut constant_pool = ConstantPool::new();
/// let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
///
/// // Define an instance method (non-static)
/// let method = Method {
///     access_flags: MethodAccessFlags::PUBLIC,
///     name_index: constant_pool.add_utf8("exampleMethod")?,
///     descriptor_index,
///     attributes: vec![], // Code attribute would normally be here
/// };
///
/// // Instructions for the method body:
/// // Assume 'this' is at slot 0.
/// // int p1 is at slot 1.
/// // long p2 is at slots 2 and 3.
/// let instructions = [
///     Instruction::Iload_1,     // Load p1 (uses slot 1)
///     Instruction::Lload_2,     // Load p2 (uses slots 2, 3)
///     Instruction::Iconst_5,    // Push 5
///     Instruction::Istore(4),   // Store into new local variable at slot 4
///     Instruction::Lconst_1,
///     Instruction::Lstore(5),   // Store into new long local variable at slots 5, 6
///     Instruction::Return,
/// ];
///
/// // Calculate max locals
/// // Slot 0: this
/// // Slot 1: p1 (int)
/// // Slots 2, 3: p2 (long)
/// // Slot 4: istore(4) - int
/// // Slots 5, 6: lstore(5) - long
/// // Highest index used is 6. So, max_locals should be 7 (0-6).
/// let max_locals_val = instructions.max_locals(&constant_pool, &method)?;
/// assert_eq!(max_locals_val, 7);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVMS §2.6.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1)
/// - [JVMS §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
pub trait MaxLocals {
    /// Calculates the maximum number of local variable slots required by the method's code.
    ///
    /// This value accounts for the `this` reference (for instance methods), method parameters,
    /// and any local variables used by the instructions in the method body.
    /// `long` and `double` types are correctly handled as occupying two local variable slots.
    ///
    /// # Arguments
    ///
    /// * `constant_pool`: A reference to the `ConstantPool` of the class file, used to parse
    ///   the method descriptor for parameter types.
    /// * `method`: A reference to the `Method` structure, used to access its access flags
    ///   (to determine if it's static) and its descriptor index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The method descriptor index in the `method` is invalid or does not point to a
    ///   `CONSTANT_Utf8_info` in the `constant_pool`.
    /// - Parsing the method descriptor fails.
    /// - Any instruction's `max_locals_index()` method returns an error (e.g., type conversion).
    /// - The calculated `max_locals` value would exceed `u16::MAX` (although this is unlikely
    ///   as individual local indices are `u16`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{ConstantPool, Method, MethodAccessFlags};
    /// use ristretto_classfile::attributes::{Instruction, MaxLocals};
    /// use std::collections::HashMap;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let descriptor_idx = constant_pool.add_utf8("(Ljava/lang/String;I)V")?;
    ///
    /// let instance_method = Method {
    ///     access_flags: MethodAccessFlags::PUBLIC,
    ///     name_index: constant_pool.add_utf8("myMethod")?,
    ///     descriptor_index: descriptor_idx,
    ///     attributes: vec![],
    /// };
    ///
    /// // `this` (slot 0), String (slot 1), int (slot 2)
    /// // Instructions use up to slot 3 for an additional int.
    /// let instructions = [Instruction::Aload_1, Instruction::Iload_2, Instruction::Istore(3)];
    /// // Max index used by params: 0 (this) + 1 (String) + 1 (int) = 3 slots (0,1,2)
    /// // Max index by instructions: istore(3) uses slot 3.
    /// // So, total slots needed = 4 (indices 0, 1, 2, 3)
    /// assert_eq!(instructions.max_locals(&constant_pool, &instance_method)?, 4);
    ///
    /// let static_method = Method {
    ///     access_flags: MethodAccessFlags::STATIC | MethodAccessFlags::PUBLIC,
    ///     name_index: constant_pool.add_utf8("staticMethod")?,
    ///     descriptor_index: descriptor_idx,
    ///     attributes: vec![],
    /// };
    /// // String (slot 0), int (slot 1)
    /// // Instructions use up to slot 2 for an additional int.
    /// let static_instructions = [Instruction::Aload_0, Instruction::Iload_1, Instruction::Istore(2)];
    /// // Max index used by params: 1 (String) + 1 (int) = 2 slots (0,1)
    /// // Max index by instructions: istore(2) uses slot 2.
    /// // So, total slots needed = 3 (indices 0, 1, 2)
    /// assert_eq!(static_instructions.max_locals(&constant_pool, &static_method)?, 3);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn max_locals(&self, constant_pool: &ConstantPool, method: &Method) -> Result<u16>;
}

impl MaxLocals for [Instruction] {
    #[expect(clippy::bool_to_int_with_if)]
    fn max_locals(&self, constant_pool: &ConstantPool, method: &Method) -> Result<u16> {
        let mut max_locals: u16 = if method.access_flags.contains(MethodAccessFlags::STATIC) {
            // 'this' reference is not present in static methods
            0
        } else {
            // 'this' reference for instance methods
            1
        };

        let method_descriptor = constant_pool.try_get_utf8(method.descriptor_index)?;
        let (parameters, _return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
        for parameter in parameters {
            match parameter {
                FieldType::Base(BaseType::Double | BaseType::Long) => {
                    // Double and long types take 2 slots each
                    max_locals = max_locals.saturating_add(2);
                }
                _ => {
                    // Other types take 1 slot each
                    max_locals = max_locals.saturating_add(1);
                }
            }
        }

        for instruction in self {
            if let Some(local_index) = instruction.max_locals_index()? {
                // Add 1 to convert from 0-based index to a count
                let max_local_index = local_index.saturating_add(1);
                max_locals = max_locals.max(max_local_index);
            }
        }

        Ok(max_locals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::Instruction;

    fn get_max_locals(
        access_flags: MethodAccessFlags,
        descriptor: &str,
        instructions: &[Instruction],
    ) -> Result<u16> {
        let mut constant_pool = ConstantPool::new();
        let descriptor_index = constant_pool.add_utf8(descriptor)?;
        let method = Method {
            access_flags,
            descriptor_index,
            ..Default::default()
        };
        instructions.max_locals(&constant_pool, &method)
    }

    #[test]
    fn test_static() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::STATIC, "()V", &[])?;
        assert_eq!(0, max_locals);
        Ok(())
    }

    #[test]
    fn test_static_with_int_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::STATIC, "(I)V", &[])?;
        assert_eq!(1, max_locals);
        Ok(())
    }

    #[test]
    fn test_static_with_double_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::STATIC, "(D)V", &[])?;
        assert_eq!(2, max_locals);
        Ok(())
    }

    #[test]
    fn test_static_with_long_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::STATIC, "(J)V", &[])?;
        assert_eq!(2, max_locals);
        Ok(())
    }

    #[test]
    fn test_virtual() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::empty(), "()V", &[])?;
        assert_eq!(1, max_locals);
        Ok(())
    }

    #[test]
    fn test_virtual_with_int_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::empty(), "(I)V", &[])?;
        assert_eq!(2, max_locals);
        Ok(())
    }

    #[test]
    fn test_virtual_with_double_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::empty(), "(D)V", &[])?;
        assert_eq!(3, max_locals);
        Ok(())
    }

    #[test]
    fn test_virtual_with_long_parameter() -> Result<()> {
        let max_locals = get_max_locals(MethodAccessFlags::empty(), "(J)V", &[])?;
        assert_eq!(3, max_locals);
        Ok(())
    }

    #[test]
    fn test_instruction_int_constant() -> Result<()> {
        let max_locals = get_max_locals(
            MethodAccessFlags::empty(),
            "()V",
            &[Instruction::Iconst_0, Instruction::Iload_0],
        )?;
        assert_eq!(1, max_locals);
        Ok(())
    }

    #[test]
    fn test_instruction_long_constant() -> Result<()> {
        let max_locals = get_max_locals(
            MethodAccessFlags::empty(),
            "()V",
            &[Instruction::Lconst_0, Instruction::Lload_0],
        )?;
        assert_eq!(2, max_locals);
        Ok(())
    }

    #[test]
    fn test_parameters_and_instructions() -> Result<()> {
        let instructions = [
            Instruction::Iload_0,
            Instruction::Lload_1,
            Instruction::Return,
        ];
        let max_locals = get_max_locals(MethodAccessFlags::empty(), "(IJ)V", &instructions)?;
        assert_eq!(4, max_locals);
        Ok(())
    }
}

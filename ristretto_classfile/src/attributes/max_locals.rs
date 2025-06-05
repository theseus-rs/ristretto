use crate::attributes::Instruction;
use crate::{BaseType, ConstantPool, FieldType, Method, MethodAccessFlags, Result};

/// Trait for calculating the maximum number of local variables needed by a method.
///
/// The maximum locals value represents the number of local variable slots required by a method,
/// including:
/// - The `this` reference for instance methods (not present in static methods)
/// - Method parameters (with `long` and `double` types taking two slots each)
/// - Local variables used within the method body
///
/// This value is used in the JVM to allocate the correct frame size when executing methods.
///
/// # Example
///
/// ```rust
/// use ristretto_classfile::{ConstantPool, Method, MethodAccessFlags};
/// use ristretto_classfile::attributes::{Instruction, MaxLocals};
///
/// // Create a constant pool and add a method descriptor
/// let mut constant_pool = ConstantPool::new();
/// let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
///
/// // Create a non-static method (includes 'this' reference)
/// let method = Method {
///     access_flags: MethodAccessFlags::empty(), // instance method
///     name_index: 0, // Not relevant for this example
///     descriptor_index,
///     attributes: vec![],
/// };
///
/// // Create instructions that use local variables
/// let instructions = [
///     Instruction::Iload_0, // Load 'this' reference
///     Instruction::Iload_1, // Load int parameter
///     Instruction::Lload_2, // Load long parameter (takes slots 2 and 3)
///     Instruction::Istore(5), // Store value in local variable 5
///     Instruction::Return,
/// ];
///
/// // Calculate max locals
/// let max_locals = instructions.max_locals(&constant_pool, &method)?;
/// assert_eq!(max_locals, 6); // Slots 0-5 are used (6 total slots)
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3>
pub trait MaxLocals {
    /// Calculates the maximum number of local variable slots required by a method.
    ///
    /// This method analyzes both the method signature and its bytecode instructions to determine
    /// the maximum number of local variable slots that will be used:
    /// - The `this` reference for instance methods (not present in static methods)
    /// - Method parameters (with `long` and `double` types taking two slots each)
    /// - Local variables used within the method body
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use ristretto_classfile::{ConstantPool, Method, MethodAccessFlags};
    /// use ristretto_classfile::attributes::{Instruction, MaxLocals};
    ///
    /// // Create a constant pool and add a method descriptor
    /// let mut constant_pool = ConstantPool::new();
    /// let descriptor_index = constant_pool.add_utf8("(IJ)V")?;
    ///
    /// // Create a non-static method (includes 'this' reference)
    /// let method = Method {
    ///     access_flags: MethodAccessFlags::empty(), // instance method
    ///     name_index: 0, // Not relevant for this example
    ///     descriptor_index,
    ///     attributes: vec![],
    /// };
    ///
    /// // Create instructions that use local variables
    /// let instructions = [
    ///     Instruction::Iload_0, // Load 'this' reference
    ///     Instruction::Iload_1, // Load int parameter
    ///     Instruction::Lload_2, // Load long parameter (takes slots 2 and 3)
    ///     Instruction::Istore(5), // Store value in local variable 5
    ///     Instruction::Return,
    /// ];
    ///
    /// // Calculate max locals
    /// let max_locals = instructions.max_locals(&constant_pool, &method)?;
    /// assert_eq!(max_locals, 6); // Slots 0-5 are used (6 total slots)
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// if the locals size exceeds `u16::MAX`
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

//! # Miscellaneous Instruction Handlers
//!
//! Handles verification of miscellaneous instructions:
//! - `nop` - no operation
//! - `monitorenter`, `monitorexit` - synchronization
//! - Constants: `aconst_null`, `iconst_*`, `lconst_*`, `fconst_*`, `dconst_*`
//! - `bipush`, `sipush` - push byte/short
//! - `ldc`, `ldc_w`, `ldc2_w` - load constant
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use std::sync::Arc;

use crate::attributes::Instruction;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `nop` - no operation.
///
/// # Errors
///
/// This function never returns an error.
///
/// # References
///
/// - [JVMS §6.5.nop](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.nop)
#[expect(clippy::unnecessary_wraps)]
#[inline]
pub fn handle_nop() -> Result<()> {
    Ok(())
}

/// Handles `aconst_null` - push null reference.
///
/// Stack: ... → ..., null
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.aconst_null](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aconst_null)
pub fn handle_aconst_null(frame: &mut Frame) -> Result<()> {
    frame.push(VerificationType::Null)
}

/// Handles `iconst_*` instructions - push int constant.
///
/// Stack: ... → ..., value
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.iconst_i](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iconst_i)
pub fn handle_iconst(frame: &mut Frame) -> Result<()> {
    frame.push(VerificationType::Integer)
}

/// Handles `lconst_*` instructions - push long constant.
///
/// Stack: ... → ..., value
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.lconst_l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lconst_l)
pub fn handle_lconst(frame: &mut Frame) -> Result<()> {
    frame.push_category2(VerificationType::Long)
}

/// Handles `fconst_*` instructions - push float constant.
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.fconst_f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fconst_f)
pub fn handle_fconst(frame: &mut Frame) -> Result<()> {
    frame.push(VerificationType::Float)
}

/// Handles `dconst_*` instructions - push double constant.
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.dconst_d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dconst_d)
pub fn handle_dconst(frame: &mut Frame) -> Result<()> {
    frame.push_category2(VerificationType::Double)
}

/// Handles `bipush` and `sipush` - push byte/short as int.
///
/// # Errors
///
/// Returns an error if stack overflow occurs.
///
/// # References
///
/// - [JVMS §6.5.bipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bipush)
/// - [JVMS §6.5.sipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sipush)
pub fn handle_push_int(frame: &mut Frame) -> Result<()> {
    frame.push(VerificationType::Integer)
}

/// Handles `ldc` and `ldc_w` - load constant from pool.
///
/// # Errors
///
/// Returns an error if the constant pool index is invalid.
///
/// # References
///
/// - [JVMS §6.5.ldc](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldc)
/// - [JVMS§6.5.ldc_w](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldc_w)
pub fn handle_ldc(frame: &mut Frame, class_file: &ClassFile, index: u16) -> Result<()> {
    let constant = class_file
        .constant_pool
        .get(index)
        .ok_or(VerifyError::InvalidConstantPoolIndex(index))?;

    match constant {
        Constant::Integer(_) => frame.push(VerificationType::Integer),
        Constant::Float(_) => frame.push(VerificationType::Float),
        Constant::String(_) => frame.push(VerificationType::java_lang_string()),
        Constant::Class(_) => frame.push(VerificationType::java_lang_class()),
        Constant::MethodHandle { .. } => frame.push(VerificationType::Object(Arc::from(
            "java/lang/invoke/MethodHandle",
        ))),
        Constant::MethodType { .. } => frame.push(VerificationType::Object(Arc::from(
            "java/lang/invoke/MethodType",
        ))),
        Constant::Dynamic { .. } => {
            // Dynamic constant - would need to resolve to determine type
            // For now, push Object
            frame.push(VerificationType::java_lang_object())
        }
        _ => Err(VerifyError::VerifyError(format!(
            "ldc: unsupported constant type at index {index}"
        ))),
    }
}

/// Handles `ldc2_w` - load long or double constant from pool.
///
/// # Errors
///
/// Returns an error if the constant pool index is invalid.
///
/// # References
///
/// - [JVMS §6.5.ldc2_w](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldc2_w)
pub fn handle_ldc2_w(frame: &mut Frame, class_file: &ClassFile, index: u16) -> Result<()> {
    let constant = class_file
        .constant_pool
        .get(index)
        .ok_or(VerifyError::InvalidConstantPoolIndex(index))?;

    match constant {
        Constant::Long(_) => frame.push_category2(VerificationType::Long),
        Constant::Double(_) => frame.push_category2(VerificationType::Double),
        _ => Err(VerifyError::VerifyError(format!(
            "ldc2_w: expected long or double constant at index {index}"
        ))),
    }
}

/// Handles `monitorenter` - enter monitor.
///
/// Stack: ..., objectref → ...
///
/// # Errors
///
/// Returns an error if the operand is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.monitorenter](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.monitorenter)
pub fn handle_monitorenter(frame: &mut Frame) -> Result<()> {
    let objectref = frame.pop()?;
    if !objectref.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "monitorenter: expected reference, got {objectref}"
        )));
    }
    Ok(())
}

/// Handles `monitorexit` - exit monitor.
///
/// Stack: ..., objectref → ...
///
/// # Errors
///
/// Returns an error if the operand is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.monitorexit](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.monitorexit)
pub fn handle_monitorexit(frame: &mut Frame) -> Result<()> {
    let objectref = frame.pop()?;
    if !objectref.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "monitorexit: expected reference, got {objectref}"
        )));
    }
    Ok(())
}

/// Handles `wide` prefix.
///
/// The wide instruction modifies the next instruction to use a 16-bit index.
/// In our representation, wide variants are separate instructions (e.g., `iload_w`).
///
/// # Errors
///
/// This function never returns an error.
///
/// # References
///
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
#[expect(clippy::unnecessary_wraps)]
pub fn handle_wide() -> Result<()> {
    // Wide is handled by the instruction representation
    // (e.g., Iload_w instead of Wide + Iload)
    Ok(())
}

/// Handles reserved instructions (breakpoint, impdep1, impdep2).
///
/// These are implementation-dependent and generally not allowed.
///
/// # Errors
///
/// Returns an error for implementation-dependent instructions.
pub fn handle_reserved(instruction: &Instruction) -> Result<()> {
    match instruction {
        Instruction::Breakpoint => {
            // Breakpoint is for debuggers - typically a no-op for verification
            Ok(())
        }
        Instruction::Impdep1 | Instruction::Impdep2 => {
            // Implementation-dependent - reject
            Err(VerifyError::VerifyError(
                "Implementation-dependent instructions are not allowed".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

/// Dispatches miscellaneous instructions.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_misc(
    instruction: &Instruction,
    frame: &mut Frame,
    class_file: &ClassFile,
) -> Result<bool> {
    match instruction {
        // No-op
        Instruction::Nop => handle_nop()?,

        // Null constant
        Instruction::Aconst_null => handle_aconst_null(frame)?,

        // Integer constants
        Instruction::Iconst_m1
        | Instruction::Iconst_0
        | Instruction::Iconst_1
        | Instruction::Iconst_2
        | Instruction::Iconst_3
        | Instruction::Iconst_4
        | Instruction::Iconst_5 => handle_iconst(frame)?,

        // Long constants
        Instruction::Lconst_0 | Instruction::Lconst_1 => handle_lconst(frame)?,

        // Float constants
        Instruction::Fconst_0 | Instruction::Fconst_1 | Instruction::Fconst_2 => {
            handle_fconst(frame)?;
        }

        // Double constants
        Instruction::Dconst_0 | Instruction::Dconst_1 => handle_dconst(frame)?,

        // Push byte/short
        Instruction::Bipush(_) | Instruction::Sipush(_) => handle_push_int(frame)?,

        // Load constant
        Instruction::Ldc(index) => handle_ldc(frame, class_file, u16::from(*index))?,
        Instruction::Ldc_w(index) => handle_ldc(frame, class_file, *index)?,
        Instruction::Ldc2_w(index) => handle_ldc2_w(frame, class_file, *index)?,

        // Monitor operations
        Instruction::Monitorenter => handle_monitorenter(frame)?,
        Instruction::Monitorexit => handle_monitorexit(frame)?,

        // Wide prefix
        Instruction::Wide => handle_wide()?,

        // Reserved
        Instruction::Breakpoint | Instruction::Impdep1 | Instruction::Impdep2 => {
            handle_reserved(instruction)?;
        }

        // Not a misc instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version;
    use crate::constant_pool::ConstantPool;

    fn create_test_class_file() -> ClassFile {
        let mut constant_pool = ConstantPool::default();
        // Index 1: Utf8 "Test"
        constant_pool
            .add(Constant::Utf8("Test".to_string()))
            .unwrap();
        // Index 2: Class(1)
        constant_pool.add(Constant::Class(1)).unwrap();
        // Index 3: Integer(42)
        constant_pool.add(Constant::Integer(42)).unwrap();
        // Index 4: Float(3.14)
        constant_pool
            .add(Constant::Float(std::f32::consts::PI))
            .unwrap();
        // Index 5-6: Long(100) - takes 2 slots
        constant_pool.add(Constant::Long(100)).unwrap();
        // Index 7-8: Double(2.718) - takes 2 slots
        constant_pool
            .add(Constant::Double(std::f64::consts::E))
            .unwrap();
        // Index 9: Utf8 "Hello"
        constant_pool
            .add(Constant::Utf8("Hello".to_string()))
            .unwrap();
        // Index 10: String(9) - references Utf8 at index 9
        constant_pool.add(Constant::String(9)).unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: 2,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    // ==================== handle_nop tests ====================

    #[test]
    fn test_nop_success() {
        assert!(handle_nop().is_ok());
    }

    // ==================== handle_aconst_null tests ====================

    #[test]
    fn test_aconst_null_success() {
        let mut frame = Frame::new(5, 10);
        handle_aconst_null(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Null);
    }

    // ==================== handle_iconst tests ====================

    #[test]
    fn test_iconst_success() {
        let mut frame = Frame::new(5, 10);
        handle_iconst(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    // ==================== handle_lconst tests ====================

    #[test]
    fn test_lconst_success() {
        let mut frame = Frame::new(5, 10);
        handle_lconst(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    // ==================== handle_fconst tests ====================

    #[test]
    fn test_fconst_success() {
        let mut frame = Frame::new(5, 10);
        handle_fconst(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    // ==================== handle_dconst tests ====================

    #[test]
    fn test_dconst_success() {
        let mut frame = Frame::new(5, 10);
        handle_dconst(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    // ==================== handle_push_int tests ====================

    #[test]
    fn test_push_int_success() {
        let mut frame = Frame::new(5, 10);
        handle_push_int(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    // ==================== handle_ldc tests ====================

    #[test]
    fn test_ldc_integer_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        handle_ldc(&mut frame, &class_file, 3).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_ldc_float_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        handle_ldc(&mut frame, &class_file, 4).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_ldc_string_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        handle_ldc(&mut frame, &class_file, 10).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::java_lang_string());
    }

    #[test]
    fn test_ldc_class_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        handle_ldc(&mut frame, &class_file, 2).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::java_lang_class());
    }

    #[test]
    fn test_ldc_invalid_index_fails() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let result = handle_ldc(&mut frame, &class_file, 999);
        assert!(result.is_err());
    }

    #[test]
    fn test_ldc_long_not_allowed_fails() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        // Long at index 5 - should fail for ldc (use ldc2_w instead)
        let result = handle_ldc(&mut frame, &class_file, 5);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unsupported constant type")
        );
    }

    // ==================== handle_ldc2_w tests ====================

    #[test]
    fn test_ldc2_w_long_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        // Long is at index 5
        handle_ldc2_w(&mut frame, &class_file, 5).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_ldc2_w_double_success() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        // Double is at index 7
        handle_ldc2_w(&mut frame, &class_file, 7).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_ldc2_w_invalid_index_fails() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let result = handle_ldc2_w(&mut frame, &class_file, 999);
        assert!(result.is_err());
    }

    #[test]
    fn test_ldc2_w_integer_not_allowed_fails() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        // Integer at index 3 - should fail for ldc2_w (use ldc instead)
        let result = handle_ldc2_w(&mut frame, &class_file, 3);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected long or double")
        );
    }

    // ==================== handle_monitorenter tests ====================

    #[test]
    fn test_monitorenter_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_monitorenter(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_monitorenter_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_monitorenter(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_monitorenter_non_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_monitorenter(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    // ==================== handle_monitorexit tests ====================

    #[test]
    fn test_monitorexit_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_monitorexit(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_monitorexit_non_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_monitorexit(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    // ==================== handle_wide tests ====================

    #[test]
    fn test_wide_success() {
        assert!(handle_wide().is_ok());
    }

    // ==================== handle_reserved tests ====================

    #[test]
    fn test_reserved_breakpoint_success() {
        assert!(handle_reserved(&Instruction::Breakpoint).is_ok());
    }

    #[test]
    fn test_reserved_impdep1_fails() {
        let result = handle_reserved(&Instruction::Impdep1);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Implementation-dependent")
        );
    }

    #[test]
    fn test_reserved_impdep2_fails() {
        let result = handle_reserved(&Instruction::Impdep2);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Implementation-dependent")
        );
    }

    // ==================== dispatch_misc tests ====================

    #[test]
    fn test_dispatch_nop() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Nop, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_aconst_null() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Aconst_null, &mut frame, &class_file).unwrap();
        assert!(handled);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Null);
    }

    #[test]
    fn test_dispatch_iconst() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Iconst_0, &mut frame, &class_file).unwrap();
        assert!(handled);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_dispatch_lconst() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Lconst_0, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fconst() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Fconst_0, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dconst() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Dconst_0, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_bipush() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Bipush(42), &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_sipush() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Sipush(1000), &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ldc() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Ldc(3), &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ldc_w() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Ldc_w(3), &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ldc2_w() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Ldc2_w(5), &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_monitorenter() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_misc(&Instruction::Monitorenter, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_monitorexit() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_misc(&Instruction::Monitorexit, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_wide() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Wide, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_breakpoint() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Breakpoint, &mut frame, &class_file).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_impdep1_fails() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let result = dispatch_misc(&Instruction::Impdep1, &mut frame, &class_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_non_misc() {
        let class_file = create_test_class_file();
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_misc(&Instruction::Iadd, &mut frame, &class_file).unwrap();
        assert!(!handled);
    }
}

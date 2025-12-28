//! # Arithmetic and Logical Instruction Handlers
//!
//! Handles verification of arithmetic and logical instructions:
//! - Binary operations: `iadd`, `isub`, `imul`, `idiv`, `irem`, etc.
//! - Unary operations: `ineg`, `lneg`, `fneg`, `dneg`
//! - Bitwise operations: `iand`, `ior`, `ixor`, `ishl`, `ishr`, `iushr`, etc.
//! - Increment: `iinc`
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

// ============================================================================
// Integer Arithmetic
// ============================================================================

/// Handles integer binary operations (iadd, isub, imul, idiv, irem).
///
/// Stack: ..., value1, value2 → ..., result
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.iadd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iadd)
pub fn handle_int_binary(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop()?;
    if value2 != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "Integer operation: expected int, got {value2}"
        )));
    }

    let value1 = frame.pop()?;
    if value1 != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "Integer operation: expected int, got {value1}"
        )));
    }

    frame.push(VerificationType::Integer)
}

/// Handles integer unary operations (ineg).
///
/// Stack: ..., value → ..., result
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.ineg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ineg)
pub fn handle_int_unary(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "Integer operation: expected int, got {value}"
        )));
    }

    frame.push(VerificationType::Integer)
}

// ============================================================================
// Long Arithmetic
// ============================================================================

/// Handles long binary operations (ladd, lsub, lmul, ldiv, lrem, land, lor, lxor).
///
/// Stack: ..., value1, value2 → ..., result
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.ladd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ladd)
pub fn handle_long_binary(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop_category2()?;
    if value2 != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "Long operation: expected long, got {value2}"
        )));
    }

    let value1 = frame.pop_category2()?;
    if value1 != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "Long operation: expected long, got {value1}"
        )));
    }

    frame.push_category2(VerificationType::Long)
}

/// Handles long shift operations (lshl, lshr, lushr).
///
/// Stack: ..., value1 (long), value2 (int) → ..., result (long)
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.lshl](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lshl)
pub fn handle_long_shift(frame: &mut Frame) -> Result<()> {
    // Shift amount is int
    let shift = frame.pop()?;
    if shift != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "Long shift: expected int shift amount, got {shift}"
        )));
    }

    // Value to shift is long
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "Long shift: expected long value, got {value}"
        )));
    }

    frame.push_category2(VerificationType::Long)
}

/// Handles long unary operations (lneg).
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.lneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lneg)
pub fn handle_long_unary(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "Long operation: expected long, got {value}"
        )));
    }

    frame.push_category2(VerificationType::Long)
}

// ============================================================================
// Float Arithmetic
// ============================================================================

/// Handles float binary operations (fadd, fsub, fmul, fdiv, frem).
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.fadd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fadd)
pub fn handle_float_binary(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop()?;
    if value2 != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "Float operation: expected float, got {value2}"
        )));
    }

    let value1 = frame.pop()?;
    if value1 != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "Float operation: expected float, got {value1}"
        )));
    }

    frame.push(VerificationType::Float)
}

/// Handles float unary operations (fneg).
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.fneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fneg)
pub fn handle_float_unary(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "Float operation: expected float, got {value}"
        )));
    }

    frame.push(VerificationType::Float)
}

// ============================================================================
// Double Arithmetic
// ============================================================================

/// Handles double binary operations (dadd, dsub, dmul, ddiv, drem).
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.dadd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dadd)
pub fn handle_double_binary(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop_category2()?;
    if value2 != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "Double operation: expected double, got {value2}"
        )));
    }

    let value1 = frame.pop_category2()?;
    if value1 != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "Double operation: expected double, got {value1}"
        )));
    }

    frame.push_category2(VerificationType::Double)
}

/// Handles double unary operations (dneg).
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.dneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dneg)
pub fn handle_double_unary(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "Double operation: expected double, got {value}"
        )));
    }

    frame.push_category2(VerificationType::Double)
}

// ============================================================================
// Increment
// ============================================================================

/// Handles `iinc` - increment local variable by constant.
///
/// This does not affect the operand stack.
///
/// # Errors
///
/// Returns an error if the local variable is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.iinc](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iinc)
pub fn handle_iinc(frame: &mut Frame, index: u16) -> Result<()> {
    let local_type = frame.get_local(index)?;
    if *local_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "iinc: expected int at local {index}, got {local_type}"
        )));
    }
    // iinc doesn't change the type, just the value
    Ok(())
}

// ============================================================================
// Dispatcher
// ============================================================================

/// Dispatches arithmetic instructions to their handlers.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_math(instruction: &Instruction, frame: &mut Frame) -> Result<bool> {
    match instruction {
        // Integer binary
        Instruction::Iadd
        | Instruction::Isub
        | Instruction::Imul
        | Instruction::Idiv
        | Instruction::Irem
        | Instruction::Iand
        | Instruction::Ior
        | Instruction::Ixor
        | Instruction::Ishl
        | Instruction::Ishr
        | Instruction::Iushr => handle_int_binary(frame)?,

        // Integer unary
        Instruction::Ineg => handle_int_unary(frame)?,

        // Long binary
        Instruction::Ladd
        | Instruction::Lsub
        | Instruction::Lmul
        | Instruction::Ldiv
        | Instruction::Lrem
        | Instruction::Land
        | Instruction::Lor
        | Instruction::Lxor => handle_long_binary(frame)?,

        // Long shift (int shift amount)
        Instruction::Lshl | Instruction::Lshr | Instruction::Lushr => handle_long_shift(frame)?,

        // Long unary
        Instruction::Lneg => handle_long_unary(frame)?,

        // Float binary
        Instruction::Fadd
        | Instruction::Fsub
        | Instruction::Fmul
        | Instruction::Fdiv
        | Instruction::Frem => handle_float_binary(frame)?,

        // Float unary
        Instruction::Fneg => handle_float_unary(frame)?,

        // Double binary
        Instruction::Dadd
        | Instruction::Dsub
        | Instruction::Dmul
        | Instruction::Ddiv
        | Instruction::Drem => handle_double_binary(frame)?,

        // Double unary
        Instruction::Dneg => handle_double_unary(frame)?,

        // Increment
        Instruction::Iinc(index, _) => handle_iinc(frame, u16::from(*index))?,
        Instruction::Iinc_w(index, _) => handle_iinc(frame, *index)?,

        // Not a math instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== handle_int_binary tests ====================

    #[test]
    fn test_int_binary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_int_binary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_int_binary_value2_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_int_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_int_binary_value1_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_int_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    // ==================== handle_int_unary tests ====================

    #[test]
    fn test_int_unary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_int_unary(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_int_unary_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_int_unary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    // ==================== handle_long_binary tests ====================

    #[test]
    fn test_long_binary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        handle_long_binary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_long_binary_value2_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_long_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_long_binary_value1_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_long_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    // ==================== handle_long_shift tests ====================

    #[test]
    fn test_long_shift_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap(); // shift amount

        handle_long_shift(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_long_shift_wrong_shift_amount_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_long_shift(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int shift amount")
        );
    }

    #[test]
    fn test_long_shift_wrong_value_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_long_shift(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected long value")
        );
    }

    // ==================== handle_long_unary tests ====================

    #[test]
    fn test_long_unary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_long_unary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_long_unary_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_long_unary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    // ==================== handle_float_binary tests ====================

    #[test]
    fn test_float_binary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_float_binary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
    }

    #[test]
    fn test_float_binary_value2_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_float_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_float_binary_value1_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_float_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    // ==================== handle_float_unary tests ====================

    #[test]
    fn test_float_unary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_float_unary(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_float_unary_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_float_unary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    // ==================== handle_double_binary tests ====================

    #[test]
    fn test_double_binary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        handle_double_binary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_double_binary_value2_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_double_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_double_binary_value1_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_double_binary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    // ==================== handle_double_unary tests ====================

    #[test]
    fn test_double_unary_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_double_unary(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_double_unary_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_double_unary(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    // ==================== handle_iinc tests ====================

    #[test]
    fn test_iinc_success() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Integer).unwrap();

        handle_iinc(&mut frame, 2).unwrap();
        // Stack should be unchanged
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_iinc_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Float).unwrap();

        let result = handle_iinc(&mut frame, 2);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    // ==================== dispatch_math tests ====================

    #[test]
    fn test_dispatch_iadd() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Iadd, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 1);
    }

    #[test]
    fn test_dispatch_isub() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Isub, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_imul() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Imul, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ineg() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Ineg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ladd() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_math(&Instruction::Ladd, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lshl() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Lshl, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lneg() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_math(&Instruction::Lneg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fadd() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_math(&Instruction::Fadd, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fneg() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_math(&Instruction::Fneg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dadd() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_math(&Instruction::Dadd, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dneg() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_math(&Instruction::Dneg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_iinc() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Iinc(2, 1), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_iinc_w() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Integer).unwrap();

        let handled = dispatch_math(&Instruction::Iinc_w(2, 100), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_non_math() {
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_math(&Instruction::Nop, &mut frame).unwrap();
        assert!(!handled);
    }
}

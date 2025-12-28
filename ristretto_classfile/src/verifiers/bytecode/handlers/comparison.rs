//! # Comparison Instruction Handlers
//!
//! Handles verification of comparison instructions:
//! - Numeric comparisons: `lcmp`, `fcmpl`, `fcmpg`, `dcmpl`, `dcmpg`
//! - Conditional branches: `ifeq`, `ifne`, `iflt`, etc.
//! - Integer comparisons: `if_icmpeq`, `if_icmpne`, etc.
//! - Reference comparisons: `if_acmpeq`, `if_acmpne`
//! - Null checks: `ifnull`, `ifnonnull`
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `lcmp` - compare two long values.
///
/// Stack: ..., value1, value2 → ..., result
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.lcmp](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lcmp)
pub fn handle_lcmp(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop_category2()?;
    if value2 != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lcmp: expected long, got {value2}"
        )));
    }

    let value1 = frame.pop_category2()?;
    if value1 != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lcmp: expected long, got {value1}"
        )));
    }

    // Result is int: -1, 0, or 1
    frame.push(VerificationType::Integer)
}

/// Handles `fcmpl` and `fcmpg` - compare two float values.
///
/// Stack: ..., value1, value2 → ..., result
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.fcmpl](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fcmpl)
/// - [JVMS §6.5.fcmpg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fcmpg)
pub fn handle_fcmp(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop()?;
    if value2 != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "fcmp: expected float, got {value2}"
        )));
    }

    let value1 = frame.pop()?;
    if value1 != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "fcmp: expected float, got {value1}"
        )));
    }

    frame.push(VerificationType::Integer)
}

/// Handles `dcmpl` and `dcmpg` - compare two double values.
///
/// Stack: ..., value1, value2 → ..., result
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.dcmpl](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dcmpl)
/// - [JVMS §6.5.dcmpg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dcmpg)
pub fn handle_dcmp(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop_category2()?;
    if value2 != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dcmp: expected double, got {value2}"
        )));
    }

    let value1 = frame.pop_category2()?;
    if value1 != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dcmp: expected double, got {value1}"
        )));
    }

    frame.push(VerificationType::Integer)
}

/// Handles single-value integer conditional branches.
///
/// `ifeq`, `ifne`, `iflt`, `ifge`, `ifgt`, `ifle`
///
/// Stack: ..., value → ...
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub fn handle_if_int(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "if<cond>: expected int, got {value}"
        )));
    }
    Ok(())
}

/// Handles two-value integer comparison branches.
///
/// `if_icmpeq`, `if_icmpne`, `if_icmplt`, `if_icmpge`, `if_icmpgt`, `if_icmple`
///
/// Stack: ..., value1, value2 → ...
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub fn handle_if_icmp(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop()?;
    if value2 != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "if_icmp: expected int, got {value2}"
        )));
    }

    let value1 = frame.pop()?;
    if value1 != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "if_icmp: expected int, got {value1}"
        )));
    }

    Ok(())
}

/// Handles reference comparison branches.
///
/// `if_acmpeq`, `if_acmpne`
///
/// Stack: ..., value1, value2 → ...
///
/// # Errors
///
/// Returns an error if the operand types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.if_acmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_acmp_cond)
pub fn handle_if_acmp(frame: &mut Frame) -> Result<()> {
    let value2 = frame.pop()?;
    if !value2.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "if_acmp: expected reference, got {value2}"
        )));
    }

    let value1 = frame.pop()?;
    if !value1.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "if_acmp: expected reference, got {value1}"
        )));
    }

    Ok(())
}

/// Handles null check branches.
///
/// `ifnull`, `ifnonnull`
///
/// Stack: ..., value → ...
///
/// # Errors
///
/// Returns an error if the operand type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.ifnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnull)
/// - [JVMS §6.5.ifnonnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnonnull)
pub fn handle_ifnull(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if !value.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "ifnull/ifnonnull: expected reference, got {value}"
        )));
    }
    Ok(())
}

/// Dispatches comparison instructions to their handlers.
///
/// Returns `true` if the instruction was handled, `false` if it's not a comparison.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_comparison(instruction: &Instruction, frame: &mut Frame) -> Result<bool> {
    match instruction {
        // Long comparison
        Instruction::Lcmp => handle_lcmp(frame)?,

        // Float comparisons
        Instruction::Fcmpl | Instruction::Fcmpg => handle_fcmp(frame)?,

        // Double comparisons
        Instruction::Dcmpl | Instruction::Dcmpg => handle_dcmp(frame)?,

        // Single-value integer conditionals
        Instruction::Ifeq(_)
        | Instruction::Ifne(_)
        | Instruction::Iflt(_)
        | Instruction::Ifge(_)
        | Instruction::Ifgt(_)
        | Instruction::Ifle(_) => handle_if_int(frame)?,

        // Two-value integer comparisons
        Instruction::If_icmpeq(_)
        | Instruction::If_icmpne(_)
        | Instruction::If_icmplt(_)
        | Instruction::If_icmpge(_)
        | Instruction::If_icmpgt(_)
        | Instruction::If_icmple(_) => handle_if_icmp(frame)?,

        // Reference comparisons
        Instruction::If_acmpeq(_) | Instruction::If_acmpne(_) => handle_if_acmp(frame)?,

        // Null checks
        Instruction::Ifnull(_) | Instruction::Ifnonnull(_) => handle_ifnull(frame)?,

        // Not a comparison instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== handle_lcmp tests ====================

    #[test]
    fn test_lcmp_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        handle_lcmp(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_lcmp_value2_not_long_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_lcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_lcmp_value1_not_long_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_lcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    // ==================== handle_fcmp tests ====================

    #[test]
    fn test_fcmp_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_fcmp(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_fcmp_value2_not_float_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_fcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_fcmp_value1_not_float_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_fcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    // ==================== handle_dcmp tests ====================

    #[test]
    fn test_dcmp_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        handle_dcmp(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
    }

    #[test]
    fn test_dcmp_value2_not_double_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_dcmp_value1_not_double_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_dcmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    // ==================== handle_if_int tests ====================

    #[test]
    fn test_if_int_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_if_int(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_if_int_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_if_int(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_if_int_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_if_int(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    // ==================== handle_if_icmp tests ====================

    #[test]
    fn test_if_icmp_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_if_icmp(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_if_icmp_value2_not_int_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_if_icmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_if_icmp_value1_not_int_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_if_icmp(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    // ==================== handle_if_acmp tests ====================

    #[test]
    fn test_if_acmp_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Null).unwrap();

        handle_if_acmp(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_if_acmp_both_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();
        frame.push(VerificationType::Null).unwrap();

        handle_if_acmp(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_if_acmp_value2_not_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_if_acmp(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_if_acmp_value1_not_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_if_acmp(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    // ==================== handle_ifnull tests ====================

    #[test]
    fn test_ifnull_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_ifnull(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_ifnull_with_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_ifnull(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_ifnull_with_uninitialized_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Uninitialized(0)).unwrap();

        handle_ifnull(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_ifnull_not_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_ifnull(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_ifnull_with_float_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_ifnull(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    // ==================== dispatch_comparison tests ====================

    #[test]
    fn test_dispatch_lcmp() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_comparison(&Instruction::Lcmp, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 1);
    }

    #[test]
    fn test_dispatch_fcmpl() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_comparison(&Instruction::Fcmpl, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fcmpg() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_comparison(&Instruction::Fcmpg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dcmpl() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_comparison(&Instruction::Dcmpl, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dcmpg() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_comparison(&Instruction::Dcmpg, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifeq() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifeq(10), &mut frame).unwrap();
        assert!(handled);
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_dispatch_ifne() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifne(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_iflt() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Iflt(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifge() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifge(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifgt() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifgt(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifle() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifle(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmpeq() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmpeq(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmpne() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmpne(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmplt() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmplt(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmpge() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmpge(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmpgt() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmpgt(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_icmple() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_comparison(&Instruction::If_icmple(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_acmpeq() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Null).unwrap();

        let handled = dispatch_comparison(&Instruction::If_acmpeq(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_if_acmpne() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Null).unwrap();

        let handled = dispatch_comparison(&Instruction::If_acmpne(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifnull() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifnull(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ifnonnull() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_comparison(&Instruction::Ifnonnull(10), &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_non_comparison_instruction() {
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_comparison(&Instruction::Nop, &mut frame).unwrap();
        assert!(!handled);
    }
}

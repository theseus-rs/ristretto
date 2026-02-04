//! # Type Conversion Instruction Handlers
//!
//! Handles verification of type conversion instructions:
//! - Widening: `i2l`, `i2f`, `i2d`, `l2f`, `l2d`, `f2d`
//! - Narrowing: `l2i`, `f2i`, `f2l`, `d2i`, `d2l`, `d2f`
//! - Integer narrowing: `i2b`, `i2c`, `i2s`
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `i2l` - convert int to long.
///
/// Stack: ..., value → ..., result
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2l)
pub fn handle_i2l(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2l: expected int, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Long)
}

/// Handles `i2f` - convert int to float.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2f)
pub fn handle_i2f(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2f: expected int, got {value}"
        )));
    }
    frame.push(VerificationType::Float)
}

/// Handles `i2d` - convert int to double.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2d)
pub fn handle_i2d(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2d: expected int, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Double)
}

/// Handles `l2i` - convert long to int.
///
/// # Errors
///
/// Returns an error if the stack value is not a `long`.
///
/// # References
///
/// - [JVMS §6.5.l2i](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.l2i)
pub fn handle_l2i(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "l2i: expected long, got {value}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Handles `l2f` - convert long to float.
///
/// # Errors
///
/// Returns an error if the stack value is not a `long`.
///
/// # References
///
/// - [JVMS §6.5.l2f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.l2f)
pub fn handle_l2f(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "l2f: expected long, got {value}"
        )));
    }
    frame.push(VerificationType::Float)
}

/// Handles `l2d` - convert long to double.
///
/// # Errors
///
/// Returns an error if the stack value is not a `long`.
///
/// # References
///
/// - [JVMS §6.5.l2d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.l2d)
pub fn handle_l2d(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "l2d: expected long, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Double)
}

/// Handles `f2i` - convert float to int.
///
/// # Errors
///
/// Returns an error if the stack value is not a `float`.
///
/// # References
///
/// - [JVMS §6.5.f2i](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.f2i)
pub fn handle_f2i(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "f2i: expected float, got {value}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Handles `f2l` - convert float to long.
///
/// # Errors
///
/// Returns an error if the stack value is not a `float`.
///
/// # References
///
/// - [JVMS §6.5.f2l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.f2l)
pub fn handle_f2l(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "f2l: expected float, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Long)
}

/// Handles `f2d` - convert float to double.
///
/// # Errors
///
/// Returns an error if the stack value is not a `float`.
///
/// # References
///
/// - [JVMS §6.5.f2d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.f2d)
pub fn handle_f2d(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "f2d: expected float, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Double)
}

/// Handles `d2i` - convert double to int.
///
/// # Errors
///
/// Returns an error if the stack value is not a `double`.
///
/// # References
///
/// - [JVMS §6.5.d2i](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.d2i)
pub fn handle_d2i(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "d2i: expected double, got {value}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Handles `d2l` - convert double to long.
///
/// # Errors
///
/// Returns an error if the stack value is not a `double`.
///
/// # References
///
/// - [JVMS §6.5.d2l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.d2l)
pub fn handle_d2l(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "d2l: expected double, got {value}"
        )));
    }
    frame.push_category2(VerificationType::Long)
}

/// Handles `d2f` - convert double to float.
///
/// # Errors
///
/// Returns an error if the stack value is not a `double`.
///
/// # References
///
/// - [JVMS §6.5.d2f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.d2f)
pub fn handle_d2f(frame: &mut Frame) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "d2f: expected double, got {value}"
        )));
    }
    frame.push(VerificationType::Float)
}

/// Handles `i2b` - convert int to byte.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2b](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2b)
pub fn handle_i2b(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2b: expected int, got {value}"
        )));
    }
    // Result is still int in the verification type system
    frame.push(VerificationType::Integer)
}

/// Handles `i2c` - convert int to char.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2c](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2c)
pub fn handle_i2c(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2c: expected int, got {value}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Handles `i2s` - convert int to short.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.i2s](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.i2s)
pub fn handle_i2s(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "i2s: expected int, got {value}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Dispatches type conversion instructions to their handlers.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_conversion(instruction: &Instruction, frame: &mut Frame) -> Result<bool> {
    match instruction {
        // Int conversions
        Instruction::I2l => handle_i2l(frame)?,
        Instruction::I2f => handle_i2f(frame)?,
        Instruction::I2d => handle_i2d(frame)?,
        Instruction::I2b => handle_i2b(frame)?,
        Instruction::I2c => handle_i2c(frame)?,
        Instruction::I2s => handle_i2s(frame)?,

        // Long conversions
        Instruction::L2i => handle_l2i(frame)?,
        Instruction::L2f => handle_l2f(frame)?,
        Instruction::L2d => handle_l2d(frame)?,

        // Float conversions
        Instruction::F2i => handle_f2i(frame)?,
        Instruction::F2l => handle_f2l(frame)?,
        Instruction::F2d => handle_f2d(frame)?,

        // Double conversions
        Instruction::D2i => handle_d2i(frame)?,
        Instruction::D2l => handle_d2l(frame)?,
        Instruction::D2f => handle_d2f(frame)?,

        // Not a conversion instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i2l_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2l(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_i2l_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2l(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_i2f_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2f(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_i2f_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2f(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_i2d_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2d(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Double takes 2 slots
    }

    #[test]
    fn test_i2d_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2d(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_l2i_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_l2i(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_l2i_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_l2i(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_l2f_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_l2f(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_l2f_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_l2f(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_l2d_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_l2d(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Double takes 2 slots
    }

    #[test]
    fn test_l2d_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_l2d(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_f2i_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_f2i(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_f2i_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_f2i(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_f2l_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_f2l(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_f2l_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_f2l(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_f2d_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_f2d(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_f2d_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_f2d(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_d2i_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_d2i(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_d2i_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_d2i(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_d2l_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_d2l(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_d2l_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_d2l(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_d2f_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_d2f(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 1);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_d2f_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_d2f(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_i2b_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2b(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_i2b_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2b(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_i2c_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2c(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_i2c_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2c(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_i2s_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_i2s(&mut frame).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_i2s_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_i2s(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_dispatch_i2l() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2l, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dispatch_i2f() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2f, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_i2d() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2d, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_i2b() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2b, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_i2c() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2c, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_i2s() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_conversion(&Instruction::I2s, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_l2i() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_conversion(&Instruction::L2i, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_l2f() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_conversion(&Instruction::L2f, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_l2d() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_conversion(&Instruction::L2d, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_f2i() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_conversion(&Instruction::F2i, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_f2l() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_conversion(&Instruction::F2l, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_f2d() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_conversion(&Instruction::F2d, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_d2i() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_conversion(&Instruction::D2i, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_d2l() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_conversion(&Instruction::D2l, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_d2f() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_conversion(&Instruction::D2f, &mut frame).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_non_conversion_instruction() {
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_conversion(&Instruction::Nop, &mut frame).unwrap();
        assert!(!handled);
    }
}

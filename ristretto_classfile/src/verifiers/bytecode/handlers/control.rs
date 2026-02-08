//! # Control Flow Instruction Handlers
//!
//! Handles verification of control flow instructions:
//! - Unconditional branches: `goto`, `goto_w`
//! - Returns: `return`, `ireturn`, `lreturn`, `freturn`, `dreturn`, `areturn`
//! - Switches: `tableswitch`, `lookupswitch`
//! - Subroutines: `jsr`, `ret` (rejected for class version >= 51.0)
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use crate::FieldType;
use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `return` - return void from method.
///
/// Stack: ... → \[empty\]
///
/// # Errors
///
/// This function never returns an error.
///
/// # References
///
/// - [JVMS §6.5.return](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.return)
#[expect(clippy::unnecessary_wraps)]
pub fn handle_return() -> Result<()> {
    // Nothing to verify - just returns void
    Ok(())
}

/// Handles `ireturn` - return int from method.
///
/// Stack: ..., value → \[empty\]
///
/// # Errors
///
/// Returns an error if the return value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.ireturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ireturn)
pub fn handle_ireturn(frame: &mut Frame, expected_return: Option<&FieldType>) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "ireturn: expected int, got {value}"
        )));
    }

    // Verify against method's return type
    if let Some(ret_type) = expected_return {
        let expected = VerificationType::from_field_type(ret_type);
        if expected != VerificationType::Integer {
            return Err(VerifyError::VerifyError(format!(
                "ireturn: method returns {ret_type}, not int"
            )));
        }
    }

    Ok(())
}

/// Handles `lreturn` - return long from method.
///
/// # Errors
///
/// Returns an error if the return value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.lreturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lreturn)
pub fn handle_lreturn(frame: &mut Frame, expected_return: Option<&FieldType>) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lreturn: expected long, got {value}"
        )));
    }

    if let Some(ret_type) = expected_return {
        let expected = VerificationType::from_field_type(ret_type);
        if expected != VerificationType::Long {
            return Err(VerifyError::VerifyError(format!(
                "lreturn: method returns {ret_type}, not long"
            )));
        }
    }

    Ok(())
}

/// Handles `freturn` - return float from method.
///
/// # Errors
///
/// Returns an error if the return value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.freturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.freturn)
pub fn handle_freturn(frame: &mut Frame, expected_return: Option<&FieldType>) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "freturn: expected float, got {value}"
        )));
    }

    if let Some(ret_type) = expected_return {
        let expected = VerificationType::from_field_type(ret_type);
        if expected != VerificationType::Float {
            return Err(VerifyError::VerifyError(format!(
                "freturn: method returns {ret_type}, not float"
            )));
        }
    }

    Ok(())
}

/// Handles `dreturn` - return double from method.
///
/// # Errors
///
/// Returns an error if the return value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.dreturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dreturn)
pub fn handle_dreturn(frame: &mut Frame, expected_return: Option<&FieldType>) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dreturn: expected double, got {value}"
        )));
    }

    if let Some(ret_type) = expected_return {
        let expected = VerificationType::from_field_type(ret_type);
        if expected != VerificationType::Double {
            return Err(VerifyError::VerifyError(format!(
                "dreturn: method returns {ret_type}, not double"
            )));
        }
    }

    Ok(())
}

/// Handles `areturn` - return reference from method.
///
/// # Errors
///
/// Returns an error if the return value type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.areturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.areturn)
pub fn handle_areturn<C: VerificationContext>(
    frame: &mut Frame,
    expected_return: Option<&FieldType>,
    context: &C,
) -> Result<()> {
    let value = frame.pop()?;
    if !value.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "areturn: expected reference, got {value}"
        )));
    }

    if let Some(ret_type) = expected_return {
        let expected = VerificationType::from_field_type(ret_type);
        if !value.is_assignable_to(&expected, context)? {
            return Err(VerifyError::VerifyError(format!(
                "areturn: {value} is not assignable to expected return type {ret_type}"
            )));
        }
    }

    Ok(())
}

/// Handles `tableswitch` - switch on int with table.
///
/// Stack: ..., index → ...
///
/// # Errors
///
/// Returns an error if the index is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.tableswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.tableswitch)
pub fn handle_tableswitch(frame: &mut Frame) -> Result<()> {
    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "tableswitch: expected int index, got {index}"
        )));
    }
    Ok(())
}

/// Handles `lookupswitch` - switch on int with lookup.
///
/// Stack: ..., key → ...
///
/// # Errors
///
/// Returns an error if the key is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.lookupswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lookupswitch)
pub fn handle_lookupswitch(frame: &mut Frame) -> Result<()> {
    let key = frame.pop()?;
    if key != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "lookupswitch: expected int key, got {key}"
        )));
    }
    Ok(())
}

/// Rejects `jsr` instruction for class files version >= 51.0.
///
/// # Errors
///
/// Always returns an error for modern class files.
///
/// # References
///
/// - [JVMS §4.9.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9.1)
pub fn reject_jsr(major_version: u16) -> Result<()> {
    if major_version >= 51 {
        Err(VerifyError::VerifyError(
            "jsr instruction is not allowed in class files version 51.0 or later".to_string(),
        ))
    } else {
        // For older class files, jsr would need special handling
        // but we can reject it for simplicity
        Err(VerifyError::VerifyError(
            "jsr/ret subroutines are not supported".to_string(),
        ))
    }
}

/// Rejects `ret` instruction for class files version >= 51.0.
///
/// # Errors
///
/// Always returns an error for modern class files.
///
/// # References
///
/// - [JVMS §4.9.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9.1)
pub fn reject_ret(major_version: u16) -> Result<()> {
    if major_version >= 51 {
        Err(VerifyError::VerifyError(
            "ret instruction is not allowed in class files version 51.0 or later".to_string(),
        ))
    } else {
        Err(VerifyError::VerifyError(
            "jsr/ret subroutines are not supported".to_string(),
        ))
    }
}

/// Dispatches control flow instructions.
///
/// Note: This doesn't handle successors - that's done separately.
/// This just verifies the stack effects.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_control<C: VerificationContext>(
    instruction: &Instruction,
    frame: &mut Frame,
    expected_return: Option<&FieldType>,
    major_version: u16,
    context: &C,
) -> Result<bool> {
    match instruction {
        // Unconditional branches (no stack effect)
        Instruction::Goto(_) | Instruction::Goto_w(_) => {}

        // Returns
        Instruction::Return => handle_return()?,
        Instruction::Ireturn => handle_ireturn(frame, expected_return)?,
        Instruction::Lreturn => handle_lreturn(frame, expected_return)?,
        Instruction::Freturn => handle_freturn(frame, expected_return)?,
        Instruction::Dreturn => handle_dreturn(frame, expected_return)?,
        Instruction::Areturn => handle_areturn(frame, expected_return, context)?,

        // Switches
        Instruction::Tableswitch(_) => handle_tableswitch(frame)?,
        Instruction::Lookupswitch(_) => handle_lookupswitch(frame)?,

        // Rejected instructions
        Instruction::Jsr(_) | Instruction::Jsr_w(_) => {
            reject_jsr(major_version)?;
        }
        Instruction::Ret(_) | Instruction::Ret_w(_) => {
            reject_ret(major_version)?;
        }

        // Not a control flow instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseType;
    use crate::attributes::{LookupSwitch, TableSwitch};
    use crate::verifiers::bytecode::handlers::test_utils::{MockContext, StrictMockContext};
    use indexmap::IndexMap;

    #[test]
    fn test_return_success() {
        assert!(handle_return().is_ok());
    }

    #[test]
    fn test_ireturn_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_ireturn(&mut frame, None).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_ireturn_with_expected_int() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let ret_type = FieldType::Base(BaseType::Int);
        handle_ireturn(&mut frame, Some(&ret_type)).unwrap();
    }

    #[test]
    fn test_ireturn_wrong_stack_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_ireturn(&mut frame, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_ireturn_method_returns_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let ret_type = FieldType::Base(BaseType::Long);
        let result = handle_ireturn(&mut frame, Some(&ret_type));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("method returns"));
    }

    #[test]
    fn test_lreturn_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_lreturn(&mut frame, None).unwrap();
    }

    #[test]
    fn test_lreturn_with_expected_long() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let ret_type = FieldType::Base(BaseType::Long);
        handle_lreturn(&mut frame, Some(&ret_type)).unwrap();
    }

    #[test]
    fn test_lreturn_wrong_stack_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_lreturn(&mut frame, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_lreturn_method_returns_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let ret_type = FieldType::Base(BaseType::Int);
        let result = handle_lreturn(&mut frame, Some(&ret_type));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("method returns"));
    }

    #[test]
    fn test_freturn_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_freturn(&mut frame, None).unwrap();
    }

    #[test]
    fn test_freturn_with_expected_float() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let ret_type = FieldType::Base(BaseType::Float);
        handle_freturn(&mut frame, Some(&ret_type)).unwrap();
    }

    #[test]
    fn test_freturn_wrong_stack_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_freturn(&mut frame, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_freturn_method_returns_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let ret_type = FieldType::Base(BaseType::Int);
        let result = handle_freturn(&mut frame, Some(&ret_type));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("method returns"));
    }

    #[test]
    fn test_dreturn_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_dreturn(&mut frame, None).unwrap();
    }

    #[test]
    fn test_dreturn_with_expected_double() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let ret_type = FieldType::Base(BaseType::Double);
        handle_dreturn(&mut frame, Some(&ret_type)).unwrap();
    }

    #[test]
    fn test_dreturn_wrong_stack_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dreturn(&mut frame, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_dreturn_method_returns_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let ret_type = FieldType::Base(BaseType::Int);
        let result = handle_dreturn(&mut frame, Some(&ret_type));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("method returns"));
    }

    #[test]
    fn test_areturn_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_areturn(&mut frame, None, &ctx).unwrap();
    }

    #[test]
    fn test_areturn_null_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_areturn(&mut frame, None, &ctx).unwrap();
    }

    #[test]
    fn test_areturn_with_expected_type() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let ret_type = FieldType::Object("java/lang/Object".to_string());
        handle_areturn(&mut frame, Some(&ret_type), &ctx).unwrap();
    }

    #[test]
    fn test_areturn_not_reference_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_areturn(&mut frame, None, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_areturn_not_assignable_fails() {
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object("some/Other".into()))
            .unwrap();

        let ret_type = FieldType::Object("java/lang/String".to_string());
        let result = handle_areturn(&mut frame, Some(&ret_type), &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not assignable"));
    }

    #[test]
    fn test_tableswitch_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_tableswitch(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_tableswitch_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_tableswitch(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_tableswitch_reference_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_tableswitch(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_lookupswitch_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_lookupswitch(&mut frame).unwrap();
    }

    #[test]
    fn test_lookupswitch_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_lookupswitch(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int key"));
    }

    #[test]
    fn test_lookupswitch_reference_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let result = handle_lookupswitch(&mut frame);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int key"));
    }

    #[test]
    fn test_reject_jsr_version_51_fails() {
        let result = reject_jsr(51);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not allowed"));
    }

    #[test]
    fn test_reject_jsr_version_52_fails() {
        let result = reject_jsr(52);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not allowed"));
    }

    #[test]
    fn test_reject_jsr_version_50_fails() {
        // Older version - still rejected in our implementation
        let result = reject_jsr(50);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not supported"));
    }

    #[test]
    fn test_reject_ret_version_51_fails() {
        let result = reject_ret(51);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not allowed"));
    }

    #[test]
    fn test_reject_ret_version_50_fails() {
        let result = reject_ret(50);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not supported"));
    }

    #[test]
    fn test_dispatch_goto() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_control(&Instruction::Goto(10), &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_goto_w() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled =
            dispatch_control(&Instruction::Goto_w(1000), &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_return() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_control(&Instruction::Return, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_ireturn() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_control(&Instruction::Ireturn, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lreturn() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_control(&Instruction::Lreturn, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_freturn() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_control(&Instruction::Freturn, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dreturn() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_control(&Instruction::Dreturn, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_areturn() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_control(&Instruction::Areturn, &mut frame, None, 52, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_tableswitch() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let tableswitch = TableSwitch {
            default: 10,
            low: 0,
            high: 2,
            offsets: vec![10, 20, 30],
        };
        let handled = dispatch_control(
            &Instruction::Tableswitch(tableswitch),
            &mut frame,
            None,
            52,
            &ctx,
        )
        .unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lookupswitch() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let lookupswitch = LookupSwitch {
            default: 10,
            pairs: IndexMap::from([(0, 10)]),
        };
        let handled = dispatch_control(
            &Instruction::Lookupswitch(lookupswitch),
            &mut frame,
            None,
            52,
            &ctx,
        )
        .unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_jsr_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = dispatch_control(&Instruction::Jsr(10), &mut frame, None, 52, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_jsr_w_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = dispatch_control(&Instruction::Jsr_w(1000), &mut frame, None, 52, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_ret_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = dispatch_control(&Instruction::Ret(0), &mut frame, None, 52, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_ret_w_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let result = dispatch_control(&Instruction::Ret_w(0), &mut frame, None, 52, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_non_control_instruction() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_control(&Instruction::Nop, &mut frame, None, 52, &ctx).unwrap();
        assert!(!handled);
    }
}

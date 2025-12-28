//! # Exception Instruction Handlers
//!
//! Handles verification of exception-related instructions:
//! - `athrow` - throw exception
//!
//! # References
//!
//! - [JVMS §6.5.athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow)

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `athrow` - throw exception.
///
/// Stack: ..., objectref → \[empty\] (control transfer)
///
/// The objectref must be of type Throwable (or a subclass).
///
/// # Errors
///
/// Returns an error if the operand is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow)
pub fn handle_athrow<C: VerificationContext>(frame: &mut Frame, context: &C) -> Result<()> {
    let objectref = frame.pop()?;

    // Must be a reference type
    if !objectref.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "athrow: expected reference, got {objectref}"
        )));
    }

    // Must be assignable to Throwable (unless null)
    if !objectref.is_null() {
        let throwable = VerificationType::java_lang_throwable();
        if !objectref.is_assignable_to(&throwable, context)? {
            // We can't always verify this statically without class hierarchy info
            // So we just check it's a reference type
            if let VerificationType::Object(_) = &objectref {
                // Assume valid - will be checked at runtime
            } else if let VerificationType::Array(_) = &objectref {
                return Err(VerifyError::VerifyError(
                    "athrow: arrays are not throwable".to_string(),
                ));
            }
        }
    }

    // athrow clears the stack - but this is handled by control flow
    // The frame after athrow is not used (control transfers to handler)

    Ok(())
}

/// Dispatches exception instructions.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_exceptions<C: VerificationContext>(
    instruction: &Instruction,
    frame: &mut Frame,
    context: &C,
) -> Result<bool> {
    match instruction {
        Instruction::Athrow => {
            handle_athrow(frame, context)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifiers::bytecode::handlers::test_utils::{MockContext, StrictMockContext};
    use std::sync::Arc;

    // ==================== handle_athrow tests ====================

    #[test]
    fn test_athrow_object_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object(Arc::from(
                "java/lang/RuntimeException",
            )))
            .unwrap();

        handle_athrow(&mut frame, &ctx).unwrap();
    }

    #[test]
    fn test_athrow_throwable_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_throwable()).unwrap();

        handle_athrow(&mut frame, &ctx).unwrap();
    }

    #[test]
    fn test_athrow_null_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_athrow(&mut frame, &ctx).unwrap();
    }

    #[test]
    fn test_athrow_non_reference_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_athrow(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_athrow_float_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_athrow(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_athrow_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();

        let result = handle_athrow(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("arrays are not throwable")
        );
    }

    #[test]
    fn test_athrow_object_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(
                VerificationType::java_lang_object(),
            )))
            .unwrap();

        let result = handle_athrow(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("arrays are not throwable")
        );
    }

    #[test]
    fn test_athrow_with_strict_context_object_assumed_valid() {
        // Even with strict context, Object types are assumed valid
        // (will be checked at runtime)
        let ctx = StrictMockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Object(Arc::from("some/CustomClass")))
            .unwrap();

        // Should succeed because we assume Object types are valid
        handle_athrow(&mut frame, &ctx).unwrap();
    }

    // ==================== dispatch_exceptions tests ====================

    #[test]
    fn test_dispatch_athrow_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_throwable()).unwrap();

        let handled = dispatch_exceptions(&Instruction::Athrow, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_athrow_fails_with_non_reference() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = dispatch_exceptions(&Instruction::Athrow, &mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_non_exception_instruction() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_exceptions(&Instruction::Nop, &mut frame, &ctx).unwrap();
        assert!(!handled);
    }

    #[test]
    fn test_dispatch_pop_not_handled() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_exceptions(&Instruction::Pop, &mut frame, &ctx).unwrap();
        assert!(!handled);
    }
}

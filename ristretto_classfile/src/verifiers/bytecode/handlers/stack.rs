//! # Stack Manipulation Instruction Handlers
//!
//! Handles verification of stack manipulation instructions:
//! - `pop`, `pop2` - Discard stack values
//! - `dup`, `dup_x1`, `dup_x2` - Duplicate stack values
//! - `dup2`, `dup2_x1`, `dup2_x2` - Duplicate category 2 or two category 1 values
//! - `swap` - Swap top two values
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

/// Handles `pop` - discard top stack value.
///
/// # Errors
///
/// Returns an error if attempting to pop a category 2 value.
///
/// # References
///
/// Pop must not be used with category 2 values.
/// - [JVMS §6.5.pop](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.pop)
pub fn handle_pop(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value.is_category2() || value == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "pop cannot be used on category 2 values".to_string(),
        ));
    }
    Ok(())
}

/// Handles `pop2` - discard top one or two stack values.
///
/// Form 1: value2, value1 → (where value1 and value2 are both category 1)
/// Form 2: value → (where value is category 2)
///
/// # Errors
///
/// Returns an error if the stack values have unexpected types.
///
/// # References
///
/// - [JVMS §6.5.pop2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.pop2)
pub fn handle_pop2(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;

    if value == VerificationType::Top {
        // This is the second slot of a category 2 value
        // Pop the actual category 2 value
        let cat2 = frame.pop()?;
        if !cat2.is_category2() {
            return Err(VerifyError::VerifyError(
                "pop2: expected category 2 value before Top".to_string(),
            ));
        }
    } else if value.is_category2() {
        // This shouldn't happen in a well-formed stack, but handle it
        return Err(VerifyError::VerifyError(
            "pop2: unexpected category 2 value (should be Top on top)".to_string(),
        ));
    } else {
        // Category 1 value - need to pop another category 1
        let value2 = frame.pop()?;
        if value2.is_category2() || value2 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "pop2: expected two category 1 values".to_string(),
            ));
        }
    }

    Ok(())
}

/// Handles `dup` - duplicate top stack value.
///
/// Stack: ..., value → ..., value, value
///
/// # Errors
///
/// Returns an error if attempting to duplicate a category 2 value.
///
/// # References
///
/// Dup must not be used on category 2 values.
/// - [JVMS §6.5.dup](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup)
pub fn handle_dup(frame: &mut Frame) -> Result<()> {
    let value = frame.pop()?;
    if value.is_category2() || value == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "dup cannot be used on category 2 values".to_string(),
        ));
    }
    frame.push(value.clone())?;
    frame.push(value)
}

/// Handles `dup_x1` - duplicate and insert below second value.
///
/// Stack: ..., value2, value1 → ..., value1, value2, value1
///
/// Both values must be category 1.
///
/// # Errors
///
/// Returns an error if either value is not category 1.
///
/// # References
///
/// - [JVMS §6.5.dup_x1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup_x1)
pub fn handle_dup_x1(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;
    if value1.is_category2() || value1 == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "dup_x1: value1 must be category 1".to_string(),
        ));
    }

    let value2 = frame.pop()?;
    if value2.is_category2() || value2 == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "dup_x1: value2 must be category 1".to_string(),
        ));
    }

    frame.push(value1.clone())?;
    frame.push(value2)?;
    frame.push(value1)
}

/// Handles `dup_x2` - duplicate and insert below second or third value.
///
/// Form 1: ..., value3, value2, value1 → ..., value1, value3, value2, value1
///         (where value1, value2, and value3 are all category 1)
///
/// Form 2: ..., value2, value1 → ..., value1, value2, value1
///         (where value1 is category 1 and value2 is category 2)
///
/// # Errors
///
/// Returns an error if the stack values have unexpected types.
///
/// # References
///
/// - [JVMS §6.5.dup_x2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup_x2)
pub fn handle_dup_x2(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;
    if value1.is_category2() || value1 == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "dup_x2: value1 must be category 1".to_string(),
        ));
    }

    let value2 = frame.pop()?;

    if value2 == VerificationType::Top {
        // Form 2: value2 is category 2
        let cat2_value = frame.pop()?;
        if !cat2_value.is_category2() {
            return Err(VerifyError::VerifyError(
                "dup_x2: expected category 2 value before Top".to_string(),
            ));
        }

        frame.push(value1.clone())?;
        frame.push(cat2_value)?;
        frame.push(VerificationType::Top)?;
        frame.push(value1)
    } else if value2.is_category2() {
        Err(VerifyError::VerifyError(
            "dup_x2: malformed category 2 value on stack".to_string(),
        ))
    } else {
        // Form 1: value2 is category 1
        let value3 = frame.pop()?;
        if value3.is_category2() || value3 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup_x2: value3 must be category 1".to_string(),
            ));
        }

        frame.push(value1.clone())?;
        frame.push(value3)?;
        frame.push(value2)?;
        frame.push(value1)
    }
}

/// Handles `dup2` - duplicate top one or two values.
///
/// Form 1: ..., value2, value1 → ..., value2, value1, value2, value1
///         (where value1 and value2 are both category 1)
///
/// Form 2: ..., value → ..., value, value
///         (where value is category 2)
///
/// # Errors
///
/// Returns an error if the stack values have unexpected types.
///
/// # References
///
/// - [JVMS §6.5.dup2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup2)
pub fn handle_dup2(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;

    if value1 == VerificationType::Top {
        // Form 2: category 2 value
        let cat2_value = frame.pop()?;
        if !cat2_value.is_category2() {
            return Err(VerifyError::VerifyError(
                "dup2: expected category 2 value before Top".to_string(),
            ));
        }

        frame.push(cat2_value.clone())?;
        frame.push(VerificationType::Top)?;
        frame.push(cat2_value)?;
        frame.push(VerificationType::Top)
    } else if value1.is_category2() {
        Err(VerifyError::VerifyError(
            "dup2: malformed category 2 value on stack".to_string(),
        ))
    } else {
        // Form 1: two category 1 values
        let value2 = frame.pop()?;
        if value2.is_category2() || value2 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup2: value2 must be category 1".to_string(),
            ));
        }

        frame.push(value2.clone())?;
        frame.push(value1.clone())?;
        frame.push(value2)?;
        frame.push(value1)
    }
}

/// Handles `dup2_x1` - duplicate one or two values and insert below.
///
/// Form 1: ..., value3, value2, value1 → ..., value2, value1, value3, value2, value1
///         (where value1, value2, and value3 are all category 1)
///
/// Form 2: ..., value2, value1 → ..., value1, value2, value1
///         (where value1 is category 2 and value2 is category 1)
///
/// # Errors
///
/// Returns an error if the stack values have unexpected types.
///
/// # References
///
/// - [JVMS §6.5.dup2_x1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup2_x1)
pub fn handle_dup2_x1(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;

    if value1 == VerificationType::Top {
        // Form 2: value1 is category 2
        let cat2_value = frame.pop()?;
        if !cat2_value.is_category2() {
            return Err(VerifyError::VerifyError(
                "dup2_x1: expected category 2 value before Top".to_string(),
            ));
        }

        let value2 = frame.pop()?;
        if value2.is_category2() || value2 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup2_x1: value2 must be category 1".to_string(),
            ));
        }

        frame.push(cat2_value.clone())?;
        frame.push(VerificationType::Top)?;
        frame.push(value2)?;
        frame.push(cat2_value)?;
        frame.push(VerificationType::Top)
    } else if value1.is_category2() {
        Err(VerifyError::VerifyError(
            "dup2_x1: malformed category 2 value on stack".to_string(),
        ))
    } else {
        // Form 1: all category 1
        let value2 = frame.pop()?;
        if value2.is_category2() || value2 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup2_x1: value2 must be category 1".to_string(),
            ));
        }

        let value3 = frame.pop()?;
        if value3.is_category2() || value3 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup2_x1: value3 must be category 1".to_string(),
            ));
        }

        frame.push(value2.clone())?;
        frame.push(value1.clone())?;
        frame.push(value3)?;
        frame.push(value2)?;
        frame.push(value1)
    }
}

/// Handles `dup2_x2` - duplicate one or two values and insert two or three below.
///
/// Form 1: ..., value4, value3, value2, value1 → ..., value2, value1, value4, value3, value2, value1
///         (where all values are category 1)
///
/// Form 2: ..., value3, value2, value1 → ..., value1, value3, value2, value1
///         (where value1 is category 2, value2 and value3 are category 1)
///
/// Form 3: ..., value3, value2, value1 → ..., value2, value1, value3, value2, value1
///         (where value1 and value2 are category 1, value3 is category 2)
///
/// Form 4: ..., value2, value1 → ..., value1, value2, value1
///         (where both values are category 2)
///
/// # Errors
///
/// Returns an error if the stack values have unexpected types.
///
/// # References
///
/// - [JVMS §6.5.dup2_x2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.5.dup2_x2)
pub fn handle_dup2_x2(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;

    if value1 == VerificationType::Top {
        // value1 is category 2
        let cat2_value1 = frame.pop()?;
        if !cat2_value1.is_category2() {
            return Err(VerifyError::VerifyError(
                "dup2_x2: expected category 2 value before Top".to_string(),
            ));
        }

        let value2 = frame.pop()?;

        if value2 == VerificationType::Top {
            // Form 4: both category 2
            let cat2_value2 = frame.pop()?;
            if !cat2_value2.is_category2() {
                return Err(VerifyError::VerifyError(
                    "dup2_x2: expected category 2 value before Top".to_string(),
                ));
            }

            frame.push(cat2_value1.clone())?;
            frame.push(VerificationType::Top)?;
            frame.push(cat2_value2)?;
            frame.push(VerificationType::Top)?;
            frame.push(cat2_value1)?;
            frame.push(VerificationType::Top)
        } else if value2.is_category2() {
            Err(VerifyError::VerifyError(
                "dup2_x2: malformed category 2 value on stack".to_string(),
            ))
        } else {
            // Form 2: value1 cat2, value2 and value3 cat1
            let value3 = frame.pop()?;
            if value3.is_category2() || value3 == VerificationType::Top {
                return Err(VerifyError::VerifyError(
                    "dup2_x2: value3 must be category 1".to_string(),
                ));
            }

            frame.push(cat2_value1.clone())?;
            frame.push(VerificationType::Top)?;
            frame.push(value3)?;
            frame.push(value2)?;
            frame.push(cat2_value1)?;
            frame.push(VerificationType::Top)
        }
    } else if value1.is_category2() {
        Err(VerifyError::VerifyError(
            "dup2_x2: malformed category 2 value on stack".to_string(),
        ))
    } else {
        // value1 is category 1
        let value2 = frame.pop()?;
        if value2.is_category2() || value2 == VerificationType::Top {
            return Err(VerifyError::VerifyError(
                "dup2_x2: value2 must be category 1".to_string(),
            ));
        }

        let value3 = frame.pop()?;

        if value3 == VerificationType::Top {
            // Form 3: value1 and value2 cat1, value3 cat2
            let cat2_value3 = frame.pop()?;
            if !cat2_value3.is_category2() {
                return Err(VerifyError::VerifyError(
                    "dup2_x2: expected category 2 value before Top".to_string(),
                ));
            }

            frame.push(value2.clone())?;
            frame.push(value1.clone())?;
            frame.push(cat2_value3)?;
            frame.push(VerificationType::Top)?;
            frame.push(value2)?;
            frame.push(value1)
        } else if value3.is_category2() {
            Err(VerifyError::VerifyError(
                "dup2_x2: malformed category 2 value on stack".to_string(),
            ))
        } else {
            // Form 1: all category 1
            let value4 = frame.pop()?;
            if value4.is_category2() || value4 == VerificationType::Top {
                return Err(VerifyError::VerifyError(
                    "dup2_x2: value4 must be category 1".to_string(),
                ));
            }

            frame.push(value2.clone())?;
            frame.push(value1.clone())?;
            frame.push(value4)?;
            frame.push(value3)?;
            frame.push(value2)?;
            frame.push(value1)
        }
    }
}

/// Handles `swap` - swap top two stack values.
///
/// Stack: ..., value2, value1 → ..., value1, value2
///
/// Both values must be category 1.
///
/// # Errors
///
/// Returns an error if either value is not category 1.
///
/// # References
///
/// - [JVMS §6.5.swap](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.swap)
pub fn handle_swap(frame: &mut Frame) -> Result<()> {
    let value1 = frame.pop()?;
    if value1.is_category2() || value1 == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "swap: value1 must be category 1".to_string(),
        ));
    }

    let value2 = frame.pop()?;
    if value2.is_category2() || value2 == VerificationType::Top {
        return Err(VerifyError::VerifyError(
            "swap: value2 must be category 1".to_string(),
        ));
    }

    frame.push(value1)?;
    frame.push(value2)
}

/// Dispatches stack manipulation instructions to their handlers.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_stack(instruction: &Instruction, frame: &mut Frame) -> Result<bool> {
    match instruction {
        Instruction::Pop => handle_pop(frame)?,
        Instruction::Pop2 => handle_pop2(frame)?,
        Instruction::Dup => handle_dup(frame)?,
        Instruction::Dup_x1 => handle_dup_x1(frame)?,
        Instruction::Dup_x2 => handle_dup_x2(frame)?,
        Instruction::Dup2 => handle_dup2(frame)?,
        Instruction::Dup2_x1 => handle_dup2_x1(frame)?,
        Instruction::Dup2_x2 => handle_dup2_x2(frame)?,
        Instruction::Swap => handle_swap(frame)?,
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_category1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_pop(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_pop_category2_top_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        // Top of stack is Top
        let result = handle_pop(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pop cannot be used on category 2 values")
        );
    }

    #[test]
    fn test_pop_category2_direct_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Long).unwrap(); // Directly push Long (malformed)

        let result = handle_pop(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pop cannot be used on category 2 values")
        );
    }

    #[test]
    fn test_pop2_category2() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_pop2(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_pop2_two_category1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_pop2(&mut frame).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_pop2_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Top).unwrap();

        let result = handle_pop2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pop2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_pop2_category2_malformed_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Long).unwrap(); // Directly push category 2 (malformed)

        let result = handle_pop2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pop2: unexpected category 2 value")
        );
    }

    #[test]
    fn test_pop2_category1_then_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        // Pop Integer (cat1), then try to pop second cat1 but find Top
        let result = handle_pop2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pop2: expected two category 1 values")
        );
    }

    #[test]
    fn test_dup_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_dup(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 2);
        assert_eq!(*frame.peek_at(0).unwrap(), VerificationType::Integer);
        assert_eq!(*frame.peek_at(1).unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_dup_category2_top_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dup(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup cannot be used on category 2 values")
        );
    }

    #[test]
    fn test_dup_category2_direct_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Double).unwrap();

        let result = handle_dup(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup cannot be used on category 2 values")
        );
    }

    #[test]
    fn test_dup_x1_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_dup_x1(&mut frame).unwrap();

        // Stack should be: int, float, int
        assert_eq!(frame.stack_depth(), 3);
        assert_eq!(*frame.peek_at(0).unwrap(), VerificationType::Integer);
        assert_eq!(*frame.peek_at(1).unwrap(), VerificationType::Float);
        assert_eq!(*frame.peek_at(2).unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_dup_x1_value1_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dup_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x1: value1 must be category 1")
        );
    }

    #[test]
    fn test_dup_x1_value2_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_dup_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x1: value2 must be category 1")
        );
    }

    #[test]
    fn test_dup_x2_form1_three_category1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // value3
        frame.push(VerificationType::Float).unwrap(); // value2
        frame.push(VerificationType::Integer).unwrap(); // value1

        handle_dup_x2(&mut frame).unwrap();

        // Stack should be: value1, value3, value2, value1
        assert_eq!(frame.stack_depth(), 4);
        assert_eq!(*frame.peek_at(0).unwrap(), VerificationType::Integer);
        assert_eq!(*frame.peek_at(1).unwrap(), VerificationType::Float);
        assert_eq!(*frame.peek_at(2).unwrap(), VerificationType::Integer);
        assert_eq!(*frame.peek_at(3).unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_dup_x2_form2_value1_cat1_value2_cat2() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap(); // value2
        frame.push(VerificationType::Integer).unwrap(); // value1

        handle_dup_x2(&mut frame).unwrap();

        // Stack should be: value1, value2, value1
        // Which is: Integer, Long, Top, Integer = 4 slots
        assert_eq!(frame.stack_depth(), 4);
    }

    #[test]
    fn test_dup_x2_value1_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dup_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x2: value1 must be category 1")
        );
    }

    #[test]
    fn test_dup_x2_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Top).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_dup_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup_x2_malformed_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Long).unwrap(); // Malformed: Long without Top
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_dup_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x2: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup_x2_value3_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap(); // value3
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        let result = handle_dup_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup_x2: value3 must be category 1")
        );
    }

    #[test]
    fn test_dup2_category2_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_dup2(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 4); // Two long values (each takes 2 slots)
    }

    #[test]
    fn test_dup2_two_category1_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_dup2(&mut frame).unwrap();
        assert_eq!(frame.stack_depth(), 4);
    }

    #[test]
    fn test_dup2_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Top).unwrap();

        let result = handle_dup2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup2_malformed_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Long).unwrap(); // Malformed: no Top

        let result = handle_dup2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup2_value2_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_dup2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2: value2 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x1_form1_three_category1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // value3
        frame.push(VerificationType::Float).unwrap(); // value2
        frame.push(VerificationType::Integer).unwrap(); // value1

        handle_dup2_x1(&mut frame).unwrap();

        // Stack should be: value2, value1, value3, value2, value1
        assert_eq!(frame.stack_depth(), 5);
    }

    #[test]
    fn test_dup2_x1_form2_value1_cat2_value2_cat1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push_category2(VerificationType::Long).unwrap(); // value1

        handle_dup2_x1(&mut frame).unwrap();

        // Stack should be: value1, value2, value1
        assert_eq!(frame.stack_depth(), 5); // Long, Top, int, Long, Top
    }

    #[test]
    fn test_dup2_x1_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Top).unwrap();

        let result = handle_dup2_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x1: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup2_x1_malformed_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Long).unwrap(); // Malformed: no Top

        let result = handle_dup2_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x1: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup2_x1_form2_value2_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap(); // value2 is category 2
        frame.push_category2(VerificationType::Long).unwrap(); // value1 is category 2

        let result = handle_dup2_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x1: value2 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x1_form1_value2_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_dup2_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x1: value2 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x1_form1_value3_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap(); // value3
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        let result = handle_dup2_x1(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x1: value3 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x2_form1_four_category1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // value4
        frame.push(VerificationType::Float).unwrap(); // value3
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        handle_dup2_x2(&mut frame).unwrap();

        // Stack should be: value2, value1, value4, value3, value2, value1
        assert_eq!(frame.stack_depth(), 6);
    }

    #[test]
    fn test_dup2_x2_form2_value1_cat2_value2_value3_cat1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // value3
        frame.push(VerificationType::Float).unwrap(); // value2
        frame.push_category2(VerificationType::Long).unwrap(); // value1

        handle_dup2_x2(&mut frame).unwrap();

        // Stack should be: value1, value3, value2, value1
        assert_eq!(frame.stack_depth(), 6);
    }

    #[test]
    fn test_dup2_x2_form3_value1_value2_cat1_value3_cat2() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap(); // value3
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        handle_dup2_x2(&mut frame).unwrap();

        // Stack should be: value2, value1, value3, value2, value1
        assert_eq!(frame.stack_depth(), 6);
    }

    #[test]
    fn test_dup2_x2_form4_both_category2() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap(); // value2
        frame.push_category2(VerificationType::Long).unwrap(); // value1

        handle_dup2_x2(&mut frame).unwrap();

        // Stack should be: value1, value2, value1
        assert_eq!(frame.stack_depth(), 6);
    }

    #[test]
    fn test_dup2_x2_top_without_category2_value1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Top).unwrap();

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup2_x2_form4_value2_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // Not category 2
        frame.push(VerificationType::Top).unwrap(); // Looks like cat2 second slot
        frame.push_category2(VerificationType::Long).unwrap(); // value1 is category 2

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup2_x2_malformed_category2_value1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Long).unwrap(); // Malformed: no Top

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup2_x2_form2_malformed_category2_value2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Long).unwrap(); // Malformed: no Top
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup2_x2_form2_value3_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap(); // value3 is category 2
        frame.push(VerificationType::Integer).unwrap(); // value2 is category 1
        frame.push_category2(VerificationType::Long).unwrap(); // value1 is category 2

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: value3 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x2_value2_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap(); // value2 is category 2
        frame.push(VerificationType::Integer).unwrap(); // value1

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: value2 must be category 1")
        );
    }

    #[test]
    fn test_dup2_x2_form3_top_without_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap(); // Not category 2 before Top
        frame.push(VerificationType::Top).unwrap();
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: expected category 2 value before Top")
        );
    }

    #[test]
    fn test_dup2_x2_form1_malformed_category2_value3_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Long).unwrap(); // Malformed: no Top for value3
        frame.push(VerificationType::Integer).unwrap(); // value2
        frame.push(VerificationType::Float).unwrap(); // value1

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: malformed category 2 value on stack")
        );
    }

    #[test]
    fn test_dup2_x2_form1_value4_not_category1_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap(); // value4 is category 2
        frame.push(VerificationType::Integer).unwrap(); // value3
        frame.push(VerificationType::Float).unwrap(); // value2
        frame.push(VerificationType::Integer).unwrap(); // value1

        let result = handle_dup2_x2(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dup2_x2: value4 must be category 1")
        );
    }

    #[test]
    fn test_swap_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_swap(&mut frame).unwrap();

        assert_eq!(*frame.peek_at(0).unwrap(), VerificationType::Integer);
        assert_eq!(*frame.peek_at(1).unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_swap_value1_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_swap(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("swap: value1 must be category 1")
        );
    }

    #[test]
    fn test_swap_value2_category2_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_swap(&mut frame);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("swap: value2 must be category 1")
        );
    }

    #[test]
    fn test_dispatch_pop() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_stack(&Instruction::Pop, &mut frame).unwrap();
        assert!(handled);
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_dispatch_pop2() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_stack(&Instruction::Pop2, &mut frame).unwrap();
        assert!(handled);
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_dispatch_dup() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_stack(&Instruction::Dup, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dispatch_dup_x1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_stack(&Instruction::Dup_x1, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 3);
    }

    #[test]
    fn test_dispatch_dup_x2() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_stack(&Instruction::Dup_x2, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 4);
    }

    #[test]
    fn test_dispatch_dup2() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_stack(&Instruction::Dup2, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 4);
    }

    #[test]
    fn test_dispatch_dup2_x1() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_stack(&Instruction::Dup2_x1, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 5);
    }

    #[test]
    fn test_dispatch_dup2_x2() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_stack(&Instruction::Dup2_x2, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 6);
    }

    #[test]
    fn test_dispatch_swap() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_stack(&Instruction::Swap, &mut frame).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dispatch_non_stack_instruction() {
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_stack(&Instruction::Nop, &mut frame).unwrap();
        assert!(!handled);
    }
}

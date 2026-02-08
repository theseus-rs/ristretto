//! # Load and Store Instruction Handlers
//!
//! Handles verification of local variable load and store instructions:
//! - `iload`, `lload`, `fload`, `dload`, `aload` and their variants
//! - `istore`, `lstore`, `fstore`, `dstore`, `astore` and their variants
//! - Array load/store: `iaload`, `laload`, `aaload`, `iastore`, etc.
//!
//! # References
//!
//! - [JVMS §6.5 - Instructions](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5)

use std::sync::Arc;

use crate::attributes::Instruction;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};

/// Handles integer load instructions.
///
/// Loads an `int` from a local variable onto the stack.
///
/// # Errors
///
/// Returns an error if the local variable is not an `int` or index is out of bounds.
///
/// # References
///
/// - [JVMS §6.5.iload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iload)
pub fn handle_iload(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.get_local(index)?;
    if *ty != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "iload: expected int at local {index}, got {ty}"
        )));
    }
    frame.push(VerificationType::Integer)
}

/// Handles long load instructions.
///
/// Loads a `long` from local variables onto the stack.
///
/// # Errors
///
/// Returns an error if the local variable type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.lload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload)
pub fn handle_lload(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.get_local_category2(index)?;
    if *ty != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lload: expected long at local {index}, got {ty}"
        )));
    }
    frame.push_category2(VerificationType::Long)
}

/// Handles float load instructions.
///
/// # Errors
///
/// Returns an error if the local variable type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.fload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload)
pub fn handle_fload(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.get_local(index)?;
    if *ty != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "fload: expected float at local {index}, got {ty}"
        )));
    }
    frame.push(VerificationType::Float)
}

/// Handles double load instructions.
///
/// # Errors
///
/// Returns an error if the local variable type is incorrect.
///
/// # References
///
/// - [JVMS §6.5.dload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload)
pub fn handle_dload(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.get_local_category2(index)?;
    if *ty != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dload: expected double at local {index}, got {ty}"
        )));
    }
    frame.push_category2(VerificationType::Double)
}

/// Handles reference load instructions.
///
/// Loads a reference from a local variable onto the stack.
///
/// # Errors
///
/// Returns an error if the local variable is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.aload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload)
pub fn handle_aload(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.get_local(index)?.clone();
    if !ty.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "aload: expected reference at local {index}, got {ty}"
        )));
    }
    frame.push(ty)
}

/// Handles integer store instructions.
///
/// # Errors
///
/// Returns an error if the stack value is not an `int`.
///
/// # References
///
/// - [JVMS §6.5.istore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.istore)
pub fn handle_istore(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.pop()?;
    if ty != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "istore: expected int on stack, got {ty}"
        )));
    }
    frame.set_local(index, VerificationType::Integer)
}

/// Handles long store instructions.
///
/// # Errors
///
/// Returns an error if the stack value is not a `long`.
///
/// # References
///
/// - [JVMS §6.5.lstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore)
pub fn handle_lstore(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.pop_category2()?;
    if ty != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lstore: expected long on stack, got {ty}"
        )));
    }
    frame.set_local_category2(index, VerificationType::Long)
}

/// Handles float store instructions.
///
/// # Errors
///
/// Returns an error if the stack value is not a `float`.
///
/// # References
///
/// - [JVMS §6.5.fstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore)
pub fn handle_fstore(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.pop()?;
    if ty != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "fstore: expected float on stack, got {ty}"
        )));
    }
    frame.set_local(index, VerificationType::Float)
}

/// Handles double store instructions.
///
/// # Errors
///
/// Returns an error if the stack value is not a `double`.
///
/// # References
///
/// - [JVMS §6.5.dstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore)
pub fn handle_dstore(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.pop_category2()?;
    if ty != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dstore: expected double on stack, got {ty}"
        )));
    }
    frame.set_local_category2(index, VerificationType::Double)
}

/// Handles reference store instructions.
///
/// # Errors
///
/// Returns an error if the stack value is not a reference type.
///
/// # References
///
/// - [JVMS §6.5.astore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore)
pub fn handle_astore(frame: &mut Frame, index: u16) -> Result<()> {
    let ty = frame.pop()?;
    if !ty.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "astore: expected reference on stack, got {ty}"
        )));
    }
    frame.set_local(index, ty)
}

/// Handles `iaload` - load int from array.
///
/// Stack: ..., arrayref, index -> ..., value
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.iaload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iaload)
pub fn handle_iaload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    // Pop index (must be int)
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "iaload: expected int index, got {index_type}"
        )));
    }

    // Pop arrayref (must be int[])
    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "iaload")?;

    // Push int result
    frame.push(VerificationType::Integer)
}

/// Handles `laload` - load long from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.laload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.laload)
pub fn handle_laload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "laload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Long, "laload")?;

    frame.push_category2(VerificationType::Long)
}

/// Handles `faload` - load float from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.faload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.faload)
pub fn handle_faload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "faload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Float, "faload")?;

    frame.push(VerificationType::Float)
}

/// Handles `daload` - load double from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.daload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.daload)
pub fn handle_daload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "daload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Double, "daload")?;

    frame.push_category2(VerificationType::Double)
}

/// Handles `aaload` - load reference from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.aaload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aaload)
pub fn handle_aaload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "aaload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;

    // Handle null reference
    if array_type == VerificationType::Null {
        frame.push(VerificationType::Null)?;
        return Ok(());
    }

    // Must be an array of references
    match &array_type {
        VerificationType::Array(component) => {
            if !component.is_reference() && !component.is_null() {
                return Err(VerifyError::VerifyError(format!(
                    "aaload: expected array of references, got array of {component}"
                )));
            }
            frame.push(component.as_ref().clone())
        }
        VerificationType::Object(name) if name.starts_with('[') => {
            // Legacy array representation - parse component type
            let component = parse_array_component(name.as_ref())?;
            frame.push(component)
        }
        _ => Err(VerifyError::VerifyError(format!(
            "aaload: expected array reference, got {array_type}"
        ))),
    }
}

/// Handles `baload` - load byte/boolean from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.baload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.baload)
pub fn handle_baload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "baload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    // baload works on both byte[] and boolean[]
    verify_byte_or_boolean_array(&array_type, "baload")?;

    frame.push(VerificationType::Integer) // byte/boolean -> int on stack
}

/// Handles `caload` - load char from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.caload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.caload)
pub fn handle_caload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "caload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "caload")?; // char is int in verification

    frame.push(VerificationType::Integer)
}

/// Handles `saload` - load short from array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.saload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.saload)
pub fn handle_saload<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let index_type = frame.pop()?;
    if index_type != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "saload: expected int index, got {index_type}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "saload")?; // short is int in verification

    frame.push(VerificationType::Integer)
}

/// Handles `iastore` - store int into array.
///
/// Stack: ..., arrayref, index, value -> ...
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.iastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.iastore)
pub fn handle_iastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "iastore: expected int value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "iastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "iastore")
}

/// Handles `lastore` - store long into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.lastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lastore)
pub fn handle_lastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Long {
        return Err(VerifyError::VerifyError(format!(
            "lastore: expected long value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "lastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Long, "lastore")
}

/// Handles `fastore` - store float into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.fastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fastore)
pub fn handle_fastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Float {
        return Err(VerifyError::VerifyError(format!(
            "fastore: expected float value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "fastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Float, "fastore")
}

/// Handles `dastore` - store double into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.dastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dastore)
pub fn handle_dastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop_category2()?;
    if value != VerificationType::Double {
        return Err(VerifyError::VerifyError(format!(
            "dastore: expected double value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "dastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Double, "dastore")
}

/// Handles `aastore` - store reference into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.aastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aastore)
pub fn handle_aastore<C: VerificationContext>(frame: &mut Frame, context: &C) -> Result<()> {
    let value = frame.pop()?;
    if !value.is_reference() {
        return Err(VerifyError::VerifyError(format!(
            "aastore: expected reference value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "aastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;

    // Null array is allowed (will throw at runtime)
    if array_type == VerificationType::Null {
        return Ok(());
    }

    // Verify it's an array of references
    match &array_type {
        VerificationType::Array(component) => {
            if !component.is_reference() && !component.is_null() {
                return Err(VerifyError::VerifyError(format!(
                    "aastore: expected array of references, got array of {component}"
                )));
            }
            // Check assignability
            if !value.is_null() {
                value.is_assignable_to(component, context)?;
            }
            Ok(())
        }
        VerificationType::Object(name) if name.starts_with('[') => {
            // Legacy representation
            Ok(())
        }
        _ => Err(VerifyError::VerifyError(format!(
            "aastore: expected array reference, got {array_type}"
        ))),
    }
}

/// Handles `bastore` - store byte/boolean into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.bastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bastore)
pub fn handle_bastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "bastore: expected int value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "bastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_byte_or_boolean_array(&array_type, "bastore")
}

/// Handles `castore` - store char into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.castore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.castore)
pub fn handle_castore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "castore: expected int value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "castore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "castore")
}

/// Handles `sastore` - store short into array.
///
/// # Errors
///
/// Returns an error if the stack types are incorrect.
///
/// # References
///
/// - [JVMS §6.5.sastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sastore)
pub fn handle_sastore<C: VerificationContext>(frame: &mut Frame, _context: &C) -> Result<()> {
    let value = frame.pop()?;
    if value != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "sastore: expected int value, got {value}"
        )));
    }

    let index = frame.pop()?;
    if index != VerificationType::Integer {
        return Err(VerifyError::VerifyError(format!(
            "sastore: expected int index, got {index}"
        )));
    }

    let array_type = frame.pop()?;
    verify_array_of(&array_type, &VerificationType::Integer, "sastore")
}

/// Verifies that a type is an array of the expected component type.
fn verify_array_of(
    array_type: &VerificationType,
    expected_component: &VerificationType,
    instruction: &str,
) -> Result<()> {
    if *array_type == VerificationType::Null {
        return Ok(()); // Null is allowed (runtime error)
    }

    match array_type {
        VerificationType::Array(component) => {
            if component.as_ref() != expected_component {
                return Err(VerifyError::VerifyError(format!(
                    "{instruction}: expected array of {expected_component}, got array of {component}"
                )));
            }
            Ok(())
        }
        _ => Err(VerifyError::VerifyError(format!(
            "{instruction}: expected array reference, got {array_type}"
        ))),
    }
}

/// Verifies that a type is a byte[] or boolean[] array.
fn verify_byte_or_boolean_array(array_type: &VerificationType, instruction: &str) -> Result<()> {
    if *array_type == VerificationType::Null {
        return Ok(());
    }

    match array_type {
        VerificationType::Array(component) => {
            // Both byte and boolean are represented as Integer in verification
            if **component != VerificationType::Integer {
                return Err(VerifyError::VerifyError(format!(
                    "{instruction}: expected byte[] or boolean[], got array of {component}"
                )));
            }
            Ok(())
        }
        _ => Err(VerifyError::VerifyError(format!(
            "{instruction}: expected array reference, got {array_type}"
        ))),
    }
}

/// Parses the component type from an array descriptor.
fn parse_array_component(descriptor: &str) -> Result<VerificationType> {
    if !descriptor.starts_with('[') {
        return Err(VerifyError::VerifyError(format!(
            "Invalid array descriptor: {descriptor}"
        )));
    }

    let component_desc = &descriptor[1..];
    match component_desc.chars().next() {
        Some('I' | 'B' | 'Z' | 'C' | 'S') => Ok(VerificationType::Integer),
        Some('J') => Ok(VerificationType::Long),
        Some('F') => Ok(VerificationType::Float),
        Some('D') => Ok(VerificationType::Double),
        Some('[') => {
            // Nested array
            let inner = parse_array_component(component_desc)?;
            Ok(VerificationType::Array(Box::new(inner)))
        }
        Some('L') => {
            // Object type - extract class name
            if let Some(end) = component_desc.find(';') {
                let class_name = &component_desc[1..end];
                Ok(VerificationType::Object(Arc::from(class_name)))
            } else {
                Err(VerifyError::VerifyError(format!(
                    "Invalid object descriptor: {component_desc}"
                )))
            }
        }
        _ => Err(VerifyError::VerifyError(format!(
            "Unknown array component: {component_desc}"
        ))),
    }
}

/// Dispatches load/store instructions to their handlers.
///
/// # Errors
///
/// Returns an error if the instruction verification fails.
pub fn dispatch_load_store<C: VerificationContext>(
    instruction: &Instruction,
    frame: &mut Frame,
    context: &C,
) -> Result<bool> {
    match instruction {
        // Integer loads
        Instruction::Iload(idx) => handle_iload(frame, u16::from(*idx))?,
        Instruction::Iload_w(idx) => handle_iload(frame, *idx)?,
        Instruction::Iload_0 => handle_iload(frame, 0)?,
        Instruction::Iload_1 => handle_iload(frame, 1)?,
        Instruction::Iload_2 => handle_iload(frame, 2)?,
        Instruction::Iload_3 => handle_iload(frame, 3)?,

        // Long loads
        Instruction::Lload(idx) => handle_lload(frame, u16::from(*idx))?,
        Instruction::Lload_w(idx) => handle_lload(frame, *idx)?,
        Instruction::Lload_0 => handle_lload(frame, 0)?,
        Instruction::Lload_1 => handle_lload(frame, 1)?,
        Instruction::Lload_2 => handle_lload(frame, 2)?,
        Instruction::Lload_3 => handle_lload(frame, 3)?,

        // Float loads
        Instruction::Fload(idx) => handle_fload(frame, u16::from(*idx))?,
        Instruction::Fload_w(idx) => handle_fload(frame, *idx)?,
        Instruction::Fload_0 => handle_fload(frame, 0)?,
        Instruction::Fload_1 => handle_fload(frame, 1)?,
        Instruction::Fload_2 => handle_fload(frame, 2)?,
        Instruction::Fload_3 => handle_fload(frame, 3)?,

        // Double loads
        Instruction::Dload(idx) => handle_dload(frame, u16::from(*idx))?,
        Instruction::Dload_w(idx) => handle_dload(frame, *idx)?,
        Instruction::Dload_0 => handle_dload(frame, 0)?,
        Instruction::Dload_1 => handle_dload(frame, 1)?,
        Instruction::Dload_2 => handle_dload(frame, 2)?,
        Instruction::Dload_3 => handle_dload(frame, 3)?,

        // Reference loads
        Instruction::Aload(idx) => handle_aload(frame, u16::from(*idx))?,
        Instruction::Aload_w(idx) => handle_aload(frame, *idx)?,
        Instruction::Aload_0 => handle_aload(frame, 0)?,
        Instruction::Aload_1 => handle_aload(frame, 1)?,
        Instruction::Aload_2 => handle_aload(frame, 2)?,
        Instruction::Aload_3 => handle_aload(frame, 3)?,

        // Integer stores
        Instruction::Istore(idx) => handle_istore(frame, u16::from(*idx))?,
        Instruction::Istore_w(idx) => handle_istore(frame, *idx)?,
        Instruction::Istore_0 => handle_istore(frame, 0)?,
        Instruction::Istore_1 => handle_istore(frame, 1)?,
        Instruction::Istore_2 => handle_istore(frame, 2)?,
        Instruction::Istore_3 => handle_istore(frame, 3)?,

        // Long stores
        Instruction::Lstore(idx) => handle_lstore(frame, u16::from(*idx))?,
        Instruction::Lstore_w(idx) => handle_lstore(frame, *idx)?,
        Instruction::Lstore_0 => handle_lstore(frame, 0)?,
        Instruction::Lstore_1 => handle_lstore(frame, 1)?,
        Instruction::Lstore_2 => handle_lstore(frame, 2)?,
        Instruction::Lstore_3 => handle_lstore(frame, 3)?,

        // Float stores
        Instruction::Fstore(idx) => handle_fstore(frame, u16::from(*idx))?,
        Instruction::Fstore_w(idx) => handle_fstore(frame, *idx)?,
        Instruction::Fstore_0 => handle_fstore(frame, 0)?,
        Instruction::Fstore_1 => handle_fstore(frame, 1)?,
        Instruction::Fstore_2 => handle_fstore(frame, 2)?,
        Instruction::Fstore_3 => handle_fstore(frame, 3)?,

        // Double stores
        Instruction::Dstore(idx) => handle_dstore(frame, u16::from(*idx))?,
        Instruction::Dstore_w(idx) => handle_dstore(frame, *idx)?,
        Instruction::Dstore_0 => handle_dstore(frame, 0)?,
        Instruction::Dstore_1 => handle_dstore(frame, 1)?,
        Instruction::Dstore_2 => handle_dstore(frame, 2)?,
        Instruction::Dstore_3 => handle_dstore(frame, 3)?,

        // Reference stores
        Instruction::Astore(idx) => handle_astore(frame, u16::from(*idx))?,
        Instruction::Astore_w(idx) => handle_astore(frame, *idx)?,
        Instruction::Astore_0 => handle_astore(frame, 0)?,
        Instruction::Astore_1 => handle_astore(frame, 1)?,
        Instruction::Astore_2 => handle_astore(frame, 2)?,
        Instruction::Astore_3 => handle_astore(frame, 3)?,

        // Array loads
        Instruction::Iaload => handle_iaload(frame, context)?,
        Instruction::Laload => handle_laload(frame, context)?,
        Instruction::Faload => handle_faload(frame, context)?,
        Instruction::Daload => handle_daload(frame, context)?,
        Instruction::Aaload => handle_aaload(frame, context)?,
        Instruction::Baload => handle_baload(frame, context)?,
        Instruction::Caload => handle_caload(frame, context)?,
        Instruction::Saload => handle_saload(frame, context)?,

        // Array stores
        Instruction::Iastore => handle_iastore(frame, context)?,
        Instruction::Lastore => handle_lastore(frame, context)?,
        Instruction::Fastore => handle_fastore(frame, context)?,
        Instruction::Dastore => handle_dastore(frame, context)?,
        Instruction::Aastore => handle_aastore(frame, context)?,
        Instruction::Bastore => handle_bastore(frame, context)?,
        Instruction::Castore => handle_castore(frame, context)?,
        Instruction::Sastore => handle_sastore(frame, context)?,

        // Not a load/store instruction
        _ => return Ok(false),
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifiers::bytecode::handlers::test_utils::MockContext;

    #[test]
    fn test_iload_success() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Integer).unwrap();

        handle_iload(&mut frame, 2).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_iload_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(2, VerificationType::Float).unwrap();

        let result = handle_iload(&mut frame, 2);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected int"));
    }

    #[test]
    fn test_lload_success() {
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(1, VerificationType::Long)
            .unwrap();

        handle_lload(&mut frame, 1).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_lload_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(1, VerificationType::Double)
            .unwrap();

        let result = handle_lload(&mut frame, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected long"));
    }

    #[test]
    fn test_fload_success() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Float).unwrap();

        handle_fload(&mut frame, 0).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_fload_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Integer).unwrap();

        let result = handle_fload(&mut frame, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected float"));
    }

    #[test]
    fn test_dload_success() {
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(0, VerificationType::Double)
            .unwrap();

        handle_dload(&mut frame, 0).unwrap();
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dload_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(0, VerificationType::Long)
            .unwrap();

        let result = handle_dload(&mut frame, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected double"));
    }

    #[test]
    fn test_aload_success() {
        let mut frame = Frame::new(5, 10);
        frame
            .set_local(0, VerificationType::java_lang_object())
            .unwrap();

        handle_aload(&mut frame, 0).unwrap();
        assert!(frame.peek().unwrap().is_reference());
    }

    #[test]
    fn test_aload_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Null).unwrap();

        handle_aload(&mut frame, 0).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Null);
    }

    #[test]
    fn test_aload_not_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Integer).unwrap();

        let result = handle_aload(&mut frame, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference")
        );
    }

    #[test]
    fn test_istore_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        handle_istore(&mut frame, 3).unwrap();
        assert_eq!(*frame.get_local(3).unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_istore_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let result = handle_istore(&mut frame, 3);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int on stack")
        );
    }

    #[test]
    fn test_lstore_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        handle_lstore(&mut frame, 1).unwrap();
        assert_eq!(*frame.get_local(1).unwrap(), VerificationType::Long);
    }

    #[test]
    fn test_lstore_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_lstore(&mut frame, 1);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected long on stack")
        );
    }

    #[test]
    fn test_fstore_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        handle_fstore(&mut frame, 2).unwrap();
        assert_eq!(*frame.get_local(2).unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_fstore_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_fstore(&mut frame, 2);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected float on stack")
        );
    }

    #[test]
    fn test_dstore_success() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        handle_dstore(&mut frame, 0).unwrap();
        assert_eq!(*frame.get_local(0).unwrap(), VerificationType::Double);
    }

    #[test]
    fn test_dstore_wrong_type_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dstore(&mut frame, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected double on stack")
        );
    }

    #[test]
    fn test_astore_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        handle_astore(&mut frame, 1).unwrap();
        assert!(frame.get_local(1).unwrap().is_reference());
    }

    #[test]
    fn test_astore_null_success() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Null).unwrap();

        handle_astore(&mut frame, 1).unwrap();
        assert_eq!(*frame.get_local(1).unwrap(), VerificationType::Null);
    }

    #[test]
    fn test_astore_not_reference_fails() {
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_astore(&mut frame, 1);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference on stack")
        );
    }

    #[test]
    fn test_iaload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_iaload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_iaload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_iaload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_iaload_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_iaload(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_iastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_iastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_iastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_iastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int value")
        );
    }

    #[test]
    fn test_iastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_iastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_aaload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_aaload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::java_lang_string());
    }

    #[test]
    fn test_aaload_null_array() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::Null).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_aaload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Null);
    }

    #[test]
    fn test_aaload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_aaload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_aaload_primitive_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_aaload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array of references")
        );
    }

    #[test]
    fn test_parse_array_component_int() {
        assert_eq!(
            parse_array_component("[I").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_array_component_object() {
        assert_eq!(
            parse_array_component("[Ljava/lang/String;").unwrap(),
            VerificationType::java_lang_string()
        );
    }

    #[test]
    fn test_parse_array_component_nested_array() {
        assert_eq!(
            parse_array_component("[[I").unwrap(),
            VerificationType::Array(Box::new(VerificationType::Integer))
        );
    }

    #[test]
    fn test_parse_array_component_long() {
        assert_eq!(parse_array_component("[J").unwrap(), VerificationType::Long);
    }

    #[test]
    fn test_parse_array_component_float() {
        assert_eq!(
            parse_array_component("[F").unwrap(),
            VerificationType::Float
        );
    }

    #[test]
    fn test_parse_array_component_double() {
        assert_eq!(
            parse_array_component("[D").unwrap(),
            VerificationType::Double
        );
    }

    #[test]
    fn test_dispatch_iload_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Iload_0, &mut frame, &ctx).unwrap();
        assert!(handled);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_dispatch_aload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .set_local(2, VerificationType::java_lang_object())
            .unwrap();

        let handled = dispatch_load_store(&Instruction::Aload(2), &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_istore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Istore(3), &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_non_load_store() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let handled = dispatch_load_store(&Instruction::Nop, &mut frame, &ctx).unwrap();
        assert!(!handled);
    }

    #[test]
    fn test_laload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_laload(&mut frame, &ctx).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Long takes 2 slots
    }

    #[test]
    fn test_laload_null_array_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::Null).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_laload(&mut frame, &ctx).unwrap();
    }

    #[test]
    fn test_laload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_laload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_laload_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_laload(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_faload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_faload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_faload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_faload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_faload_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_faload(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_daload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_daload(&mut frame, &ctx).unwrap();
        assert_eq!(frame.stack_depth(), 2); // Double takes 2 slots
    }

    #[test]
    fn test_daload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_daload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_daload_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_daload(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_baload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // byte[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_baload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_baload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_baload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_baload_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_baload(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_caload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // char[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_caload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_caload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_caload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_saload_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // short[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_saload(&mut frame, &ctx).unwrap();
        assert_eq!(*frame.peek().unwrap(), VerificationType::Integer);
    }

    #[test]
    fn test_saload_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_saload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_lastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        handle_lastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_lastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_lastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected long value")
        );
    }

    #[test]
    fn test_lastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_lastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_fastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        handle_fastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_fastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_fastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected float value")
        );
    }

    #[test]
    fn test_fastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_fastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_dastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        handle_dastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_dastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let result = handle_dastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected double value")
        );
    }

    #[test]
    fn test_dastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let result = handle_dastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_aastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_string()).unwrap();

        handle_aastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_aastore_null_value_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Null).unwrap();

        handle_aastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_aastore_null_array_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::Null).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_string()).unwrap();

        handle_aastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_aastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_aastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected reference value")
        );
    }

    #[test]
    fn test_aastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        let string_array = VerificationType::Array(Box::new(VerificationType::java_lang_string()));
        frame.push(string_array).unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::java_lang_string()).unwrap();

        let result = handle_aastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_aastore_primitive_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_string()).unwrap();

        let result = handle_aastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array of references")
        );
    }

    #[test]
    fn test_aastore_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_string()).unwrap();

        let result = handle_aastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_bastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // byte[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_bastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_bastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_bastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int value")
        );
    }

    #[test]
    fn test_bastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_bastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_castore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // char[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_castore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_castore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_castore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int value")
        );
    }

    #[test]
    fn test_castore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_castore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_sastore_success() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // short[] is represented as Integer[] in verification
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_sastore(&mut frame, &ctx).unwrap();
        assert!(frame.is_stack_empty());
    }

    #[test]
    fn test_sastore_wrong_value_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let result = handle_sastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int value")
        );
    }

    #[test]
    fn test_sastore_wrong_index_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Float).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_sastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected int index")
        );
    }

    #[test]
    fn test_parse_array_component_byte() {
        assert_eq!(
            parse_array_component("[B").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_array_component_char() {
        assert_eq!(
            parse_array_component("[C").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_array_component_short() {
        assert_eq!(
            parse_array_component("[S").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_array_component_boolean() {
        assert_eq!(
            parse_array_component("[Z").unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_parse_array_component_invalid_no_bracket() {
        let result = parse_array_component("I");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid array descriptor")
        );
    }

    #[test]
    fn test_parse_array_component_invalid_object_no_semicolon() {
        let result = parse_array_component("[Ljava/lang/String");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid object descriptor")
        );
    }

    #[test]
    fn test_parse_array_component_unknown_type() {
        let result = parse_array_component("[X");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unknown array component")
        );
    }

    #[test]
    fn test_aaload_legacy_array_representation() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        // Legacy representation: Object with name starting with '['
        frame
            .push(VerificationType::Object(Arc::from("[Ljava/lang/String;")))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        handle_aaload(&mut frame, &ctx).unwrap();
    }

    #[test]
    fn test_aaload_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_aaload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_dispatch_lload_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(0, VerificationType::Long)
            .unwrap();

        let handled = dispatch_load_store(&Instruction::Lload_0, &mut frame, &ctx).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dispatch_fload_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.set_local(0, VerificationType::Float).unwrap();

        let handled = dispatch_load_store(&Instruction::Fload_0, &mut frame, &ctx).unwrap();
        assert!(handled);
        assert_eq!(*frame.peek().unwrap(), VerificationType::Float);
    }

    #[test]
    fn test_dispatch_dload_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .set_local_category2(0, VerificationType::Double)
            .unwrap();

        let handled = dispatch_load_store(&Instruction::Dload_0, &mut frame, &ctx).unwrap();
        assert!(handled);
        assert_eq!(frame.stack_depth(), 2);
    }

    #[test]
    fn test_dispatch_aload_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .set_local(0, VerificationType::java_lang_object())
            .unwrap();

        let handled = dispatch_load_store(&Instruction::Aload_0, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lstore_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_load_store(&Instruction::Lstore_0, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fstore_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_load_store(&Instruction::Fstore_0, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dstore_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_load_store(&Instruction::Dstore_0, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_astore_0() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_load_store(&Instruction::Astore_0, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_iaload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Iaload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_laload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Laload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_faload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Faload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_daload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Daload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_aaload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(
                VerificationType::java_lang_object(),
            )))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Aaload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_baload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Baload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_caload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Caload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_saload() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Saload, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_iastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Iastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_lastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Long)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Long).unwrap();

        let handled = dispatch_load_store(&Instruction::Lastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_fastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Float).unwrap();

        let handled = dispatch_load_store(&Instruction::Fastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_dastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Double)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push_category2(VerificationType::Double).unwrap();

        let handled = dispatch_load_store(&Instruction::Dastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_aastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(
                VerificationType::java_lang_object(),
            )))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::java_lang_object()).unwrap();

        let handled = dispatch_load_store(&Instruction::Aastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_bastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Bastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_castore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Castore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_dispatch_sastore() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);
        frame
            .push(VerificationType::Array(Box::new(VerificationType::Integer)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let handled = dispatch_load_store(&Instruction::Sastore, &mut frame, &ctx).unwrap();
        assert!(handled);
    }

    #[test]
    fn test_iaload_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_iaload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_iastore_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_iastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_iastore_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_iastore(&mut frame, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_baload_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_baload(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_bastore_not_array_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame.push(VerificationType::java_lang_object()).unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_bastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected array reference")
        );
    }

    #[test]
    fn test_bastore_wrong_array_type_fails() {
        let ctx = MockContext;
        let mut frame = Frame::new(5, 10);

        frame
            .push(VerificationType::Array(Box::new(VerificationType::Float)))
            .unwrap();
        frame.push(VerificationType::Integer).unwrap();
        frame.push(VerificationType::Integer).unwrap();

        let result = handle_bastore(&mut frame, &ctx);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected byte[] or boolean[]")
        );
    }
}

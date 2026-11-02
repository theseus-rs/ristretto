//! Validation of byte offsets embedded in Code metadata, without relocation.

use super::{Attribute, StackFrame, TargetType, VerificationType};
use crate::{Error::InvalidInstructionOffset, Result};

/// Validate metadata against instruction starts, allowing `code_length` only as a range end.
pub(crate) fn validate(
    attribute: &Attribute,
    code_length: u16,
    is_instruction_start: impl Fn(u16) -> bool,
) -> Result<()> {
    let offset = |pc: u16| {
        if pc < code_length && is_instruction_start(pc) {
            Ok(())
        } else {
            Err(InvalidInstructionOffset(u32::from(pc)))
        }
    };
    let range = |start: u16, length: u16| {
        offset(start)?;
        let end = u32::from(start) + u32::from(length);
        let end = u16::try_from(end).map_err(|_| InvalidInstructionOffset(end))?;
        if end == code_length {
            Ok(())
        } else {
            offset(end)
        }
    };
    match attribute {
        Attribute::LineNumberTable { line_numbers, .. } => {
            for line in line_numbers {
                offset(line.start_pc)?;
            }
        }
        Attribute::LocalVariableTable { variables, .. } => {
            for local in variables {
                range(local.start_pc, local.length)?;
            }
        }
        Attribute::LocalVariableTypeTable { variable_types, .. } => {
            for local in variable_types {
                range(local.start_pc, local.length)?;
            }
        }
        Attribute::RuntimeVisibleTypeAnnotations {
            type_annotations, ..
        }
        | Attribute::RuntimeInvisibleTypeAnnotations {
            type_annotations, ..
        } => {
            for annotation in type_annotations {
                match &annotation.target_type {
                    TargetType::LocalVar {
                        local_variable_targets,
                        ..
                    } => {
                        for local in local_variable_targets {
                            range(local.start_pc, local.length)?;
                        }
                    }
                    TargetType::Offset { offset: pc, .. }
                    | TargetType::TypeArgument { offset: pc, .. } => offset(*pc)?,
                    _ => {}
                }
            }
        }
        Attribute::StackMapTable { frames, .. } => validate_frames(frames, offset)?,
        _ => {}
    }
    Ok(())
}

fn validate_frames(
    frames: &[StackFrame],
    validate_offset: impl Fn(u16) -> Result<()>,
) -> Result<()> {
    let mut next_offset = 0;
    for frame in frames {
        frame.validate()?;
        let offset = next_offset + u32::from(frame.offset_delta());
        validate_offset(u16::try_from(offset).map_err(|_| InvalidInstructionOffset(offset))?)?;
        next_offset = offset + 1;
        let (locals, stack): (&[_], &[_]) = match frame {
            StackFrame::FullFrame { locals, stack, .. } => (locals, stack),
            StackFrame::AppendFrame { locals, .. } => (locals, &[]),
            StackFrame::SameLocals1StackItemFrame { stack, .. }
            | StackFrame::SameLocals1StackItemFrameExtended { stack, .. } => (&[], stack),
            _ => (&[], &[]),
        };
        for ty in locals.iter().chain(stack) {
            if let VerificationType::Uninitialized { offset } = ty {
                validate_offset(*offset)?;
            }
        }
    }
    Ok(())
}

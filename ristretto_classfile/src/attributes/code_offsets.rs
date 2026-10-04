//! Relocation of offsets embedded in Code metadata.
use super::{Attribute, StackFrame, TargetType, VerificationType};
use crate::{Error, Result};

pub(super) fn relocate(attribute: &mut Attribute, map: impl Fn(u16) -> Result<u16>) -> Result<()> {
    match attribute {
        Attribute::LocalVariableTable { variables, .. } => {
            for local in variables {
                range(&mut local.start_pc, &mut local.length, &map)?;
            }
        }
        Attribute::LocalVariableTypeTable { variable_types, .. } => {
            for local in variable_types {
                range(&mut local.start_pc, &mut local.length, &map)?;
            }
        }
        Attribute::RuntimeVisibleTypeAnnotations {
            type_annotations, ..
        }
        | Attribute::RuntimeInvisibleTypeAnnotations {
            type_annotations, ..
        } => {
            for annotation in type_annotations {
                match &mut annotation.target_type {
                    TargetType::LocalVar {
                        local_variable_targets,
                        ..
                    } => {
                        for local in local_variable_targets {
                            range(&mut local.start_pc, &mut local.length, &map)?;
                        }
                    }
                    TargetType::Offset { offset, .. } | TargetType::TypeArgument { offset, .. } => {
                        *offset = map(*offset)?;
                    }
                    _ => {}
                }
            }
        }
        Attribute::StackMapTable { frames, .. } => {
            for frame in frames {
                match frame {
                    StackFrame::SameLocals1StackItemFrame { stack, .. }
                    | StackFrame::SameLocals1StackItemFrameExtended { stack, .. } => {
                        types(stack, &map)?;
                    }
                    StackFrame::AppendFrame { locals, .. } => types(locals, &map)?,
                    StackFrame::FullFrame { locals, stack, .. } => {
                        types(locals, &map)?;
                        types(stack, &map)?;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn range(start: &mut u16, length: &mut u16, map: &impl Fn(u16) -> Result<u16>) -> Result<()> {
    let end = start
        .checked_add(*length)
        .ok_or(Error::InvalidInstructionOffset(
            u32::from(*start) + u32::from(*length),
        ))?;
    let new_start = map(*start)?;
    *length = map(end)?
        .checked_sub(new_start)
        .ok_or(Error::InvalidInstructionOffset(u32::from(end)))?;
    *start = new_start;
    Ok(())
}

fn types(types: &mut [VerificationType], map: &impl Fn(u16) -> Result<u16>) -> Result<()> {
    for ty in types {
        if let VerificationType::Uninitialized { offset } = ty {
            *offset = map(*offset)?;
        }
    }
    Ok(())
}

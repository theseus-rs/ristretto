//! Local-array requirements imposed by retained Code attributes (JVMS 4.7.3).

use crate::attributes::{Attribute, StackFrame, VerificationType};
use crate::verifiers::VerifyError;
use crate::{ConstantPool, Error, FieldType, Method, Result};

pub(super) fn slot_count(locals: &[u8]) -> Result<u16> {
    locals.iter().try_fold(0_u16, |count, &width| {
        Ok(u16::try_from(u32::from(count) + u32::from(width))?)
    })
}

pub(super) fn max_locals(
    constant_pool: &ConstantPool<'_>,
    method: &Method,
    initial_locals: &[u8],
) -> Result<u16> {
    let mut maximum = 0;
    for attribute in &method.attributes {
        let Attribute::Code { attributes, .. } = attribute else {
            continue;
        };
        for attribute in attributes {
            match attribute {
                Attribute::LocalVariableTable { variables, .. } => {
                    for variable in variables {
                        let descriptor = constant_pool.try_get_utf8(variable.descriptor_index)?;
                        let width = FieldType::parse_java_str(descriptor)?.slot_count();
                        let count = u16::try_from(u32::from(variable.index) + u32::from(width))?;
                        maximum = maximum.max(count);
                    }
                }
                Attribute::LocalVariableTypeTable { variable_types, .. } => {
                    // Field signatures describe reference types, each occupying one slot.
                    for variable in variable_types {
                        let count = u16::try_from(u32::from(variable.index) + 1)?;
                        maximum = maximum.max(count);
                    }
                }
                Attribute::StackMapTable { frames, .. } => {
                    maximum = maximum.max(stack_map_locals(frames, initial_locals)?);
                }
                _ => {}
            }
        }
    }
    Ok(maximum)
}

fn verification_width(local: &VerificationType) -> u8 {
    match local {
        VerificationType::Long | VerificationType::Double => 2,
        _ => 1,
    }
}

fn stack_map_locals(frames: &[StackFrame], initial_locals: &[u8]) -> Result<u16> {
    let mut locals = initial_locals.to_vec();
    let mut current = slot_count(&locals)?;
    let mut maximum = current;
    for frame in frames {
        match frame {
            StackFrame::FullFrame {
                frame_type: 255,
                locals: entries,
                ..
            } => {
                locals.clear();
                locals.extend(entries.iter().map(verification_width));
                current = slot_count(&locals)?;
            }
            StackFrame::AppendFrame {
                frame_type: frame_type @ 252..=254,
                locals: entries,
                ..
            } if entries.len() == usize::from(frame_type - 251) => {
                for width in entries.iter().map(verification_width) {
                    current = u16::try_from(u32::from(current) + u32::from(width))?;
                    locals.push(width);
                }
            }
            StackFrame::ChopFrame {
                frame_type: frame_type @ 248..=250,
                ..
            } => {
                let remaining = locals
                    .len()
                    .checked_sub(usize::from(251 - frame_type))
                    .ok_or_else(|| {
                        Error::VerificationError(VerifyError::VerifyError(
                            "StackMapTable chop frame removes more locals than the previous frame"
                                .to_string(),
                        ))
                    })?;
                for width in locals.drain(remaining..) {
                    current -= u16::from(width);
                }
            }
            StackFrame::SameFrame { frame_type: 0..=63 }
            | StackFrame::SameFrameExtended {
                frame_type: 251, ..
            }
            | StackFrame::SameLocals1StackItemFrame {
                frame_type: 64..=127,
                ..
            }
            | StackFrame::SameLocals1StackItemFrameExtended {
                frame_type: 247, ..
            } => {}
            _ => return Err(Error::InvalidStackFrameType(frame.frame_type())),
        }
        // Top entries occupy real slots too. Keep the peak even if a later
        // frame chops locals or replaces the array with a smaller full frame.
        maximum = maximum.max(current);
    }
    Ok(maximum)
}

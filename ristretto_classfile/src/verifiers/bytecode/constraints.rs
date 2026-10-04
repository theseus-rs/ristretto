//! Constraints shared by both bytecode verification algorithms.
use super::frame::Frame;
use super::handlers::references::ConstantPoolResolver;
use super::type_system::VerificationType;
use crate::attributes::Instruction;
use crate::verifiers::error::{Result, VerifyError};
use crate::{ClassFile, FieldType};

fn invalid(message: &str) -> VerifyError {
    VerifyError::VerifyError(message.to_string())
}

fn receiver<'a>(frame: &'a Frame, descriptor: &str) -> Result<&'a VerificationType> {
    let descriptor = crate::JavaStr::cow_from_str(descriptor);
    let (parameters, _) = FieldType::parse_method_descriptor(&descriptor)?;
    let slots = parameters
        .iter()
        .map(|p| {
            if VerificationType::from_field_type(p).is_category2() {
                2
            } else {
                1
            }
        })
        .sum();
    frame.peek_at(slots)
}

/// Validate constructor receiver identity before an instruction changes the frame.
#[expect(
    clippy::too_many_lines,
    reason = "receiver and constructor constraints"
)]
pub(super) fn verify_state<C: crate::verifiers::context::VerificationContext>(
    class: &ClassFile<'_>,
    code: &[Instruction],
    instruction: &Instruction,
    frame: &Frame,
    _context: &C,
) -> Result<()> {
    let resolver = ConstantPoolResolver::new(class);
    match instruction {
        Instruction::Invokespecial(index) => {
            let (owner, name, descriptor) = resolver.resolve_method_ref(*index)?;
            if name == "<init>" {
                match receiver(frame, &descriptor)? {
                    VerificationType::Uninitialized(offset) => {
                        let Some(Instruction::New(class_index)) = code.get(usize::from(*offset))
                        else {
                            return Err(invalid("Constructor receiver does not refer to new"));
                        };
                        if resolver.resolve_class(*class_index)? != owner {
                            return Err(invalid(
                                "Constructor owner does not match allocated class",
                            ));
                        }
                    }
                    VerificationType::UninitializedThis => {
                        let current = class.class_name()?.to_str_lossy();
                        let parent = if class.super_class == 0 {
                            String::new()
                        } else {
                            resolver.resolve_class(class.super_class)?
                        };
                        if owner != current && owner != parent {
                            return Err(invalid(
                                "Constructor must initialize this or its direct superclass",
                            ));
                        }
                    }
                    _ => return Err(invalid("Constructor requires an uninitialized receiver")),
                }
            }
        }
        Instruction::Putfield(index) => {
            let (owner, name, descriptor) = resolver.resolve_field_ref(*index)?;
            let ty = FieldType::parse_java_str(&crate::JavaStr::cow_from_str(&descriptor))?;
            let slots = if VerificationType::from_field_type(&ty).is_category2() {
                2
            } else {
                1
            };
            if *frame.peek_at(slots)? == VerificationType::UninitializedThis
                && (owner != class.class_name()?.to_str_lossy()
                    || !class.fields.iter().any(|field| {
                        class
                            .constant_pool
                            .try_get_utf8(field.name_index)
                            .is_ok_and(|n| n == name.as_str())
                            && class
                                .constant_pool
                                .try_get_utf8(field.descriptor_index)
                                .is_ok_and(|d| d == descriptor.as_str())
                    }))
            {
                return Err(invalid(
                    "putfield on uninitialized this must name a field declared by this class",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

/// A failed constructor call makes aliases of its receiver unusable in a handler.
pub(super) fn invalidate_constructor_receiver(
    class: &ClassFile<'_>,
    code: &[Instruction],
    offset: u16,
    before: &Frame,
    handler: &mut Frame,
) -> Result<()> {
    if let Some(Instruction::Invokespecial(index)) = code.get(usize::from(offset)) {
        let (_, name, descriptor) = ConstantPoolResolver::new(class).resolve_method_ref(*index)?;
        if name == "<init>" {
            let object = receiver(before, &descriptor)?;
            for local in &mut handler.locals {
                if local == object {
                    *local = VerificationType::Top;
                }
            }
        }
    }
    Ok(())
}

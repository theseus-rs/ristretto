//! Indexed field references shared by the synchronous field instructions.

use crate::JavaError::NullPointerException;
use crate::Result;
use crate::field_ref_cache::{ResolvedFieldRef, resolve_field_ref};
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use ristretto_classfile::attributes::Instruction;
use std::sync::OnceLock;

impl ResolvedFieldRef {
    fn execute(
        &self,
        frame: &Frame,
        stack: &mut OperandStack,
        instruction: &Instruction,
    ) -> Result<ExecutionResult> {
        if matches!(
            instruction,
            Instruction::Putfield(_) | Instruction::Putstatic(_)
        ) {
            self.check_write(frame)?;
        }
        match instruction {
            Instruction::Getstatic(_) => stack.push(self.get_static()?)?,
            Instruction::Putstatic(_) => self.put_static(stack.pop()?)?,
            Instruction::Getfield(_) => {
                let object = stack.pop()?;
                if object.is_null() {
                    return Err(NullPointerException(None).into());
                }
                let object = object.as_object_ref()?;
                stack.push(self.get(&object)?)?;
            }
            Instruction::Putfield(_) => {
                let value = stack.pop()?;
                let object = stack.pop()?;
                if object.is_null() {
                    return Err(NullPointerException(None).into());
                }
                let mut object = object.as_object_mut()?;
                self.put(&mut object, value)?;
            }
            _ => {
                return Err(crate::Error::InternalError(format!(
                    "Expected field instruction: {instruction:?}"
                )));
            }
        }
        Ok(ExecutionResult::Continue)
    }
}

pub(crate) fn try_field(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
    instruction: &Instruction,
) -> Result<Option<ExecutionResult>> {
    let Some(resolved) = frame.field_refs()?.get(index).and_then(OnceLock::get) else {
        return Ok(None);
    };
    resolved.check_kind(matches!(
        instruction,
        Instruction::Getstatic(_) | Instruction::Putstatic(_)
    ))?;
    if resolved.is_static() && !resolved.declaring_class.is_initialized()? {
        return Ok(None);
    }
    resolved.execute(frame, stack, instruction).map(Some)
}

pub(crate) async fn execute_field(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
    instruction: Instruction,
) -> Result<ExecutionResult> {
    let resolved = resolve_field_ref(frame, index).await?;
    resolved.check_kind(matches!(
        instruction,
        Instruction::Getstatic(_) | Instruction::Putstatic(_)
    ))?;
    if matches!(
        instruction,
        Instruction::Putfield(_) | Instruction::Putstatic(_)
    ) {
        resolved.check_write(frame)?;
    }
    if resolved.is_static() && !resolved.declaring_class.is_initialized()? {
        frame
            .thread()?
            .initialize_class(&resolved.declaring_class)
            .await?;
    }
    resolved.execute(frame, stack, &instruction)
}

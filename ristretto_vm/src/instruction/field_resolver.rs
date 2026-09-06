//! Indexed field references shared by the synchronous field instructions.

use crate::JavaError::{IncompatibleClassChangeError, NullPointerException};
use crate::Result;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::reference_cache::ReferenceCache;
use ristretto_classfile::{JavaString, attributes::Instruction};
use ristretto_classloader::Class;
use std::sync::{Arc, OnceLock};

#[derive(Debug)]
pub(crate) struct ResolvedFieldRef {
    class: Arc<Class>,
    name: JavaString,
    is_static: bool,
}

pub(crate) type FieldRefEntry = OnceLock<Arc<ResolvedFieldRef>>;
pub(crate) type FieldRefCache = ReferenceCache<FieldRefEntry>;

impl ResolvedFieldRef {
    fn check_kind(&self, instruction: &Instruction) -> Result<()> {
        let is_static = matches!(
            instruction,
            Instruction::Getstatic(_) | Instruction::Putstatic(_)
        );
        if self.is_static != is_static {
            return Err(IncompatibleClassChangeError(format!(
                "Field {}.{} {} static",
                self.class.name(),
                self.name,
                if self.is_static { "is" } else { "is not" }
            ))
            .into());
        }
        Ok(())
    }

    fn execute(
        &self,
        stack: &mut OperandStack,
        instruction: &Instruction,
    ) -> Result<ExecutionResult> {
        match instruction {
            Instruction::Getstatic(_) => stack.push(self.class.static_value(&*self.name)?)?,
            Instruction::Putstatic(_) => self.class.set_static_value(&*self.name, stack.pop()?)?,
            Instruction::Getfield(_) => {
                let object = stack.pop()?;
                if object.is_null() {
                    return Err(NullPointerException(None).into());
                }
                let object = object.as_object_ref()?;
                stack.push(object.value_in_class(&self.class, &*self.name)?)?;
            }
            Instruction::Putfield(_) => {
                let value = stack.pop()?;
                let object = stack.pop()?;
                if object.is_null() {
                    return Err(NullPointerException(None).into());
                }
                object
                    .as_object_mut()?
                    .set_value_in_class(&self.class, &*self.name, value)?;
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
    resolved.check_kind(instruction)?;
    if resolved.is_static && !resolved.class.is_initialized()? {
        return Ok(None);
    }
    resolved.execute(stack, instruction).map(Some)
}

pub(crate) async fn execute_field(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
    instruction: Instruction,
) -> Result<ExecutionResult> {
    let entry = frame
        .field_refs()?
        .get(index)
        .ok_or(ristretto_classfile::Error::InvalidConstantPoolIndex(index))?;
    let resolved = if let Some(resolved) = entry.get() {
        resolved.clone()
    } else {
        let pool = frame.class().constant_pool();
        let (class_index, name_and_type) = pool.try_get_field_ref(index)?;
        let (name_index, _descriptor_index) = pool.try_get_name_and_type(*name_and_type)?;
        let name = pool.try_get_utf8(*name_index)?.to_owned();
        let thread = frame.thread()?;
        let class = thread
            .load_and_link_class(pool.try_get_class(*class_index)?)
            .await?;
        let field = class.declared_field(&*name)?;
        let resolved = Arc::new(ResolvedFieldRef {
            class,
            name,
            is_static: field
                .access_flags()
                .contains(ristretto_classfile::FieldAccessFlags::STATIC),
        });
        entry.get_or_init(|| resolved).clone()
    };

    resolved.check_kind(&instruction)?;
    if resolved.is_static && !resolved.class.is_initialized()? {
        frame.thread()?.initialize_class(&resolved.class).await?;
    }

    resolved.execute(stack, &instruction)
}

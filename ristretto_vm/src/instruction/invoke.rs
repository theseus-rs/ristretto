//! Synchronous invocation fast path shared by the four symbolic invoke instructions.

use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::frame::{ExecutionResult, Frame, MethodCall};
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;
use ristretto_classloader::{Class, Reference, Value};
use std::sync::Arc;

/// Ordinary objects and reference arrays carry their actual class, including loader identity.
/// Primitive arrays need the class-loading slow path.
pub(crate) fn receiver_class(value: &Value) -> Result<Option<Arc<Class>>> {
    match value {
        Value::Object(Some(reference)) => {
            let reference = reference.read();
            Ok(match &*reference {
                Reference::Object(object) => Some(object.class().clone()),
                Reference::Array(array) => Some(array.class.clone()),
                _ => None,
            })
        }
        Value::Object(None) => Err(NullPointerException(None).into()),
        _ => Err(InternalError("Expected object reference".to_string())),
    }
}

/// A miss leaves every operand in place for the explicit slow instruction.
pub(crate) fn try_invoke(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
    kind: InvokeKind,
) -> Result<Option<ExecutionResult>> {
    let Some(resolution) = frame
        .method_refs()?
        .get(index)
        .and_then(|entry| entry.get(kind))
    else {
        return Ok(None);
    };
    let receiver_count = usize::from(kind != InvokeKind::Static);
    let (class, method) = if kind == InvokeKind::Static {
        if !resolution.declaring_class.is_initialized()? {
            return Ok(None);
        }
        (
            resolution.declaring_class.clone(),
            resolution.method.clone(),
        )
    } else {
        let receiver = stack.peek_at(resolution.param_count)?;
        // Check null even for private methods and invokespecial.
        let receiver_class = receiver_class(receiver)?;
        if kind == InvokeKind::Special
            || (kind == InvokeKind::Virtual && resolution.method.is_private())
        {
            (
                resolution.declaring_class.clone(),
                resolution.method.clone(),
            )
        } else {
            let Some(receiver_class) = receiver_class else {
                return Ok(None);
            };
            let Some(target) = resolution.dispatch.get(&receiver_class) else {
                return Ok(None);
            };
            (target.class.clone(), target.method.clone())
        }
    };
    Ok(Some(ExecutionResult::Call(MethodCall {
        class,
        method,
        parameters: stack.drain_last(resolution.param_count + receiver_count),
        has_return_type: resolution.has_return_type,
    })))
}

//! Synchronous invocation fast path shared by the four symbolic invoke instructions.

use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::frame::{CallParameters, ExecutionResult, Frame, MethodCall};
use crate::method_ref_cache::{InvokeKind, ReceiverCache, ReceiverTarget};
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

/// Borrow the receiver's class only long enough to probe the checked dispatch cache.
/// The target is owned by the cache, so a hit needs no temporary class Arc clone.
fn receiver_target<'a>(
    value: &Value,
    cache: &'a ReceiverCache,
) -> Result<Option<&'a ReceiverTarget>> {
    match value {
        Value::Object(Some(reference)) => {
            let reference = reference.read();
            Ok(match &*reference {
                Reference::Object(object) => cache.get(object.class()),
                Reference::Array(array) => cache.get(&array.class),
                _ => None,
            })
        }
        Value::Object(None) => Err(NullPointerException(None).into()),
        _ => Err(InternalError("Expected object reference".to_string())),
    }
}

/// A miss leaves every operand in place for the explicit slow instruction.
#[inline]
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
        if kind == InvokeKind::Special
            || (kind == InvokeKind::Virtual && resolution.method.is_private())
        {
            // These calls still require a non-null reference, but no receiver-class lookup.
            match receiver {
                Value::Object(Some(_)) => {}
                Value::Object(None) => return Err(NullPointerException(None).into()),
                _ => return Err(InternalError("Expected object reference".to_string())),
            }
            (
                resolution.declaring_class.clone(),
                resolution.method.clone(),
            )
        } else {
            let Some(target) = receiver_target(receiver, &resolution.dispatch)? else {
                return Ok(None);
            };
            (target.class.clone(), target.method.clone())
        }
    };
    Ok(Some(ExecutionResult::Call(MethodCall {
        class,
        method,
        parameters: CallParameters::Stack(resolution.param_count + receiver_count),
        has_return_type: resolution.has_return_type,
    })))
}

use crate::Error::InternalError;
use crate::JavaError::{
    AbstractMethodError, IllegalAccessError, IncompatibleClassChangeError, NullPointerException,
};
use crate::Result;
use crate::assignable::Assignable;
use crate::frame::{ExecutionResult, Frame, MethodCall};
use crate::instruction::{lookup_method, receiver_class, resolve_method_ref};
use crate::method_ref_cache::{InvokeKind, ReceiverTarget};
use crate::operand_stack::OperandStack;
use ristretto_classloader::Value;

/// Invokeinterface instruction implementation.
///
/// # References
///
/// - [JVMS §6.5.invokeinterface](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.5.invokeinterface)
#[inline]
pub(crate) async fn invokeinterface(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
    _count: u8,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;

    // Resolve the interface method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Interface).await?;

    let parameters = stack.drain_last(resolution.param_count + 1);
    let receiver = parameters
        .first()
        .ok_or_else(|| InternalError("Expected object reference".to_string()))?;
    let object_class = if let Some(class) = receiver_class(receiver)? {
        class
    } else {
        let Value::Object(Some(reference)) = receiver else {
            return Err(NullPointerException(None).into());
        };
        let class_name = reference.read().class_name()?;
        thread.class(&class_name).await?
    };
    if let Some(target) = resolution.dispatch.get(&object_class) {
        return Ok(ExecutionResult::Call(MethodCall {
            class: target.class.clone(),
            method: target.method.clone(),
            parameters,
            has_return_type: resolution.has_return_type,
        }));
    }

    // Check object implements interface
    if !resolution
        .referenced_class
        .is_assignable_from(&thread, &object_class)
        .await?
    {
        return Err(IncompatibleClassChangeError(format!(
            "{} does not implement {}",
            object_class.name(),
            resolution.referenced_class.name()
        ))
        .into());
    }

    // Find the method implementation in the actual receiver class
    let (resolved_class, resolved_method) = lookup_method(
        &object_class,
        &resolution.method_name,
        &resolution.method_descriptor,
    )?;

    if resolved_method.is_static() {
        return Err(IncompatibleClassChangeError(format!(
            "Method {}.{} is static",
            resolved_class.name(),
            resolution.method_name
        ))
        .into());
    }

    // Check resolved method accessibility
    // Lambda methods (like lambda$andThen$0) are private but can be invoked through method handles
    if !resolved_method.is_public() && !resolution.method_name.starts_with("lambda$") {
        return Err(IllegalAccessError(format!(
            "Method {}.{} is not public",
            resolved_class.name(),
            resolution.method_name,
        ))
        .into());
    }

    if resolved_method.is_abstract() {
        return Err(AbstractMethodError(format!(
            "Method {}.{} is abstract",
            resolved_class.name(),
            resolution.method_name,
        ))
        .into());
    }

    resolution.dispatch.store(ReceiverTarget {
        receiver_class: object_class,
        class: resolved_class.clone(),
        method: resolved_method.clone(),
    });

    Ok(ExecutionResult::Call(MethodCall {
        class: resolved_class,
        method: resolved_method,
        parameters,
        has_return_type: resolution.has_return_type,
    }))
}

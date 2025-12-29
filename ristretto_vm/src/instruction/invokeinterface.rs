use crate::Error::InternalError;
use crate::JavaError::{
    AbstractMethodError, IllegalAccessError, IncompatibleClassChangeError, NullPointerException,
};
use crate::Result;
use crate::assignable::Assignable;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::{lookup_method, resolve_method_ref};
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;
use ristretto_classfile::FieldType;
use ristretto_classloader::Value;

/// Invokeinterface instruction implementation.
///
/// # References
///
/// - [JVMS §6.5.invokeinterface](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokeinterface)
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

    let (method_parameters, _method_return_type) =
        FieldType::parse_method_descriptor(&resolution.method_descriptor)?;
    let parameters = stack.drain_last(method_parameters.len() + 1);

    let object_class = match parameters.first() {
        Some(Value::Object(Some(reference))) => {
            let class_name = {
                let guard = reference.read();
                guard.class_name()?.clone()
            };
            thread.class(&class_name).await?
        }
        Some(Value::Object(None)) => {
            return Err(NullPointerException("null 'this' reference".to_string()).into());
        }
        _ => return Err(InternalError("Expected object reference".to_string())),
    };

    // Check object implements interface
    if !resolution
        .declaring_class
        .is_assignable_from(&thread, &object_class)
        .await?
    {
        return Err(IncompatibleClassChangeError(format!(
            "{} does not implement {}",
            object_class.name(),
            resolution.declaring_class.name()
        ))
        .into());
    }

    // Find the method implementation in the actual receiver class
    let (resolved_class, resolved_method) = lookup_method(
        &object_class,
        &resolution.method_name,
        &resolution.method_descriptor,
    )?;

    // Check resolved method accessibility
    if !resolved_method.is_public() {
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

    // Execute the method
    let result = Box::pin(thread.execute(&resolved_class, &resolved_method, &parameters)).await?;
    if let Some(value) = result {
        stack.push(value)?;
    }
    Ok(Continue)
}

use crate::Error::InternalError;
use crate::JavaError::{
    AbstractMethodError, IllegalAccessError, IncompatibleClassChangeError, NullPointerException,
};
use crate::Result;
use crate::assignable::Assignable;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::resolve_method;
use crate::operand_stack::OperandStack;
use ristretto_classfile::FieldType;
use ristretto_classloader::Value;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokeinterface>
#[inline]
pub(crate) async fn invokeinterface(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
    _count: u8,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) =
        constant_pool.try_get_interface_method_ref(method_index)?;
    let interface_name = constant_pool.try_get_class(*class_index)?;
    let interface_class = thread.class(interface_name).await?;

    if !interface_class.is_interface() {
        return Err(IncompatibleClassChangeError(format!(
            "{} is not an interface",
            interface_class.name()
        ))
        .into());
    }

    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let (method_parameters, _method_return_type) =
        FieldType::parse_method_descriptor(method_descriptor)?;
    let parameters = stack.drain_last(method_parameters.len() + 1);

    let object_class = match parameters.first() {
        Some(Value::Object(Some(reference))) => {
            let class_name = {
                let guard = reference.read();
                guard.class_name()?.to_string()
            };
            thread.class(&class_name).await?
        }
        Some(Value::Object(None)) => {
            return Err(NullPointerException("null 'this' reference".to_string()).into());
        }
        _ => return Err(InternalError("Expected object reference".to_string())),
    };

    // Check object implements interface
    if !interface_class
        .is_assignable_from(&thread, &object_class)
        .await?
    {
        return Err(IncompatibleClassChangeError(format!(
            "{} does not implement {}",
            object_class.name(),
            interface_class.name()
        ))
        .into());
    }

    // Find the method implementation
    let (resolved_class, resolved_method) =
        resolve_method(&object_class, method_name, method_descriptor)?;

    // Check resolved method accessibility
    if !resolved_method.is_public() {
        return Err(IllegalAccessError(format!(
            "Method {}.{method_name} is not public",
            resolved_class.name(),
        ))
        .into());
    }

    if resolved_method.is_abstract() {
        return Err(AbstractMethodError(format!(
            "Method {}.{method_name} is abstract",
            resolved_class.name(),
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

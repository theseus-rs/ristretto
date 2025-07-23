use crate::JavaError::{IncompatibleClassChangeError, NoSuchMethodError};
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::lookup_method_in_hierarchy;
use crate::operand_stack::OperandStack;
use ristretto_classfile::Constant;
use ristretto_classfile::Error::InvalidConstantPoolIndexType;
use ristretto_classloader::{Class, Method};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokestatic>
#[inline]
pub(crate) async fn invokestatic(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let current_class = frame.class();
    let constant_pool = current_class.constant_pool();
    let constant = constant_pool.try_get(method_index)?;

    let (class, name_and_type_index) = match constant {
        Constant::MethodRef {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.try_get_class(*class_index)?;
            let resolved_class = thread.class(class_name).await?;
            if resolved_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Expected class, found interface: {class_name}"
                ))
                .into());
            }
            (resolved_class, name_and_type_index)
        }
        Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.try_get_class(*class_index)?;
            let resolved_class = thread.class(class_name).await?;
            if !resolved_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Expected interface, found class: {class_name}"
                ))
                .into());
            }
            (resolved_class, name_and_type_index)
        }
        _ => return Err(InvalidConstantPoolIndexType(method_index).into()),
    };

    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let (resolved_class, method) =
        lookup_method_in_hierarchy(class, method_name, method_descriptor)?;

    if !method.is_static() {
        return Err(IncompatibleClassChangeError(format!(
            "Method {method_name}{method_descriptor} is not static"
        ))
        .into());
    }

    let parameters = stack.drain_last(method.parameters().len());
    let result = thread
        .execute(&resolved_class, &method, &parameters)
        .await?;
    if let Some(value) = result {
        stack.push(value)?;
    }

    Ok(Continue)
}

/// Get a static method by name and descriptor, searching the inheritance hierarchy.
///
/// # Errors
///
/// if the method is not found.
fn try_get_static_method(
    class: Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<(Arc<Class>, Arc<Method>)> {
    if let Some(method) = class.method(name, descriptor) {
        return Ok((class, method));
    }

    let mut super_class = class.parent()?;
    while let Some(class) = super_class {
        if let Some(method) = class.method(name, descriptor) {
            return Ok((class, method));
        }
        super_class = class.parent()?;
    }
    Err(NoSuchMethodError(format!(
        "Static method {name}{descriptor} not found in class {}",
        class.name()
    ))
    .into())
}

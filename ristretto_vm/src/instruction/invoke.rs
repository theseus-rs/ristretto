use crate::call_stack::CallStack;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::RuntimeError;
use crate::{Result, VM};
use ristretto_classfile::ConstantPool;
use ristretto_classloader::{Class, Method, Reference, Value};
use std::sync::Arc;

#[derive(Debug)]
enum InvocationType {
    Interface,
    Special,
    Static,
    Virtual,
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokevirtual>
#[inline]
pub(crate) fn invokevirtual(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<ExecutionResult> {
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(call_stack, class_name)?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = class.try_get_virtual_method(method_name, method_descriptor)?;

    invoke_method(
        vm,
        call_stack,
        stack,
        class,
        method,
        &InvocationType::Virtual,
    )
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokespecial>
#[inline]
pub(crate) fn invokespecial(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<ExecutionResult> {
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(call_stack, class_name)?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = class.try_get_method(method_name, method_descriptor)?;

    invoke_method(
        vm,
        call_stack,
        stack,
        class,
        method,
        &InvocationType::Special,
    )
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokestatic>
#[inline]
pub(crate) fn invokestatic(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<ExecutionResult> {
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(call_stack, class_name)?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = class.try_get_method(method_name, method_descriptor)?;

    invoke_method(
        vm,
        call_stack,
        stack,
        class,
        method,
        &InvocationType::Static,
    )
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokeinterface>
#[inline]
pub(crate) fn invokeinterface(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<ExecutionResult> {
    let (class_index, name_and_type_index) =
        constant_pool.try_get_interface_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(call_stack, class_name)?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = class.try_get_virtual_method(method_name, method_descriptor)?;

    invoke_method(
        vm,
        call_stack,
        stack,
        class,
        method,
        &InvocationType::Interface,
    )
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokedynamic>
#[inline]
pub(crate) fn invokedynamic(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _stack: &mut OperandStack,
    _constant_pool: &ConstantPool,
    _method_index: u16,
) -> Result<ExecutionResult> {
    todo!()
}

/// Invoke the method at the specified index
///
/// # Errors
/// if the method is not found
#[inline]
fn invoke_method(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    mut class: Arc<Class>,
    mut method: Arc<Method>,
    invocation_type: &InvocationType,
) -> Result<ExecutionResult> {
    let parameters = method.parameters().len();
    let mut arguments = if method.is_static() {
        Vec::with_capacity(parameters)
    } else {
        // Add one for the object reference
        Vec::with_capacity(parameters + 1)
    };
    for _ in 0..parameters {
        arguments.push(stack.pop()?);
    }
    if !method.is_static() {
        let object = stack.pop_object()?;
        arguments.push(Value::Object(object));
    }
    arguments.reverse();

    // TODO: evaluate refactoring this
    match invocation_type {
        InvocationType::Interface | InvocationType::Virtual => {
            let Some(Value::Object(Some(reference))) = arguments.first() else {
                return Err(RuntimeError("No reference found".to_string()));
            };
            class = match reference {
                Reference::Array(class, _) => class.clone(),
                Reference::Object(object) => object.class().clone(),
                _ => {
                    // Primitive types do not have a class associated with them so the class must be
                    // created from the class name.
                    let class_name = reference.class_name();
                    vm.class(call_stack, &class_name)?
                }
            };
            let method_name = method.name();
            let method_descriptor = method.descriptor();

            // Find the method in the class hierarchy; the Method.try_get_virtual_method() cannot
            // currently be used here because the class constant pool associated with the method is
            // required for execution.
            loop {
                if let Some(class_method) = class.method(method_name, method_descriptor) {
                    method = class_method;
                    break;
                }
                let Some(parent_class) = class.parent()? else {
                    return Err(RuntimeError(
                        "No virtual method found for class".to_string(),
                    ));
                };
                class = parent_class;
            }
        }
        _ => {}
    }

    let result = call_stack.execute(vm, &class, &method, arguments)?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    // #[test]
    // fn test_invokevirtual() -> Result<()> {
    //     todo!()
    // }

    // #[test]
    // fn test_invokespecial() -> Result<()> {
    //     todo!()
    // }

    // #[test]
    // fn test_invokestatic() -> Result<()> {
    //     todo!()
    // }

    // #[test]
    // fn test_invokeinterface() -> Result<()> {
    //     todo!()
    // }

    // #[test]
    // fn test_invokedynamic() -> Result<()> {
    //     todo!()
    // }
}

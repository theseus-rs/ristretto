use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::{Error, Result};
use ristretto_classfile::Constant;
use ristretto_classfile::Error::InvalidConstantPoolIndexType;
use ristretto_classloader::Error::MethodNotFound;
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
pub(crate) async fn invokevirtual(frame: &Frame, method_index: u16) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = try_get_virtual_method(&class, method_name, method_descriptor)?;

    invoke_method(&thread, frame, class, method, &InvocationType::Virtual).await
}

/// Get a virtual method by name and descriptor.
///
/// # Errors
/// if the method is not found.
fn try_get_virtual_method<S: AsRef<str>>(
    class: &Arc<Class>,
    name: S,
    descriptor: S,
) -> Result<Arc<Method>> {
    let name = name.as_ref();
    let descriptor = descriptor.as_ref();

    if let Some(method) = class.method(name, descriptor) {
        return Ok(method);
    }

    for interface in class.interfaces()? {
        if let Ok(method) = try_get_virtual_method(&interface, name, descriptor) {
            return Ok(method);
        }
    }

    let Some(parent) = class.parent()? else {
        return Err(Error::from(MethodNotFound {
            class_name: class.name().to_string(),
            method_name: name.to_string(),
            method_descriptor: descriptor.to_string(),
        }));
    };

    let Ok(method) = try_get_virtual_method(&parent, name, descriptor) else {
        return Err(Error::from(MethodNotFound {
            class_name: class.name().to_string(),
            method_name: name.to_string(),
            method_descriptor: descriptor.to_string(),
        }));
    };
    Ok(method)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokespecial>
#[inline]
pub(crate) async fn invokespecial(frame: &Frame, method_index: u16) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let (method_class, method) = try_get_special_method(&class, method_name, method_descriptor)?;

    invoke_method(
        &thread,
        frame,
        method_class,
        method,
        &InvocationType::Special,
    )
    .await
}

/// Get a special method by name and descriptor.
///
/// # Errors
/// if the method is not found.
fn try_get_special_method<S: AsRef<str>>(
    class: &Arc<Class>,
    name: S,
    descriptor: S,
) -> Result<(Arc<Class>, Arc<Method>)> {
    let name = name.as_ref();
    let descriptor = descriptor.as_ref();

    if let Some(method) = class.method(name, descriptor) {
        return Ok((class.clone(), method));
    }

    let Some(parent) = class.parent()? else {
        return Err(Error::from(MethodNotFound {
            class_name: class.name().to_string(),
            method_name: name.to_string(),
            method_descriptor: descriptor.to_string(),
        }));
    };

    let Ok((class, method)) = try_get_special_method(&parent, name, descriptor) else {
        return Err(Error::from(MethodNotFound {
            class_name: class.name().to_string(),
            method_name: name.to_string(),
            method_descriptor: descriptor.to_string(),
        }));
    };
    Ok((class, method))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokestatic>
#[inline]
pub(crate) async fn invokestatic(frame: &Frame, method_index: u16) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let constant = constant_pool.try_get(method_index)?;
    let (Constant::MethodRef {
        class_index,
        name_and_type_index,
    }
    | Constant::InterfaceMethodRef {
        class_index,
        name_and_type_index,
    }) = constant
    else {
        return Err(InvalidConstantPoolIndexType(method_index).into());
    };
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = class.try_get_method(method_name, method_descriptor)?;

    invoke_method(&thread, frame, class, method, &InvocationType::Static).await
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokeinterface>
#[inline]
pub(crate) async fn invokeinterface(
    frame: &Frame,
    method_index: u16,
    _count: u8,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) =
        constant_pool.try_get_interface_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method = try_get_virtual_method(&class, method_name, method_descriptor)?;

    invoke_method(&thread, frame, class, method, &InvocationType::Interface).await
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.invokedynamic>
#[inline]
pub(crate) async fn invokedynamic(_frame: &Frame, _method_index: u16) -> Result<ExecutionResult> {
    todo!("invokedynamic")
}

/// Invoke the method at the specified index
///
/// # Errors
/// if the method is not found
#[inline]
async fn invoke_method(
    thread: &Thread,
    frame: &Frame,
    mut class: Arc<Class>,
    mut method: Arc<Method>,
    invocation_type: &InvocationType,
) -> Result<ExecutionResult> {
    let stack = frame.stack();
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
                return Err(InternalError("No reference found".to_string()));
            };
            class = match reference {
                Reference::Array(class, _) => class.clone(),
                Reference::Object(object) => object.class().clone(),
                _ => {
                    // Primitive types do not have a class associated with them so the class must be
                    // created from the class name.
                    let class_name = reference.class_name();
                    thread.class(&class_name).await?
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
                    return Err(InternalError(
                        "No virtual method found for class".to_string(),
                    ));
                };
                class = parent_class;
            }
        }
        _ => {}
    }

    // Execute the method on the current thread
    let result = thread.execute(&class, &method, arguments, true).await?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::ClassLoaderError;
    use crate::VM;

    #[tokio::test]
    async fn test_try_get_virtual_method_hierarchy() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java/util/TreeMap").await?;
        let method = try_get_virtual_method(&class, "size", "()I");
        assert!(method.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_virtual_method_interface_hierarchy() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java/util/NavigableMap").await?;
        let method = try_get_virtual_method(&class, "size", "()I");
        assert!(method.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_virtual_method_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java/util/TreeMap").await?;
        let result = try_get_virtual_method(&class, "foo", "()V");
        assert!(matches!(
            result,
            Err(ClassLoaderError(MethodNotFound {
                class_name,
                method_name,
                method_descriptor
            })) if class_name == "java/util/TreeMap" && method_name == "foo" && method_descriptor == "()V"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_special_method() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java/util/AbstractSet").await?;
        let (method_class, method) =
            try_get_special_method(&class, "addAll", "(Ljava/util/Collection;)Z")?;
        assert_eq!(method_class.name(), "java/util/AbstractCollection");
        assert_eq!(method.name(), "addAll");
        assert_eq!(method.descriptor(), "(Ljava/util/Collection;)Z");
        Ok(())
    }

    #[tokio::test]
    async fn test_try_get_special_method_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java/util/AbstractSet").await?;
        let result = try_get_special_method(&class, "foo", "()V");
        assert!(matches!(
            result,
            Err(ClassLoaderError(MethodNotFound {
                class_name,
                method_name,
                method_descriptor
            })) if class_name == "java/util/AbstractSet" && method_name == "foo" && method_descriptor == "()V"
        ));
        Ok(())
    }
}

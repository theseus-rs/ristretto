use crate::Error::InternalError;
use crate::JavaError::{NoSuchMethodError, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use ristretto_classloader::{Class, Method, Value};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokevirtual>
#[inline]
pub(crate) async fn invokevirtual(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(method_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let (resolved_class, resolved_method) = resolve_method(class, method_name, method_descriptor)?;

    let parameters = stack.drain_last(resolved_method.parameters().len() + 1);
    let reference = match parameters.first() {
        Some(Value::Object(Some(reference))) => reference,
        Some(Value::Object(None)) => {
            return Err(NullPointerException("null 'this' reference".to_string()).into());
        }
        _ => return Err(InternalError("Expected object reference".to_string())),
    };

    let (class, method) = if resolved_method.is_private() {
        (resolved_class, resolved_method)
    } else {
        let class_name = reference.class_name();
        let object_class = thread.class(class_name).await?;
        resolve_method(object_class, method_name, method_descriptor)?
    };

    let result = thread.execute(&class, &method, &parameters).await?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}

/// Get a virtual method by name and descriptor.
///
/// # Errors
///
/// if the method is not found.
pub(crate) fn resolve_method(
    class: Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<(Arc<Class>, Arc<Method>)> {
    if let Ok(result) = lookup_method_in_hierarchy(class.clone(), name, descriptor) {
        return Ok(result);
    }

    // Search all interfaces from the current class up to the root for a default method
    let mut current_class = Some(class.clone());
    while let Some(class) = current_class {
        let mut interfaces = class.interfaces()?;
        while let Some(interface) = interfaces.pop() {
            if let Ok((interface, method)) =
                lookup_method_in_hierarchy(interface.clone(), name, descriptor)
            {
                if !method.is_abstract() {
                    return Ok((interface, method));
                }
            }
            let super_interfaces = interface.interfaces()?;
            interfaces.extend(super_interfaces);
        }
        current_class = class.parent()?;
    }

    Err(NoSuchMethodError(format!(
        "Method {name}{descriptor} not found in class {}",
        class.name(),
    ))
    .into())
}

/// Get a method in the class hierarchy by name and descriptor.
///
/// # Errors
///
/// if the method is not found.
pub(crate) fn lookup_method_in_hierarchy(
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
        "Method {name}{descriptor} not found in class {}",
        class.name()
    ))
    .into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::JavaError;
    use crate::VM;

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let (resolved_class, method) = lookup_method_in_hierarchy(class, "size", "()I")?;
        assert_eq!(resolved_class.name(), "java/util/TreeMap");
        assert_eq!(method.name(), "size");
        assert_eq!(method.descriptor(), "()I");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_super_class() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.ArrayList").await?;
        let (resolved_class, method) =
            lookup_method_in_hierarchy(class, "toString", "()Ljava/lang/String;")?;
        assert_eq!(resolved_class.name(), "java/util/AbstractCollection");
        assert_eq!(method.name(), "toString");
        assert_eq!(method.descriptor(), "()Ljava/lang/String;");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let result = lookup_method_in_hierarchy(class, "foo", "()V");
        assert!(matches!(
            result,
            Err(JavaError(NoSuchMethodError(message)))
            if message == "Method foo()V not found in class java/util/TreeMap"
        ));
        Ok(())
    }
}

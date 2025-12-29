use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::{lookup_method, resolve_method_ref};
use crate::method_ref_cache::InvokeKind;
use crate::operand_stack::OperandStack;
use ristretto_classloader::Value;

/// Invokevirtual instruction implementation.
///
/// # References
///
/// - [JVMS §6.5.invokevirtual](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokevirtual)
#[inline]
pub(crate) async fn invokevirtual(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;

    // Resolve the method with JPMS checks and caching
    let resolution = resolve_method_ref(frame, method_index, InvokeKind::Virtual).await?;

    let parameters = stack.drain_last(resolution.method.parameters().len() + 1);
    let reference = match parameters.first() {
        Some(Value::Object(Some(reference))) => reference,
        Some(Value::Object(None)) => {
            return Err(NullPointerException("null 'this' reference".to_string()).into());
        }
        _ => return Err(InternalError("Expected object reference".to_string())),
    };

    // Virtual dispatch: if method is not private, look up in receiver's actual class
    let (class, method) = if resolution.method.is_private() {
        (resolution.declaring_class, resolution.method)
    } else {
        let class_name = {
            let guard = reference.read();
            guard.class_name()?.clone()
        };
        let object_class = thread.class(&class_name).await?;
        lookup_method(
            &object_class,
            &resolution.method_name,
            &resolution.method_descriptor,
        )?
    };

    let result = Box::pin(thread.execute(&class, &method, &parameters)).await?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::JavaError;
    use crate::JavaError::NoSuchMethodError;
    use crate::VM;

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let (resolved_class, method) = lookup_method(&class, "size", "()I")?;
        assert_eq!(resolved_class.name(), "java/util/TreeMap");
        assert_eq!(method.name(), "size");
        assert_eq!(method.descriptor(), "()I");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_super_class() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.ArrayList").await?;
        let (resolved_class, method) = lookup_method(&class, "toString", "()Ljava/lang/String;")?;
        assert_eq!(resolved_class.name(), "java/util/AbstractCollection");
        assert_eq!(method.name(), "toString");
        assert_eq!(method.descriptor(), "()Ljava/lang/String;");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let result = lookup_method(&class, "foo", "()V");
        assert!(matches!(
            result,
            Err(JavaError(NoSuchMethodError(message)))
            if message == "Method foo()V not found in class java/util/TreeMap"
        ));
        Ok(())
    }
}

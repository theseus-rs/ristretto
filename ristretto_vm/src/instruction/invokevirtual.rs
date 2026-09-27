use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::frame::{ExecutionResult, Frame, MethodCall};
use crate::instruction::{receiver_class, resolve_method_ref};
use crate::method_ref_cache::{InvokeKind, ReceiverTarget};
use crate::operand_stack::OperandStack;
use ristretto_classloader::{ResolvedMethod, Value};
use ristretto_types::JavaError;

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

    // +1 for the receiver (this)
    let parameters = stack.drain_last(resolution.param_count + 1);
    let receiver = parameters
        .first()
        .ok_or_else(|| InternalError("Expected object reference".to_string()))?;
    let reference = match receiver {
        Value::Object(Some(reference)) => reference,
        Value::Object(None) => {
            return Err(NullPointerException(None).into());
        }
        _ => {
            return Err(InternalError("Expected object reference".to_string()));
        }
    };

    // Virtual dispatch: if method is not private, look up in receiver's actual class
    let resolved = ResolvedMethod {
        declaring_class: resolution.declaring_class.clone(),
        method: resolution.method.clone(),
    };
    let (class, method, intrinsic) = if resolution.method.is_private() {
        (
            resolution.declaring_class.clone(),
            resolution.method.clone(),
            resolution.intrinsic,
        )
    } else {
        let object_class = if let Some(class) = receiver_class(receiver)? {
            class
        } else {
            let class_name = reference.read().class_name()?;
            thread.class(&class_name).await?
        };
        if let Some(target) = resolution.dispatch.get(&object_class) {
            return Ok(ExecutionResult::Call(MethodCall {
                intrinsic: crate::frame::IntrinsicBinding::Resolved(target.intrinsic),
                class: target.class.clone(),
                method: target.method.clone(),
                parameters: parameters.into(),
                has_return_type: resolution.has_return_type,
            }));
        }
        let target = match object_class.select_method(&resolved) {
            Ok(result) => result,
            Err(_) if object_class.is_interface() => {
                // Per JVMS §5.4.6, interfaces implicitly extend java.lang.Object.
                // If the method isn't found on the interface hierarchy, check Object.
                let object_class = thread.class("java/lang/Object").await?;
                object_class.select_method(&resolved)?
            }
            Err(e) => {
                return Err(e.into());
            }
        };
        if target.method.is_static() {
            return Err(JavaError::IncompatibleClassChangeError(format!(
                "Method {}.{} is static",
                target.declaring_class.name(),
                resolution.method_name
            ))
            .into());
        }
        if target.method.is_abstract() {
            return Err(JavaError::AbstractMethodError(format!(
                "Method {}.{} is abstract",
                target.declaring_class.name(),
                resolution.method_name
            ))
            .into());
        }
        let intrinsic = thread
            .vm()?
            .method_registry()
            .method(
                target.declaring_class.name(),
                target.method.name(),
                target.method.descriptor(),
            )
            .copied();
        resolution.dispatch.store(ReceiverTarget {
            intrinsic,
            receiver_class: object_class,
            class: target.declaring_class.clone(),
            method: target.method.clone(),
        });
        (target.declaring_class, target.method, intrinsic)
    };

    Ok(ExecutionResult::Call(MethodCall {
        intrinsic: crate::frame::IntrinsicBinding::Resolved(intrinsic),
        class,
        method,
        parameters: parameters.into(),
        has_return_type: resolution.has_return_type,
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::VM;
    use ristretto_classloader::MethodResolutionError;

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let resolved = class.resolve_method("size", "()I")?;
        assert_eq!(resolved.declaring_class.name(), "java/util/TreeMap");
        assert_eq!(resolved.method.name(), "size");
        assert_eq!(resolved.method.descriptor(), "()I");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_super_class() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.ArrayList").await?;
        let resolved = class.resolve_method("toString", "()Ljava/lang/String;")?;
        assert_eq!(
            resolved.declaring_class.name(),
            "java/util/AbstractCollection"
        );
        assert_eq!(resolved.method.name(), "toString");
        assert_eq!(resolved.method.descriptor(), "()Ljava/lang/String;");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_in_hierarchy_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.TreeMap").await?;
        let result = class.resolve_method("foo", "()V");
        assert!(matches!(
            result,
            Err(MethodResolutionError::NoSuchMethod(message))
            if message == "Method foo()V not found in class java/util/TreeMap"
        ));
        Ok(())
    }
}

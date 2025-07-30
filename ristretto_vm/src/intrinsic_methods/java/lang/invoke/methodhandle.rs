use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::invoke::methodhandlenatives::MemberNameFlags;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, ReferenceKind};
use ristretto_classloader::{Class, Object, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let method_handle = parameters.pop_object()?;
    let target_member = method_handle.value("member")?;
    let target_member = target_member.as_object_ref()?;
    let result = call_method_handle_target(thread, target_member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_basic(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let method_handle = parameters.pop_object()?;
    let target_member = method_handle.value("member")?;
    let target_member = target_member.as_object_ref()?;
    let result = call_method_handle_target(thread, target_member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_exact(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let method_handle = parameters.pop_object()?;
    let member = method_handle.value("member")?;
    let target_member = member.as_object_ref()?;
    let result = call_method_handle_target(thread, target_member, arguments).await?;
    Ok(Some(result))
}

/// Helper: Actually invokes the target referenced by a `MethodHandle`.
pub async fn call_method_handle_target(
    thread: Arc<Thread>,
    member: &Object,
    mut arguments: Vec<Value>,
) -> Result<Value> {
    let member_class = member.value("clazz")?;
    let target_class_object = member_class.as_object_ref()?;
    let target_class_name = target_class_object.value("name")?.as_string()?;
    let target_class = thread.class(target_class_name).await?;
    let member_name = member.value("name")?.as_string()?;

    // Get flags to determine the kind of member and operation
    let flags = member.value("flags")?.as_i32()?;
    let reference_kind_value = (flags
        & (MemberNameFlags::REFERENCE_KIND_MASK.bits()
            << MemberNameFlags::REFERENCE_KIND_SHIFT.bits()))
        >> MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
    let reference_kind_value = u8::try_from(reference_kind_value)?;
    let reference_kind = ReferenceKind::try_from(reference_kind_value)?;

    // Get the descriptor (method signature or field type)
    let member_descriptor = if let Ok(method_type) = member.value("type") {
        let method_descriptor = thread
            .invoke(
                "java.lang.invoke.MethodType",
                "toMethodDescriptorString()Ljava/lang/String;",
                &[method_type],
            )
            .await?;
        match method_descriptor {
            Some(descriptor) => descriptor.as_string()?,
            _ => return Err(InternalError("Invalid MethodType".to_string())),
        }
    } else if let Ok(descriptor) = member
        .value("descriptor")
        .and_then(|value| value.as_string())
    {
        descriptor
    } else {
        return Err(InternalError(
            "MemberName missing type/descriptor".to_string(),
        ));
    };

    match reference_kind {
        ReferenceKind::InvokeVirtual | ReferenceKind::InvokeInterface => {
            let receiver = arguments.remove(0);
            let method = target_class.try_get_method(member_name, member_descriptor)?;
            let mut call_arguments = vec![receiver];
            call_arguments.extend(arguments);
            thread
                .try_execute(&target_class, &method, &call_arguments)
                .await
        }
        ReferenceKind::InvokeStatic => {
            let method = target_class.try_get_method(member_name, member_descriptor)?;
            thread.try_execute(&target_class, &method, &arguments).await
        }
        ReferenceKind::InvokeSpecial | ReferenceKind::NewInvokeSpecial => {
            invoke_special(
                thread,
                target_class,
                member_name,
                member_descriptor,
                arguments,
                matches!(reference_kind, ReferenceKind::NewInvokeSpecial),
            )
            .await
        }
        ReferenceKind::GetField => {
            let argument = arguments.remove(0);
            let receiver = argument.as_object_ref()?;
            Ok(receiver.value(&member_name)?)
        }
        ReferenceKind::GetStatic => {
            let value = target_class.static_value(&member_name)?;
            Ok(value)
        }
        ReferenceKind::PutField => {
            let argument = arguments.remove(0);
            let receiver = argument.as_object_ref()?;
            let value = arguments.remove(0);
            receiver.set_value(&member_name, value)?;
            Ok(Value::Object(None))
        }
        ReferenceKind::PutStatic => {
            let value = arguments.remove(0);
            target_class.set_static_value(&member_name, value)?;
            Ok(Value::Object(None))
        }
    }
}

/// Helper: Invokes a special method (constructor, private method, or super call).
///
/// # Errors
///
/// Returns an error if the method cannot be found or executed.
async fn invoke_special(
    thread: Arc<Thread>,
    target_class: Arc<Class>,
    method_name: String,
    method_descriptor: String,
    mut arguments: Vec<Value>,
    is_constructor: bool,
) -> Result<Value> {
    if is_constructor {
        let start_index = method_descriptor.find('(').unwrap_or_default();
        let end_index = method_descriptor
            .rfind(')')
            .unwrap_or(method_descriptor.len());
        let descriptor = &method_descriptor[start_index..end_index];
        let instance = thread
            .object(target_class.name(), descriptor, arguments.as_slice())
            .await?;
        Ok(instance)
    } else {
        // Regular special invocation (private methods, super calls)
        let receiver = arguments.remove(0);
        let method = target_class.try_get_method(method_name, method_descriptor)?;
        let mut call_arguments = vec![receiver];
        call_arguments.extend(arguments);
        thread
            .try_execute(&target_class, &method, &call_arguments)
            .await
    }
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_interface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_special(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_static(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_virtual(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_interface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_interface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_special(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_static() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_static(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_virtual() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_virtual(thread, Parameters::default()).await;
    }
}

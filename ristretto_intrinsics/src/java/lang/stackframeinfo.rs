use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{JavaObject, Parameters, Result, Thread};
use std::sync::Arc;

/// Expands a `StackFrameInfo` object by resolving its `classOrMemberName` (a `ResolvedMethodName`
/// or `MemberName`) into the `name` (method name) and `type` (descriptor) fields.
///
/// This is the native implementation backing `java.lang.StackFrameInfo.expandStackFrameInfo()`.
/// The JVM calls this method lazily when stack frame details are first accessed through the
/// `StackWalker` API (e.g. from `getMethodName()` or `getMethodType()`).
#[intrinsic_method(
    "java/lang/StackFrameInfo.expandStackFrameInfo()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn expand_stack_frame_info<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Pop `this` (the StackFrameInfo instance)
    let this = parameters.pop()?;

    // The field classOrMemberName may hold a ResolvedMethodName (with `vmholder` = declaring class)
    // or a MemberName (with `clazz` = declaring class).
    let member_name_value = {
        let this_ref = this.as_object_ref()?;
        this_ref
            .value("classOrMemberName")
            .unwrap_or(Value::Object(None))
    };

    // If the member/class reference is null, there is nothing to expand
    if member_name_value.is_null() {
        return Ok(None);
    }

    // Extract class mirror and method name from the member object.
    // ResolvedMethodName stores class in `vmholder`; MemberName in `clazz`.
    let (class_value, method_name_str, descriptor_str) = {
        let member_ref = member_name_value.as_object_ref()?;
        let clazz = member_ref
            .value("vmholder")
            .or_else(|_| member_ref.value("clazz"))?;
        let name = member_ref.value("name").and_then(|v| v.as_string()).ok();
        let descriptor = member_ref.value("vmindex").and_then(|v| v.as_string()).ok();
        (clazz, name, descriptor)
    };

    // If we don't have a method name from the member, try reading it from
    // the StackFrameInfo itself (may have been set by callStackWalk).
    let method_name_str = if let Some(name) = method_name_str {
        name
    } else {
        let this_ref = this.as_object_ref()?;
        if let Ok(name) = this_ref.value("name").and_then(|v| v.as_string()) {
            name
        } else {
            return Ok(None);
        }
    };

    // Resolve the internal class name from the Class mirror object
    let class_name = {
        let class_ref = class_value.as_object_ref()?;
        class_ref.value("name")?.as_string()?
    };

    // Load the class to look up method descriptor information
    let declaring_class = thread.class(&class_name).await?;

    // Find the method by name (and descriptor if available) to resolve the descriptor
    let method = if let Some(ref desc) = descriptor_str {
        declaring_class.method(&method_name_str, desc)
    } else {
        declaring_class
            .methods()
            .into_iter()
            .find(|m| m.name() == method_name_str)
    };

    let descriptor_value = if let Some(method) = method {
        method.descriptor().to_object(&thread).await?
    } else if let Some(d) = descriptor_str {
        d.to_object(&thread).await?
    } else {
        Value::Object(None)
    };

    // Convert method name to a Java String object
    let method_name_obj = method_name_str.to_object(&thread).await?;

    let mut this_mut = this.as_object_mut()?;
    this_mut.set_value("name", method_name_obj)?;
    this_mut.set_value("type", descriptor_value)?;

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::{Object, Reference};
    use ristretto_types::VM;

    #[tokio::test]
    async fn test_expand_stack_frame_info_null_member_name() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Load the StackFrameInfo class; skip if not available in this JDK
        let Ok(sfi_class) = thread.class("java/lang/StackFrameInfo").await else {
            return Ok(());
        };
        let sfi = Object::new(sfi_class)?;
        let this = Value::new_object(thread.vm()?.garbage_collector(), Reference::Object(sfi));

        let parameters = Parameters::new(vec![this]);
        let result = expand_stack_frame_info(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_expand_stack_frame_info_with_member() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Load required classes; skip if not available
        let Ok(sfi_class) = thread.class("java/lang/StackFrameInfo").await else {
            return Ok(());
        };
        let Ok(rmn_class) = thread.class("java/lang/invoke/ResolvedMethodName").await else {
            return Ok(());
        };

        // Build a ResolvedMethodName pointing at java.lang.Object
        let object_class = thread.class("java/lang/Object").await?;
        let class_mirror = object_class.to_object(&thread).await?;

        let mut rmn = Object::new(rmn_class)?;
        rmn.set_value("vmholder", class_mirror)?;

        let rmn_value = Value::new_object(thread.vm()?.garbage_collector(), Reference::Object(rmn));

        // Create StackFrameInfo and set classOrMemberName, name, and bci
        let mut sfi = Object::new(sfi_class)?;
        let _ = sfi.set_value("classOrMemberName", rmn_value);
        let method_name = "<init>".to_object(&thread).await?;
        let _ = sfi.set_value("name", method_name);
        let _ = sfi.set_value("bci", Value::Int(0));

        let this = Value::new_object(thread.vm()?.garbage_collector(), Reference::Object(sfi));

        let parameters = Parameters::new(vec![this.clone()]);
        let result = expand_stack_frame_info(thread, parameters).await?;
        assert_eq!(None, result);

        // Verify the name field is still populated
        let this_ref = this.as_object_ref()?;
        if let Ok(name) = this_ref.value("name") {
            assert_eq!("<init>", name.as_string()?);
        }
        Ok(())
    }
}

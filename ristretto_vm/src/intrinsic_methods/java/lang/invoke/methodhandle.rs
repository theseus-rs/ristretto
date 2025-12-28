use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::invoke::methodhandlenatives::MemberNameFlags;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, ReferenceKind};
use ristretto_classloader::{Class, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

// Constants for method handle type detection
const DIRECT_METHOD_HANDLE: &str = "java/lang/invoke/DirectMethodHandle";
const BOUND_METHOD_HANDLE: &str = "java/lang/invoke/BoundMethodHandle";
const LAMBDA_FORM: &str = "java/lang/invoke/LambdaForm";

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
    let method_handle = parameters.pop()?;
    let result = invoke_method_handle_native(thread, method_handle, arguments).await?;
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
    let method_handle = parameters.pop()?;
    let result = invoke_method_handle_native(thread, method_handle, arguments).await?;
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
    let method_handle = parameters.pop()?;
    let result = invoke_method_handle_native(thread, method_handle, arguments).await?;
    Ok(Some(result))
}

/// Native method handle invocation similar to rusty-jvm's approach.
/// This handles different method handle types without triggering complex
/// LambdaForm initialization chains.
#[async_recursion(?Send)]
pub async fn invoke_method_handle_native(
    thread: Arc<Thread>,
    method_handle: Value,
    arguments: Vec<Value>,
) -> Result<Value> {
    let handle_class_name = {
        let handle_ref = method_handle.as_object_ref()?;
        handle_ref.class().name().to_string()
    };

    // Dispatch based on method handle type (similar to rusty-jvm)
    if handle_class_name.starts_with(DIRECT_METHOD_HANDLE) {
        // DirectMethodHandle - extract member and invoke directly
        let member = {
            let handle_ref = method_handle.as_object_ref()?;
            handle_ref.value("member")?
        };
        call_method_handle_target(thread, &member, arguments).await
    } else if handle_class_name.contains("AsVarargsCollector") {
        // AsVarargsCollector - collect trailing arguments into an array
        invoke_as_varargs_collector(thread, method_handle, arguments).await
    } else if handle_class_name.starts_with(BOUND_METHOD_HANDLE)
        || handle_class_name.contains("SimpleMethodHandle")
        || handle_class_name.contains("CountingWrapper")
    {
        // BoundMethodHandle and similar - get form.vmentry and invoke
        invoke_bound_method_handle(thread, method_handle, arguments).await
    } else if handle_class_name.contains("CallSite") {
        // CallSite - get target and invoke it
        invoke_call_site(thread, method_handle, arguments).await
    } else {
        // Fallback: try to extract member directly
        let member_result = {
            let handle_ref = method_handle.as_object_ref()?;
            handle_ref.value("member")
        };
        match member_result {
            Ok(member) => call_method_handle_target(thread, &member, arguments).await,
            Err(_) => {
                // Try form.vmentry path
                invoke_bound_method_handle(thread, method_handle, arguments).await
            }
        }
    }
}

use tracing::debug;

// ...

/// Invokes an AsVarargsCollector by collecting trailing arguments into an array.
#[async_recursion(?Send)]
async fn invoke_as_varargs_collector(
    thread: Arc<Thread>,
    method_handle: Value,
    mut arguments: Vec<Value>,
) -> Result<Value> {
    let (target, array_type) = {
        let handle_ref = method_handle.as_object_ref()?;
        (handle_ref.value("target")?, handle_ref.value("arrayType")?)
    };

    // Get the target's type to determine parameter count
    let param_count = {
        let target_ref = target.as_object_ref()?;
        let target_type = target_ref.value("type")?;
        let target_type_ref = target_type.as_object_ref()?;

        // MethodType.ptypes is the array of parameter types
        let ptypes = target_type_ref.value("ptypes")?;

        if let Value::Object(Some(ptypes_ref)) = ptypes {
            let guard = ptypes_ref.read();
            if let ristretto_classloader::Reference::Array(array) = &*guard {
                array.elements.len()
            } else {
                0
            }
        } else {
            0
        }
    };

    let arg_count = arguments.len();
    let fixed_param_count = if param_count > 0 { param_count - 1 } else { 0 };

    debug!(
        "invoke_as_varargs_collector: arg_count={}, param_count={}, fixed_param_count={}",
        arg_count, param_count, fixed_param_count
    );

    if arg_count >= fixed_param_count {
        // Check if the last argument is already the correct array type
        let mut needs_collection = true;
        if arg_count == param_count {
            let last_arg = &arguments[arg_count - 1];
            if !last_arg.is_null() {
                if let Value::Object(Some(ref_val)) = last_arg {
                    let is_array = {
                        let guard = ref_val.read();
                        matches!(
                            *guard,
                            ristretto_classloader::Reference::Array(_)
                                | ristretto_classloader::Reference::ByteArray(_)
                                | ristretto_classloader::Reference::CharArray(_)
                                | ristretto_classloader::Reference::ShortArray(_)
                                | ristretto_classloader::Reference::IntArray(_)
                                | ristretto_classloader::Reference::LongArray(_)
                                | ristretto_classloader::Reference::FloatArray(_)
                                | ristretto_classloader::Reference::DoubleArray(_)
                        )
                    };

                    if is_array {
                        needs_collection = false;
                    }
                } else {
                    // Null passed as array? Valid.
                    needs_collection = false;
                }
            } else {
                // Null passed as array
                needs_collection = false;
            }
        }

        if needs_collection {
            debug!(
                "invoke_as_varargs_collector: Collecting {} args into array",
                arg_count - fixed_param_count
            );

            // Collect trailing arguments
            let trailing_args: Vec<Value> = arguments.drain(fixed_param_count..).collect();

            // Create the array
            let array_class_name = {
                let array_type_ref = array_type.as_object_ref()?;
                array_type_ref.value("name")?.as_string()?
            };

            let array_class = thread.class(&array_class_name).await?;

            let array_ref =
                if array_class_name.starts_with("[L") || array_class_name.starts_with("[[") {
                    // Object array - box primitives if needed
                    let mut boxed_args = Vec::with_capacity(trailing_args.len());
                    for arg in trailing_args {
                        let boxed = if matches!(arg, Value::Object(_)) {
                            arg
                        } else {
                            // TODO: Use correct boxing method for each primitive type
                            // For now, to_object() handles basic boxing (Int -> Integer, etc.)
                            // But need to be careful with context.
                            // Ideally we should use the method handle type conversion logic.
                            // ristretto_vm::rust_value::RustValue::to_object might help?
                            // Value has to_object() which boxes.
                            arg.to_object(&thread).await?
                        };
                        boxed_args.push(boxed);
                    }
                    Value::try_from((array_class, boxed_args))?
                } else {
                    // Primitive array - assume values are already correct primitive values
                    // But Value::try_from for primitive arrays is not implemented in reference.rs
                    // Reference::from(Vec<i32>) exists.
                    // We need to convert Vec<Value> to Vec<i32> etc.
                    // For this specific bug (Records), it uses Object[], so we might be safe for now.
                    // If we encounter primitive array varargs, we'll need implementation.
                    // Fallback to try_from which might fail if not Object array
                    Value::try_from((array_class, trailing_args))?
                };

            arguments.push(array_ref);
        }
    }

    invoke_method_handle_native(thread, target, arguments).await
}

/// Invokes a LambdaForm by extracting the vmentry and invoking that.
#[async_recursion(?Send)]
async fn invoke_lambda_form(
    thread: Arc<Thread>,
    lambda_form: Value,
    arguments: Vec<Value>,
) -> Result<Value> {
    let form_ref = lambda_form.as_object_ref()?;
    let vmentry = form_ref.value("vmentry")?;

    if vmentry.is_null() {
        return Err(InternalError("LambdaForm.vmentry is null".to_string()));
    }

    call_method_handle_target(thread, &vmentry, arguments).await
}

/// Invokes a BoundMethodHandle by extracting the form.vmentry and invoking that.
/// Similar to rusty-jvm's bound_method_handle_invocation.
#[async_recursion(?Send)]
async fn invoke_bound_method_handle(
    thread: Arc<Thread>,
    method_handle: Value,
    mut arguments: Vec<Value>,
) -> Result<Value> {
    let handle_ref = method_handle.as_object_ref()?;

    // Get the LambdaForm from the "form" field
    let lambda_form = handle_ref.value("form")?;
    if lambda_form.is_null() {
        return Err(InternalError("BoundMethodHandle.form is null".to_string()));
    }

    // Get vmentry from LambdaForm - this is the actual target MemberName
    let vmentry = {
        let form_ref = lambda_form.as_object_ref()?;
        form_ref.value("vmentry")?
    };

    if vmentry.is_null() {
        // vmentry might be null during early initialization or for constant handles.
        // Try to return the bound value from various argument fields.
        // BoundMethodHandle species have fields like argL0, argL1, argI0, etc.
        for field_name in &["argL0", "argL1", "argI0", "argI1", "argJ0", "argD0", "argF0"] {
            if let Ok(arg) = handle_ref.value(*field_name) {
                if !arg.is_null() {
                    return Ok(arg);
                }
            }
        }
        // If no bound arguments found, return null (for void or identity handles)
        return Ok(Value::Object(None));
    }

    // Prepend the method handle itself as the first argument
    // (BoundMethodHandle methods expect the handle as first arg)
    let mut new_args = Vec::with_capacity(arguments.len() + 1);
    new_args.push(method_handle.clone());
    new_args.append(&mut arguments);

    // Invoke the vmentry target
    call_method_handle_target(thread, &vmentry, new_args).await
}

/// Invokes through a CallSite by getting its target MethodHandle.
#[async_recursion(?Send)]
async fn invoke_call_site(
    thread: Arc<Thread>,
    call_site: Value,
    arguments: Vec<Value>,
) -> Result<Value> {
    let call_site_ref = call_site.as_object_ref()?;
    let target = call_site_ref.value("target")?;

    if target.is_null() {
        return Err(InternalError("CallSite.target is null".to_string()));
    }

    invoke_method_handle_native(thread, target, arguments).await
}

/// Helper: Actually invokes the target referenced by a `MethodHandle`.
pub async fn call_method_handle_target(
    thread: Arc<Thread>,
    member: &Value,
    mut arguments: Vec<Value>,
) -> Result<Value> {
    let (member_class, member_type, member_name, member_descriptor, flags) = {
        let member = member.as_object_ref()?;
        let class = member.value("clazz")?;
        let member_type = member.value("type");
        let member_name = member.value("name")?.as_string()?;
        let member_descriptor = member.value("descriptor");
        let flags = member.value("flags")?.as_i32()?;
        (class, member_type, member_name, member_descriptor, flags)
    };
    let target_class_name = {
        let target_class_object = member_class.as_object_ref()?;
        target_class_object.value("name")?.as_string()?
    };
    let target_class = thread.class(target_class_name).await?;

    // Get flags to determine the kind of member and operation
    let reference_kind_value = (flags
        & (MemberNameFlags::REFERENCE_KIND_MASK.bits()
            << MemberNameFlags::REFERENCE_KIND_SHIFT.bits()))
        >> MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
    let reference_kind_value = u8::try_from(reference_kind_value)?;
    let reference_kind = ReferenceKind::try_from(reference_kind_value)?;

    // Get the descriptor (method signature or field type)
    let member_descriptor = if let Ok(method_type) = member_type {
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
    } else if let Ok(descriptor) = member_descriptor.and_then(|value| value.as_string()) {
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
            let mut receiver = argument.as_object_mut()?;
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
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_native(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_special(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_static(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_virtual(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;
    use ristretto_classfile::{
        ClassAccessFlags, ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType,
    };
    use ristretto_classloader::Object;

    /// Helper function to create a minimal `MemberName` object for testing
    async fn create_test_member_name(
        thread: &Thread,
        class_name: &str,
        member_name: &str,
        descriptor: &str,
        reference_kind: ReferenceKind,
    ) -> Result<Value> {
        let member_class = thread.class("java/lang/invoke/MemberName").await?;
        let member = Value::from(Object::new(member_class)?);

        // Set up the MemberName fields
        let class_name_string = class_name.replace('/', ".");
        let class_object = thread
            .invoke(
                "java/lang/Class",
                "forName(Ljava/lang/String;)Ljava/lang/Class;",
                &[class_name_string.to_object(thread).await?],
            )
            .await?
            .unwrap_or(Value::Object(None));

        // Create a MethodType from the descriptor
        let method_type = thread
            .invoke(
                "java/lang/invoke/MethodType",
                "fromMethodDescriptorString(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType;",
                &[descriptor.to_object(thread).await?, Value::Object(None)],
            )
            .await?
            .unwrap_or(Value::Object(None));

        {
            let mut object = member.as_object_mut()?;
            object.set_value("clazz", class_object)?;
            object.set_value("name", member_name.to_object(thread).await?)?;
            object.set_value("type", method_type)?;

            // Calculate flags with reference kind and IS_METHOD flag
            let mut flags =
                i32::from(reference_kind.kind()) << MemberNameFlags::REFERENCE_KIND_SHIFT.bits();

            // Set IS_METHOD flag for method invocations
            if matches!(
                reference_kind,
                ReferenceKind::InvokeVirtual
                    | ReferenceKind::InvokeStatic
                    | ReferenceKind::InvokeSpecial
                    | ReferenceKind::InvokeInterface
            ) {
                flags |= MemberNameFlags::IS_METHOD.bits();
            }
            object.set_value("flags", Value::Int(flags))?;
        }

        Ok(member)
    }

    /// Helper function to register a mock `MethodHandle` class with a "member" field
    async fn register_method_handle_class(thread: &Thread) -> Result<Arc<Class>> {
        let class_name = "java/lang/invoke/MethodHandle";
        let mut constant_pool = ConstantPool::new();
        let this_class = constant_pool.add_class(class_name)?;
        let super_class = constant_pool.add_class("java/lang/Object")?;
        let member_name_index = constant_pool.add_utf8("member")?;
        let member_descriptor_index = constant_pool.add_utf8("Ljava/lang/invoke/MemberName;")?;

        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: member_name_index,
            descriptor_index: member_descriptor_index,
            field_type: FieldType::Object("java/lang/invoke/MemberName".to_string()),
            attributes: vec![],
        };

        let class_file = ClassFile {
            version: JAVA_17,
            constant_pool,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
            this_class,
            super_class,
            fields: vec![field],
            ..Default::default()
        };

        let vm = thread.vm()?;
        let class_loader = vm.class_loader().read().await.clone();
        let class = Class::from(Some(Arc::downgrade(&class_loader)), class_file)?;
        class_loader.register(class.clone()).await?;
        Ok(class)
    }

    #[tokio::test]
    async fn test_invoke() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let member = create_test_member_name(
            &thread,
            "java/lang/Integer",
            "valueOf",
            "(Ljava/lang/String;)Ljava/lang/Integer;",
            ReferenceKind::InvokeStatic,
        )
        .await?;

        let mh_class = register_method_handle_class(&thread).await?;
        let method_handle = Value::from(Object::new(mh_class)?);
        method_handle.as_object_mut()?.set_value("member", member)?;

        let args_class = thread.class("[Ljava/lang/Object;").await?;
        let arg = "42".to_object(&thread).await?;
        let args = vec![arg];
        let arguments = Value::try_from((args_class, args))?;

        let mut parameters = Parameters::default();
        parameters.push(method_handle);
        parameters.push(arguments);

        let result = invoke(thread, parameters).await;
        assert!(result.is_ok(), "invoke failed: {result:?}");
        Ok(())
    }

    #[tokio::test]
    async fn test_invoke_basic() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let member = create_test_member_name(
            &thread,
            "java/lang/Integer",
            "valueOf",
            "(Ljava/lang/String;)Ljava/lang/Integer;",
            ReferenceKind::InvokeStatic,
        )
        .await?;

        let mh_class = register_method_handle_class(&thread).await?;
        let method_handle = Value::from(Object::new(mh_class)?);
        method_handle.as_object_mut()?.set_value("member", member)?;

        let args_class = thread.class("[Ljava/lang/Object;").await?;
        let arg = "42".to_object(&thread).await?;
        let args = vec![arg];
        let arguments = Value::try_from((args_class, args))?;

        let mut parameters = Parameters::default();
        parameters.push(method_handle);
        parameters.push(arguments);

        let result = invoke_basic(thread, parameters).await;
        assert!(result.is_ok(), "invoke_basic failed: {result:?}");
        Ok(())
    }

    #[tokio::test]
    async fn test_invoke_exact() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let member = create_test_member_name(
            &thread,
            "java/lang/String",
            "valueOf",
            "(Ljava/lang/Object;)Ljava/lang/String;",
            ReferenceKind::InvokeStatic,
        )
        .await?;

        let mh_class = register_method_handle_class(&thread).await?;
        let method_handle = Value::from(Object::new(mh_class)?);
        method_handle.as_object_mut()?.set_value("member", member)?;

        let args_class = thread.class("[Ljava/lang/Object;").await?;
        let int_object = thread
            .object("java/lang/Integer", "I", &[Value::Int(42)])
            .await?;
        let args = vec![int_object];
        let arguments = Value::try_from((args_class, args))?;

        let mut parameters = Parameters::default();
        parameters.push(method_handle);
        parameters.push(arguments);

        let result = invoke_exact(thread, parameters).await;
        assert!(result.is_ok(), "invoke_exact failed: {result:?}");
        Ok(())
    }

    #[tokio::test]
    async fn test_link_to_static() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let member = create_test_member_name(
            &thread,
            "java/lang/Integer",
            "valueOf",
            "(I)Ljava/lang/Integer;",
            ReferenceKind::InvokeStatic,
        )
        .await?;

        let mut parameters = Parameters::default();
        parameters.push(Value::Int(42));
        parameters.push(member);

        let result = link_to_static(thread, parameters).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_link_to_virtual() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let string_object = "test".to_object(&thread).await?;

        let member = create_test_member_name(
            &thread,
            "java/lang/String",
            "toString",
            "()Ljava/lang/String;",
            ReferenceKind::InvokeVirtual,
        )
        .await?;

        let mut parameters = Parameters::default();
        parameters.push(string_object);
        parameters.push(member);

        let result = link_to_virtual(thread, parameters).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_link_to_interface() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let empty_values: &[Value] = &[];
        let list_object = thread
            .object("java/util/ArrayList", "", empty_values)
            .await?;

        let member = create_test_member_name(
            &thread,
            "java/util/ArrayList",
            "size",
            "()I",
            ReferenceKind::InvokeVirtual,
        )
        .await?;

        let mut parameters = Parameters::default();
        parameters.push(list_object);
        parameters.push(member);

        let result = link_to_interface(thread, parameters).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_link_to_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let member = create_test_member_name(
            &thread,
            "java/lang/Integer",
            "valueOf",
            "(I)Ljava/lang/Integer;",
            ReferenceKind::InvokeStatic,
        )
        .await?;

        let mut parameters = Parameters::default();
        parameters.push(Value::Int(42));
        parameters.push(member);

        let result = link_to_native(thread, parameters).await;
        assert!(result.is_ok());
        Ok(())
    }
}

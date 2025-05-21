use crate::Error::InternalError;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{JavaObject, Result, VM};
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{Constant, ConstantPool, FieldType, MethodAccessFlags};
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::sync::Arc;
use tracing::debug;

/// Get the bootstrap method attribute name and type for the specified method index.
///
/// # Errors
/// - if the method index is invalid
/// - if the constant for the index is an invalid type
fn get_bootstrap_method_attribute_name_and_type(
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<(u16, u16)> {
    let (bootstrap_method_attr_index, name_and_type_index) =
        match constant_pool.try_get(method_index)? {
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Field descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2
                (*bootstrap_method_attr_index, *name_and_type_index)
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Method descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.3
                (*bootstrap_method_attr_index, *name_and_type_index)
            } // Method
            _ => {
                return Err(InternalError(format!(
                    "Invalid constant pool index for invokedynamic: {method_index}"
                )));
            }
        };
    Ok((bootstrap_method_attr_index, name_and_type_index))
}

/// Get the bootstrap method definition for the specified class.
///
/// # Errors
/// if the bootstrap method cannot be found.
fn get_bootstrap_method_attribute(
    frame: &Frame,
    bootstrap_method_attr_index: u16,
) -> Result<BootstrapMethod> {
    let class = frame.class();
    class
        .class_file()
        .attributes
        .iter()
        .find_map(|attribute| {
            if let Attribute::BootstrapMethods { methods, .. } = attribute {
                methods.get(bootstrap_method_attr_index as usize).cloned()
            } else {
                None
            }
        })
        .ok_or_else(|| {
            InternalError(format!(
                "No bootstrap methods found for class {} at index {}",
                class.name(),
                bootstrap_method_attr_index
            ))
        })
}

/// Get the class object for the specified field type.
/// If the field type is `None` then the class object for `void` is returned.
///
/// # Errors
/// if the class object cannot be found.
async fn get_field_type_class(vm: &VM, field_type: Option<FieldType>) -> Result<Value> {
    let class_name = if let Some(field_type) = field_type {
        field_type.class_name()
    } else {
        "void".to_string()
    };
    let class = vm.class(class_name).await?;
    class.to_object(vm).await
}

/// Get the method type for the specified method descriptor.
///
/// # Errors
/// if the method type cannot be found.
async fn get_method_type(vm: &VM, thread: &Thread, method_descriptor: &str) -> Result<Value> {
    let (argument_types, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
    let return_class = get_field_type_class(vm, return_type).await?;

    let first_argument = get_field_type_class(vm, argument_types.first().cloned()).await?;
    let argument_classes = ConcurrentVec::from(Vec::with_capacity(argument_types.len() - 1));
    for argument_type in argument_types.iter().skip(1) {
        let argument_class = get_field_type_class(vm, Some(argument_type.clone())).await?;
        let argument_reference: Reference = argument_class.try_into()?;
        argument_classes.push(Some(argument_reference))?;
    }
    let class_array = vm.class("[Ljava/lang/Class;").await?;
    let arguments = Value::from(Reference::Array(class_array, argument_classes));

    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    let method = method_type_class.try_get_method(
        "methodType",
        "(Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
    )?;
    let arguments = vec![return_class, first_argument, arguments];
    thread
        .try_execute(&method_type_class, &method, arguments)
        .await
}

/// Get the caller method handle lookup for the specified frame.
///
/// # Errors
/// if the lookup cannot be found
async fn get_caller_method_handle_lookup(vm: &VM, thread: &Thread, frame: &Frame) -> Result<Value> {
    // Get the MethodHandles.Lookup object
    let method_handles_class = thread.class("java.lang.invoke.MethodHandles").await?;
    let lookup_method = method_handles_class
        .try_get_method("lookup", "()Ljava/lang/invoke/MethodHandles$Lookup;")?;
    let lookup = thread
        .try_execute(&method_handles_class, &lookup_method, Vec::<Value>::new())
        .await?;

    // Get the caller MethodHandle
    let class = frame.class();
    let class_object = class.to_object(vm).await?;
    let method = frame.method();
    let method_name = method.name();
    let method_name_object = method_name.to_object(vm).await?;
    let method_descriptor = method.descriptor();
    let method_type = get_method_type(vm, thread, method_descriptor).await?;
    let mut arguments = vec![method_name_object, method_type];

    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;
    let _lookup_method = if method_name == "<init>" {
        lookup_class.try_get_method(
            "findConstructor",
            "(Ljava/lang/Class;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    } else if method.access_flags().contains(MethodAccessFlags::STATIC) {
        arguments.insert(0, class_object);
        lookup_class.try_get_method(
            "findStatic",
            "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    } else {
        arguments.insert(0, class_object);
        lookup_class.try_get_method(
            "findVirtual",
            "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    };

    arguments.insert(0, lookup);
    // TODO: fix stack overflow error when calling MethodHandles$Lookup.findStatic
    //       "fatal runtime error: stack overflow"
    // let method_handle = thread
    //     .try_execute(&lookup_class, &lookup_method, arguments)
    //     .await?;
    let method_handle = Value::Object(None);
    Ok(method_handle)
}

/// Append the static arguments from the bootstrap method to the arguments vector.
async fn append_bootstrap_arguments(
    vm: &Arc<VM>,
    constant_pool: &ConstantPool,
    bootstrap_method: &BootstrapMethod,
    arguments: &mut Vec<Value>,
) -> Result<()> {
    for argument in &bootstrap_method.arguments {
        let constant = constant_pool.try_get(*argument)?;
        match constant {
            Constant::Integer(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Float(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Long(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Double(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::String(value) => {
                let string = constant_pool.try_get_utf8(*value)?;
                let value = string.to_object(vm).await?;
                arguments.push(value);
            }
            _ => {
                return Err(InternalError(format!(
                    "Invalid bootstrap argument type: {argument}"
                )));
            }
        }
    }
    Ok(())
}

/// Resolves the invokedynamic call site from the frame and constant pool.
pub async fn get_call_site(frame: &Frame, method_index: u16) -> Result<Value> {
    let thread = frame.thread()?;
    let current_class = frame.class();
    let constant_pool = current_class.constant_pool();

    // Get the bootstrap method attribute name and type
    let (bootstrap_method_attr_index, name_and_type_index) =
        get_bootstrap_method_attribute_name_and_type(constant_pool, method_index)?;
    let (bootstrap_method_attr_name_index, bootstrap_method_attr_descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let bootstrap_method_attr_name =
        constant_pool.try_get_utf8(*bootstrap_method_attr_name_index)?;
    let _bootstrap_method_attr_descriptor =
        constant_pool.try_get_utf8(*bootstrap_method_attr_descriptor_index)?;

    // Get the bootstrap method
    let bootstrap_method_attribute =
        get_bootstrap_method_attribute(frame, bootstrap_method_attr_index)?;
    let (reference_kind, method_ref) =
        constant_pool.try_get_method_handle(bootstrap_method_attribute.bootstrap_method_ref)?;
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(*method_ref)?;
    let bootstrap_class_name = constant_pool.try_get_class(*class_index)?;
    let bootstrap_class = thread.class(bootstrap_class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let bootstrap_method_name = constant_pool.try_get_utf8(*name_index)?;
    let bootstrap_method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let bootstrap_method =
        bootstrap_class.try_get_method(bootstrap_method_name, bootstrap_method_descriptor)?;

    debug!(
        "invokedynamic: bootstrap method({reference_kind}): {}.{}{}",
        bootstrap_class.name(),
        bootstrap_method.name(),
        bootstrap_method.descriptor()
    );

    // Invoke the bootstrap method
    let vm = thread.vm()?;
    let method_handle = get_caller_method_handle_lookup(&vm, &thread, frame).await?;
    let method_name = bootstrap_method_attr_name.to_object(&vm).await?;
    let method_type = get_method_type(&vm, &thread, bootstrap_method_descriptor).await?;
    let mut arguments = vec![method_handle, method_name, method_type];
    append_bootstrap_arguments(
        &vm,
        constant_pool,
        &bootstrap_method_attribute,
        &mut arguments,
    )
    .await?;

    todo!("invokedynamic get_call_site")
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic>
#[inline]
pub(crate) async fn invokedynamic(
    frame: &Frame,
    _stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let _call_site = get_call_site(frame, method_index).await?;
    todo!("invokedynamic")
}

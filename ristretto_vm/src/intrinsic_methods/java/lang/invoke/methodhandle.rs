use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::JavaObject;
use crate::Result;
use crate::intrinsic_methods::java::lang::invoke::methodhandlenatives::MemberNameFlags;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, ReferenceKind};
use ristretto_classloader::{Class, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
use tracing::debug;

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub(crate) async fn invoke(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    // For signature polymorphic methods, parameters are passed individually, not as an array
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(crate::Error::InternalError(
            "invoke requires at least the method handle".to_string(),
        ));
    }

    let method_handle = all_params[0].clone();
    let arguments: Vec<Value> = all_params[1..].to_vec();

    // Try to get the LambdaForm's vmentry first
    let (form, member) = {
        let mh_ref = method_handle.as_object_ref()?;
        let form = mh_ref.value("form").ok();
        let member = mh_ref.value("member").ok();
        (form, member)
    };

    // Check if we have a LambdaForm with a vmentry
    if let Some(ref form_val) = form
        && !form_val.is_null()
    {
        let vmentry = {
            let form_ref = form_val.as_object_ref()?;
            form_ref.value("vmentry").ok()
        };

        if let Some(ref vmentry_val) = vmentry
            && !vmentry_val.is_null()
        {
            let mut lf_args = Vec::with_capacity(arguments.len() + 1);
            lf_args.push(method_handle.clone());
            lf_args.extend(arguments);

            return call_method_handle_target(thread, vmentry_val, lf_args)
                .await
                .map(Some);
        }
    }

    // Fallback: use the member field directly
    if let Some(ref member_val) = member
        && !member_val.is_null()
    {
        let result = call_method_handle_target(thread, member_val, arguments).await?;
        return Ok(Some(result));
    }

    Err(crate::Error::InternalError(
        "MethodHandle has neither vmentry nor member".to_string(),
    ))
}

/// Interpret a `LambdaForm` by executing its names array.
/// This is an alternative to dispatching through Holder methods, which avoids the argument layout
/// issues that occur with compiled `LambdaForm`s.
#[async_method]
async fn interpret_lambda_form(
    thread: Arc<Thread>,
    form: &Value,
    input_args: Vec<Value>,
) -> Result<Value> {
    let (arity, result_index, names_elements) = {
        let form_ref = form.as_object_ref()?;
        let arity = usize::try_from(form_ref.value("arity")?.as_i32()?)?;
        let result_index = form_ref.value("result")?.as_i32()?;
        let names = form_ref.value("names")?;
        let names_elements = get_object_array_elements(&names)?;
        (arity, result_index, names_elements)
    };

    // Create the values array initialized with input arguments
    let mut values: Vec<Value> = Vec::with_capacity(names_elements.len());

    // First `arity` entries are the input arguments
    for (i, arg) in input_args.into_iter().enumerate() {
        if i >= arity {
            break;
        }
        values.push(arg);
    }

    // Pad with nulls if we don't have enough input arguments
    while values.len() < arity {
        values.push(Value::Object(None));
    }

    // Execute each Name beyond the input parameters
    for name in names_elements.iter().skip(arity) {
        if name.is_null() {
            values.push(Value::Object(None));
            continue;
        }

        let result = interpret_name(thread.clone(), name, &values).await?;
        values.push(result);
    }

    // Return the result
    if result_index < 0 {
        Ok(Value::Object(None))
    } else {
        let result_idx = usize::try_from(result_index)?;
        if result_idx < values.len() {
            Ok(values[result_idx].clone())
        } else {
            Err(InternalError(format!(
                "LambdaForm result index {} out of bounds (values len={})",
                result_index,
                values.len()
            )))
        }
    }
}

/// Interpret a single `Name` from a `LambdaForm`'s names array.
#[async_method]
async fn interpret_name(thread: Arc<Thread>, name: &Value, values: &[Value]) -> Result<Value> {
    let (args_elements, function, type_val_opt) = {
        let name_ref = name.as_object_ref()?;
        let args = name_ref.value("arguments")?;
        let args_elements = get_object_array_elements(&args)?;
        let function = name_ref.value("function")?;
        let type_val_opt = name_ref.value("type").ok();
        (args_elements, function, type_val_opt)
    };

    // Resolve each argument - if it's a Name, look it up in values
    let mut resolved_args: Vec<Value> = Vec::with_capacity(args_elements.len());
    for arg_val in args_elements {
        let resolved = resolve_lambda_form_argument(&arg_val, values)?;
        resolved_args.push(resolved);
    }

    // Extract function information
    let (intrinsic_ordinal, member) = {
        let function_ref = function.as_object_ref()?;

        // Check for intrinsic operations first
        let intrinsic_ordinal = if let Ok(intrinsic_name_val) = function_ref.value("intrinsicName")
            && !intrinsic_name_val.is_null()
        {
            intrinsic_name_val
                .as_object_ref()
                .and_then(|obj| obj.value("ordinal"))
                .and_then(|v| v.as_i32())
                .ok()
        } else {
            None
        };

        let member = function_ref.value("member")?;
        (intrinsic_ordinal, member)
    };

    // Handle intrinsic operations
    if let Some(ordinal) = intrinsic_ordinal {
        match ordinal {
            1 => {
                // IDENTITY - just return the first argument
                if let Some(arg) = resolved_args.first() {
                    return Ok(arg.clone());
                }
                return Ok(Value::Object(None));
            }
            2 => {
                // ZERO - return zero/null value
                if let Some(ref type_val) = type_val_opt
                    && !type_val.is_null()
                {
                    return Ok(get_zero_value(type_val));
                }
                return Ok(Value::Object(None));
            }
            // NONE (0) and other intrinsics - fall through to regular dispatch
            _ => {}
        }
    }

    // If member is null, this is an error - we can't invoke without a target
    if member.is_null() {
        return Err(InternalError(
            "NamedFunction.member is null - cannot invoke".to_string(),
        ));
    }

    // Call the method with resolved arguments
    let result = call_method_handle_target(thread.clone(), &member, resolved_args).await?;

    // Check if we need to box the result based on the Name's expected type
    if let Some(ref type_val) = type_val_opt {
        let expected_object = is_object_type(type_val);
        if expected_object && is_primitive(&result) {
            // Box the primitive result
            return box_primitive(&thread, result).await;
        }
    }

    Ok(result)
}

/// Get elements from an object array Value
fn get_object_array_elements(value: &Value) -> Result<Vec<Value>> {
    let ref_guard = value.as_reference()?;
    let (_, elements) = ref_guard.as_class_vec_ref()?;
    Ok(elements.to_vec())
}

/// Resolve a `LambdaForm` argument - if it's a `Name`, look up its value
fn resolve_lambda_form_argument(arg_val: &Value, values: &[Value]) -> Result<Value> {
    if arg_val.is_null() {
        return Ok(arg_val.clone());
    }

    // Check if this is a Name (index reference to earlier value)
    let class_name = get_object_class_name(arg_val);

    if let Some(name) = class_name {
        if name == "java/lang/invoke/LambdaForm$Name" {
            // It's a Name - get its index and look up the value
            let arg_ref = arg_val.as_object_ref()?;
            let index = usize::try_from(arg_ref.value("index")?.as_i32()?)?;
            if index < values.len() {
                return Ok(values[index].clone());
            }
            return Err(InternalError(format!(
                "Name index {index} out of bounds (values len={})",
                values.len()
            )));
        }
        // It's a constant value - unbox if needed
        return unbox_if_wrapper(arg_val);
    }

    Ok(arg_val.clone())
}

/// Get the class name of an object Value, or None if not an object
fn get_object_class_name(value: &Value) -> Option<String> {
    value
        .as_object_ref()
        .ok()
        .map(|obj| obj.class().name().to_string())
}

/// Unbox a primitive wrapper to its value
fn unbox_if_wrapper(value: &Value) -> Result<Value> {
    let Ok(obj_ref) = value.as_object_ref() else {
        return Ok(value.clone());
    };

    let class_name = obj_ref.class().name();

    match class_name {
        "java/lang/Integer"
        | "java/lang/Boolean"
        | "java/lang/Byte"
        | "java/lang/Short"
        | "java/lang/Character" => {
            let v = obj_ref.value("value")?.as_i32()?;
            Ok(Value::Int(v))
        }
        "java/lang/Long" => {
            let v = obj_ref.value("value")?.as_i64()?;
            Ok(Value::Long(v))
        }
        "java/lang/Float" => {
            let v = obj_ref.value("value")?.as_f32()?;
            Ok(Value::Float(v))
        }
        "java/lang/Double" => {
            let v = obj_ref.value("value")?.as_f64()?;
            Ok(Value::Double(v))
        }
        _ => Ok(value.clone()),
    }
}

/// Get a zero value for the given type.
/// Used by the `ZERO` intrinsic in `LambdaForm` interpretation.
fn get_zero_value(type_val: &Value) -> Value {
    // The type field in Name is a BasicType byte
    // BasicType: L=0 (object), I=1 (int), J=2 (long), F=3 (float), D=4 (double), V=5 (void)
    if let Ok(basic_type) = type_val.as_i32() {
        return match basic_type {
            1 => Value::Int(0),      // I_TYPE = int
            2 => Value::Long(0),     // J_TYPE = long
            3 => Value::Float(0.0),  // F_TYPE = float
            4 => Value::Double(0.0), // D_TYPE = double
            // L_TYPE (0), V_TYPE (5), and others default to null
            _ => Value::Object(None),
        };
    }

    // Try to get BasicType from its ordinal if it's an enum
    if let Ok(obj_ref) = type_val.as_object_ref()
        && obj_ref.class().name() == "java/lang/invoke/LambdaForm$BasicType"
        && let Ok(ordinal) = obj_ref.value("ordinal").and_then(|v| v.as_i32())
    {
        return match ordinal {
            1 => Value::Int(0),      // I_TYPE
            2 => Value::Long(0),     // J_TYPE
            3 => Value::Float(0.0),  // F_TYPE
            4 => Value::Double(0.0), // D_TYPE
            // L_TYPE (0), V_TYPE (5), and others default to null
            _ => Value::Object(None),
        };
    }

    // Default to null object
    Value::Object(None)
}

/// Check if a `BasicType` value indicates an object type (`L_TYPE` = 0).
fn is_object_type(type_val: &Value) -> bool {
    // BasicType: L_TYPE=0 (object), I_TYPE=1 (int), J_TYPE=2 (long), etc.
    if let Ok(basic_type) = type_val.as_i32() {
        return basic_type == 0; // L_TYPE
    }

    // Try to get BasicType from its ordinal if it's an enum
    if let Ok(obj_ref) = type_val.as_object_ref()
        && obj_ref.class().name() == "java/lang/invoke/LambdaForm$BasicType"
        && let Ok(ordinal) = obj_ref.value("ordinal").and_then(|v| v.as_i32())
    {
        return ordinal == 0; // L_TYPE
    }

    false
}

/// Check if a Value is a primitive type.
fn is_primitive(value: &Value) -> bool {
    matches!(
        value,
        Value::Int(_) | Value::Long(_) | Value::Float(_) | Value::Double(_)
    )
}

/// Box a primitive value into its wrapper object.
async fn box_primitive(thread: &Thread, value: Value) -> Result<Value> {
    match value {
        Value::Int(v) => v.to_object(thread).await,
        Value::Long(v) => v.to_object(thread).await,
        Value::Float(v) => v.to_object(thread).await,
        Value::Double(v) => v.to_object(thread).await,
        other => Ok(other), // Not a primitive, return as-is
    }
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub(crate) async fn invoke_basic(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, parameters are passed individually, not as an array
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(crate::Error::InternalError(
            "invokeBasic requires at least the method handle".to_string(),
        ));
    }

    let method_handle = all_params[0].clone();
    let arguments: Vec<Value> = all_params[1..].to_vec();

    // Try to get the LambdaForm's vmentry first
    // This is the proper way to invoke a MethodHandle via its compiled LambdaForm
    let (form, member) = {
        let mh_ref = method_handle.as_object_ref()?;
        let form = mh_ref.value("form").ok();
        let member = mh_ref.value("member").ok();
        (form, member)
    };

    // Check if we have a LambdaForm with a vmentry
    if let Some(ref form_val) = form
        && !form_val.is_null()
    {
        let vmentry = {
            let form_ref = form_val.as_object_ref()?;
            form_ref.value("vmentry").ok()
        };

        if let Some(ref vmentry_val) = vmentry
            && !vmentry_val.is_null()
        {
            // The vmentry is a MemberName pointing to the compiled LambdaForm method
            // The LambdaForm method signature is (MethodHandle, args...) -> result
            // We need to prepend the MethodHandle to the arguments
            let mut lf_args = Vec::with_capacity(arguments.len() + 1);
            lf_args.push(method_handle.clone());
            lf_args.extend(arguments);

            return call_method_handle_target(thread, vmentry_val, lf_args)
                .await
                .map(Some);
        }
    }

    // Fallback: use the member field directly (for DirectMethodHandle)
    if let Some(ref member_val) = member
        && !member_val.is_null()
    {
        let result = call_method_handle_target(thread, member_val, arguments).await?;
        return Ok(Some(result));
    }

    Err(crate::Error::InternalError(
        "MethodHandle has neither vmentry nor member".to_string(),
    ))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub(crate) async fn invoke_exact(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods like invokeExact, the actual call site signature
    // determines the parameters. The declared signature ([Ljava/lang/Object;)Ljava/lang/Object;
    // is a placeholder. The actual arguments are passed as individual values, not as an array.
    //
    // When called from invokevirtual with the call site descriptor (String,String)String,
    // parameters will be: [method_handle, "Hello", " World"]
    //
    // We need to extract the method_handle first, then treat remaining parameters as arguments.
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(crate::Error::InternalError(
            "invokeExact requires at least the method handle".to_string(),
        ));
    }

    let method_handle = all_params[0].clone();
    let arguments: Vec<Value> = all_params[1..].to_vec();

    // Try to get the LambdaForm's vmentry first
    let (form, member) = {
        let mh_ref = method_handle.as_object_ref()?;
        let form = mh_ref.value("form").ok();
        let member = mh_ref.value("member").ok();
        (form, member)
    };

    // Check if we have a LambdaForm with a vmentry
    if let Some(ref form_val) = form
        && !form_val.is_null()
    {
        let vmentry = {
            let form_ref = form_val.as_object_ref()?;
            form_ref.value("vmentry").ok()
        };

        if let Some(ref vmentry_val) = vmentry
            && !vmentry_val.is_null()
        {
            let mut lf_args = Vec::with_capacity(arguments.len() + 1);
            lf_args.push(method_handle.clone());
            lf_args.extend(arguments);

            return call_method_handle_target(thread, vmentry_val, lf_args)
                .await
                .map(Some);
        }
    }

    // Fallback: use the member field directly
    if let Some(ref member_val) = member
        && !member_val.is_null()
    {
        let result = call_method_handle_target(thread, member_val, arguments).await?;
        return Ok(Some(result));
    }

    Err(crate::Error::InternalError(
        "MethodHandle has neither vmentry nor member".to_string(),
    ))
}

/// Finds a method in the class hierarchy, starting with the target class and falling back
/// to the receiver's class hierarchy if not found.
///
/// This is needed for lambda methods where MemberName.clazz may be Object but the actual
/// lambda method is defined in an interface (like Function.andThen).
#[async_method]
async fn find_method_in_hierarchy(
    _thread: &Arc<Thread>,
    target_class: &Arc<Class>,
    receiver: &Value,
    method_name: &str,
    method_descriptor: &str,
) -> Result<(Arc<Class>, Arc<ristretto_classloader::Method>)> {
    // For virtual method dispatch, we MUST start from the receiver's actual class to properly
    // implement polymorphism (overridden methods)
    if let Ok(receiver_obj) = receiver.as_object_ref() {
        let receiver_class = receiver_obj.class().clone();

        // Search the receiver's class hierarchy for the method implementation
        if let Some((class, method)) =
            search_class_hierarchy_for_method(&receiver_class, method_name, method_descriptor)?
        {
            // Make sure we found a non-abstract implementation
            if !method
                .access_flags()
                .contains(ristretto_classfile::MethodAccessFlags::ABSTRACT)
            {
                return Ok((class, method));
            }
        }
    }

    // Fallback: If not found on receiver's class hierarchy, try the target class. This handles
    // cases where the method might be on an interface or abstract class
    if let Ok(method) = target_class.try_get_method(method_name, method_descriptor) {
        // If the method is not abstract, we can use it
        if !method
            .access_flags()
            .contains(ristretto_classfile::MethodAccessFlags::ABSTRACT)
        {
            return Ok((target_class.clone(), method));
        }
    }

    // If the method is a lambda method and target is Object, search receiver's class hierarchy
    if method_name.starts_with("lambda$")
        && target_class.name() == "java/lang/Object"
        && let Ok(receiver_obj) = receiver.as_object_ref()
    {
        let receiver_class = receiver_obj.class().clone();

        // Search the receiver's class hierarchy (including interfaces)
        if let Some((class, method)) =
            search_class_hierarchy_for_method(&receiver_class, method_name, method_descriptor)?
        {
            return Ok((class, method));
        }
    }

    // Include receiver info in error for debugging
    let receiver_info = if let Ok(obj) = receiver.as_object_ref() {
        format!("receiver class: {}", obj.class().name())
    } else {
        format!("receiver: {receiver:?}")
    };
    Err(InternalError(format!(
        "Method not found: {}.{method_name}{method_descriptor} ({receiver_info})",
        target_class.name()
    )))
}

/// Searches for a method in a class's hierarchy, including interfaces.
fn search_class_hierarchy_for_method(
    class: &Arc<Class>,
    method_name: &str,
    method_descriptor: &str,
) -> Result<Option<(Arc<Class>, Arc<ristretto_classloader::Method>)>> {
    // Check the class itself
    if let Ok(method) = class.try_get_method(method_name, method_descriptor) {
        return Ok(Some((class.clone(), method)));
    }

    // Check interfaces
    if let Ok(interfaces) = class.interfaces() {
        for interface in interfaces {
            if let Ok(method) = interface.try_get_method(method_name, method_descriptor) {
                return Ok(Some((interface.clone(), method)));
            }
            // Recursively check super-interfaces
            if let Some(result) =
                search_class_hierarchy_for_method(&interface, method_name, method_descriptor)?
            {
                return Ok(Some(result));
            }
        }
    }

    // Check parent class
    if let Ok(Some(parent)) = class.parent()
        && let Some(result) =
            search_class_hierarchy_for_method(&parent, method_name, method_descriptor)?
    {
        return Ok(Some(result));
    }

    Ok(None)
}

/// Finds a static lambda method by searching the class hierarchy of the first argument.
///
/// This is used when a static lambda method (like those in interface default methods)
/// cannot be found on the specified target class.
#[async_method]
async fn find_static_lambda_method(
    _thread: &Arc<Thread>,
    arguments: &[Value],
    method_name: &str,
    method_descriptor: &str,
) -> Option<(Arc<Class>, Arc<ristretto_classloader::Method>)> {
    // For static lambda methods, the first argument is often the captured 'this'
    // which is the interface instance. Search its class hierarchy.
    for arg in arguments {
        if let Ok(arg_obj) = arg.as_object_ref() {
            let arg_class = arg_obj.class().clone();
            if let Ok(Some(result)) =
                search_class_hierarchy_for_method(&arg_class, method_name, method_descriptor)
            {
                return Some(result);
            }
        }
    }
    None
}

/// Helper: Actually invokes the target referenced by a `MethodHandle`.
#[expect(clippy::too_many_lines)]
#[async_method]
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

    // Handle Holder classes and LambdaForm classes specially - these are synthetic entry points
    // that may have dynamically generated methods not present in the pre-compiled class files.
    // We use the LambdaForm interpreter to dispatch these methods.
    if is_holder_class(&target_class_name) {
        debug!(
            "call_method_handle_target: dispatching to holder method {target_class_name}::{member_name}"
        );
        return dispatch_holder_method(thread, &member_name, arguments).await;
    }
    debug!("call_method_handle_target: NOT a holder class: {target_class_name}");

    let target_class = match thread.class(&target_class_name).await {
        Ok(c) => c,
        Err(e) => {
            debug!(
                "call_method_handle_target: Failed to load class '{}': {:?}",
                target_class_name, e
            );
            return Err(e);
        }
    };

    debug!(
        "call_method_handle_target: target_class={}, member_name={}, args={}",
        target_class_name,
        member_name,
        arguments.len()
    );

    // Get flags to determine the kind of member and operation
    let reference_kind_value = (flags
        & (MemberNameFlags::REFERENCE_KIND_MASK.bits()
            << MemberNameFlags::REFERENCE_KIND_SHIFT.bits()))
        >> MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
    let reference_kind_value = u8::try_from(reference_kind_value)?;
    let reference_kind = ReferenceKind::try_from(reference_kind_value)?;

    // Get the descriptor (method signature or field type)
    // For methods, type is MethodType; for fields, type is Class
    let member_descriptor = if let Ok(ref type_val) = member_type {
        let type_class = get_object_class_name(type_val);

        if let Some(class_name) = type_class {
            if class_name == "java/lang/invoke/MethodType" {
                // It's a method - get the descriptor string
                let method_descriptor = thread
                    .invoke(
                        "java.lang.invoke.MethodType",
                        "toMethodDescriptorString()Ljava/lang/String;",
                        std::slice::from_ref(type_val),
                    )
                    .await?;
                match method_descriptor {
                    Some(descriptor) => descriptor.as_string()?,
                    _ => return Err(InternalError("Invalid MethodType".to_string())),
                }
            } else if class_name == "java/lang/Class" {
                // It's a field - get the class name and convert to descriptor
                let type_ref = type_val.as_object_ref()?;
                let field_type_name = type_ref.value("name")?.as_string()?;
                // For field access, we just need the field name, not the type descriptor
                // The type is used to verify the field but we don't need it for GetField
                format!("L{};", field_type_name.replace('.', "/"))
            } else {
                // Unknown type - try to use descriptor field
                if let Ok(descriptor) = member_descriptor.and_then(|value| value.as_string()) {
                    descriptor
                } else {
                    return Err(InternalError(format!(
                        "MemberName has unknown type class: {class_name}"
                    )));
                }
            }
        } else if let Ok(descriptor) = member_descriptor.and_then(|value| value.as_string()) {
            descriptor
        } else {
            return Err(InternalError(
                "MemberName missing type/descriptor".to_string(),
            ));
        }
    } else if let Ok(descriptor) = member_descriptor.and_then(|value| value.as_string()) {
        descriptor
    } else {
        return Err(InternalError(
            "MemberName missing type/descriptor".to_string(),
        ));
    };

    debug!(
        "call_method_handle_target: class={}, name={}, descriptor={}, kind={:?}, args={}",
        target_class_name,
        member_name,
        member_descriptor,
        reference_kind,
        arguments.len()
    );

    match reference_kind {
        ReferenceKind::InvokeVirtual | ReferenceKind::InvokeInterface => {
            if arguments.is_empty() {
                return Err(InternalError(format!(
                    "InvokeVirtual/Interface requires receiver: {target_class_name}.{member_name}{member_descriptor}"
                )));
            }
            let receiver = arguments.remove(0);

            // Check for null receiver; this is a NullPointerException
            if receiver.is_null() {
                return Err(NullPointerException(Some(format!(
                    "Cannot invoke virtual method {target_class_name}.{member_name}{member_descriptor} on null"
                )))
                .into());
            }

            // Try to find method on target class first, then on receiver's class if not found
            // This handles lambda methods where MemberName.clazz may be Object but the
            // lambda method is defined on the actual interface/class
            let (actual_class, method) = find_method_in_hierarchy(
                &thread,
                &target_class,
                &receiver,
                &member_name,
                &member_descriptor,
            )
            .await?;

            let mut call_arguments = vec![receiver];
            call_arguments.extend(arguments);
            let result = thread
                .execute(&actual_class, &method, &call_arguments)
                .await?;
            // For void methods, return null; otherwise return the result
            Ok(result.unwrap_or(Value::Object(None)))
        }
        ReferenceKind::InvokeStatic => {
            // For static methods (including static lambda methods), try the target class first
            // If not found and it's a lambda method, try to find it in arguments' class hierarchy
            let (actual_class, method) = match target_class
                .try_get_method(&member_name, &member_descriptor)
            {
                Ok(m) => (target_class.clone(), m),
                Err(_) if member_name.starts_with("lambda$") => {
                    // For lambda methods, try to find in argument's class hierarchy
                    find_static_lambda_method(&thread, &arguments, &member_name, &member_descriptor)
                        .await
                        .ok_or_else(|| {
                            InternalError(format!(
                                "Static lambda method not found: {target_class_name}.{member_name}{member_descriptor}"
                            ))
                        })?
                }
                Err(e) => return Err(e.into()),
            };
            let result = thread.execute(&actual_class, &method, &arguments).await?;
            Ok(result.unwrap_or(Value::Object(None)))
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
            if arguments.is_empty() {
                return Err(InternalError(format!(
                    "GetField requires receiver: {target_class_name}.{member_name}"
                )));
            }
            let argument = arguments.remove(0);
            if argument.is_null() {
                return Err(NullPointerException(Some(format!(
                    "Cannot get field {target_class_name}.{member_name} from null"
                )))
                .into());
            }
            let receiver = argument.as_object_ref()?;
            Ok(receiver.value(&member_name)?)
        }
        ReferenceKind::GetStatic => {
            let value = target_class.static_value(&member_name)?;
            Ok(value)
        }
        ReferenceKind::PutField => {
            if arguments.is_empty() {
                return Err(InternalError(format!(
                    "PutField requires receiver: {target_class_name}.{member_name}"
                )));
            }
            let argument = arguments.remove(0);
            if argument.is_null() {
                return Err(NullPointerException(Some(format!(
                    "Cannot set field {target_class_name}.{member_name} on null"
                )))
                .into());
            }
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
    // For constructor invocations (<init> methods):
    // - NewInvokeSpecial (is_constructor=true): Always allocate + invoke
    // - InvokeSpecial with <init>: If arguments is empty, treat as NewInvokeSpecial
    //   This handles the case where MethodHandle machinery uses InvokeSpecial for constructors
    let is_constructor_invocation =
        is_constructor || (method_name == "<init>" && arguments.is_empty());

    if is_constructor_invocation {
        // Allocate object and invoke constructor
        let start_index = method_descriptor.find('(').map_or(0, |i| i + 1);
        let end_index = method_descriptor
            .rfind(')')
            .unwrap_or(method_descriptor.len());
        let descriptor = &method_descriptor[start_index..end_index];
        let instance = thread
            .object(target_class.name(), descriptor, arguments.as_slice())
            .await?;
        Ok(instance)
    } else if method_name == "<init>" {
        // InvokeSpecial with <init> and a receiver - complete initialization of existing object
        let receiver = arguments.remove(0);
        let method = target_class.try_get_method(method_name, method_descriptor)?;
        let mut call_arguments = vec![receiver.clone()];
        call_arguments.extend(arguments);
        thread
            .execute(&target_class, &method, &call_arguments)
            .await?;
        Ok(receiver)
    } else {
        // Regular special invocation (private methods, super calls)
        if arguments.is_empty() {
            let class_name = target_class.name();
            return Err(InternalError(format!(
                "InvokeSpecial requires receiver: {class_name}.{method_name}{method_descriptor}"
            )));
        }
        let receiver = arguments.remove(0);
        let method = target_class.try_get_method(method_name, method_descriptor)?;
        let mut call_arguments = vec![receiver];
        call_arguments.extend(arguments);
        let result = thread
            .execute(&target_class, &method, &call_arguments)
            .await?;
        Ok(result.unwrap_or(Value::Object(None)))
    }
}

/// Checks if a class is a Holder class or similar class used for `LambdaForm` entry points.
/// These classes have dynamically generated methods that don't have real bytecode.
fn is_holder_class(class_name: &str) -> bool {
    // Class names may be in either format: java.lang.invoke.X or java/lang/invoke/X
    let normalized = class_name.replace('.', "/");

    // Exact matches
    if matches!(
        normalized.as_str(),
        "java/lang/invoke/DirectMethodHandle$Holder"
            | "java/lang/invoke/DelegatingMethodHandle$Holder"
            | "java/lang/invoke/Invokers$Holder"
            | "java/lang/invoke/LambdaForm$Holder"
            | "java/lang/invoke/VarHandleGuards"
    ) {
        return true;
    }

    // Pattern matches for dynamically generated LambdaForm classes
    // These include LambdaForm$MH, LambdaForm$DMH, LambdaForm$BMH, etc.
    if normalized.starts_with("java/lang/invoke/LambdaForm$") {
        return true;
    }

    false
}

/// Dispatches a Holder method call to the actual target.
///
/// Holder methods are entry points for `LambdaForms`. The first argument is typically
/// the `MethodHandle` itself, which contains the actual target in its `member` field.
///
/// `depth` is used to prevent infinite recursion in complex method handle chains.
#[expect(clippy::too_many_lines)]
#[async_method]
async fn dispatch_holder_method_internal(
    thread: Arc<Thread>,
    method_name: &str,
    mut arguments: Vec<Value>,
    depth: usize,
) -> Result<Value> {
    const MAX_DEPTH: usize = 50;
    if depth > MAX_DEPTH {
        return Err(InternalError(format!(
            "Maximum method handle chain depth ({MAX_DEPTH}) exceeded for {method_name}"
        )));
    }

    debug!(
        "dispatch_holder_method: method_name={}, args={}, depth={}",
        method_name,
        arguments.len(),
        depth
    );

    // The first argument should be the MethodHandle
    if arguments.is_empty() {
        return Err(InternalError(format!(
            "Holder method {method_name} requires at least a MethodHandle argument"
        )));
    }

    let method_handle = arguments.remove(0);

    // For constant_* methods, extract the constant from the MethodHandle's bound arguments
    // These holder methods return a constant value that was bound when the MethodHandle was created
    if method_name.starts_with("constant_") {
        let constant_value = {
            let mh_obj = method_handle.as_object_ref()?;
            let mh_class = mh_obj.class().name().to_string();

            // For BoundMethodHandle, the constant is in argL0
            if mh_class.starts_with("java/lang/invoke/BoundMethodHandle") {
                mh_obj.value("argL0").ok()
            } else {
                None
            }
        };

        if let Some(value) = constant_value {
            return Ok(value);
        }

        // Fallback - just return the first argument
        return Ok(method_handle);
    }

    // For newInvokeSpecial, we need to create a new instance and invoke the constructor
    // The MethodHandle contains the constructor target
    if method_name == "newInvokeSpecial" {
        let (mh_class, member_info, form_opt) = {
            let mh_ref = method_handle.as_object_ref()?;
            let mh_class = mh_ref.class().name().to_string();

            // Get the member info
            let member_info = if let Ok(member) = mh_ref.value("member")
                && !member.is_null()
            {
                let member_ref = member.as_object_ref()?;
                let member_clazz = member_ref.value("clazz").ok();
                let target_class_name = if let Some(ref clazz) = member_clazz
                    && !clazz.is_null()
                {
                    clazz
                        .as_object_ref()
                        .ok()
                        .and_then(|c| c.value("name").ok())
                        .and_then(|n| n.as_string().ok())
                } else {
                    None
                };
                Some((member.clone(), target_class_name))
            } else {
                None
            };

            let form_opt = mh_ref.value("form").ok().filter(|v| !v.is_null());

            (mh_class, member_info, form_opt)
        };

        debug!("newInvokeSpecial: mh_class={}", mh_class);

        // Check if member points to a real constructor
        if let Some((member, target_class_name)) = member_info
            && let Some(ref class_name) = target_class_name
        {
            if is_holder_class(class_name) {
                debug!("newInvokeSpecial: member points to holder class, skipping");
            } else {
                // The member points to a real constructor - use it
                return call_method_handle_target(thread, &member, arguments).await;
            }
        }

        // For DirectMethodHandle$Constructor, the target is in the form's names
        if let Some(form) = form_opt {
            debug!("newInvokeSpecial: has form, trying LambdaForm interpretation");
            let mut input_args = Vec::with_capacity(arguments.len() + 1);
            input_args.push(method_handle.clone());
            input_args.extend(arguments.clone());

            // Try LambdaForm interpretation - this should handle newInvokeSpecial correctly
            match interpret_lambda_form(thread.clone(), &form, input_args).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    debug!(
                        "newInvokeSpecial: LambdaForm interpretation failed: {:?}",
                        e
                    );
                }
            }
        }

        debug!("newInvokeSpecial: falling through to default handling");
    }

    // Handle special cases where the "MethodHandle" might not be a proper MethodHandle
    // This can happen with dynamically generated holder methods
    let is_object = method_handle.as_object_ref().is_ok();
    if !is_object {
        // Not an object - might be a primitive value or null
        debug!(
            "dispatch_holder_method: first arg is not an object for {}",
            method_name
        );
        return Ok(method_handle);
    }

    let mh_class = method_handle.as_object_ref()?.class().name().to_string();

    // Try to use LambdaForm interpretation first
    // This properly handles the argument layout that compiled LambdaForms expect
    // Extract form before await
    let form_opt = {
        let mh_ref = method_handle.as_object_ref()?;
        mh_ref.value("form").ok().filter(|v| !v.is_null())
    };

    if let Some(form) = form_opt {
        // Build input args: [MethodHandle, ...remaining_arguments]
        let mut input_args = Vec::with_capacity(arguments.len() + 1);
        input_args.push(method_handle.clone());
        input_args.extend(arguments.clone());

        match interpret_lambda_form(thread.clone(), &form, input_args).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                debug!(
                    "dispatch_holder_method: LambdaForm interpretation failed for {}: {:?}",
                    method_name, e
                );
                // Fall through to direct dispatch
            }
        }
    }

    // Fallback: For DirectMethodHandle, use the member directly
    if mh_class == "java/lang/invoke/DirectMethodHandle"
        || mh_class.starts_with("java/lang/invoke/DirectMethodHandle$")
    {
        // Extract member before await
        let member_opt = {
            let mh_ref = method_handle.as_object_ref()?;
            mh_ref.value("member").ok().filter(|v| !v.is_null())
        };

        if let Some(member) = member_opt {
            return call_method_handle_target(thread, &member, arguments).await;
        }
    }

    // Navigate through the MethodHandle chain to find the ultimate target and collect bound arguments
    let (target_member, mut bound_args) =
        extract_target_and_bound_args(&method_handle, depth, MAX_DEPTH)?;

    // Prepend bound arguments to the call arguments
    bound_args.append(&mut arguments);

    // Dispatch to the target
    call_method_handle_target(thread, &target_member, bound_args).await
}

/// Extract the ultimate target `MemberName` and all bound arguments from a `MethodHandle` chain.
///
/// This recursively traverses `BoundMethodHandle` wrappers to find the `DirectMethodHandle`
/// at the bottom and collect all bound arguments along the way.
fn extract_target_and_bound_args(
    method_handle: &Value,
    depth: usize,
    max_depth: usize,
) -> Result<(Value, Vec<Value>)> {
    if depth > max_depth {
        return Err(InternalError(
            "Maximum depth exceeded in MethodHandle chain".to_string(),
        ));
    }

    let mh_ref = method_handle.as_object_ref()?;

    // First check if this MethodHandle has a direct member field (DirectMethodHandle)
    if let Ok(member) = mh_ref.value("member")
        && !member.is_null()
    {
        return Ok((member, Vec::new()));
    }

    // Check if this is a BoundMethodHandle
    let mh_class_name = mh_ref.class().name().to_string();
    if mh_class_name.starts_with("java/lang/invoke/BoundMethodHandle") {
        // Collect bound arguments from this level
        let mut bound_args = Vec::new();

        // Try to get argL0 - this is often the inner MethodHandle
        if let Ok(arg_l0) = mh_ref.value("argL0")
            && !arg_l0.is_null()
        {
            // Check if argL0 is a MethodHandle (inner target) or a regular bound argument
            let arg_l0_class = arg_l0.as_object_ref()?.class().name().to_string();

            if arg_l0_class.contains("MethodHandle") {
                // argL0 is the inner MethodHandle - recurse into it
                let (inner_target, mut inner_bound) =
                    extract_target_and_bound_args(&arg_l0, depth + 1, max_depth)?;

                // Collect any additional bound arguments from this level (argL1, argL2, etc.)
                for i in 1..10 {
                    let field_name = format!("argL{i}");
                    if let Ok(arg) = mh_ref.value(&field_name) {
                        if !arg.is_null() {
                            bound_args.push(arg);
                        }
                    } else {
                        break;
                    }
                }

                // Collect primitive bound arguments (argI0, argJ0, etc.)
                for i in 0..10 {
                    let field_name = format!("argI{i}");
                    if let Ok(arg) = mh_ref.value(&field_name) {
                        bound_args.push(arg);
                    } else {
                        break;
                    }
                }

                // The bound args from this level come after inner bound args
                inner_bound.extend(bound_args);
                return Ok((inner_target, inner_bound));
            }

            // argL0 is a regular bound argument (e.g., receiver object)
            bound_args.push(arg_l0);
        }

        // Collect additional bound arguments
        for i in 1..10 {
            let field_name = format!("argL{i}");
            if let Ok(arg) = mh_ref.value(&field_name) {
                if !arg.is_null() {
                    bound_args.push(arg);
                }
            } else {
                break;
            }
        }

        for i in 0..10 {
            let field_name = format!("argI{i}");
            if let Ok(arg) = mh_ref.value(&field_name) {
                bound_args.push(arg);
            } else {
                break;
            }
        }

        // Try to find the target through form.vmentry
        if let Ok(form) = mh_ref.value("form")
            && !form.is_null()
        {
            let form_ref = form.as_object_ref()?;
            if let Ok(vmentry) = form_ref.value("vmentry")
                && !vmentry.is_null()
            {
                return Ok((vmentry, bound_args));
            }
        }
    }

    // Fallback: try form.vmentry
    if let Ok(form) = mh_ref.value("form")
        && !form.is_null()
    {
        let form_ref = form.as_object_ref()?;
        if let Ok(vmentry) = form_ref.value("vmentry")
            && !vmentry.is_null()
        {
            return Ok((vmentry, Vec::new()));
        }
    }

    Err(InternalError(format!(
        "Could not find target in MethodHandle of class {mh_class_name}"
    )))
}

/// Wrapper for `dispatch_holder_method_internal` that starts with depth=0.
pub(crate) async fn dispatch_holder_method(
    thread: Arc<Thread>,
    method_name: &str,
    arguments: Vec<Value>,
) -> Result<Value> {
    dispatch_holder_method_internal(thread, method_name, arguments, 0).await
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub(crate) async fn link_to_interface(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, the parameters are passed as individual values
    // after being drained from the stack using the call site descriptor.
    // The last parameter is the MemberName.
    let mut arguments: Vec<Value> = parameters.into_vec();
    let member = arguments
        .pop()
        .ok_or(InternalError("Missing MemberName".to_string()))?;
    let result = call_method_handle_target(thread, &member, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn link_to_native(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, the parameters are passed as individual values
    // after being drained from the stack using the call site descriptor.
    // The last parameter is the MemberName.
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
#[async_method]
pub(crate) async fn link_to_special(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, the parameters are passed as individual values
    // after being drained from the stack using the call site descriptor.
    // The last parameter is the MemberName.
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
#[async_method]
pub(crate) async fn link_to_static(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, the parameters are passed as individual values
    // after being drained from the stack using the call site descriptor.
    // The last parameter is the MemberName.
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
#[async_method]
pub(crate) async fn link_to_virtual(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // For signature polymorphic methods, the parameters are passed as individual values
    // after being drained from the stack using the call site descriptor.
    // The last parameter is the MemberName.
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
        ClassAccessFlags, ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType, JAVA_17,
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

        let arg = "42".to_object(&thread).await?;

        // For signature polymorphic methods, push method_handle first, then individual arguments
        let mut parameters = Parameters::default();
        parameters.push(method_handle);
        parameters.push(arg);

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

        let arg = "42".to_object(&thread).await?;

        // For signature polymorphic methods, push method_handle first, then individual arguments
        let mut parameters = Parameters::default();
        parameters.push(method_handle);
        parameters.push(arg);

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

use ristretto_classfile::JavaStr;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Class, Method, Object, Reference, Value};
use ristretto_gc::Gc;
use ristretto_gc::sync::RwLock;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::{IndexOutOfBoundsException, NullPointerException};
use ristretto_types::ModuleAccess;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// The VM-private information needed to initialize one `StackTraceElement`.
///
/// Ristretto stores Throwable backtraces in a platform-independent representation, while
/// `StackFrameInfo` follows the private layout of the selected JDK. Both are normalized to this
/// representation before the public `StackTraceElement` fields are populated.
struct StackTraceFrame {
    class_mirror: Value,
    class_name: String,
    method_name: String,
    descriptor: Option<String>,
    bytecode_index: Option<i32>,
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_stack_trace_element<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stack_frame_info_value = parameters.pop()?;
    let element = parameters.pop()?;
    if stack_frame_info_value.is_null() || element.is_null() {
        return Err(NullPointerException(None).into());
    }

    let frame = stack_frame_info(&stack_frame_info_value)?;
    fill_stack_trace_element(&thread, &element, &frame).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Throwable;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn init_stack_trace_elements_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Java 11-17: parameters are (stack_trace_elements[], throwable).
    let throwable = parameters.pop()?;
    if throwable.is_null() {
        return Err(NullPointerException(None).into());
    }

    let (back_trace, depth) = {
        let throwable_ref = throwable.as_object_ref()?;
        let back_trace = throwable_ref.value("backtrace")?;
        let depth = throwable_ref.value("depth")?.as_i32()?;
        (back_trace, depth)
    };

    let Some(stack_trace_ref) = parameters.pop_reference()? else {
        return Err(NullPointerException(None).into());
    };

    init_stack_trace_elements_impl(thread, stack_trace_ref, back_trace, depth).await
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn init_stack_trace_elements_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Java 18+: parameters are (stack_trace_elements[], backtrace, depth).
    let depth = parameters.pop_int()?;
    let back_trace = parameters.pop()?;
    let Some(stack_trace_ref) = parameters.pop_reference()? else {
        return Err(NullPointerException(None).into());
    };

    init_stack_trace_elements_impl(thread, stack_trace_ref, back_trace, depth).await
}

/// Common implementation for `init_stack_trace_elements`.
///
/// The backtrace is an Object[] where each element is an Object[] containing:
/// \[0\] = Class object
/// \[1\] = method name (String)
/// \[2\] = method descriptor (String)
/// \[3\] = Integer (program counter/BCI)
async fn init_stack_trace_elements_impl<T: Thread + 'static>(
    thread: Arc<T>,
    stack_trace_ref: Gc<RwLock<Reference>>,
    back_trace: Value,
    depth: i32,
) -> Result<Option<Value>> {
    if back_trace.is_null() {
        return Err(NullPointerException(None).into());
    }

    let depth = usize::try_from(depth).map_err(|_| IndexOutOfBoundsException {
        index: depth,
        size: depth,
    })?;

    // OpenJDK requires the caller to allocate both the array and every element. The native fills
    // those exact objects; replacing them would violate observable object identity.
    let stack_trace_elements = {
        let stack_trace_guard = stack_trace_ref.read();
        let Reference::Array(stack_trace_array) = &*stack_trace_guard else {
            return Err(InternalError(
                "Stack trace object is not an object array".to_string(),
            ));
        };
        if stack_trace_array.elements.len() != depth {
            return Err(IndexOutOfBoundsException {
                index: i32::try_from(stack_trace_array.elements.len()).unwrap_or(i32::MAX),
                size: i32::try_from(depth).unwrap_or(i32::MAX),
            }
            .into());
        }
        stack_trace_array.elements.to_vec()
    };

    let back_trace_elements = {
        let back_trace_guard = back_trace.as_reference()?;
        let Reference::Array(back_trace_array) = &*back_trace_guard else {
            return Err(InternalError(
                "Back trace object is not an object array".to_string(),
            ));
        };
        if back_trace_array.elements.len() < depth {
            return Err(InternalError(format!(
                "Back trace has {} frames but depth is {depth}",
                back_trace_array.elements.len()
            )));
        }
        back_trace_array
            .elements
            .get(..depth)
            .ok_or_else(|| InternalError("Back trace depth changed while decoding".to_string()))?
            .to_vec()
    };

    for (index, (element, back_trace_element)) in stack_trace_elements
        .iter()
        .zip(&back_trace_elements)
        .enumerate()
    {
        if element.is_null() {
            return Err(NullPointerException(None).into());
        }
        let frame = back_trace_frame(back_trace_element, index)?;
        fill_stack_trace_element(&thread, element, &frame).await?;
    }
    Ok(None)
}

/// Decode Ristretto's platform-independent Throwable backtrace entry.
///
/// Each entry is an `Object[]` containing the declaring `Class`, method name, descriptor, and
/// boxed bytecode index, in that order.
fn back_trace_frame(value: &Value, index: usize) -> Result<StackTraceFrame> {
    if value.is_null() {
        return Err(InternalError(format!(
            "Back trace frame at index {index} is null"
        )));
    }

    let values = {
        let frame_guard = value.as_reference()?;
        let Reference::Array(frame_array) = &*frame_guard else {
            return Err(InternalError(format!(
                "Back trace frame at index {index} is not an object array"
            )));
        };
        if frame_array.elements.len() < 4 {
            return Err(InternalError(format!(
                "Back trace frame at index {index} has {} fields; expected 4",
                frame_array.elements.len()
            )));
        }
        frame_array
            .elements
            .get(..4)
            .ok_or_else(|| InternalError(format!("Back trace frame at index {index} changed")))?
            .to_vec()
    };

    let [
        class_mirror,
        method_name_value,
        descriptor_value,
        bytecode_index_value,
    ] = values.as_slice()
    else {
        return Err(InternalError(format!(
            "Back trace frame at index {index} does not contain four fields"
        )));
    };
    let class_name = class_mirror_name(class_mirror)?;
    let method_name = method_name_value.as_string()?;
    let descriptor = if descriptor_value.is_null() {
        None
    } else {
        Some(descriptor_value.as_string()?)
    };
    let bytecode_index = if bytecode_index_value.is_null() {
        None
    } else {
        Some(bytecode_index_value.as_i32()?)
    };

    Ok(StackTraceFrame {
        class_mirror: class_mirror.clone(),
        class_name,
        method_name,
        descriptor,
        bytecode_index,
    })
}

/// Decode the private `StackFrameInfo` layouts used by all supported JDKs.
///
/// JDK 11 through 21 store a VM-initialized `MemberName` in `memberName`. JDK 25 stores a
/// `ResolvedMethodName` in the inherited `classOrMemberName` field and keeps the expanded name and
/// type directly on `StackFrameInfo`.
fn stack_frame_info(value: &Value) -> Result<StackTraceFrame> {
    let (bytecode_index, direct_name, direct_type, member) = {
        let object = value.as_object_ref()?;
        let bytecode_index = object.value("bci")?.as_i32()?;
        let direct_name = first_non_null_value(&object, &["name"]);
        let direct_type = first_non_null_value(&object, &["type"]);
        let member = first_non_null_value(&object, &["classOrMemberName", "memberName"])
            .ok_or_else(|| InternalError("StackFrameInfo has no member information".to_string()))?;
        (bytecode_index, direct_name, direct_type, member)
    };

    let (class_mirror, member_name, member_type) = {
        let member_object = member.as_object_ref()?;
        if member_object.class().name() == "java/lang/Class" {
            (member.clone(), None, None)
        } else {
            let class_mirror = first_non_null_value(&member_object, &["vmholder", "clazz"])
                .ok_or_else(|| {
                    InternalError("StackFrameInfo member has no declaring class".to_string())
                })?;
            let name = first_non_null_value(&member_object, &["name"]);
            let member_type = first_non_null_value(&member_object, &["type"]);
            (class_mirror, name, member_type)
        }
    };

    let method_name = direct_name
        .or(member_name)
        .ok_or_else(|| InternalError("StackFrameInfo has no method name".to_string()))?
        .as_string()?;
    let descriptor = match direct_type.or(member_type) {
        Some(value) => method_descriptor(&value)?,
        None => None,
    };

    Ok(StackTraceFrame {
        class_mirror: class_mirror.clone(),
        class_name: class_mirror_name(&class_mirror)?,
        method_name,
        descriptor,
        bytecode_index: Some(bytecode_index),
    })
}

fn first_non_null_value(object: &Object, names: &[&str]) -> Option<Value> {
    names
        .iter()
        .find_map(|name| object.value(*name).ok().filter(|value| !value.is_null()))
}

fn class_mirror_name(value: &Value) -> Result<String> {
    if value.is_null() {
        return Err(InternalError("Declaring class mirror is null".to_string()));
    }
    let class_object = value.as_object_ref()?;
    if class_object.class().name() != "java/lang/Class" {
        return Err(InternalError(format!(
            "Expected a java.lang.Class mirror; found {}",
            class_object.class().name()
        )));
    }
    class_object.value("name")?.as_string().map_err(Into::into)
}

/// Convert the private `MemberName`/`StackFrameInfo.type` representation to a JVM descriptor.
fn method_descriptor(value: &Value) -> Result<Option<String>> {
    if value.is_null() {
        return Ok(None);
    }
    if let Ok(descriptor) = value.as_string() {
        return Ok(Some(descriptor));
    }

    let value_class = value.as_reference()?.class_name()?;
    if value_class == "java/lang/invoke/MethodType" {
        let (cached_descriptor, return_type, parameter_types) = {
            let method_type = value.as_object_ref()?;
            (
                first_non_null_value(&method_type, &["methodDescriptor"]),
                method_type.value("rtype")?,
                method_type.value("ptypes")?,
            )
        };
        if let Some(cached_descriptor) = cached_descriptor {
            return Ok(Some(cached_descriptor.as_string()?));
        }

        let parameters = object_array_values(&parameter_types, "MethodType.ptypes")?;
        let mut descriptor = String::from("(");
        for parameter in parameters {
            descriptor.push_str(&class_mirror_descriptor(&parameter)?);
        }
        descriptor.push(')');
        descriptor.push_str(&class_mirror_descriptor(&return_type)?);
        return Ok(Some(descriptor));
    }

    // The VM may temporarily represent MemberName.type as Object[] { return type, parameter
    // Class[] } before Java converts it to a MethodType.
    if value_class.starts_with('[') {
        let types = object_array_values(value, "MemberName.type")?;
        let [return_type, parameter_types] = types.as_slice() else {
            return Err(InternalError(
                "MemberName type array must contain a return type and parameter array".to_string(),
            ));
        };
        let parameter_types = object_array_values(parameter_types, "MemberName parameter types")?;
        let mut descriptor = String::from("(");
        for parameter in &parameter_types {
            descriptor.push_str(&class_mirror_descriptor(parameter)?);
        }
        descriptor.push(')');
        descriptor.push_str(&class_mirror_descriptor(return_type)?);
        return Ok(Some(descriptor));
    }

    Ok(None)
}

fn object_array_values(value: &Value, description: &str) -> Result<Vec<Value>> {
    let reference = value.as_reference()?;
    let Reference::Array(array) = &*reference else {
        return Err(InternalError(format!(
            "{description} is not an object array"
        )));
    };
    Ok(array.elements.to_vec())
}

fn class_mirror_descriptor(value: &Value) -> Result<String> {
    let name = class_mirror_name(value)?;
    let descriptor = match name.as_str() {
        "boolean" => "Z".to_string(),
        "byte" => "B".to_string(),
        "char" => "C".to_string(),
        "double" => "D".to_string(),
        "float" => "F".to_string(),
        "int" => "I".to_string(),
        "long" => "J".to_string(),
        "short" => "S".to_string(),
        "void" => "V".to_string(),
        name if name.starts_with('[') => name.replace('.', "/"),
        name => format!("L{};", name.replace('.', "/")),
    };
    Ok(descriptor)
}

async fn fill_stack_trace_element<T: Thread + 'static>(
    thread: &Arc<T>,
    element: &Value,
    frame: &StackTraceFrame,
) -> Result<()> {
    let internal_class_name = frame.class_name.replace('.', "/");
    let class_name = JavaStr::cow_from_str(&internal_class_name);
    let class = thread.load_and_link_class(&class_name).await?;
    let method = resolve_frame_method(&class, frame);

    // Class.getName() is already in the public StackTraceElement form. In particular, retaining
    // its slash before a hidden-class suffix is required; blindly replacing every slash would
    // corrupt names such as `pkg.Hidden/0x...`.
    let declaring_class_value = thread.intern_string(&frame.class_name).await?;
    // Preserve the exact Class mirror supplied by the VM. This is observable through
    // StackTraceElement's formatting path and matters for custom/hidden class loaders.
    let declaring_class_object = frame.class_mirror.clone();
    let method_name = thread.intern_string(&frame.method_name).await?;

    let (file_name, line_number) = if let Some(method) = method.as_deref() {
        let file_name = match class.source_file() {
            Some(file_name) => thread.intern_string(file_name).await?,
            None => Value::Object(None),
        };
        (file_name, line_number(method, frame.bytecode_index))
    } else {
        (Value::Object(None), -1)
    };

    let class_loader_name = match class_loader_name(&frame.class_mirror)? {
        Some(class_loader_name) => thread.intern_string(&class_loader_name).await?,
        None => Value::Object(None),
    };

    let module_name = class.module_name()?;
    let module_version = match module_name.as_deref() {
        Some(module_name) => module_version(thread, module_name)?,
        None => None,
    };
    let module_name = match module_name {
        Some(module_name) => thread.intern_string(&module_name).await?,
        None => Value::Object(None),
    };
    let module_version = match module_version {
        Some(module_version) => thread.intern_string(&module_version).await?,
        None => Value::Object(None),
    };

    let mut element = element.as_object_mut()?;
    element.set_value("declaringClassObject", declaring_class_object)?;
    element.set_value("classLoaderName", class_loader_name)?;
    element.set_value("moduleName", module_name)?;
    element.set_value("moduleVersion", module_version)?;
    element.set_value("declaringClass", declaring_class_value)?;
    element.set_value("methodName", method_name)?;
    element.set_value("fileName", file_name)?;
    element.set_value("lineNumber", Value::Int(line_number))?;
    Ok(())
}

fn class_loader_name(class_mirror: &Value) -> Result<Option<String>> {
    let class = class_mirror.as_object_ref()?;
    let class_loader = class.value("classLoader")?;
    if class_loader.is_null() {
        return Ok(None);
    }

    let class_loader = class_loader.as_object_ref()?;
    let name = class_loader.value("name")?;
    if name.is_null() {
        Ok(None)
    } else {
        Ok(Some(name.as_string()?))
    }
}

fn resolve_frame_method(class: &Class, frame: &StackTraceFrame) -> Option<Arc<Method>> {
    if let Some(descriptor) = frame.descriptor.as_deref() {
        return class.method(&frame.method_name, descriptor);
    }

    let mut methods = class
        .methods()
        .into_iter()
        .filter(|method| method.name() == frame.method_name);
    let method = methods.next()?;
    if methods.next().is_none() {
        Some(method)
    } else {
        None
    }
}

/// Return the source line for a logical bytecode index using JVMS 4.7.12 semantics.
fn line_number(method: &Method, bytecode_index: Option<i32>) -> i32 {
    if method.is_native() {
        return -2;
    }
    let Some(bytecode_index) =
        bytecode_index.and_then(|bytecode_index| u16::try_from(bytecode_index).ok())
    else {
        return -1;
    };

    // JVMS permits multiple LineNumberTable attributes and does not require table order. Select
    // the entry whose start_pc is the nearest one at or before the current instruction.
    let line_number = method
        .definition()
        .attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Code { attributes, .. } => Some(attributes),
            _ => None,
        })
        .flat_map(|attributes| attributes.iter())
        .filter_map(|attribute| match attribute {
            Attribute::LineNumberTable { line_numbers, .. } => Some(line_numbers),
            _ => None,
        })
        .flat_map(|line_numbers| line_numbers.iter())
        .filter(|line_number| line_number.start_pc <= bytecode_index)
        .max_by_key(|line_number| line_number.start_pc);
    let Some(line_number) = line_number else {
        return -1;
    };
    i32::from(line_number.line_number)
}

fn module_version<T: Thread + 'static>(
    thread: &Arc<T>,
    module_name: &str,
) -> Result<Option<String>> {
    let vm = thread.vm()?;
    let module_system = vm.module_system();
    if let Some(version) = module_system
        .get_module(module_name)
        .and_then(|module| module.version)
    {
        return Ok(Some(version));
    }
    Ok(module_system
        .resolved_configuration()
        .get(module_name)
        .and_then(|module| module.descriptor().version.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::JAVA_25;
    use ristretto_classloader::{Object, Reference};
    use ristretto_types::JavaObject;

    /// Create a backtrace Object[] with the given number of frames.
    /// Each frame references "java/lang/Object" class with method "hashCode()I".
    async fn create_backtrace(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        create_backtrace_for_method(vm, thread, count, "hashCode", "()I").await
    }

    async fn create_backtrace_for_method(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
        method: &str,
        descriptor: &str,
    ) -> Result<Value> {
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let integer_class = thread.class("java/lang/Integer").await?;
        let obj_class = thread.class("java/lang/Object").await?;
        let class_obj = obj_class.to_object(thread).await?;

        let mut backtrace_elements = Vec::new();
        for i in 0..count {
            let method_name = method.to_object(thread).await?;
            let descriptor = descriptor.to_object(thread).await?;

            let mut pc_obj = Object::new(integer_class.clone())?;
            pc_obj.set_value("value", Value::Int(i32::try_from(i)?))?;
            let pc_value = Value::new_object(vm.garbage_collector(), Reference::Object(pc_obj));

            let frame_info = vec![class_obj.clone(), method_name, descriptor, pc_value];
            let reference = Reference::try_from((object_array_class.clone(), frame_info))?;
            backtrace_elements.push(Value::new_object(vm.garbage_collector(), reference));
        }

        let reference = Reference::try_from((object_array_class, backtrace_elements))?;
        Ok(Value::new_object(vm.garbage_collector(), reference))
    }

    /// Create the fully pre-allocated array required by `StackTraceElement.of(...)`.
    async fn create_stack_trace_array(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let stack_element_array_class = thread.class("[Ljava/lang/StackTraceElement;").await?;
        let stack_element_class = thread.class("java/lang/StackTraceElement").await?;
        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            let element = Object::new(stack_element_class.clone())?;
            elements.push(Value::new_object(
                vm.garbage_collector(),
                Reference::Object(element),
            ));
        }
        let reference = Reference::try_from((stack_element_array_class, elements))?;
        Ok(Value::new_object(vm.garbage_collector(), reference))
    }

    async fn create_null_stack_trace_array(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let stack_element_array_class = thread.class("[Ljava/lang/StackTraceElement;").await?;
        let reference =
            Reference::try_from((stack_element_array_class, vec![Value::Object(None); count]))?;
        Ok(Value::new_object(vm.garbage_collector(), reference))
    }

    /// Build the private JDK-specific input consumed by `initStackTraceElement`.
    async fn create_stack_frame_info(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
    ) -> Result<Value> {
        let stack_frame_info_class = thread.class("java/lang/StackFrameInfo").await?;
        let object_class = thread.class("java/lang/Object").await?;
        let class_mirror = object_class.to_object(thread).await?;
        let method_name = "hashCode".to_object(thread).await?;
        let descriptor = "()I".to_object(thread).await?;
        let mut stack_frame_info = Object::new(stack_frame_info_class)?;
        stack_frame_info.set_value("bci", Value::Int(0))?;

        if vm.java_major_version() >= JAVA_25.java() {
            let resolved_method_name_class =
                thread.class("java/lang/invoke/ResolvedMethodName").await?;
            let mut resolved_method_name = Object::new(resolved_method_name_class)?;
            resolved_method_name.set_value("vmholder", class_mirror)?;
            let resolved_method_name = Value::new_object(
                vm.garbage_collector(),
                Reference::Object(resolved_method_name),
            );
            stack_frame_info.set_value("classOrMemberName", resolved_method_name)?;
            stack_frame_info.set_value("name", method_name)?;
            stack_frame_info.set_value("type", descriptor)?;
        } else {
            let member_name_class = thread.class("java/lang/invoke/MemberName").await?;
            let mut member_name = Object::new(member_name_class)?;
            member_name.set_value("clazz", class_mirror)?;
            member_name.set_value("name", method_name)?;
            member_name.set_value("type", descriptor)?;
            let member_name =
                Value::new_object(vm.garbage_collector(), Reference::Object(member_name));
            stack_frame_info.set_value("memberName", member_name)?;
        }

        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(stack_frame_info),
        ))
    }

    async fn verify_init_stack_trace_element(
        vm: Arc<impl VM + 'static>,
        thread: Arc<impl Thread + 'static>,
    ) -> Result<()> {
        let stack_trace_element_class = thread.class("java/lang/StackTraceElement").await?;
        let element = Value::new_object(
            vm.garbage_collector(),
            Reference::Object(Object::new(stack_trace_element_class)?),
        );
        let stack_frame_info = create_stack_frame_info(&vm, &thread).await?;

        let result = init_stack_trace_element(
            thread,
            Parameters::new(vec![element.clone(), stack_frame_info]),
        )
        .await?;
        assert_eq!(None, result);

        let element = element.as_object_ref()?;
        assert_eq!(
            "java.lang.Object",
            element.value("declaringClass")?.as_string()?
        );
        assert_eq!("hashCode", element.value("methodName")?.as_string()?);
        assert_eq!("Object.java", element.value("fileName")?.as_string()?);
        assert_eq!(-2, element.value("lineNumber")?.as_i32()?);
        assert!(!element.value("declaringClassObject")?.is_null());
        assert!(element.value("classLoaderName")?.is_null());
        assert_eq!("java.base", element.value("moduleName")?.as_string()?);
        assert!(!element.value("moduleVersion")?.is_null());
        Ok(())
    }

    /// Create a Throwable with backtrace and depth fields set.
    async fn create_throwable_with_backtrace(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let throwable_class = thread.class("java/lang/Throwable").await?;
        let backtrace = create_backtrace(vm, thread, count).await?;
        let depth = i32::try_from(count)?;

        let mut throwable_object = Object::new(throwable_class)?;
        throwable_object.set_value("backtrace", backtrace)?;
        throwable_object.set_value("depth", Value::Int(depth))?;
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(throwable_object),
        ))
    }

    #[tokio::test]
    async fn test_init_stack_trace_element() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_stack_trace_element(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(NullPointerException(
                None
            )))
        ));
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        verify_init_stack_trace_element(vm, thread).await
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java17() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        verify_init_stack_trace_element(vm, thread).await
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        verify_init_stack_trace_element(vm, thread).await
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java25() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        verify_init_stack_trace_element(vm, thread).await
    }

    #[tokio::test]
    async fn test_method_descriptor_vm_object_array_representation() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let int_class = thread.class("int").await?;
        let return_type = int_class.to_object(&thread).await?;

        let parameter_array_class = thread.class("[Ljava/lang/Class;").await?;
        let parameter_types = Reference::try_from((parameter_array_class, Vec::<Value>::new()))?;
        let parameter_types = Value::new_object(vm.garbage_collector(), parameter_types);

        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let type_info =
            Reference::try_from((object_array_class, vec![return_type, parameter_types]))?;
        let type_info = Value::new_object(vm.garbage_collector(), type_info);

        assert_eq!(Some("()I".to_string()), method_descriptor(&type_info)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_empty_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_empty_java17() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_single_frame_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let Some(element) = array.elements.first() else {
            panic!("Expected stack trace element");
        };
        assert!(!element.is_null());
        let element_ref = element.as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_single_frame_java17() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(
            array
                .elements
                .first()
                .is_some_and(|element| !element.is_null())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_null_backtrace() {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let stack_trace = create_stack_trace_array(&vm, &thread, 1)
            .await
            .expect("stack trace");
        let throwable_class = thread
            .class("java/lang/Throwable")
            .await
            .expect("throwable class");
        let mut throwable_object = Object::new(throwable_class).expect("throwable object");
        throwable_object
            .set_value("backtrace", Value::Object(None))
            .expect("set backtrace");
        throwable_object
            .set_value("depth", Value::Int(1))
            .expect("set depth");
        let throwable =
            Value::new_object(vm.garbage_collector(), Reference::Object(throwable_object));

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_multiple_frames_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let count = 3;
        let stack_trace = create_stack_trace_array(&vm, &thread, count).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, count).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        for (i, element) in array.elements.iter().take(count).enumerate() {
            assert!(!element.is_null(), "element {i} should not be null");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_null_stack_trace_ref() {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1)
            .await
            .expect("throwable");

        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_empty_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let backtrace = create_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(backtrace);
        parameters.push(Value::Int(0));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_empty_java25() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let backtrace = create_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(backtrace);
        parameters.push(Value::Int(0));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_single_frame_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let Some(element) = array.elements.first() else {
            panic!("Expected stack trace element");
        };
        assert!(!element.is_null());
        let element_ref = element.as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_single_frame_java25() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(
            array
                .elements
                .first()
                .is_some_and(|element| !element.is_null())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_null_backtrace() {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let stack_trace = create_stack_trace_array(&vm, &thread, 1)
            .await
            .expect("stack trace");

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(Value::Object(None));
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_multiple_frames_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let count = 3;
        let stack_trace = create_stack_trace_array(&vm, &thread, count).await?;
        let backtrace = create_backtrace(&vm, &thread, count).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(i32::try_from(count)?));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        for (i, element) in array.elements.iter().take(count).enumerate() {
            assert!(!element.is_null(), "element {i} should not be null");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_null_stack_trace_ref() {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let backtrace = create_backtrace(&vm, &thread, 1).await.expect("backtrace");

        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_preserves_element_identity() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let original_element = {
            let stack_trace_ref = stack_trace.as_reference()?;
            let Reference::Array(array) = &*stack_trace_ref else {
                panic!("Expected array");
            };
            array
                .elements
                .first()
                .cloned()
                .ok_or_else(|| InternalError("Expected original element".to_string()))?
        };
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let parameters = Parameters::new(vec![stack_trace.clone(), backtrace, Value::Int(1)]);
        init_stack_trace_elements_1(thread, parameters).await?;

        let populated_element = {
            let stack_trace_ref = stack_trace.as_reference()?;
            let Reference::Array(array) = &*stack_trace_ref else {
                panic!("Expected array");
            };
            array
                .elements
                .first()
                .cloned()
                .ok_or_else(|| InternalError("Expected populated element".to_string()))?
        };
        let Value::Object(Some(original_element)) = original_element else {
            panic!("Expected original element");
        };
        let Value::Object(Some(populated_element)) = populated_element else {
            panic!("Expected populated element");
        };
        assert!(original_element.ptr_eq(&populated_element));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_maps_bytecode_index_to_line() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace_for_method(&vm, &thread, 1, "<init>", "()V").await?;
        let parameters = Parameters::new(vec![stack_trace.clone(), backtrace, Value::Int(1)]);

        init_stack_trace_elements_1(thread, parameters).await?;

        let stack_trace_ref = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_ref else {
            panic!("Expected array");
        };
        let line_number = array
            .elements
            .first()
            .ok_or_else(|| InternalError("Expected stack trace element".to_string()))?
            .as_object_ref()?
            .value("lineNumber")?
            .as_i32()?;
        assert!(line_number > 0, "Expected a source line, got {line_number}");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_rejects_null_element() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_null_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;
        let parameters = Parameters::new(vec![stack_trace, backtrace, Value::Int(1)]);

        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(NullPointerException(
                None
            )))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_rejects_wrong_array_length() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 2).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;
        let parameters = Parameters::new(vec![stack_trace, backtrace, Value::Int(1)]);

        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(
                IndexOutOfBoundsException { .. }
            ))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_rejects_negative_depth() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let backtrace = create_backtrace(&vm, &thread, 0).await?;
        let parameters = Parameters::new(vec![stack_trace, backtrace, Value::Int(-1)]);

        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(
                IndexOutOfBoundsException { .. }
            ))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_verifies_fields_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let Some(element) = array.elements.first() else {
            panic!("Expected stack trace element");
        };
        let element_ref = element.as_object_ref()?;

        // Verify declaringClass is set
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");

        // Verify declaringClassObject is not null
        let declaring_class_obj = element_ref.value("declaringClassObject")?;
        assert!(!declaring_class_obj.is_null());

        // Verify methodName is set
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");

        // StackTraceElement uses -2 to identify native methods.
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert_eq!(-2, line_number);

        assert!(element_ref.value("classLoaderName")?.is_null());
        assert_eq!("java.base", element_ref.value("moduleName")?.as_string()?);
        assert!(!element_ref.value("moduleVersion")?.is_null());

        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_verifies_fields_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let Some(element) = array.elements.first() else {
            panic!("Expected stack trace element");
        };
        let element_ref = element.as_object_ref()?;

        // Verify declaringClass
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");

        // Verify declaringClassObject is not null
        let declaring_class_obj = element_ref.value("declaringClassObject")?;
        assert!(!declaring_class_obj.is_null());

        // Verify methodName
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");

        // Verify lineNumber
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert_eq!(-2, line_number);

        assert!(element_ref.value("classLoaderName")?.is_null());
        assert_eq!("java.base", element_ref.value("moduleName")?.as_string()?);
        assert!(!element_ref.value("moduleVersion")?.is_null());

        Ok(())
    }
}

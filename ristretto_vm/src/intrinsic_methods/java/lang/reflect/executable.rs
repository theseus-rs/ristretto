use crate::JavaObject;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use byteorder::{BigEndian, WriteBytesExt};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/reflect/Executable.getParameters0()[Ljava/lang/reflect/Parameter;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_parameters_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // 'this' is the Executable (Method or Constructor)
    let executable = parameters.pop()?;
    let executable_obj = executable.as_object_ref()?;

    // Get the declaring class and slot to find the method
    let declaring_class_value = executable_obj.value("clazz")?;
    let slot_i32 = executable_obj.value("slot")?.as_i32()?;
    let slot = usize::try_from(slot_i32)
        .map_err(|_| crate::Error::InternalError(format!("Invalid slot value: {slot_i32}")))?;

    // Get the actual class from the Class object
    let declaring_class_obj = declaring_class_value.as_object_ref()?;
    let class_name = declaring_class_obj.value("name")?.as_string()?;
    let class_name = class_name.replace('.', "/");
    let declaring_class = thread.class(&class_name).await?;

    // Get the method from the slot (index into class.methods())
    let methods = declaring_class.methods();
    let method = methods
        .get(slot)
        .ok_or_else(|| crate::Error::InternalError(format!("Method slot {slot} out of bounds")))?;

    // Look for MethodParameters attribute
    for attribute in method.attributes() {
        if let Attribute::MethodParameters {
            parameters: method_parameters,
            ..
        } = attribute
        {
            let constant_pool = declaring_class.constant_pool();
            let parameter_array_class = thread.class("[Ljava/lang/reflect/Parameter;").await?;
            let mut result_params = Vec::new();

            for (index, param) in method_parameters.iter().enumerate() {
                // Get parameter name from constant pool (if name_index is non-zero)
                let name_value = if param.name_index == 0 {
                    Value::Object(None)
                } else {
                    let name = constant_pool.try_get_utf8(param.name_index)?;
                    name.to_object(&thread).await?
                };

                // Modifiers from access flags
                let modifiers = Value::Int(i32::from(param.access_flags.bits()));

                // Create Parameter object using constructor: (String, int, Executable, int)
                let parameter = thread
                    .object(
                        "java/lang/reflect/Parameter",
                        "Ljava/lang/String;ILjava/lang/reflect/Executable;I",
                        &[
                            name_value,
                            modifiers,
                            executable.clone(),
                            Value::Int(i32::try_from(index)?),
                        ],
                    )
                    .await?;
                result_params.push(parameter);
            }

            let result = Reference::try_from((parameter_array_class, result_params))?;
            return Ok(Some(Value::from(result)));
        }
    }

    // No MethodParameters attribute - return null (JVM behavior)
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/lang/reflect/Executable.getTypeAnnotationBytes0()[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_type_annotation_bytes_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // 'this' is the Executable (Method or Constructor)
    let executable = parameters.pop()?;
    let executable_obj = executable.as_object_ref()?;

    // Get the declaring class and slot to find the method
    let declaring_class_value = executable_obj.value("clazz")?;
    let slot_i32 = executable_obj.value("slot")?.as_i32()?;
    let slot = usize::try_from(slot_i32)
        .map_err(|_| crate::Error::InternalError(format!("Invalid slot value: {slot_i32}")))?;

    // Get the actual class from the Class object
    let declaring_class_obj = declaring_class_value.as_object_ref()?;
    let class_name = declaring_class_obj.value("name")?.as_string()?;
    let class_name = class_name.replace('.', "/");
    let declaring_class = thread.class(&class_name).await?;

    // Get the method from the slot (index into class.methods())
    let methods = declaring_class.methods();
    let method = methods
        .get(slot)
        .ok_or_else(|| crate::Error::InternalError(format!("Method slot {slot} out of bounds")))?;

    // Collect all type annotations from both visible and invisible attributes
    let mut all_type_annotations = Vec::new();

    for attribute in method.attributes() {
        match attribute {
            Attribute::RuntimeVisibleTypeAnnotations {
                type_annotations, ..
            }
            | Attribute::RuntimeInvisibleTypeAnnotations {
                type_annotations, ..
            } => {
                all_type_annotations.extend(type_annotations.iter().cloned());
            }
            _ => {}
        }
    }

    // If no type annotations, return null
    if all_type_annotations.is_empty() {
        return Ok(Some(Value::Object(None)));
    }

    // Serialize the type annotations to bytes
    let mut bytes = Vec::new();
    let type_annotations_length = u16::try_from(all_type_annotations.len())?;
    bytes.write_u16::<BigEndian>(type_annotations_length)?;
    for type_annotation in &all_type_annotations {
        type_annotation.to_bytes(&mut bytes)?;
    }

    // Create byte array
    let byte_array_class = thread.class("[B").await?;
    let byte_values: Vec<Value> = bytes
        .into_iter()
        .map(|b| Value::Int(i32::from(b)))
        .collect();
    let result = Reference::try_from((byte_array_class, byte_values))?;
    Ok(Some(Value::from(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: get_parameters_0 is now implemented and requires a proper Executable object
    // Testing it properly requires setting up a reflection context which is complex

    #[tokio::test]
    async fn test_get_type_annotation_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_type_annotation_bytes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

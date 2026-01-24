use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use byteorder::{BigEndian, WriteBytesExt};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/reflect/Field.getTypeAnnotationBytes0()[B", Any)]
#[async_method]
pub(crate) async fn get_type_annotation_bytes_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // 'this' is the Field object
    let field_obj = parameters.pop()?;

    let (class_name, slot) = {
        let field_ref = field_obj.as_object_ref()?;
        let declaring_class_value = field_ref.value("clazz")?;
        let slot_i32 = field_ref.value("slot")?.as_i32()?;
        let slot = usize::try_from(slot_i32)
            .map_err(|_| crate::Error::InternalError(format!("Invalid slot value: {slot_i32}")))?;
        let declaring_class_obj = declaring_class_value.as_object_ref()?;
        let class_name = declaring_class_obj.value("name")?.as_string()?;
        let class_name = class_name.replace('.', "/");
        (class_name, slot)
    };

    let declaring_class = thread.class(&class_name).await?;

    // Get the field name from the slot (offset)
    let field_name = declaring_class.field_name(slot)?;

    // Find the field in declared_fields
    let field = declaring_class
        .declared_fields()
        .into_iter()
        .find(|f| f.name() == field_name)
        .ok_or_else(|| {
            crate::Error::InternalError(format!("Field {field_name} not found in {class_name}"))
        })?;

    // Collect all type annotations from both visible and invisible attributes
    let mut all_type_annotations = Vec::new();

    for attribute in field.attributes() {
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

    #[tokio::test]
    async fn test_get_type_annotation_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_type_annotation_bytes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

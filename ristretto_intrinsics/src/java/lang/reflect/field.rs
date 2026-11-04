use crate::java::lang::class::get_class_no_init;
use byteorder::{BigEndian, WriteBytesExt};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/reflect/Field.getTypeAnnotationBytes0()[B", Any)]
pub async fn get_type_annotation_bytes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // 'this' is the Field object
    let field_obj = parameters.pop()?;

    let (declaring_class_value, slot) = {
        let field_ref = field_obj.as_object_ref()?;
        let declaring_class_value = field_ref.value("clazz")?;
        let slot_i32 = field_ref.value("slot")?.as_i32()?;
        let slot = usize::try_from(slot_i32).map_err(|_| {
            ristretto_types::Error::InternalError(format!("Invalid slot value: {slot_i32}"))
        })?;
        (declaring_class_value, slot)
    };

    let declaring_class = get_class_no_init(&thread, &declaring_class_value).await?;

    // Get the field name from the slot (offset)
    let field_name = declaring_class.field_name(slot)?;

    // Find the field in declared_fields
    let field = declaring_class
        .declared_fields()
        .into_iter()
        .find(|f| f.name() == field_name)
        .ok_or_else(|| {
            ristretto_types::Error::InternalError(
                [
                    "Field ",
                    &field_name,
                    " not found in ",
                    declaring_class.name(),
                ]
                .concat(),
            )
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
    let result = Reference::from(bytes);
    Ok(Some(Value::new_object(
        thread.vm()?.garbage_collector(),
        result,
    )))
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

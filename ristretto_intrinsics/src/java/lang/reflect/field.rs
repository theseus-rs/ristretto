use crate::java::lang::class::get_class_no_init;
use byteorder::{BigEndian, WriteBytesExt};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Class, Field, Reference, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Recover the declaration identified by java.lang.reflect.Field.slot.
pub(crate) async fn reflected_field<T: Thread + 'static>(
    thread: &T,
    mirror: &Value,
) -> Result<(Arc<Class>, Arc<Field>)> {
    let (owner, slot) = {
        let mirror = mirror.as_object_ref()?;
        (mirror.value("clazz")?, mirror.value("slot")?.as_i32()?)
    };
    let class = get_class_no_init(thread, &owner).await?;
    let field = class
        .declared_fields()
        .into_iter()
        .find(|field| i32::from(field.offset()) == slot)
        .ok_or_else(|| {
            ristretto_types::Error::InternalError(format!(
                "Invalid field slot {slot} in {}",
                class.name()
            ))
        })?;
    Ok((class, field))
}

#[intrinsic_method("java/lang/reflect/Field.getTypeAnnotationBytes0()[B", Any)]
pub async fn get_type_annotation_bytes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // 'this' is the Field object
    let field_obj = parameters.pop()?;

    let (_, field) = reflected_field(&*thread, &field_obj).await?;

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

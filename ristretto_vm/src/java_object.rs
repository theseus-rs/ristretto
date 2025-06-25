use crate::Error::InternalError;
use crate::Result;
use crate::thread::Thread;
use ristretto_classfile::{JAVA_8, JAVA_17};
use ristretto_classloader::{Class, ConcurrentVec, Object, Reference, Value};
use std::sync::Arc;

/// Trait for converting a Rust value to a Java object.  Converts to objects of the primitive
/// wrapper, classes, and strings.
pub trait JavaObject {
    #[expect(async_fn_in_trait)]
    async fn to_object(&self, thread: &Thread) -> Result<Value>;
}

impl JavaObject for bool {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Boolean",
                "valueOf(Z)Ljava/lang/Boolean;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for char {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Character",
                "valueOf(C)Ljava/lang/Character;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for i8 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Byte", "valueOf(B)Ljava/lang/Byte;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u8 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i8;
        value.to_object(thread).await
    }
}

impl JavaObject for i16 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Short", "valueOf(S)Ljava/lang/Short;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u16 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i16;
        value.to_object(thread).await
    }
}

impl JavaObject for i32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Integer",
                "valueOf(I)Ljava/lang/Integer;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for u32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i32;
        value.to_object(thread).await
    }
}

impl JavaObject for i64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Long", "valueOf(J)Ljava/lang/Long;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i64;
        value.to_object(thread).await
    }
}

impl JavaObject for isize {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = *self as i64;
        value.to_object(thread).await
    }
}

impl JavaObject for usize {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = *self as u64;
        value.to_object(thread).await
    }
}

impl JavaObject for f32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Float", "valueOf(F)Ljava/lang/Float;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for f64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Double", "valueOf(D)Ljava/lang/Double;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for &str {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let class = thread.class("java.lang.String").await?;
        let object = Object::new(class)?;

        let vm = thread.vm()?;
        let java_class_file_version = vm.java_class_file_version();
        let array = if java_class_file_version <= &JAVA_8 {
            // Java 8 and below: store as UTF-16 char array
            let chars = self.encode_utf16().collect::<Vec<u16>>();
            Reference::CharArray(ConcurrentVec::from(chars))
        } else {
            if java_class_file_version >= &JAVA_17 {
                object.set_value("hashIsZero", Value::Int(0))?;
            }

            // Determine coder and value
            let use_latin1 = self.chars().all(|c| (c as u32) <= 0xFF);
            let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                // All chars fit in Latin1
                (0, self.chars().map(|c| c as i8).collect())
            } else {
                // Must use UTF-16
                (
                    1,
                    self.encode_utf16()
                        .flat_map(|u| u.to_be_bytes())
                        .map(|b| b as i8)
                        .collect(),
                )
            };

            object.set_value("coder", Value::Int(coder))?;
            Reference::from(bytes)
        };

        object.set_value("value", Value::Object(Some(array)))?;
        object.set_value("hash", Value::Int(0))?;

        let value = Value::from(object);
        Ok(value)
    }
}

impl JavaObject for String {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = self.as_str();
        value.to_object(thread).await
    }
}

async fn to_class_object(thread: &Thread, class: &Arc<Class>) -> Result<Value> {
    if let Some(object) = class.object()? {
        return Ok(Value::from(object));
    }

    let java_lang_class = thread.class("java.lang.Class").await?;
    let object = Object::new(java_lang_class)?;
    let class_name = class.name().replace('/', ".");
    let name = class_name.to_object(thread).await?;
    object.set_value("name", name)?;
    // TODO: a "null" class loader indicates a system class loader; this should be re-evaluated
    // to support custom class loaders
    let class_loader_field = object.field("classLoader")?;
    class_loader_field.unsafe_set_value(Value::Object(None))?;
    class.set_object(Some(object.clone()))?;
    let value = Value::from(object);
    Ok(value)
}

impl JavaObject for Arc<Class> {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let class_object = to_class_object(thread, self).await?;
        let Value::Object(Some(Reference::Object(ref object))) = class_object else {
            return Err(InternalError("Expected class object".to_string()));
        };

        let vm = thread.vm()?;
        if *vm.java_class_file_version() > JAVA_8 && self.is_array() {
            let Some(component_type) = self.component_type() else {
                return Err(InternalError(
                    "array class missing component type".to_string(),
                ));
            };
            let component_type_class = thread.class(component_type).await?;
            let component_type_object = to_class_object(thread, &component_type_class).await?;
            object.set_value("componentType", component_type_object)?;
        }

        Ok(class_object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bool_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = true;
        let value: Value = original_value.to_object(&thread).await?;
        let value: bool = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_char_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = '*';
        let value: Value = original_value.to_object(&thread).await?;
        let value: char = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i8_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i8;
        let value: Value = original_value.to_object(&thread).await?;
        let value: i8 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u8_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u8;
        let value: Value = original_value.to_object(&thread).await?;
        let value: u8 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i16_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i16;
        let value: Value = original_value.to_object(&thread).await?;
        let value: i16 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u16_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u16;
        let value: Value = original_value.to_object(&thread).await?;
        let value: u16 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i32;
        let value: Value = original_value.to_object(&thread).await?;
        let value: i32 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u32;
        let value: Value = original_value.to_object(&thread).await?;
        let value: u32 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i64;
        let value: Value = original_value.to_object(&thread).await?;
        let value: i64 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u64;
        let value: Value = original_value.to_object(&thread).await?;
        let value: u64 = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_isize_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42isize;
        let value: Value = original_value.to_object(&thread).await?;
        let value: isize = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_usize_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42usize;
        let value: Value = original_value.to_object(&thread).await?;
        let value: usize = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_f32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42.1f32;
        let value: Value = original_value.to_object(&thread).await?;
        let value: f32 = value.try_into()?;
        let value = value - original_value;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_f64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42.1f64;
        let value: Value = original_value.to_object(&thread).await?;
        let value: f64 = value.try_into()?;
        let value = value - original_value;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_str_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = "foo";
        let value: Value = original_value.to_object(&thread).await?;
        let value: String = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = "foo".to_string();
        let value: Value = original_value.to_object(&thread).await?;
        let value: String = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_class_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = Class::new_named("[I")?;
        let value: Value = original_value.to_object(&thread).await?;
        let value: Arc<Class> = value.try_into()?;
        assert_eq!("java/lang/Class", value.name());
        Ok(())
    }
}

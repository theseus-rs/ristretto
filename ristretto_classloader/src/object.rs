use crate::Error::{FieldNotFound, InvalidValueType, ParseError};
use crate::Reference::{ByteArray, CharArray};
use crate::{Class, Field, Result, Value};
use ristretto_classfile::{mutf8, FieldAccessFlags, Version};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Represents an object in the Ristretto VM.
#[derive(Clone, PartialEq)]
pub struct Object {
    class: Arc<Class>,
    fields: HashMap<String, Field>,
}

impl Object {
    /// Create a new object with the given class.
    ///
    /// # Errors
    /// if the fields of the class cannot be read.
    pub fn new(class: Arc<Class>) -> Result<Self> {
        let mut fields = HashMap::new();
        let mut current_class = Some(class.clone());
        while let Some(class) = current_class {
            let class_file = class.class_file();
            for class_file_field in &class_file.fields {
                if class_file_field
                    .access_flags
                    .contains(FieldAccessFlags::STATIC)
                {
                    continue;
                }

                let field = Field::from(class_file, class_file_field)?;
                if !fields.contains_key(field.name()) {
                    fields.insert(field.name().to_string(), field);
                }
            }

            current_class = class.parent()?;
        }
        Ok(Self { class, fields })
    }

    /// Get the class.
    #[must_use]
    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    /// Check if the object is an instance of the given class.
    ///
    /// # Errors
    /// if the parent class cannot be read.
    pub fn instance_of(&self, class: &Arc<Class>) -> Result<bool> {
        class.is_assignable_from(&self.class)
    }

    /// Get field by name.
    ///
    /// # Errors
    /// if the field cannot be found.
    pub fn field<S: AsRef<str>>(&self, name: S) -> Result<&Field> {
        let name = name.as_ref();
        let Some(field) = self.fields.get(name) else {
            return Err(FieldNotFound {
                class_name: self.class.name().to_string(),
                field_name: name.to_string(),
            });
        };
        Ok(field)
    }

    /// Get value for a field.
    ///
    /// # Errors
    /// if the field cannot be found.
    pub fn value<S: AsRef<str>>(&self, name: S) -> Result<Value> {
        let field = self.field(name)?;
        field.value()
    }

    /// Sets value for field.
    ///
    /// # Errors
    /// if the field cannot be found.
    pub fn set_value<S: AsRef<str>>(&self, name: S, value: Value) -> Result<()> {
        let field = self.field(name)?;
        field.set_value(value)
    }

    /// Returns a string value for a java.lang.String object.
    ///
    /// # Errors
    /// if the value is not a string Object
    pub fn as_string(&self) -> Result<String> {
        let class_name = self.class().name();
        if class_name != "java/lang/String" {
            return Err(InvalidValueType(
                "Expected a java.lang.String value".to_string(),
            ));
        }
        let Value::Object(Some(reference)) = self.value("value")? else {
            return Err(InvalidValueType(
                "Expected an object field value".to_string(),
            ));
        };
        let class_file_version = &self.class.class_file().version;
        let value = if *class_file_version <= JAVA_8 {
            let CharArray(bytes) = reference else {
                return Err(InvalidValueType("Expected a byte array value".to_string()));
            };
            let bytes = bytes.to_vec()?;
            String::from_utf16(&bytes).map_err(|error| ParseError(error.to_string()))?
        } else {
            let ByteArray(bytes) = reference else {
                return Err(InvalidValueType("Expected a byte array value".to_string()));
            };
            let bytes = bytes.to_vec()?;
            #[expect(clippy::cast_sign_loss)]
            let bytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
            mutf8::from_bytes(&bytes)?
        };
        Ok(value)
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.class.name())?;
        for (name, field) in &self.fields {
            write!(f, " {name}: {:?}", field.value())?;
        }
        Ok(())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.class.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, ConcurrentVec};

    async fn java8_string_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader("8.432.06.1").await?;
        class_loader.load("java/lang/String").await
    }

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    async fn string_class() -> Result<Arc<Class>> {
        load_class("java/lang/String").await
    }

    #[tokio::test]
    async fn test_new() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let object_class = object.class();
        assert_eq!("java/lang/String", object_class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_instance_of() -> Result<()> {
        let class_name = "java/lang/Object";
        let class = load_class(class_name).await?;
        let object = Object::new(class.clone())?;
        assert!(object.instance_of(&class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_field() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let field = object.field("value");
        assert!(field.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_field_not_found() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let field = object.field("foo");
        assert!(matches!(
            field,
            Err(FieldNotFound { class_name, field_name })
            if class_name == "java/lang/String" && field_name == "foo"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_debug() -> Result<()> {
        let class_name = "java/lang/Object";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        assert_eq!("Object(java/lang/Object)", format!("{object:?}"));
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let class_name = "java/lang/Object";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        assert_eq!("class java/lang/Object", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_invalid_class() -> Result<()> {
        let class = load_class("java/lang/Object").await?;
        let object = Object::new(class)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_invalid_value() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_java8() -> Result<()> {
        let class = java8_string_class().await?;
        let object = Object::new(class)?;
        let string_bytes = "foo"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| u16::from(b))
            .collect();
        let string_value = Value::Object(Some(CharArray(ConcurrentVec::from(string_bytes))));
        object.set_value("value", string_value)?;
        assert_eq!("foo".to_string(), object.as_string()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_java8_invalid_byte_array_value() -> Result<()> {
        let class = java8_string_class().await?;
        let object = Object::new(class)?;
        let string_value = Value::Object(Some(ByteArray(ConcurrentVec::from(vec![]))));
        object.set_value("value", string_value)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_as_string() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let string_bytes = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::Object(Some(ByteArray(ConcurrentVec::from(string_bytes))));
        object.set_value("value", string_value)?;
        assert_eq!("foo".to_string(), object.as_string()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_invalid_char_array_value() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let string_value = Value::Object(Some(CharArray(ConcurrentVec::from(vec![]))));
        object.set_value("value", string_value)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }
}

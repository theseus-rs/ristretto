use crate::Error::{FieldNotFound, InvalidValueType, ParseError};
use crate::Reference::{ByteArray, CharArray};
use crate::{Class, Field, Result, Value};
use ristretto_classfile::{mutf8, FieldAccessFlags, Version};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Represents an object in the Ristretto VM.
#[derive(Clone, Debug, PartialEq)]
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
    pub fn instanceof<S: AsRef<str>>(&self, class_name: S) -> Result<bool> {
        self.class.is_assignable_from(class_name)
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
        let Value::Object(Some(reference)) = self.field("value")?.value()? else {
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

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.class)
    }
}

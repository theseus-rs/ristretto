use crate::Error::{FieldNotFound, InvalidValueType, ParseError};
use crate::Reference::{ByteArray, CharArray};
use crate::{Class, Field, Reference, Result, Value};
use ristretto_classfile::{FieldAccessFlags, Version, mutf8};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Represents an object in the Ristretto VM.
#[derive(Clone)]
pub struct Object {
    class: Arc<Class>,
    fields: Arc<HashMap<String, Field>>,
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
        Ok(Self {
            class,
            fields: Arc::new(fields),
        })
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

    /// Get the fields.
    #[must_use]
    pub fn fields(&self) -> Vec<&Field> {
        self.fields.values().collect()
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

    /// Check if the object is an instance of the given class and return the "value".
    ///
    /// # Errors
    /// if the value is not a string Object
    fn class_value(&self, expected_class_name: &str) -> Result<Value> {
        let class_name = self.class().name();
        if class_name != expected_class_name {
            return Err(InvalidValueType(format!(
                "Expected class {expected_class_name}; found {class_name}"
            )));
        }
        self.value("value")
    }

    /// Recursively compare two `Object` instances for equality and avoid cycles.
    #[expect(clippy::type_complexity)]
    fn equal_with_visited(
        &self,
        other: &Object,
        visited: &mut HashSet<(
            (*const Class, *const HashMap<String, Field>),
            (*const Class, *const HashMap<String, Field>),
        )>,
    ) -> bool {
        // Optimization for the case where the two objects are the same reference.
        if std::ptr::eq(self, other) {
            return true;
        }

        if self.class != other.class {
            return false;
        }

        // Optimization for the case where the two objects are the same but have different references.
        if Arc::ptr_eq(&self.fields, &other.fields) {
            return true;
        }

        let self_ptr = (Arc::as_ptr(&self.class), Arc::as_ptr(&self.fields));
        let other_ptr = (Arc::as_ptr(&other.class), Arc::as_ptr(&other.fields));
        let object_ptr_pair = (self_ptr, other_ptr);
        if visited.contains(&object_ptr_pair) {
            return true;
        }

        visited.insert(object_ptr_pair);

        if self.fields.len() != other.fields.len() {
            return false;
        }

        for (name, field) in self.fields.iter() {
            let Some(other_field) = other.fields.get(name) else {
                return false;
            };
            let (Ok(value), Ok(other_value)) = (field.value(), other_field.value()) else {
                return false;
            };
            match (value, other_value) {
                (
                    Value::Object(Some(Reference::Object(object))),
                    Value::Object(Some(Reference::Object(other_object))),
                ) => {
                    if !object.equal_with_visited(&other_object, visited) {
                        return false;
                    }
                }
                (
                    Value::Object(Some(Reference::Array(class, array))),
                    Value::Object(Some(Reference::Array(other_class, other_array))),
                ) => {
                    if class != other_class {
                        return false;
                    }
                    let array = array.to_vec().unwrap_or_default();
                    let other_array = other_array.to_vec().unwrap_or_default();
                    if array.len() != other_array.len() {
                        return false;
                    }
                    for (element, other_element) in array.iter().zip(other_array.iter()) {
                        match (element, other_element) {
                            (
                                Some(Reference::Object(object)),
                                Some(Reference::Object(other_object)),
                            ) => {
                                if !object.equal_with_visited(other_object, visited) {
                                    return false;
                                }
                            }
                            _ => {
                                if element != other_element {
                                    return false;
                                }
                            }
                        }
                    }
                }
                (value, other_value) => {
                    if value != other_value {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Deep clone the object.
    ///
    /// # Errors
    /// if the fields cannot be cloned.
    pub fn deep_clone(&self) -> Result<Self> {
        let mut fields = HashMap::new();
        for (name, field) in self.fields.iter() {
            let field = field.deep_clone()?;
            fields.insert(name.clone(), field);
        }
        Ok(Self {
            class: self.class.clone(),
            fields: Arc::new(fields),
        })
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.class.name())?;
        if !self.fields.is_empty() {
            writeln!(f)?;
        }

        // Print fields by name to ensure consistent output
        let mut names = self.fields.keys().collect::<Vec<_>>();
        names.sort();
        for name in names {
            let field = self.fields.get(name).ok_or(std::fmt::Error)?;
            let value = field.value().map_err(|_| std::fmt::Error)?;
            writeln!(f, "  {name}={value}")?;
        }

        Ok(())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_name = self.class().name();
        match class_name {
            "java/lang/Boolean" => {
                let object = self.clone();
                let value: bool = object.try_into().unwrap_or_default();
                write!(f, "Boolean({value})")
            }
            "java/lang/Character" => {
                let object = self.clone();
                let value: char = object.try_into().unwrap_or_default();
                write!(f, "Character('{value}')")
            }
            "java/lang/Byte" => {
                let object = self.clone();
                let value: i8 = object.try_into().unwrap_or_default();
                write!(f, "Byte({value})")
            }
            "java/lang/Short" => {
                let object = self.clone();
                let value: i16 = object.try_into().unwrap_or_default();
                write!(f, "Short({value})")
            }
            "java/lang/Integer" => {
                let object = self.clone();
                let value: i32 = object.try_into().unwrap_or_default();
                write!(f, "Integer({value})")
            }
            "java/lang/Long" => {
                let object = self.clone();
                let value: i64 = object.try_into().unwrap_or_default();
                write!(f, "Long({value})")
            }
            "java/lang/Float" => {
                let object = self.clone();
                let value: f32 = object.try_into().unwrap_or_default();
                write!(f, "Float({value})")
            }
            "java/lang/Double" => {
                let object = self.clone();
                let value: f64 = object.try_into().unwrap_or_default();
                write!(f, "Double({value})")
            }
            "java/lang/String" => {
                let object = self.clone();
                let value: String = object.try_into().unwrap_or_default();
                write!(f, "String(\"{value}\")")
            }
            "java/lang/Class" => {
                let object = self.clone();
                let value = object.value("name").unwrap_or(Value::Unused);
                let value: String = value.try_into().unwrap_or_default();
                write!(f, "Class({value})")
            }
            _ => write!(f, "Object(class {class_name})"),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let mut visited = HashSet::new();
        self.equal_with_visited(other, &mut visited)
    }
}

impl TryInto<bool> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<bool> {
        let value = self.class_value("java/lang/Boolean")?;
        let value = value.to_int()?;
        Ok(value != 0)
    }
}

impl TryInto<char> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<char> {
        let value = self.class_value("java/lang/Character")?;
        #[expect(clippy::cast_sign_loss)]
        let value = value.to_int()? as u32;
        let character = char::try_from(value)
            .map_err(|_| InvalidValueType("Invalid character value".to_string()))?;
        Ok(character)
    }
}

impl TryInto<i8> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<i8> {
        let value = self.class_value("java/lang/Byte")?;
        let value = value.to_int()?;
        let value =
            i8::try_from(value).map_err(|_| InvalidValueType("Invalid byte value".to_string()))?;
        Ok(value)
    }
}

impl TryInto<u8> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<u8> {
        let value: i8 = self.try_into()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u8)
    }
}

impl TryInto<i16> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<i16> {
        let value = self.class_value("java/lang/Short")?;
        let value = value.to_int()?;
        let value = i16::try_from(value)
            .map_err(|_| InvalidValueType("Invalid short value".to_string()))?;
        Ok(value)
    }
}

impl TryInto<u16> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<u16> {
        let value: i16 = self.try_into()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u16)
    }
}

impl TryInto<i32> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<i32> {
        let value = self.class_value("java/lang/Integer")?;
        let value = value.to_int()?;
        Ok(value)
    }
}

impl TryInto<u32> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<u32> {
        let value: i32 = self.try_into()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u32)
    }
}

impl TryInto<i64> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<i64> {
        let value = self.class_value("java/lang/Long")?;
        let value = value.to_long()?;
        Ok(value)
    }
}

impl TryInto<u64> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<u64> {
        let value: i64 = self.try_into()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u64)
    }
}

impl TryInto<isize> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<isize> {
        let value: i64 = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        Ok(value as isize)
    }
}

impl TryInto<usize> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<usize> {
        let value: u64 = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        Ok(value as usize)
    }
}

impl TryInto<f32> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<f32> {
        let value = self.class_value("java/lang/Float")?;
        let value = value.to_float()?;
        Ok(value)
    }
}

impl TryInto<f64> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<f64> {
        let value = self.class_value("java/lang/Double")?;
        let value = value.to_double()?;
        Ok(value)
    }
}

impl TryInto<String> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<String> {
        let value = self.class_value("java/lang/String")?;
        let Value::Object(Some(reference)) = value else {
            return Err(InvalidValueType(
                "Expected an object field value".to_string(),
            ));
        };
        match reference {
            ByteArray(bytes) => {
                let bytes = bytes.to_vec()?;
                #[expect(clippy::cast_sign_loss)]
                let bytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
                let value = mutf8::from_bytes(&bytes)?;
                Ok(value)
            }
            CharArray(bytes) => {
                let bytes = bytes.to_vec()?;
                let value =
                    String::from_utf16(&bytes).map_err(|error| ParseError(error.to_string()))?;
                Ok(value)
            }
            _ => {
                let class_file_version = &self.class.class_file().version;
                if *class_file_version <= JAVA_8 {
                    Err(InvalidValueType("Expected a char array value".to_string()))
                } else {
                    Err(InvalidValueType("Expected a byte array value".to_string()))
                }
            }
        }
    }
}

impl TryInto<Arc<Class>> for Object {
    type Error = crate::Error;

    fn try_into(self) -> Result<Arc<Class>> {
        Ok(self.class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime;

    async fn java8_string_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader("8.452.09.1").await?;
        class_loader.load("java.lang.String").await
    }

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    async fn string_class() -> Result<Arc<Class>> {
        load_class("java.lang.String").await
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
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let object = Object::new(class.clone())?;
        assert!(object.instance_of(&class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_fields() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let fields = object.fields();
        assert_eq!(4, fields.len());
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
    async fn test_clone() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let clone = object.clone();
        assert_eq!(object, clone);

        clone.set_value("value", Value::Int(1))?;
        assert_eq!(object, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let clone = object.deep_clone()?;
        assert_eq!(object, clone);

        clone.set_value("value", Value::Int(2))?;
        assert_ne!(object, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_debug() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!(
            "Object(java/lang/Integer)\n  value=int(42)\n",
            format!("{object:?}")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_same_references() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!(object, object);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_different_references() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(42))?;
        let object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_eq!(object1, object2);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_not_equal() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(3))?;
        let object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_ne!(object1, object2);
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_bool() -> Result<()> {
        let class = load_class("java.lang.Boolean").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        assert_eq!("Boolean(true)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_char() -> Result<()> {
        let class = load_class("java.lang.Character").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Character('*')", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_byte() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Byte(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_short() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Short(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_integer() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Integer(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_long() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        assert_eq!("Long(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_float() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        assert_eq!("Float(42.1)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_double() -> Result<()> {
        let class = load_class("java.lang.Double").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        assert_eq!("Double(42.1)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_string() -> Result<()> {
        let class = load_class("java.lang.String").await?;
        let object = Object::new(class)?;
        #[expect(clippy::cast_possible_wrap)]
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        assert_eq!("String(\"foo\")", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_class() -> Result<()> {
        let string_class = load_class("java.lang.String").await?;
        let string_object = Object::new(string_class)?;
        #[expect(clippy::cast_possible_wrap)]
        let string_bytes: Vec<i8> = "java.lang.Integer"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| b as i8)
            .collect();
        let string_value = Value::from(string_bytes);
        string_object.set_value("value", string_value)?;

        let class_name = "java.lang.Class";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("name", Value::from(string_object))?;
        assert_eq!("Class(java.lang.Integer)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        assert_eq!("Object(class java/lang/Object)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_bool() -> Result<()> {
        let class = load_class("java.lang.Boolean").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let value: bool = object.try_into()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char() -> Result<()> {
        let class = load_class("java.lang.Character").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: char = object.try_into()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i8() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: i8 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u8() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: u8 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i16() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: i16 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u16() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: u16 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i32() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: i32 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u32() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: u32 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i64() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value: i64 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u64() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value: u64 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_isize() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value: isize = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_usize() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value: usize = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f32() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let value: f32 = object.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f64() -> Result<()> {
        let class = load_class("java.lang.Double").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let value: f64 = object.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_string_invalid_class() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let result: Result<String> = object.try_into();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_string_invalid_value() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let result: Result<String> = object.try_into();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_string_java8() -> Result<()> {
        let class = java8_string_class().await?;
        let object = Object::new(class)?;
        #[expect(clippy::cast_possible_wrap)]
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_string_java8_invalid_byte_array_value() -> Result<()> {
        let class = java8_string_class().await?;
        let object = Object::new(class)?;
        let string_value = Value::from(Vec::<i32>::new());
        object.set_value("value", string_value)?;
        let result: Result<String> = object.try_into();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_try_into_byte_array_string() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char_array_string() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let string_bytes: Vec<char> = "foo"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| b as char)
            .collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_string_invalid_char_array_value() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        let string_value = Value::from(Vec::<i32>::new());
        object.set_value("value", string_value)?;
        let result: Result<String> = object.try_into();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_class() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: Arc<Class> = object.try_into()?;
        assert_eq!("java/lang/Integer", value.name());
        Ok(())
    }
}

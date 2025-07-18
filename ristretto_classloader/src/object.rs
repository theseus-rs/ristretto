use crate::Error::{FieldNotFound, InvalidValueType, ParseError, PoisonedLock};
use crate::Reference::{ByteArray, CharArray};
use crate::field::FieldKey;
use crate::{Class, Field, Reference, Result, Value};
use ristretto_classfile::JAVA_8;
use ristretto_gc::{GarbageCollector, Gc, Trace};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::sync::{Arc, RwLock};

/// Represents an object in the Ristretto VM.
#[derive(Clone)]
pub struct Object {
    class: Arc<Class>,
    values: Gc<Vec<RwLock<Value>>>,
}

impl Object {
    /// Create a new object with the given class.
    ///
    /// # Errors
    ///
    /// if the fields of the class cannot be read.
    pub fn new(class: Arc<Class>) -> Result<Self> {
        let object_fields = class.all_object_fields()?;
        let mut values = Vec::with_capacity(object_fields.len());
        for field in object_fields {
            let value = field.default_value();
            values.push(RwLock::new(value));
        }

        Ok(Self {
            class,
            values: Gc::new(values),
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
    ///
    /// if the parent class cannot be read.
    pub fn instance_of(&self, class: &Arc<Class>) -> Result<bool> {
        class.is_assignable_from(&self.class)
    }

    /// Get field and value lock by key.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    fn field_value<K: FieldKey>(
        &self,
        accessing_class: &Class,
        key: K,
    ) -> Result<(Arc<Field>, &RwLock<Value>)> {
        // TODO: Optimize this function to avoid the field resolution for every access.
        let object_fields = self.class.all_object_fields()?;

        // Fast path for numeric keys (direct index access)
        if key.is_numeric_key() {
            if let Some((index, field)) = key.get_field(&object_fields)
                && let Some(value_lock) = self.values.get(index)
            {
                return Ok((field.clone(), value_lock));
            }
            return Err(FieldNotFound {
                class_name: accessing_class.name().to_string(),
                field_name: key.to_string(),
            });
        }

        // For string keys, we need to handle field shadowing correctly. In Java, field access is
        // resolved based on the static type of the reference, not the dynamic type

        // If accessing through the object's actual class, get the most derived field
        if accessing_class.name() == self.class.name() {
            // Find the last (most derived) field that matches the key
            for (index, field) in object_fields.iter().enumerate().rev() {
                if key.matches_field(field)
                    && let Some(value_lock) = self.values.get(index)
                {
                    return Ok((field.clone(), value_lock));
                }
            }
        } else {
            // Accessing through a parent class reference. We need to find the field that would be
            // visible from that class level

            // Build the class hierarchy to understand which field belongs to which class
            let mut class_hierarchy = Vec::new();
            let mut current_class = Some(self.class.clone());

            // Build hierarchy from most derived to root
            while let Some(class) = current_class {
                class_hierarchy.push(class.clone());
                current_class = class.parent()?;
            }

            // Find the accessing class in the hierarchy
            let accessing_class_index = class_hierarchy
                .iter()
                .position(|class| class.name() == accessing_class.name());

            if let Some(accessing_index) = accessing_class_index {
                // Look for the field starting from the accessing class and going up to more derived
                // classes. This handles field shadowing: if a field exists in the accessing class,
                // use that one. If not, use the most derived version available

                let mut field_offset = 0;
                let mut found_field = None;

                // Calculate field offset for classes above the accessing class (towards root)
                for class in class_hierarchy.iter().skip(accessing_index + 1) {
                    field_offset += class.object_fields().len();
                }

                // First, check if the accessing class itself has the field (for shadowing)
                let accessing_class_fields = class_hierarchy[accessing_index].object_fields();
                for (local_index, field) in accessing_class_fields.iter().enumerate() {
                    if key.matches_field(field) {
                        let global_index = field_offset + local_index;
                        if let Some(value_lock) = self.values.get(global_index) {
                            return Ok((field.clone(), value_lock));
                        }
                    }
                }

                // If not found in accessing class, look in more derived classes (towards the
                // object's actual class)
                let mut current_offset = 0;
                for i in (0..accessing_index).rev() {
                    let current_class_fields = class_hierarchy[i].object_fields();
                    for (local_index, field) in current_class_fields.iter().enumerate() {
                        if key.matches_field(field) {
                            let global_index = field_offset
                                + accessing_class_fields.len()
                                + current_offset
                                + local_index;
                            if let Some(value_lock) = self.values.get(global_index) {
                                found_field = Some((field.clone(), value_lock));
                            }
                        }
                    }
                    current_offset += current_class_fields.len();
                }

                if let Some((field, value_lock)) = found_field {
                    return Ok((field, value_lock));
                }
            }

            // If accessing class is not in the hierarchy, fall back to searching all fields. This
            // handles cases where we're accessing through an interface or other reference type
            for (index, field) in object_fields.iter().enumerate() {
                if key.matches_field(field)
                    && let Some(value_lock) = self.values.get(index)
                {
                    return Ok((field.clone(), value_lock));
                }
            }
        }

        Err(FieldNotFound {
            class_name: accessing_class.name().to_string(),
            field_name: key.to_string(),
        })
    }

    /// Get value for a field in the class.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn value_in_class<K: FieldKey>(&self, class: &Class, key: K) -> Result<Value> {
        let (_field, value_lock) = self.field_value(class, key)?;
        let value_guard = value_lock
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(value_guard.clone())
    }

    /// Get value for a field.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn value<K: FieldKey>(&self, key: K) -> Result<Value> {
        self.value_in_class(&self.class, key)
    }

    /// Sets the value for a field in the class.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn set_value_in_class<S: FieldKey>(
        &self,
        class: &Class,
        key: S,
        value: Value,
    ) -> Result<()> {
        let (field, value_lock) = self.field_value(class, key)?;
        field.check_value(&value)?;
        let mut value_guard = value_lock
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *value_guard = value;
        Ok(())
    }

    /// Sets value for field.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn set_value<S: FieldKey>(&self, key: S, value: Value) -> Result<()> {
        self.set_value_in_class(&self.class, key, value)
    }

    /// Sets value for field without checking the field constraints.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn set_value_unchecked<S: FieldKey>(&self, key: S, value: Value) -> Result<()> {
        let (_field, value_lock) = self.field_value(&self.class, key)?;
        let mut value_guard = value_lock
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *value_guard = value;
        Ok(())
    }

    /// Check if the object is an instance of the given class and return the "value".
    ///
    /// # Errors
    ///
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
    pub(crate) fn equal_with_visited(
        &self,
        other: &Object,
        visited: &mut HashSet<(
            (*const Class, *const Vec<RwLock<Value>>),
            (*const Class, *const Vec<RwLock<Value>>),
        )>,
    ) -> bool {
        // Optimization for the case where the two objects are the same reference.
        if std::ptr::eq(self, other) {
            return true;
        }

        let self_ptr = (Arc::as_ptr(&self.class), Gc::as_ptr(&self.values));
        let other_ptr = (Arc::as_ptr(&other.class), Gc::as_ptr(&other.values));
        let object_ptr_pair = (self_ptr, other_ptr);

        // Check if we've already visited this pair to avoid infinite recursion
        if visited.contains(&object_ptr_pair) {
            return true;
        }

        // Add this pair to visited set before recursive calls
        visited.insert(object_ptr_pair);

        if !Arc::ptr_eq(&self.class, &other.class) {
            return false;
        }
        if self.class.name() != other.class.name() {
            return false;
        }

        // Compare values by iterating over the Vec<RwLock<Value>>
        for (self_value_lock, other_value_lock) in self.values.iter().zip(other.values.iter()) {
            if std::ptr::eq(self_value_lock, other_value_lock) {
                continue;
            }
            let self_value = self_value_lock.read().expect("poisoned lock");
            let other_value = other_value_lock.read().expect("poisoned lock");
            match (&*self_value, &*other_value) {
                (
                    Value::Object(Some(Reference::Object(self_object))),
                    Value::Object(Some(Reference::Object(other_object))),
                ) => {
                    if !self_object.equal_with_visited(other_object, visited) {
                        return false;
                    }
                }
                _ => {
                    if *self_value != *other_value {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Returns hash code implementation based on memory address.
    #[must_use]
    pub fn hash_code(&self) -> usize {
        Gc::as_ptr(&self.values).cast::<Vec<Value>>() as usize
    }

    /// Check if two references point to the same memory location.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.class, &other.class) && Gc::ptr_eq(&self.values, &other.values)
    }

    /// Deep clone the object.
    ///
    /// # Errors
    ///
    /// if the fields cannot be cloned.
    pub fn deep_clone(&self) -> Result<Self> {
        if self.class.name() == "java/lang/Class" {
            // Special case for Class objects, which should not be deep cloned.
            return Ok(self.clone());
        }

        let mut values = Vec::new();
        for value_lock in self.values.iter() {
            let value = value_lock
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            values.push(RwLock::new(value.clone()));
        }
        Ok(Self {
            class: self.class.clone(),
            values: Gc::new(values),
        })
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.class.name())?;
        let object_fields = self
            .class
            .all_object_fields()
            .map_err(|_| std::fmt::Error)?;
        if !object_fields.is_empty() {
            writeln!(f)?;
        }

        // Print fields by name to ensure consistent output
        for (index, field) in object_fields.iter().enumerate() {
            let name = field.name();
            let value_lock = self.values.get(index).ok_or(std::fmt::Error)?;
            let value = value_lock.read().map_err(|_| std::fmt::Error)?;
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

impl Trace for Object {
    fn trace(&self, collector: &GarbageCollector) {
        for value_lock in self.values.iter() {
            if let Ok(value_guard) = value_lock.read()
                && let Value::Object(Some(value)) = &*value_guard
            {
                value.trace(collector);
            }
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
                let coder = self.value("coder")?.to_int()?;
                if coder == 0 {
                    // Latin-1 encoded string
                    let bytes = bytes.to_vec()?;
                    #[expect(clippy::cast_sign_loss)]
                    let value = bytes.iter().map(|&byte| char::from(byte as u8)).collect();
                    Ok(value)
                } else {
                    // UTF-16 encoded string
                    let bytes = bytes.to_vec()?;
                    #[expect(clippy::cast_sign_loss)]
                    let code_units = bytes
                        .chunks(2)
                        .map(|chunk| u16::from_be_bytes([chunk[0] as u8, chunk[1] as u8]))
                        .collect::<Vec<u16>>();
                    let value = String::from_utf16(&code_units)
                        .map_err(|error| ParseError(error.to_string()))?;
                    Ok(value)
                }
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
            runtime::version_class_loader("8.462.08.1").await?;
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
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class.clone())?;
        assert!(object.instance_of(&class)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_clone() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let clone = object.clone();
        assert_eq!(object, clone);

        clone.set_value("value", Value::Int(1))?;
        assert_eq!(object, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_code() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object1 = Object::new(class.clone())?;
        let object2 = Object::new(class)?;
        assert_ne!(0, object1.hash_code());
        assert_ne!(object1.hash_code(), object2.hash_code());
        Ok(())
    }

    #[tokio::test]
    async fn test_trace() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let collector = GarbageCollector::new();
        object.trace(&collector);
        Ok(())
    }

    #[tokio::test]
    async fn test_ptr_eq() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object1 = Object::new(class.clone())?;
        let object2 = Object::new(class)?;
        let object3 = object1.clone();
        assert!(object1.ptr_eq(&object1));
        assert!(!object1.ptr_eq(&object2));
        assert!(object1.ptr_eq(&object3));
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let clone = object.deep_clone()?;
        assert_eq!(object, clone);
        assert!(!object.ptr_eq(&clone));

        clone.set_value("value", Value::Int(2))?;
        assert_ne!(object, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_class() -> Result<()> {
        let class = load_class("java.lang.Class").await?;
        let object = Object::new(class)?;
        let clone = object.deep_clone()?;
        assert_eq!(object, clone);
        assert!(object.ptr_eq(&clone));
        Ok(())
    }

    #[tokio::test]
    async fn test_debug() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
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
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!(object, object);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_different_references() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(42))?;
        let object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_eq!(object1, object2);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_not_equal() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
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

        let class = load_class("java.lang.Class").await?;
        let object = Object::new(class)?;
        object.set_value("name", Value::from(string_object))?;
        assert_eq!("Class(java.lang.Integer)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
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
        let string_chars: Vec<char> = "foo"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| b as char)
            .collect();
        let string_value = Value::from(string_chars);
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
    async fn test_try_into_latin1_byte_array_string() -> Result<()> {
        let class = string_class().await?;
        let object = Object::new(class)?;
        object.set_value("coder", Value::Int(0))?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_try_into_utf16_byte_array_string() -> Result<()> {
        let value = "😃";
        let class = string_class().await?;
        let object = Object::new(class)?;
        object.set_value("coder", Value::Int(1))?;
        let string_bytes: Vec<i8> = value
            .encode_utf16()
            .flat_map(u16::to_be_bytes)
            .map(|b| b as i8)
            .collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!(value.to_string(), result);
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
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value: Arc<Class> = object.try_into()?;
        assert_eq!("java/lang/Integer", value.name());
        Ok(())
    }
}

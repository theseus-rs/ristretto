use crate::Error::{FieldNotFound, InvalidValueType, ParseError};
use crate::Reference::{ByteArray, CharArray};
use crate::field::FieldKey;
use crate::{Class, Field, Reference, Result, Value};
use ahash::AHashSet;
use ristretto_classfile::JAVA_8;
use ristretto_gc::{GarbageCollector, Trace};
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Represents an object in the Ristretto VM.
///
/// # Instance Field Initialization (JLS §12.5)
///
/// Objects are created through two distinct phases:
///
/// ## 1. Allocation Phase (`Object::new`)
///
/// When an object is allocated:
/// - Memory is allocated for all instance fields in the class hierarchy
/// - All fields are set to their **default zero values** (0, 0.0, false, null)
/// - This happens BEFORE any constructor code runs
///
/// ## 2. Initialization Phase (`<init>` constructor)
///
/// Constructors execute in this order:
/// 1. Call to `super(...)` (recursively up to `Object.<init>`)
/// 2. Instance field initializers (in textual order of field declarations)
/// 3. Instance initializer blocks (in textual order)
/// 4. Constructor body statements
///
/// ## Field Shadowing
///
/// Each class initializes only its own declared fields:
/// ```java
/// class A { int x = 1; }
/// class B extends A { int x = 2; }
/// ```
/// - `A.x` is initialized in `A.<init>`, occupies one slot
/// - `B.x` is initialized in `B.<init>`, occupies a different slot
///
/// ## Separation from Static Fields
///
/// Instance field initialization is **completely separate** from static field initialization:
/// - Static fields: initialized during class initialization (`<clinit>`)
/// - Instance fields: initialized during object construction (`<init>`)
///
/// ## Unsafe.allocateInstance
///
/// When using `Unsafe.allocateInstance()`:
/// - Memory is allocated and zeroed (via `Object::new`)
/// - NO constructor (`<init>`) is called
/// - Field initializers are NOT run
#[derive(Clone)]
pub struct Object {
    class: Arc<Class>,
    /// Field values for all instance fields in the class hierarchy.
    /// Fields are stored in order: root class fields first, then subclass fields.
    /// Initially set to default zero values; actual initialization happens in constructors.
    values: Box<[Value]>,
}

impl Object {
    /// Create a new object with the given class.
    ///
    /// This performs the **allocation phase** of object creation:
    /// - Allocates storage for all instance fields in the class hierarchy
    /// - Sets all fields to their **default zero values** (0, 0.0, false, null)
    ///
    /// **This does NOT call any constructor (`<init>`).**
    ///
    /// # JVM Specification Reference
    ///
    /// Per [JLS §12.5](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.5):
    /// > Just before a reference to the newly created object is returned as the result,
    /// > the indicated constructor is processed to initialize the new object...
    ///
    /// The constructor invocation is handled separately by the VM's `invokespecial` instruction.
    ///
    /// # Zero Initialization
    ///
    /// All fields are initialized to their type's default value:
    /// - `int`, `short`, `byte`, `char`: `0`
    /// - `long`: `0L`
    /// - `float`: `0.0f`
    /// - `double`: `0.0d`
    /// - `boolean`: `false`
    /// - Object references and arrays: `null`
    ///
    /// # Errors
    ///
    /// if the fields of the class cannot be read.
    pub fn new(class: Arc<Class>) -> Result<Self> {
        let object_fields = class.all_object_fields()?;
        let values = object_fields
            .iter()
            .map(|field| field.default_value())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self { class, values })
    }

    /// Get the class.
    #[must_use]
    pub fn class(&self) -> &Arc<Class> {
        &self.class
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
    ) -> Result<(usize, Arc<Field>, &Value)> {
        // TODO: Optimize this function to avoid the field resolution for every access.
        let object_fields = self.class.all_object_fields()?;

        // Fast path for numeric keys (direct index access)
        if key.is_numeric_key() {
            if let Some((index, field)) = key.get_field(&object_fields)
                && let Some(value) = self.values.get(index)
            {
                return Ok((index, field.clone(), value));
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
                    && let Some(value) = self.values.get(index)
                {
                    return Ok((index, field.clone(), value));
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

                // Calculate field offset for classes above the accessing class (towards root)
                for class in class_hierarchy.iter().skip(accessing_index + 1) {
                    field_offset += class.object_fields().len();
                }

                // First, check if the accessing class itself has the field (for shadowing)
                let accessing_class_fields = class_hierarchy[accessing_index].object_fields();
                for (local_index, field) in accessing_class_fields.iter().enumerate() {
                    if key.matches_field(field) {
                        let global_index = field_offset + local_index;
                        if let Some(value) = self.values.get(global_index) {
                            return Ok((global_index, field.clone(), value));
                        }
                    }
                }

                // If not found in accessing class, look in more derived classes (towards the
                // object's actual class)
                let mut current_offset = field_offset + accessing_class_fields.len();
                for i in (0..accessing_index).rev() {
                    let current_class_fields = class_hierarchy[i].object_fields();
                    for (local_index, field) in current_class_fields.iter().enumerate() {
                        if key.matches_field(field) {
                            let global_index = current_offset + local_index;
                            if let Some(value) = self.values.get(global_index) {
                                return Ok((global_index, field.clone(), value));
                            }
                        }
                    }
                    current_offset += current_class_fields.len();
                }
            }

            // If accessing class is not in the hierarchy, fall back to searching all fields. This
            // handles cases where we're accessing through an interface or other reference type
            for (index, field) in object_fields.iter().enumerate() {
                if key.matches_field(field)
                    && let Some(value) = self.values.get(index)
                {
                    return Ok((index, field.clone(), value));
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
        let (_index, _field, value) = self.field_value(class, key)?;
        Ok(value.clone())
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
        &mut self,
        class: &Class,
        key: S,
        value: Value,
    ) -> Result<()> {
        let (index, field, _value) = self.field_value(class, key)?;
        field.check_value(&value)?;
        if let Some(element) = self.values.get_mut(index) {
            *element = value;
            Ok(())
        } else {
            Err(FieldNotFound {
                class_name: class.name().to_string(),
                field_name: key.to_string(),
            })
        }
    }

    /// Sets value for field.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn set_value<S: FieldKey>(&mut self, key: S, value: Value) -> Result<()> {
        let class = self.class().clone();
        self.set_value_in_class(&class, key, value)
    }

    /// Sets value for field without checking the field constraints.
    ///
    /// # Errors
    ///
    /// if the field cannot be found.
    pub fn set_value_unchecked<S: FieldKey>(&mut self, key: S, value: Value) -> Result<()> {
        let (index, _field, _value) = self.field_value(&self.class, key)?;
        if let Some(element) = self.values.get_mut(index) {
            *element = value;
            Ok(())
        } else {
            let class = self.class();
            Err(FieldNotFound {
                class_name: class.name().to_string(),
                field_name: key.to_string(),
            })
        }
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
        visited: &mut AHashSet<((*const Class, *const Value), (*const Class, *const Value))>,
    ) -> bool {
        // Optimization for the case where the two objects are the same reference.
        if std::ptr::eq(self, other) {
            return true;
        }

        let self_ptr = (Arc::as_ptr(&self.class), self.values.as_ptr());
        let other_ptr = (Arc::as_ptr(&other.class), other.values.as_ptr());
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

        // Compare values by iterating over the Box<[Value]>
        for (self_value, other_value) in self.values.iter().zip(other.values.iter()) {
            if std::ptr::eq(self_value, other_value) {
                continue;
            }
            match (self_value, other_value) {
                (Value::Object(Some(self_ref)), Value::Object(Some(other_ref))) => {
                    let self_ref = self_ref.read();
                    let other_ref = other_ref.read();
                    match (&*self_ref, &*other_ref) {
                        (Reference::Object(self_object), Reference::Object(other_object)) => {
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
                _ => {
                    if *self_value != *other_value {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Check if two references point to the same memory location.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }

    /// Convert the object to a bool value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Boolean` or if the value cannot be converted
    /// to a boolean.
    pub fn as_bool(&self) -> Result<bool> {
        let value = self.class_value("java/lang/Boolean")?;
        let value = value.as_i32()?;
        Ok(value != 0)
    }

    /// Convert the object to a character value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Character` or if the value cannot be
    /// converted to a character.
    pub fn as_char(&self) -> Result<char> {
        let value = self.class_value("java/lang/Character")?;
        let value = value.as_i32()?;
        #[expect(clippy::cast_sign_loss)]
        let value = value as u32;
        let character = char::try_from(value)
            .map_err(|_| InvalidValueType("Invalid character value".to_string()))?;
        Ok(character)
    }

    /// Convert the object to a signed byte value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Byte` or if the value cannot be converted to
    /// a signed byte.
    pub fn as_i8(&self) -> Result<i8> {
        let value = self.class_value("java/lang/Byte")?;
        let value = value.as_i32()?;
        let value =
            i8::try_from(value).map_err(|_| InvalidValueType("Invalid byte value".to_string()))?;
        Ok(value)
    }

    /// Convert the object to an unsigned byte value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Byte` or if the value cannot be converted to
    /// an unsigned byte.
    pub fn as_u8(&self) -> Result<u8> {
        let value = self.as_i8()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u8)
    }

    /// Convert the object to a signed short value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Short` or if the value cannot be converted to
    /// a signed short.
    pub fn as_i16(&self) -> Result<i16> {
        let value = self.class_value("java/lang/Short")?;
        let value = value.as_i32()?;
        let value = i16::try_from(value)
            .map_err(|_| InvalidValueType("Invalid short value".to_string()))?;
        Ok(value)
    }

    /// Convert the object to an unsigned short value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Short` or if the value cannot be converted to
    /// an unsigned short.
    pub fn as_u16(&self) -> Result<u16> {
        let value = self.as_i16()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u16)
    }

    /// Convert the object to a signed integer value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Integer` or if the value cannot be converted
    /// to a signed integer.
    pub fn as_i32(&self) -> Result<i32> {
        let value = self.class_value("java/lang/Integer")?;
        let value = value.as_i32()?;
        Ok(value)
    }

    /// Convert the object to an unsigned integer value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Integer` or if the value cannot be converted
    /// to an unsigned integer.
    pub fn as_u32(&self) -> Result<u32> {
        let value = self.as_i32()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u32)
    }

    /// Convert the object to a signed long value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Long` or if the value cannot be converted to
    /// a signed long.
    pub fn as_i64(&self) -> Result<i64> {
        let value = self.class_value("java/lang/Long")?;
        let value = value.as_i64()?;
        Ok(value)
    }

    /// Convert the object to an unsigned long value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Long` or if the value cannot be converted to
    /// an unsigned long.
    pub fn as_u64(&self) -> Result<u64> {
        let value = self.as_i64()?;
        #[expect(clippy::cast_sign_loss)]
        Ok(value as u64)
    }

    /// Convert the object to a signed isize value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Long` or if the value cannot be converted to
    /// a signed isize.
    pub fn as_isize(&self) -> Result<isize> {
        let value = self.as_i64()?;
        #[expect(clippy::cast_possible_truncation)]
        Ok(value as isize)
    }

    /// Convert the object to an unsigned usize value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Long` or if the value cannot be converted to
    /// an unsigned usize.
    pub fn as_usize(&self) -> Result<usize> {
        let value = self.as_u64()?;
        #[expect(clippy::cast_possible_truncation)]
        Ok(value as usize)
    }

    /// Convert the object to a floating-point value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Float` or if the value cannot be converted to
    /// a floating-point value.
    pub fn as_f32(&self) -> Result<f32> {
        let value = self.class_value("java/lang/Float")?;
        let value = value.as_f32()?;
        Ok(value)
    }

    /// Convert the object to a double-precision floating-point value.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/Double` or if the value cannot be converted
    /// to a double-precision floating-point value.
    pub fn as_f64(&self) -> Result<f64> {
        let value = self.class_value("java/lang/Double")?;
        let value = value.as_f64()?;
        Ok(value)
    }

    /// Convert a Java string object to a `String`.
    ///
    /// # Errors
    ///
    /// if the object is not an instance of `java/lang/String`, or if the value cannot be converted
    /// to a `String`.
    pub fn as_string(&self) -> Result<String> {
        let value = self.class_value("java/lang/String")?;
        let Value::Object(Some(reference)) = value else {
            return Err(InvalidValueType(
                "Expected an object field value".to_string(),
            ));
        };
        let reference = reference.read();
        match &*reference {
            ByteArray(bytes) => {
                let coder = self.value("coder")?.as_i32()?;
                if coder == 0 {
                    // Latin-1 encoded string
                    #[expect(clippy::cast_sign_loss)]
                    let value = bytes.iter().map(|&byte| char::from(byte as u8)).collect();
                    Ok(value)
                } else {
                    // UTF-16 encoded string
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
                let value =
                    String::from_utf16(bytes).map_err(|error| ParseError(error.to_string()))?;
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
            let value = self.values.get(index).ok_or(std::fmt::Error)?;
            writeln!(f, "  {name}={value}")?;
        }

        Ok(())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_name = self.class().name();
        match class_name {
            "java/lang/Boolean" => match self.as_bool() {
                Ok(value) => write!(f, "Boolean({value})"),
                Err(error) => write!(f, "Boolean({error:?})"),
            },
            "java/lang/Character" => match self.as_char() {
                Ok(value) => write!(f, "Character('{value}')"),
                Err(error) => write!(f, "Character({error:?})"),
            },
            "java/lang/Byte" => match self.as_i8() {
                Ok(value) => write!(f, "Byte({value})"),
                Err(error) => write!(f, "Byte({error:?})"),
            },
            "java/lang/Short" => match self.as_i16() {
                Ok(value) => write!(f, "Short({value})"),
                Err(error) => write!(f, "Short({error:?})"),
            },
            "java/lang/Integer" => match self.as_i32() {
                Ok(value) => write!(f, "Integer({value})"),
                Err(error) => write!(f, "Integer({error:?})"),
            },
            "java/lang/Long" => match self.as_i64() {
                Ok(value) => write!(f, "Long({value})"),
                Err(error) => write!(f, "Long({error:?})"),
            },
            "java/lang/Float" => match self.as_f32() {
                Ok(value) => write!(f, "Float({value})"),
                Err(error) => write!(f, "Float({error:?})"),
            },
            "java/lang/Double" => match self.as_f64() {
                Ok(value) => write!(f, "Double({value})"),
                Err(error) => write!(f, "Double({error:?})"),
            },
            "java/lang/String" => match self.as_string() {
                Ok(value) => write!(f, "String(\"{value}\")"),
                Err(error) => write!(f, "String({error:?})"),
            },
            "java/lang/Class" => {
                let value = match self.value("name") {
                    Ok(value) => value,
                    Err(error) => return write!(f, "Class({error:?})"),
                };
                match value.as_string() {
                    Ok(value) => write!(f, "Class({value})"),
                    Err(error) => write!(f, "Class({error:?})"),
                }
            }
            _ => write!(f, "Object(class {class_name})"),
        }
    }
}

impl Hash for Object {
    /// Hash an `Object` based on its class name and field values.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.class.name().hash(state);
        for value in &self.values {
            value.hash(state);
        }
    }
}

impl Trace for Object {
    fn trace(&self, collector: &GarbageCollector) {
        for value in &self.values {
            if let Value::Object(Some(reference)) = value {
                reference.trace(collector);
            }
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let mut visited = AHashSet::default();
        self.equal_with_visited(other, &mut visited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Reference;
    use crate::runtime;
    use ristretto_gc::GarbageCollector;
    use std::hash::DefaultHasher;

    fn test_ref(reference: impl Into<Reference>) -> Value {
        Value::new_object(&GarbageCollector::new(), reference.into())
    }

    async fn java8_string_class() -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader("8.472.08.1").await?;
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
    async fn test_clone() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let mut clone = object.clone();
        assert_eq!(object, clone);

        clone.set_value("value", Value::Int(1))?;
        assert_eq!(object, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_debug() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!(
            "Object(java/lang/Integer)\n  value=int(42)\n",
            format!("{object:?}")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_hash() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(42))?;
        let mut object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_eq!(object1, object2);
        let mut hasher1 = DefaultHasher::new();
        object1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        object2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
        Ok(())
    }

    #[tokio::test]
    async fn test_ptr_eq() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object1 = Object::new(class.clone())?;
        let object2 = Object::new(class)?;
        assert!(object1.ptr_eq(&object1));
        assert!(!object1.ptr_eq(&object2));
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
    async fn test_eq_same_references() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!(object, object);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_different_references() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(42))?;
        let mut object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_eq!(object1, object2);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_not_equal() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object1 = Object::new(class.clone())?;
        object1.set_value("value", Value::Int(3))?;
        let mut object2 = Object::new(class)?;
        object2.set_value("value", Value::Int(42))?;
        assert_ne!(object1, object2);
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_bool() -> Result<()> {
        let class = load_class("java.lang.Boolean").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        assert_eq!("Boolean(true)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_char() -> Result<()> {
        let class = load_class("java.lang.Character").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Character('*')", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_byte() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Byte(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_short() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Short(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_integer() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        assert_eq!("Integer(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_long() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        assert_eq!("Long(42)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_float() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        assert_eq!("Float(42.1)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_double() -> Result<()> {
        let class = load_class("java.lang.Double").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        assert_eq!("Double(42.1)", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_string() -> Result<()> {
        let class = load_class("java.lang.String").await?;
        let mut object = Object::new(class)?;
        #[expect(clippy::cast_possible_wrap)]
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = test_ref(string_bytes);
        object.set_value("value", string_value)?;
        assert_eq!("String(\"foo\")", object.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_to_string_class() -> Result<()> {
        let string_class = load_class("java.lang.String").await?;
        let mut string_object = Object::new(string_class)?;
        #[expect(clippy::cast_possible_wrap)]
        let string_bytes: Vec<i8> = "java.lang.Integer"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| b as i8)
            .collect();
        let string_value = test_ref(string_bytes);
        string_object.set_value("value", string_value)?;

        let class = load_class("java.lang.Class").await?;
        let mut object = Object::new(class)?;
        object.set_value("name", test_ref(string_object))?;
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
    async fn test_as_bool() -> Result<()> {
        let class = load_class("java.lang.Boolean").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let value = object.as_bool()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_char() -> Result<()> {
        let class = load_class("java.lang.Character").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_char()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i8() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_i8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u8() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_u8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i16() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_i16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u16() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_u16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i32() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u32() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = object.as_u32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i64() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = object.as_i64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u64() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = object.as_u64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_isize() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = object.as_isize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_usize() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = object.as_usize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f32() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let value = object.as_f32()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f64() -> Result<()> {
        let class = load_class("java.lang.Double").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let value = object.as_f64()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_invalid_class() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
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
        let mut object = Object::new(class)?;
        let string_chars: Vec<char> = "foo"
            .as_bytes()
            .to_vec()
            .iter()
            .map(|&b| b as char)
            .collect();
        let string_value = test_ref(string_chars);
        object.set_value("value", string_value)?;
        let result = object.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_java8_invalid_byte_array_value() -> Result<()> {
        let class = java8_string_class().await?;
        let mut object = Object::new(class)?;
        let string_value = test_ref(Vec::<i32>::new());
        object.set_value("value", string_value)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_as_string_latin1_byte_array() -> Result<()> {
        let class = string_class().await?;
        let mut object = Object::new(class)?;
        object.set_value("coder", Value::Int(0))?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = test_ref(string_bytes);
        object.set_value("value", string_value)?;
        let result = object.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_as_string_utf16_byte_array() -> Result<()> {
        let value = "😃";
        let class = string_class().await?;
        let mut object = Object::new(class)?;
        object.set_value("coder", Value::Int(1))?;
        let string_bytes: Vec<i8> = value
            .encode_utf16()
            .flat_map(u16::to_be_bytes)
            .map(|b| b as i8)
            .collect();
        let string_value = test_ref(string_bytes);
        object.set_value("value", string_value)?;
        let result = object.as_string()?;
        assert_eq!(value.to_string(), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string_invalid_char_array_value() -> Result<()> {
        let class = string_class().await?;
        let mut object = Object::new(class)?;
        let string_value = test_ref(Vec::<i32>::new());
        object.set_value("value", string_value)?;
        let result = object.as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }
}

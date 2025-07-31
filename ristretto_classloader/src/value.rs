use crate::Error::InvalidValueType;
use crate::reference::Reference;
use crate::{Class, Object, Result};
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

/// Represents a value in the Ristretto VM.
#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Reference>),
    Unused,
}

impl Value {
    /// Convert the object to a bool value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Boolean`.
    /// - if the value cannot be converted to a boolean.
    pub fn as_bool(&self) -> Result<bool> {
        if let Value::Int(value) = self {
            Ok(*value != 0)
        } else {
            let reference = self.as_reference()?;
            reference.as_bool()
        }
    }

    /// Convert the object to a char value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Character`.
    /// - if the value cannot be converted to a char.
    pub fn as_char(&self) -> Result<char> {
        if let Value::Int(value) = self {
            #[expect(clippy::cast_sign_loss)]
            let value = *value as u32;
            let value =
                char::from_u32(value).ok_or(InvalidValueType("Invalid char value".to_string()))?;
            Ok(value)
        } else {
            let reference = self.as_reference()?;
            reference.as_char()
        }
    }

    /// Convert the object to an `i8` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Byte`.
    /// - if the value cannot be converted to an `i8`.
    pub fn as_i8(&self) -> Result<i8> {
        if let Value::Int(value) = self {
            Ok(i8::try_from(*value)?)
        } else {
            let reference = self.as_reference()?;
            reference.as_i8()
        }
    }

    /// Convert the object to a `u8` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Byte`.
    /// - if the value cannot be converted to a `u8`.
    pub fn as_u8(&self) -> Result<u8> {
        let value = self.as_i8()?;
        Ok(u8::try_from(value)?)
    }

    /// Convert the object to an `i16` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Short`.
    /// - if the value cannot be converted to an `i16`.
    pub fn as_i16(&self) -> Result<i16> {
        if let Value::Int(value) = self {
            Ok(i16::try_from(*value)?)
        } else {
            let reference = self.as_reference()?;
            reference.as_i16()
        }
    }

    /// Convert the object to a `u16` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Short`.
    /// - if the value cannot be converted to a `u16`.
    pub fn as_u16(&self) -> Result<u16> {
        let value = self.as_i16()?;
        Ok(u16::try_from(value)?)
    }

    /// Convert the object to an `i32` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Integer`.
    /// - if the value cannot be converted to an `i32`.
    pub fn as_i32(&self) -> Result<i32> {
        if let Value::Int(value) = self {
            Ok(*value)
        } else {
            let reference = self.as_reference()?;
            reference.as_i32()
        }
    }

    /// Convert the object to a `u32` value.
    ///
    /// # Errors
    ///
    /// - if the value is not an `Int`, and the object is not an instance of `java/lang/Integer`.
    /// - if the value cannot be converted to a `u32`.
    pub fn as_u32(&self) -> Result<u32> {
        let value = self.as_i32()?;
        Ok(u32::try_from(value)?)
    }

    /// Convert the object to an `i64` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Long`, and the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to an `i64`.
    pub fn as_i64(&self) -> Result<i64> {
        if let Value::Long(value) = self {
            Ok(*value)
        } else {
            let reference = self.as_reference()?;
            reference.as_i64()
        }
    }

    /// Convert the object to a `u64` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Long`, and the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to a `u64`.
    pub fn as_u64(&self) -> Result<u64> {
        let value = self.as_i64()?;
        Ok(u64::try_from(value)?)
    }

    /// Convert the object to an `isize` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Long`, and the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to an `isize`.
    pub fn as_isize(&self) -> Result<isize> {
        let value = self.as_i64()?;
        Ok(isize::try_from(value)?)
    }

    /// Convert the object to a `usize` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Long`, and the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to a `usize`.
    pub fn as_usize(&self) -> Result<usize> {
        let value = self.as_u64()?;
        Ok(usize::try_from(value)?)
    }

    /// Convert the object to a `f32` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Float`, and the object is not an instance of `java/lang/Float`.
    /// - if the value cannot be converted to a `f32`.
    pub fn as_f32(&self) -> Result<f32> {
        if let Value::Float(value) = self {
            Ok(*value)
        } else {
            let reference = self.as_reference()?;
            reference.as_f32()
        }
    }

    /// Convert the object to a `f64` value.
    ///
    /// # Errors
    ///
    /// - if the value is not a `Double`, and the object is not an instance of `java/lang/Double`.
    /// - if the value cannot be converted to a `f64`.
    pub fn as_f64(&self) -> Result<f64> {
        if let Value::Double(value) = self {
            Ok(*value)
        } else {
            let reference = self.as_reference()?;
            reference.as_f64()
        }
    }

    /// Returns true if the value is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Object(None))
    }

    /// Returns true if the value is an object.
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(Some(_)))
    }

    /// Returns a reference to the value if it is an object.
    ///
    /// # Errors
    ///
    /// If the value is not an object or if it cannot be converted to a reference.
    pub fn as_reference(&self) -> Result<&Reference> {
        match self {
            Value::Object(Some(reference)) => Ok(reference),
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns true if the value is a category 1 value.
    #[must_use]
    pub fn is_category_1(&self) -> bool {
        !self.is_category_2()
    }

    /// Returns true if the value is a category 2 value.
    #[must_use]
    pub fn is_category_2(&self) -> bool {
        matches!(self, Value::Long(_) | Value::Double(_))
    }

    /// Returns a deep clone of the value.
    ///
    /// # Errors
    ///
    /// if the value cannot be cloned.
    pub fn deep_clone(&self) -> Result<Self> {
        let value = match self {
            Value::Object(Some(reference)) => Value::Object(Some(reference.deep_clone()?)),
            _ => self.clone(),
        };

        Ok(value)
    }

    /// Returns the reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i8>>> {
        let reference = self.as_reference()?;
        reference.as_byte_vec_ref()
    }

    /// Returns a mutable reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i8>>> {
        let reference = self.as_reference()?;
        reference.as_byte_vec_mut()
    }

    /// Returns a reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<u16>>> {
        let reference = self.as_reference()?;
        reference.as_char_vec_ref()
    }

    /// Returns a mutable reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<u16>>> {
        let reference = self.as_reference()?;
        reference.as_char_vec_mut()
    }

    /// Returns a reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i16>>> {
        let reference = self.as_reference()?;
        reference.as_short_vec_ref()
    }

    /// Returns a mutable reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i16>>> {
        let reference = self.as_reference()?;
        reference.as_short_vec_mut()
    }

    /// Returns a reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i32>>> {
        let reference = self.as_reference()?;
        reference.as_int_vec_ref()
    }

    /// Returns a mutable reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i32>>> {
        let reference = self.as_reference()?;
        reference.as_int_vec_mut()
    }

    /// Returns a reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i64>>> {
        let reference = self.as_reference()?;
        reference.as_long_vec_ref()
    }

    /// Returns a mutable reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i64>>> {
        let reference = self.as_reference()?;
        reference.as_long_vec_mut()
    }

    /// Returns a reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<f32>>> {
        let reference = self.as_reference()?;
        reference.as_float_vec_ref()
    }

    /// Returns a mutable reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<f32>>> {
        let reference = self.as_reference()?;
        reference.as_float_vec_mut()
    }

    /// Returns a reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<f64>>> {
        let reference = self.as_reference()?;
        reference.as_double_vec_ref()
    }

    /// Returns a mutable reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<f64>>> {
        let reference = self.as_reference()?;
        reference.as_double_vec_mut()
    }

    /// Returns a reference to `Vec<Option<Reference>>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `Reference::Array`
    #[expect(clippy::type_complexity)]
    pub fn as_class_vec_ref(
        &self,
    ) -> Result<(&Arc<Class>, RwLockReadGuard<'_, Vec<Option<Reference>>>)> {
        let reference = self.as_reference()?;
        reference.as_class_vec_ref()
    }

    /// Returns a mutable reference to `Vec<Option<Reference>>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `Reference::Array`
    #[expect(clippy::type_complexity)]
    pub fn as_class_vec_mut(
        &self,
    ) -> Result<(&Arc<Class>, RwLockWriteGuard<'_, Vec<Option<Reference>>>)> {
        let reference = self.as_reference()?;
        reference.as_class_vec_mut()
    }

    /// Returns a reference to an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_ref(&self) -> Result<RwLockReadGuard<'_, Object>> {
        let reference = self.as_reference()?;
        reference.as_object_ref()
    }

    /// Returns a mutable reference to an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_mut(&self) -> Result<RwLockWriteGuard<'_, Object>> {
        let reference = self.as_reference()?;
        reference.as_object_mut()
    }

    /// Returns a reference to a `String`.
    ///
    /// # Errors
    ///
    /// - if the value is not a `String`, and the object is not an instance of `java/lang/String`.
    /// - if the value cannot be converted to a `String`.
    pub fn as_string(&self) -> Result<String> {
        let reference = self.as_reference()?;
        reference.as_string()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "int({value})"),
            Value::Long(value) => write!(f, "long({value})"),
            Value::Float(value) => write!(f, "float({value})"),
            Value::Double(value) => write!(f, "double({value})"),
            Value::Object(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    write!(f, "Object(null)")
                }
            }
            Value::Unused => write!(f, "unused"),
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    /// Computes a hash for the `Value` instance.  Handles the following cases:
    ///
    /// - `Int` and `Long` values are hashed directly.
    /// - `Float` and `Double` values are hashed using their bit representations to avoid issues
    ///   with floating-point precision.
    /// - `Object` values are hashed by their references, allowing for `None` values to be
    ///   considered equal.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Int(value) => value.hash(state),
            Value::Long(value) => value.hash(state),
            Value::Float(value) => value.to_bits().hash(state),
            Value::Double(value) => value.to_bits().hash(state),
            Value::Object(Some(reference)) => reference.hash(state),
            Value::Object(None) => 0.hash(state),
            Value::Unused => (-1).hash(state),
        }
    }
}

impl PartialEq for Value {
    /// Compares two `Value` instances for equality.  Handles the following cases:
    ///
    /// - `Int` values are compared directly.
    /// - `Long` values are compared directly.
    /// - `Float` and `Double` values are compared using their bit representations to avoid issues
    ///   with floating-point precision.
    /// - `Object` values are compared by their references, allowing for `None` values to be
    ///   considered equal.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Long(a), Value::Long(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a.to_bits() == b.to_bits(),
            (Value::Double(a), Value::Double(b)) => a.to_bits() == b.to_bits(),
            (Value::Object(Some(a)), Value::Object(Some(b))) => a == b,
            (Value::Object(None), Value::Object(None)) | (Value::Unused, Value::Unused) => true,
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Int(value as i32)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Int(value as i32)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Long(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Long(value as i64)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::Long(value as i64)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Long(value as i64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<Vec<bool>> for Value {
    fn from(value: Vec<bool>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<i8>> for Value {
    fn from(value: Vec<i8>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<char>> for Value {
    fn from(value: Vec<char>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<u64>> for Value {
    fn from(value: Vec<u64>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<isize>> for Value {
    fn from(value: Vec<isize>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<usize>> for Value {
    fn from(value: Vec<usize>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<f32>> for Value {
    fn from(value: Vec<f32>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<(Arc<Class>, Vec<Option<Reference>>)> for Value {
    fn from(value: (Arc<Class>, Vec<Option<Reference>>)) -> Self {
        Value::from(Reference::from(value))
    }
}

impl TryFrom<(Arc<Class>, Vec<Value>)> for Value {
    type Error = crate::Error;

    fn try_from(value: (Arc<Class>, Vec<Value>)) -> Result<Self> {
        let reference = Reference::try_from(value)?;
        Ok(Value::Object(Some(reference)))
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Value::from(Reference::from(value))
    }
}

impl From<Reference> for Value {
    fn from(reference: Reference) -> Self {
        Value::from(Some(reference))
    }
}

impl From<Option<Reference>> for Value {
    fn from(reference: Option<Reference>) -> Self {
        Value::Object(reference)
    }
}

impl TryInto<Vec<Value>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<Value>> {
        let reference = self.as_reference()?;
        let (_class, values) = reference.as_class_vec_ref()?;
        let values = values
            .iter()
            .cloned()
            .map(|value| {
                if let Some(value) = value {
                    Value::Object(Some(value))
                } else {
                    Value::Object(None)
                }
            })
            .collect();
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Object, runtime};
    use std::hash::DefaultHasher;
    use std::sync::Arc;

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    #[test]
    fn test_int_format() {
        let value = Value::Int(42);
        assert_eq!("int(42)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
    }

    #[test]
    fn test_long_format() {
        let value = Value::Long(42);
        assert_eq!("long(42)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
    }

    #[test]
    fn test_float_format() {
        let value = Value::Float(42.1);
        assert_eq!("float(42.1)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
    }

    #[test]
    fn test_double_format() {
        let value = Value::Double(42.1);
        assert_eq!("double(42.1)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
    }

    #[test]
    fn test_object_format() {
        let value = Value::Object(None);
        assert_eq!("Object(null)", value.to_string());
        assert!(value.is_null());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        assert_eq!(
            "byte[1, 2, 3]",
            format!("{}", Value::from(vec![1i8, 2i8, 3i8]))
        );
    }

    #[test]
    fn test_is_null() {
        let value = Value::Object(None);
        assert!(value.is_null());

        let value = Value::Object(Some(Reference::from(vec![1i8, 2i8, 3i8])));
        assert!(!value.is_null());
    }

    #[test]
    fn test_is_object() {
        let value = Value::Object(None);
        assert!(!value.is_object());

        let value = Value::Object(Some(Reference::from(vec![1i8, 2i8, 3i8])));
        assert!(value.is_object());
    }

    #[test]
    fn test_as_reference() -> Result<()> {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        let value = Value::Object(Some(reference));
        let reference = value.as_reference()?;
        let array = reference.as_byte_vec_ref()?;
        assert_eq!(array.to_vec(), vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_as_reference_error() {
        let result: Result<&Reference> = Value::Int(42).as_reference();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_string_format() -> Result<()> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let class = class_loader.load("java.lang.String").await?;
        let mut object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        assert!(!string_value.is_null());
        object.set_value("value", string_value)?;
        let value = Value::from(object);
        assert_eq!("String(\"foo\")", value.to_string());
        let value = value.as_string()?;
        assert_eq!("foo".to_string(), value);
        Ok(())
    }

    #[test]
    fn test_as_string_error() {
        let result = Value::Int(42).as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_unused_format() {
        let value = Value::Unused;
        assert_eq!("unused", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
    }

    #[test]
    fn test_clone() -> Result<()> {
        let value = Value::from(vec![1i32]);
        let clone = value.clone();
        assert_eq!(value, clone);

        let Value::Object(Some(ref reference)) = clone else {
            unreachable!("Expected an IntArray reference");
        };

        {
            let mut array = reference.as_int_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(value, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone() -> Result<()> {
        let value = Value::from(vec![1i32]);
        let clone = value.deep_clone()?;
        assert_eq!(value, clone);

        let Value::Object(Some(ref reference)) = clone else {
            unreachable!("Expected an IntArray reference");
        };

        {
            let mut array = reference.as_int_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(value, clone);
        Ok(())
    }

    #[test]
    fn test_hash_int() {
        let mut hasher = DefaultHasher::new();
        Value::Int(42).hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Int(42).hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_eq_int() {
        assert_eq!(Value::Int(1), Value::Int(1));
        assert_ne!(Value::Int(1), Value::Int(2));
        assert_ne!(Value::Int(1), Value::Long(1));
        let value = Value::Int(42);
        assert_eq!(value, value);
    }

    #[test]
    fn test_hash_long() {
        let mut hasher = DefaultHasher::new();
        Value::Long(42).hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Long(42).hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_eq_long() {
        assert_eq!(Value::Long(1), Value::Long(1));
        assert_ne!(Value::Long(1), Value::Long(2));
        assert_ne!(Value::Long(1), Value::Int(1));
        let value = Value::Long(42);
        assert_eq!(value, value);
    }

    #[test]
    fn test_hash_float() {
        let mut hasher = DefaultHasher::new();
        Value::Float(42.1).hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Float(42.1).hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_eq_float() {
        assert_eq!(Value::Float(1.0), Value::Float(1.0));
        assert_ne!(Value::Float(1.0), Value::Float(2.0));
        assert_ne!(Value::Float(1.0), Value::Int(1));
        let value = Value::Float(42.0);
        assert_eq!(value, value);
    }

    #[test]
    fn test_hash_double() {
        let mut hasher = DefaultHasher::new();
        Value::Double(42.1).hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Double(42.1).hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_eq_double() {
        assert_eq!(Value::Double(1.0), Value::Double(1.0));
        assert_ne!(Value::Double(1.0), Value::Double(2.0));
        assert_ne!(Value::Double(1.0), Value::Int(1));
        let value = Value::Double(42.0);
        assert_eq!(value, value);
    }

    #[tokio::test]
    async fn test_hash_object() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let value1 = Value::from(Object::new(class.clone())?);
        let value2 = Value::from(Object::new(class.clone())?);

        let mut hasher = DefaultHasher::new();
        value1.hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        value2.hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
        Ok(())
    }

    #[test]
    fn test_hash_object_none() {
        let mut hasher = DefaultHasher::new();
        Value::Object(None).hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Object(None).hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_eq_object() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let value = Value::from(Object::new(class)?);
        assert_eq!(Value::Object(None), Value::Object(None));
        assert_eq!(value, value);
        assert_ne!(Value::Object(None), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_eq_recursive_object() -> Result<()> {
        let class_name = "java.util.concurrent.atomic.AtomicReference";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        let value1 = Value::from(object);
        let mut object = value1.as_object_mut()?;
        object.set_value("value", value1.clone())?;
        let value2 = value1.clone();
        assert_eq!(value1, value2);
        Ok(())
    }

    #[test]
    fn test_hash_unused() {
        let mut hasher = DefaultHasher::new();
        Value::Unused.hash(&mut hasher);
        let hash1 = hasher.finish();
        hasher = DefaultHasher::new();
        Value::Unused.hash(&mut hasher);
        let hash2 = hasher.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_eq_unused() {
        assert_eq!(Value::Unused, Value::Unused);
        assert_ne!(Value::Unused, Value::Int(1));
    }

    #[test]
    fn test_from_bool() {
        assert_eq!(Value::Int(1), true.into());
        assert_eq!(Value::Int(0), false.into());
    }

    #[test]
    fn test_from_i8() {
        let value: Value = 42i8.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u8() {
        let value: Value = 42u8.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_char() {
        let value: Value = 'a'.into();
        assert_eq!(Value::Int(97), value);
    }

    #[test]
    fn test_from_i16() {
        let value: Value = 42i16.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u16() {
        let value: Value = 42u16.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_i32() {
        let value: Value = 42i32.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u32() {
        let value: Value = 42u32.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_i64() {
        let value: Value = 42i64.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_u64() {
        let value: Value = 42u64.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_isize() {
        let value: Value = 42isize.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_usize() {
        let value: Value = 42usize.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_f32() {
        let value: Value = 42.1f32.into();
        assert_eq!(Value::Float(42.1), value);
    }

    #[test]
    fn test_from_f64() {
        let value: Value = 42.1f64.into();
        assert_eq!(Value::Double(42.1), value);
    }

    #[test]
    fn test_from_vec_bool() {
        let value: Value = vec![true, false].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 0i8]))), value);
    }

    #[test]
    fn test_from_vec_i8() {
        let value: Value = vec![1i8, 2i8].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 2i8]))), value);
    }

    #[test]
    fn test_from_vec_u8() {
        let value: Value = vec![1u8, 2u8].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 2i8]))), value);
    }

    #[test]
    fn test_from_vec_char() {
        let value: Value = vec!['a', 'b'].into();
        assert_eq!(Value::Object(Some(Reference::from(vec!['a', 'b']))), value);
    }

    #[test]
    fn test_from_vec_i16() {
        let value: Value = vec![1i16, 2i16].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i16, 2i16]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u16() {
        let value: Value = vec![1u16, 2u16].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i16, 2i16]))),
            value
        );
    }

    #[test]
    fn test_from_vec_i32() {
        let value: Value = vec![1i32, 2i32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i32, 2i32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u32() {
        let value: Value = vec![1u32, 2u32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i32, 2i32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_i64() {
        let value: Value = vec![1i64, 2i64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u64() {
        let value: Value = vec![1u64, 2u64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_isize() {
        let value: Value = vec![1isize, 2isize].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_usize() {
        let value: Value = vec![1usize, 2usize].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_f32() {
        let value: Value = vec![1.1f32, 2.2f32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1.1f32, 2.2f32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_f64() {
        let value: Value = vec![1.1f64, 2.2f64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1.1f64, 2.2f64]))),
            value
        );
    }

    #[tokio::test]
    async fn test_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![Value::Object(None)];
        let value = Value::try_from((original_class.clone(), original_value.clone()))?;
        assert!(matches!(value, Value::Object(Some(Reference::Array(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_values = vec![value];
        let value = Value::try_from((original_class.clone(), original_values.clone()))?;
        assert!(matches!(value, Value::Object(Some(Reference::Array(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec_error() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let value = Value::from(42);
        let original_value = vec![value];
        let value = Value::try_from((original_class.clone(), original_value.clone()));
        assert!(matches!(value, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_object() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let value = Value::from(object);
        assert!(matches!(value, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_reference() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        let value = Value::from(reference);
        assert!(matches!(value, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_option_reference() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        let value = Value::from(Some(reference));
        assert!(matches!(value, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_as_bool() -> Result<()> {
        let value = Value::Int(1).as_bool()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_bool_object() -> Result<()> {
        let class = load_class("java.lang.Boolean").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let value = Value::from(object);
        let value = value.as_bool()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_char() -> Result<()> {
        let value = Value::Int(42).as_char()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_char_object() -> Result<()> {
        let class = load_class("java.lang.Character").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_char()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i8() -> Result<()> {
        let value = Value::Int(42).as_i8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i8_object() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_i8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u8() -> Result<()> {
        let value = Value::Int(42).as_u8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u8_object() -> Result<()> {
        let class = load_class("java.lang.Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_u8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i16() -> Result<()> {
        let value = Value::Int(42).as_i16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i16_object() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_i16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u16() -> Result<()> {
        let value = Value::Int(42).as_u16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u16_object() -> Result<()> {
        let class = load_class("java.lang.Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_u16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i32() -> Result<()> {
        let value = Value::Int(42).as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i32_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u32() -> Result<()> {
        let value = Value::Int(42).as_u32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u32_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value = value.as_u32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i64() -> Result<()> {
        let value = Value::Long(42).as_i64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i64_object() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value = value.as_i64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u64() -> Result<()> {
        let value = Value::Long(42).as_u64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u64_object() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value = value.as_u64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_isize() -> Result<()> {
        let value = Value::Long(42).as_isize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_isize_object() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value = value.as_isize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_usize() -> Result<()> {
        let value = Value::Long(42).as_usize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_usize_object() -> Result<()> {
        let class = load_class("java.lang.Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value = value.as_usize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f32() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let value = Value::from(object);
        let value = value.as_f32()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f32_object() -> Result<()> {
        let value = Value::Float(42.1).as_f32()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f64() -> Result<()> {
        let value = Value::Double(42.1).as_f64()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f64_object() -> Result<()> {
        let class = load_class("java.lang.Double").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let value = Value::from(object);
        let value = value.as_f64()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_vec_value() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java/lang/Integer";
        let class = load_class(class_name).await?;
        let mut object = Object::new(class.clone())?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_values = vec![value];
        let value = Value::try_from((original_class.clone(), original_values.clone()))?;
        let values: Vec<Value> = value.try_into()?;
        assert_eq!(original_values, values);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref() -> Result<()> {
        let original_value = vec![42i8];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_byte_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_byte_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_byte_vec_mut() -> Result<()> {
        let value = Value::from(vec![42i8]);
        {
            let mut mutable_value = value.as_byte_vec_mut()?;
            mutable_value.push(3i8);
        }
        assert_eq!(value.as_byte_vec_ref()?.to_vec(), vec![42i8, 3i8]);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_mut_error() {
        let original_value = vec![42i32];
        let value = Value::from(original_value.clone());
        let result = value.as_byte_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_char_vec_ref() -> Result<()> {
        let value = Value::from(vec!['*']);
        assert_eq!(vec![42u16], value.as_char_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_char_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_char_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_char_vec_mut() -> Result<()> {
        let value = Value::from(vec!['*']);
        {
            let mut mutable_value = value.as_char_vec_mut()?;
            mutable_value.push(50u16);
        }
        assert_eq!(value.as_char_vec_ref()?.to_vec(), vec![42u16, 50u16]);
        Ok(())
    }

    #[test]
    fn test_as_char_vec_mut_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_char_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_short_vec_ref() -> Result<()> {
        let original_value = vec![42i16];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_short_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_short_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_short_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_short_vec_mut() -> Result<()> {
        let value = Value::from(vec![42i16]);
        {
            let mut mutable_value = value.as_short_vec_mut()?;
            mutable_value.push(3i16);
        }
        assert_eq!(value.as_short_vec_ref()?.to_vec(), vec![42i16, 3i16]);
        Ok(())
    }

    #[test]
    fn test_as_short_vec_mut_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_short_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_int_vec_ref() -> Result<()> {
        let original_value = vec![42i32];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_int_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_int_vec_ref_error() {
        let value = Value::from(vec![42i8]);
        let result = value.as_int_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_int_vec_mut() -> Result<()> {
        let value = Value::from(vec![42i32]);
        {
            let mut mutable_value = value.as_int_vec_mut()?;
            mutable_value.push(3i32);
        }
        assert_eq!(value.as_int_vec_ref()?.to_vec(), vec![42i32, 3i32]);
        Ok(())
    }

    #[test]
    fn test_as_int_vec_mut_error() {
        let value = Value::from(vec![42i8]);
        let result = value.as_int_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_long_vec_ref() -> Result<()> {
        let original_value = vec![42i64];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_long_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_long_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_long_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_long_vec_mut() -> Result<()> {
        let value = Value::from(vec![42i64]);
        {
            let mut mutable_value = value.as_long_vec_mut()?;
            mutable_value.push(3i64);
        }
        assert_eq!(value.as_long_vec_ref()?.to_vec(), vec![42i64, 3i64]);
        Ok(())
    }

    #[test]
    fn test_as_long_vec_mut_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_long_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_float_vec_ref() -> Result<()> {
        let original_value = vec![42.1f32];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_float_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_float_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_float_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_float_vec_mut() -> Result<()> {
        let value = Value::from(vec![42.1f32]);
        {
            let mut mutable_value = value.as_float_vec_mut()?;
            mutable_value.push(3.45f32);
        }
        assert_eq!(value.as_float_vec_ref()?.to_vec(), vec![42.1f32, 3.45f32]);
        Ok(())
    }

    #[test]
    fn test_as_float_vec_mut_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_float_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_double_vec_ref() -> Result<()> {
        let original_value = vec![42.1f64];
        let value = Value::from(original_value.clone());
        assert_eq!(original_value, value.as_double_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_double_vec_ref_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_double_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_double_vec_mut() -> Result<()> {
        let value = Value::from(vec![42.1f64]);
        {
            let mut mutable_value = value.as_double_vec_mut()?;
            mutable_value.push(3.45f64);
        }
        assert_eq!(value.as_double_vec_ref()?.to_vec(), vec![42.1f64, 3.45f64]);
        Ok(())
    }

    #[test]
    fn test_as_double_vec_mut_error() {
        let value = Value::from(vec![42i32]);
        let result = value.as_double_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_as_class_vec_ref() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![None];
        let value = Value::from((original_class.clone(), original_value.clone()));
        let (class, value) = value.as_class_vec_ref()?;
        assert_eq!(&original_class, class);
        assert_eq!(original_value, value.to_vec());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_class_vec_mut() -> Result<()> {
        let object_class = load_class("[Ljava/lang/Object;").await?;
        let value = Value::from((object_class.clone(), vec![]));
        {
            let (_class, mut mutable_reference) = value.as_class_vec_mut()?;
            mutable_reference.push(None);
        }
        let (_class, array) = value.as_class_vec_ref()?;
        assert_eq!(array.to_vec(), vec![None]);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_object_ref() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let value = Value::from(object.clone());
        let result = value.as_object_ref()?.clone();
        assert_eq!(object, result);
        assert_eq!("java/lang/Object", object.class().name());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_object_mut() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        let value = Value::from(object.clone());
        let mut result = value.as_object_mut()?;
        result.set_value("value", Value::Int(42))?;
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_as_string() -> Result<()> {
        let class = load_class("java.lang.String").await?;
        let mut object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let value = Value::from(object);
        let result = value.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }
}

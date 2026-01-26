use crate::Error::InvalidValueType;
use crate::reference::Reference;
use crate::{Class, Object, Result};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use ristretto_gc::{GarbageCollector, Gc};
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Represents a value in the Ristretto VM.
#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Gc<RwLock<Reference>>>),
    Unused,
}

impl Value {
    /// Create a new object value.
    pub fn new_object(collector: &GarbageCollector, reference: Reference) -> Self {
        Value::Object(Some(Gc::new(collector, RwLock::new(reference)).clone_gc()))
    }

    /// Create a new nullable object value.
    pub fn new_opt_object(collector: &GarbageCollector, reference: Option<Reference>) -> Self {
        match reference {
            Some(reference) => Self::new_object(collector, reference),
            None => Value::Object(None),
        }
    }

    /// Create a new object value from an Object.
    pub fn from_object(collector: &GarbageCollector, object: Object) -> Self {
        Self::new_object(collector, Reference::from(object))
    }

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
    pub fn as_reference(&self) -> Result<RwLockReadGuard<'_, Reference>> {
        match self {
            Value::Object(Some(reference)) => Ok(reference.read()),
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a mutable reference to the value if it is an object.
    ///
    /// # Errors
    ///
    /// If the value is not an object or if it cannot be converted to a reference.
    pub fn as_reference_mut(&self) -> Result<RwLockWriteGuard<'_, Reference>> {
        match self {
            Value::Object(Some(reference)) => Ok(reference.write()),
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

    /// Returns the reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [i8]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_byte_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected byte array".to_string()))
    }

    /// Returns a mutable reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [i8]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_byte_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected byte array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [u16]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_char_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected char array".to_string()))
    }

    /// Returns a mutable reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [u16]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_char_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected char array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [i16]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_short_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected short array".to_string()))
    }

    /// Returns a mutable reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [i16]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_short_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected short array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [i32]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_int_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected int array".to_string()))
    }

    /// Returns a mutable reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [i32]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_int_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected int array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [i64]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_long_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected long array".to_string()))
    }

    /// Returns a mutable reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [i64]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_long_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected long array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [f32]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_float_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected float array".to_string()))
    }

    /// Returns a mutable reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [f32]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_float_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected float array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_ref(&self) -> Result<MappedRwLockReadGuard<'_, [f64]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_double_vec_ref().ok())
            .map_err(|_| InvalidValueType("Expected double array".to_string()))
    }

    /// Returns a mutable reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [f64]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_double_vec_mut().ok())
                    .map_err(|_| InvalidValueType("Expected double array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to `Vec<Value>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `Reference::Array`
    pub fn as_class_vec_ref(&self) -> Result<(Arc<Class>, MappedRwLockReadGuard<'_, [Value]>)> {
        let reference = self.as_reference()?;
        let mut class = None;
        let guard = RwLockReadGuard::try_map(reference, |r| {
            if let Ok((c, v)) = r.as_class_vec_ref() {
                class = Some(c.clone());
                Some(v)
            } else {
                None
            }
        })
        .map_err(|_| InvalidValueType("Expected array".to_string()))?;

        let class = class.ok_or_else(|| InvalidValueType("Expected array".to_string()))?;
        Ok((class, guard))
    }

    /// Returns a mutable reference to `Vec<Value>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `Reference::Array`
    pub fn as_class_vec_mut(&self) -> Result<(Arc<Class>, MappedRwLockWriteGuard<'_, [Value]>)> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                let mut class = None;
                let guard = RwLockWriteGuard::try_map(reference, |r| {
                    if let Ok((c, v)) = r.as_class_vec_mut() {
                        class = Some(c.clone());
                        Some(v)
                    } else {
                        None
                    }
                })
                .map_err(|_| InvalidValueType("Expected array".to_string()))?;
                let class = class.ok_or_else(|| InvalidValueType("Expected array".to_string()))?;
                Ok((class, guard))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a reference to an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_ref(&self) -> Result<MappedRwLockReadGuard<'_, Object>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, |r| r.as_object_ref().ok())
            .map_err(|_| InvalidValueType("Expected object".to_string()))
    }

    /// Returns a mutable reference to an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_mut(&self) -> Result<MappedRwLockWriteGuard<'_, Object>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, |r| r.as_object_mut().ok())
                    .map_err(|_| InvalidValueType("Expected object".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
    }

    /// Returns a read-only byte slice view of the underlying primitive array data.
    /// This provides raw byte access to primitive arrays for low-level memory operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not an object reference or if the reference
    /// is not a primitive array type (i.e., it's an `Object` or object `Array`).
    pub fn as_bytes(&self) -> Result<MappedRwLockReadGuard<'_, [u8]>> {
        let reference = self.as_reference()?;
        RwLockReadGuard::try_map(reference, Reference::as_bytes)
            .map_err(|_| InvalidValueType("Expected primitive array".to_string()))
    }

    /// Returns a mutable byte slice view of the underlying primitive array data.
    /// This provides raw byte access to primitive arrays for low-level memory operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not an object reference or if the reference
    /// is not a primitive array type (i.e., it's an `Object` or object `Array`).
    pub fn as_bytes_mut(&self) -> Result<MappedRwLockWriteGuard<'_, [u8]>> {
        match self {
            Value::Object(Some(reference)) => {
                let reference = reference.write();
                RwLockWriteGuard::try_map(reference, Reference::as_bytes_mut)
                    .map_err(|_| InvalidValueType("Expected primitive array".to_string()))
            }
            _ => Err(InvalidValueType("Expected a reference value".to_string())),
        }
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
                    let value = value.read();
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
            Value::Object(Some(reference)) => {
                let reference = reference.read();
                reference.hash(state);
            }
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
            (Value::Object(Some(a)), Value::Object(Some(b))) => {
                if Gc::ptr_eq(a, b) {
                    return true;
                }
                let a = a.read();
                let b = b.read();
                *a == *b
            }
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

impl TryInto<Vec<Value>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<Value>> {
        let (_class, values) = self.as_class_vec_ref()?;
        let values = values.iter().cloned().collect();
        Ok(values)
    }
}

impl From<Gc<RwLock<Reference>>> for Value {
    fn from(value: Gc<RwLock<Reference>>) -> Self {
        Value::Object(Some(value))
    }
}

impl From<Option<Gc<RwLock<Reference>>>> for Value {
    fn from(value: Option<Gc<RwLock<Reference>>>) -> Self {
        Value::Object(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Reference;
    use crate::{Class, Object, runtime};
    use ristretto_gc::GarbageCollector;
    use std::hash::DefaultHasher;
    use std::sync::Arc;

    fn test_ref(reference: impl Into<Reference>) -> Value {
        Value::new_object(&GarbageCollector::new(), reference.into())
    }

    fn test_opt_ref(reference: Option<Reference>) -> Value {
        Value::new_opt_object(&GarbageCollector::new(), reference)
    }

    fn test_value(value: impl Into<Value>) -> Value {
        value.into()
    }

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
            format!("{}", test_ref(vec![1i8, 2i8, 3i8]))
        );
    }

    #[test]
    fn test_is_null() {
        let value = Value::Object(None);
        assert!(value.is_null());

        let value = test_ref(Reference::from(vec![1i8, 2i8, 3i8]));
        assert!(!value.is_null());
    }

    #[test]
    fn test_is_object() {
        let value = Value::Object(None);
        assert!(!value.is_object());

        let value = test_ref(Reference::from(vec![1i8, 2i8, 3i8]));
        assert!(value.is_object());
    }

    #[test]
    fn test_as_reference() -> Result<()> {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        let value = test_ref(reference);
        let reference = value.as_reference()?;
        let array = reference.as_byte_vec_ref()?;
        assert_eq!(array, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_as_reference_error() {
        let result = Value::Int(42).as_reference();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_string_format() -> Result<()> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let class = class_loader.load("java.lang.String").await?;
        let mut object = Object::new(class)?;
        let bytes = "foo".as_bytes();
        let string_bytes: &[i8] = zerocopy::transmute_ref!(bytes);
        let string_value = test_ref(string_bytes.to_vec());
        assert!(!string_value.is_null());
        object.set_value("value", string_value)?;
        let value = test_ref(object);
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
        let value = test_ref(vec![1i32]);
        let clone = value.clone();
        assert_eq!(value, clone);

        let Value::Object(Some(ref reference)) = clone else {
            unreachable!("Expected an IntArray reference");
        };

        {
            let mut reference = reference.write();
            let array = reference.as_int_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(value, clone);
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
        let value1 = test_ref(Object::new(class.clone())?);
        let value2 = test_ref(Object::new(class.clone())?);

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
        let value = test_ref(Object::new(class)?);
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
        let value1 = test_ref(object);
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
        let value = Value::Int(i32::from(42i8));
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u8() {
        let value = Value::Int(i32::from(42u8));
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_char() {
        let value = Value::Int('a' as i32);
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
        let value = Value::Float(42.1f32);
        assert_eq!(Value::Float(42.1), value);
    }

    #[test]
    fn test_from_f64() {
        let value = Value::Double(42.1f64);
        assert_eq!(Value::Double(42.1), value);
    }

    #[test]
    fn test_from_vec_bool() {
        let value = test_ref(Reference::from(vec![true, false]));
        assert_eq!(
            test_ref(Reference::BooleanArray(vec![1i8, 0i8].into_boxed_slice())),
            value
        );
    }

    #[test]
    fn test_from_vec_i8() {
        let value = test_ref(Reference::from(vec![1i8, 2i8]));
        assert_eq!(test_ref(Reference::from(vec![1i8, 2i8])), value);
    }

    #[test]
    fn test_from_vec_u8() {
        let value = test_ref(Reference::from(vec![1u8, 2u8]));
        assert_eq!(test_ref(Reference::from(vec![1i8, 2i8])), value);
    }

    #[test]
    fn test_from_vec_char() {
        let value = test_ref(Reference::from(vec!['a', 'b']));
        assert_eq!(test_ref(Reference::from(vec!['a', 'b'])), value);
    }

    #[test]
    fn test_from_vec_i16() {
        let value = test_ref(Reference::from(vec![1i16, 2i16]));
        assert_eq!(test_ref(Reference::from(vec![1i16, 2i16])), value);
    }

    #[test]
    fn test_from_vec_u16() {
        let value = test_ref(Reference::from(vec![1u16, 2u16]));
        assert_eq!(test_ref(Reference::from(vec![1i16, 2i16])), value);
    }

    #[test]
    fn test_from_vec_i32() {
        let value = test_ref(Reference::from(vec![1i32, 2i32]));
        assert_eq!(test_ref(Reference::from(vec![1i32, 2i32])), value);
    }

    #[test]
    fn test_from_vec_u32() {
        let value = test_ref(Reference::from(vec![1u32, 2u32]));
        assert_eq!(test_ref(Reference::from(vec![1i32, 2i32])), value);
    }

    #[test]
    fn test_from_vec_i64() {
        let value = test_ref(Reference::from(vec![1i64, 2i64]));
        assert_eq!(test_ref(Reference::from(vec![1i64, 2i64])), value);
    }

    #[test]
    fn test_from_vec_u64() {
        let value = test_ref(Reference::from(vec![1u64, 2u64]));
        assert_eq!(test_ref(Reference::from(vec![1i64, 2i64])), value);
    }

    #[test]
    fn test_from_vec_isize() {
        let value = test_ref(Reference::from(vec![1isize, 2isize]));
        assert_eq!(test_ref(Reference::from(vec![1i64, 2i64])), value);
    }

    #[test]
    fn test_from_vec_usize() {
        let value = test_ref(Reference::from(vec![1usize, 2usize]));
        assert_eq!(test_ref(Reference::from(vec![1i64, 2i64])), value);
    }

    #[test]
    fn test_from_vec_f32() {
        let value = test_ref(Reference::from(vec![1.1f32, 2.2f32]));
        assert_eq!(test_ref(Reference::from(vec![1.1f32, 2.2f32])), value);
    }

    #[test]
    fn test_from_vec_f64() {
        let value = test_ref(Reference::from(vec![1.1f64, 2.2f64]));
        assert_eq!(test_ref(Reference::from(vec![1.1f64, 2.2f64])), value);
    }

    #[tokio::test]
    async fn test_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![Value::Object(None)];
        let reference = Reference::try_from((original_class.clone(), original_value.clone()))?;
        let value = test_ref(reference);
        assert!(matches!(value, Value::Object(Some(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = test_ref(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        let value = test_ref(reference);
        assert!(matches!(value, Value::Object(Some(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec_error() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let value = Value::Int(42);
        let original_value = vec![value];
        let result = Reference::try_from((original_class.clone(), original_value.clone()));
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_object() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let value = test_ref(object);
        assert!(matches!(value, Value::Object(Some(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_reference() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        let value = test_ref(reference);
        assert!(matches!(value, Value::Object(Some(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_option_reference() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        let value = test_opt_ref(Some(reference));
        assert!(matches!(value, Value::Object(Some(_))));
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
        let value = value.as_usize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f32() -> Result<()> {
        let class = load_class("java.lang.Float").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let value = test_ref(object);
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
        let value = test_ref(object);
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
        let value = test_ref(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        let value = test_ref(reference);
        let values: Vec<Value> = value.try_into()?;
        assert_eq!(original_values, values);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref() -> Result<()> {
        let original_value = vec![42i8];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_byte_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_byte_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_byte_vec_mut() -> Result<()> {
        let value = test_ref(vec![42i8]);
        {
            let mut mutable_value = value.as_byte_vec_mut()?;
            mutable_value[0] = 3i8;
        }
        assert_eq!(value.as_byte_vec_ref()?.to_vec(), vec![3i8]);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_mut_error() {
        let original_value = vec![42i32];
        let value = test_ref(original_value.clone());
        let result = value.as_byte_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_char_vec_ref() -> Result<()> {
        let value = test_ref(vec!['*']);
        assert_eq!(vec![42u16], value.as_char_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_char_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_char_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_char_vec_mut() -> Result<()> {
        let value = test_ref(vec!['*']);
        {
            let mut mutable_value = value.as_char_vec_mut()?;
            mutable_value[0] = 50u16;
        }
        assert_eq!(value.as_char_vec_ref()?.to_vec(), vec![50u16]);
        Ok(())
    }

    #[test]
    fn test_as_char_vec_mut_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_char_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_short_vec_ref() -> Result<()> {
        let original_value = vec![42i16];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_short_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_short_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_short_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_short_vec_mut() -> Result<()> {
        let value = test_ref(vec![42i16]);
        {
            let mut mutable_value = value.as_short_vec_mut()?;
            mutable_value[0] = 3i16;
        }
        assert_eq!(value.as_short_vec_ref()?.to_vec(), vec![3i16]);
        Ok(())
    }

    #[test]
    fn test_as_short_vec_mut_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_short_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_int_vec_ref() -> Result<()> {
        let original_value = vec![42i32];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_int_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_int_vec_ref_error() {
        let value = test_ref(vec![42i8]);
        let result = value.as_int_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_int_vec_mut() -> Result<()> {
        let value = test_ref(vec![42i32]);
        {
            let mut mutable_value = value.as_int_vec_mut()?;
            mutable_value[0] = 3i32;
        }
        assert_eq!(value.as_int_vec_ref()?.to_vec(), vec![3i32]);
        Ok(())
    }

    #[test]
    fn test_as_int_vec_mut_error() {
        let value = test_ref(vec![42i8]);
        let result = value.as_int_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_long_vec_ref() -> Result<()> {
        let original_value = vec![42i64];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_long_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_long_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_long_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_long_vec_mut() -> Result<()> {
        let value = test_ref(vec![42i64]);
        {
            let mut mutable_value = value.as_long_vec_mut()?;
            mutable_value[0] = 3i64;
        }
        assert_eq!(value.as_long_vec_ref()?.to_vec(), vec![3i64]);
        Ok(())
    }

    #[test]
    fn test_as_long_vec_mut_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_long_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_float_vec_ref() -> Result<()> {
        let original_value = vec![42.1f32];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_float_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_float_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_float_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_float_vec_mut() -> Result<()> {
        let value = test_ref(vec![42.1f32]);
        {
            let mut mutable_value = value.as_float_vec_mut()?;
            mutable_value[0] = 3.45f32;
        }
        assert_eq!(value.as_float_vec_ref()?.to_vec(), vec![3.45f32]);
        Ok(())
    }

    #[test]
    fn test_as_float_vec_mut_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_float_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_double_vec_ref() -> Result<()> {
        let original_value = vec![42.1f64];
        let value = test_ref(original_value.clone());
        assert_eq!(original_value, value.as_double_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_double_vec_ref_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_double_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_double_vec_mut() -> Result<()> {
        let value = test_ref(vec![42.1f64]);
        {
            let mut mutable_value = value.as_double_vec_mut()?;
            mutable_value[0] = 3.45f64;
        }
        assert_eq!(value.as_double_vec_ref()?.to_vec(), vec![3.45f64]);
        Ok(())
    }

    #[test]
    fn test_as_double_vec_mut_error() {
        let value = test_ref(vec![42i32]);
        let result = value.as_double_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_as_class_vec_ref() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![Value::Object(None)];
        let values = vec![Value::Object(None)];
        let reference = Reference::try_from((original_class.clone(), values))?;
        let value = test_ref(reference);
        let (class, value) = value.as_class_vec_ref()?;
        assert_eq!(original_class, class);
        assert_eq!(original_value, value.to_vec());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_class_vec_mut() -> Result<()> {
        let object_class = load_class("[Ljava/lang/Object;").await?;
        let values = vec![Value::Object(None)];
        let reference = Reference::try_from((object_class.clone(), values))?;
        let value = test_ref(reference);
        {
            let (_class, mut mutable_reference) = value.as_class_vec_mut()?;
            mutable_reference[0] = Value::Int(42);
        }
        let (_class, array) = value.as_class_vec_ref()?;
        assert_eq!(array.to_vec(), vec![Value::Int(42)]);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_object_ref() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let value = test_ref(object.clone());
        let result = value.as_object_ref()?.clone();
        assert_eq!(object, result);
        assert_eq!("java/lang/Object", object.class().name());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_object_mut() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        let value = test_ref(object.clone());
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
        let string_value = test_ref(string_bytes);
        object.set_value("value", string_value)?;
        let value = test_ref(object);
        let result = value.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    // Tests for as_bytes()

    #[test]
    fn test_as_bytes_byte_array() -> Result<()> {
        let value = test_ref(vec![1i8, 2i8, 3i8, 4i8]);
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 4);
        assert_eq!(&*bytes, &[1u8, 2u8, 3u8, 4u8]);
        Ok(())
    }

    #[test]
    fn test_as_bytes_int_array() -> Result<()> {
        let value = test_ref(vec![0x0102_0304i32]);
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 4);
        Ok(())
    }

    #[test]
    fn test_as_bytes_long_array() -> Result<()> {
        let value = test_ref(vec![1i64, 2i64]);
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 16); // 2 longs * 8 bytes each
        Ok(())
    }

    #[test]
    fn test_as_bytes_float_array() -> Result<()> {
        let value = test_ref(vec![1.0f32, 2.0f32]);
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 8); // 2 floats * 4 bytes each
        Ok(())
    }

    #[test]
    fn test_as_bytes_double_array() -> Result<()> {
        let value = test_ref(vec![1.0f64]);
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 8);
        Ok(())
    }

    #[test]
    fn test_as_bytes_empty_array() -> Result<()> {
        let value = test_ref(Vec::<i32>::new());
        let bytes = value.as_bytes()?;
        assert!(bytes.is_empty());
        Ok(())
    }

    #[test]
    fn test_as_bytes_primitive_value_error() {
        let value = Value::Int(42);
        let result = value.as_bytes();
        assert!(result.is_err());
    }

    #[test]
    fn test_as_bytes_null_error() {
        let value = Value::Object(None);
        let result = value.as_bytes();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_as_bytes_object_error() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let value = test_ref(object);
        let result = value.as_bytes();
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_bytes_object_array_error() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let values = vec![Value::Object(None)];
        let reference = Reference::try_from((class, values))?;
        let value = test_ref(reference);
        let result = value.as_bytes();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_as_bytes_mut_byte_array() -> Result<()> {
        let value = test_ref(vec![1i8, 2i8, 3i8, 4i8]);
        {
            let mut bytes = value.as_bytes_mut()?;
            bytes[0] = 10;
            bytes[1] = 20;
        }
        let bytes = value.as_bytes()?;
        assert_eq!(bytes[0], 10);
        assert_eq!(bytes[1], 20);
        Ok(())
    }

    #[test]
    fn test_as_bytes_mut_int_array() -> Result<()> {
        let value = test_ref(vec![0i32]);
        {
            let mut bytes = value.as_bytes_mut()?;
            bytes.fill(0xFF);
        }
        let ints = value.as_int_vec_ref()?;
        assert_eq!(ints[0], -1i32);
        Ok(())
    }

    #[test]
    fn test_as_bytes_mut_long_array() -> Result<()> {
        let value = test_ref(Reference::from(vec![0i64]));
        {
            let mut bytes = value.as_bytes_mut()?;
            bytes.fill(0xFF);
        }
        let longs = value.as_long_vec_ref()?;
        assert_eq!(longs[0], -1i64);
        Ok(())
    }

    #[test]
    fn test_as_bytes_mut_float_array() -> Result<()> {
        let value = test_ref(Reference::from(vec![0.0f32]));
        {
            let mut bytes = value.as_bytes_mut()?;
            bytes.copy_from_slice(&1.0f32.to_ne_bytes());
        }
        let floats = value.as_float_vec_ref()?;
        assert!((floats[0] - 1.0f32).abs() < f32::EPSILON);
        Ok(())
    }

    #[test]
    fn test_as_bytes_mut_primitive_value_error() {
        let value = Value::Int(42);
        let result = value.as_bytes_mut();
        assert!(result.is_err());
    }

    #[test]
    fn test_as_bytes_mut_null_error() {
        let value = Value::Object(None);
        let result = value.as_bytes_mut();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_as_bytes_mut_object_error() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let value = test_ref(object);
        let result = value.as_bytes_mut();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_as_bytes_roundtrip() -> Result<()> {
        let original = vec![1i32, 2i32, 3i32, 4i32];
        let value = test_ref(Reference::from(original.clone()));
        let bytes = value.as_bytes()?.to_vec();

        let new_value = test_ref(Reference::from(vec![0i32; 4]));
        {
            let mut new_bytes = new_value.as_bytes_mut()?;
            new_bytes.copy_from_slice(&bytes);
        }

        let result = new_value.as_int_vec_ref()?;
        assert_eq!(result.to_vec(), original);
        Ok(())
    }
}

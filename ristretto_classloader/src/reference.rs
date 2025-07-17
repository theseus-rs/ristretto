use crate::Error::InvalidValueType;
use crate::concurrent_vec::ConcurrentVec;
use crate::{Class, Object, Result, Value};
use ristretto_gc::{GarbageCollector, Trace};
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use zerocopy::transmute_ref;

/// Represents an array of objects in the Ristretto VM.
///
/// `ObjectArray` groups the array's class `Arc<Class>` and its elements
/// `ConcurrentVec<Option<Reference>>` together in order to reduce the amount of memory required by
/// values in the Reference enum.
#[derive(Clone, Debug)]
pub struct ObjectArray {
    pub class: Arc<Class>,
    pub elements: ConcurrentVec<Option<Reference>>,
}

impl Eq for ObjectArray {}

impl PartialEq for ObjectArray {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.elements == other.elements
    }
}

/// Represents a reference to an object in the Ristretto VM.
#[derive(Clone, Debug)]
pub enum Reference {
    ByteArray(ConcurrentVec<i8>),
    CharArray(ConcurrentVec<u16>),
    ShortArray(ConcurrentVec<i16>),
    IntArray(ConcurrentVec<i32>),
    LongArray(ConcurrentVec<i64>),
    FloatArray(ConcurrentVec<f32>),
    DoubleArray(ConcurrentVec<f64>),
    Array(ObjectArray),
    Object(Object),
}

impl Reference {
    /// Get the class name of the reference
    #[must_use]
    pub fn class_name(&self) -> &str {
        match self {
            Reference::ByteArray(_) => "[B",
            Reference::CharArray(_) => "[C",
            Reference::ShortArray(_) => "[S",
            Reference::IntArray(_) => "[I",
            Reference::LongArray(_) => "[J",
            Reference::FloatArray(_) => "[F",
            Reference::DoubleArray(_) => "[D",
            Reference::Array(object_array) => object_array.class.name(),
            Reference::Object(value) => value.class().name(),
        }
    }

    /// Returns the reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i8>>> {
        match self {
            Reference::ByteArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected byte array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i8>>> {
        match self {
            Reference::ByteArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected byte array".to_string())),
        }
    }

    /// Returns a reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<u16>>> {
        match self {
            Reference::CharArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected char array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<u16>>> {
        match self {
            Reference::CharArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected char array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i16>>> {
        match self {
            Reference::ShortArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected short array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i16>>> {
        match self {
            Reference::ShortArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected short array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i32>>> {
        match self {
            Reference::IntArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected int array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i32>>> {
        match self {
            Reference::IntArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected int array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i64>>> {
        match self {
            Reference::LongArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected long array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<i64>>> {
        match self {
            Reference::LongArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected long array".to_string())),
        }
    }

    /// Returns a reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<f32>>> {
        match self {
            Reference::FloatArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected float array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<f32>>> {
        match self {
            Reference::FloatArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected float array".to_string())),
        }
    }

    /// Returns a reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<f64>>> {
        match self {
            Reference::DoubleArray(value) => Ok(value.as_ref()?),
            _ => Err(InvalidValueType("Expected double array".to_string())),
        }
    }

    /// Returns a mutable reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<f64>>> {
        match self {
            Reference::DoubleArray(value) => Ok(value.as_mut()?),
            _ => Err(InvalidValueType("Expected double array".to_string())),
        }
    }

    /// Returns a reference to `Vec<Option<Reference>>`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Array`.
    #[expect(clippy::type_complexity)]
    pub fn as_class_vec_ref(
        &self,
    ) -> Result<(&Arc<Class>, RwLockReadGuard<'_, Vec<Option<Reference>>>)> {
        match self {
            Reference::Array(object_array) => {
                Ok((&object_array.class, object_array.elements.as_ref()?))
            }
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<Option<Reference>>`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Array`.
    #[expect(clippy::type_complexity)]
    pub fn as_class_vec_mut(
        &self,
    ) -> Result<(&Arc<Class>, RwLockWriteGuard<'_, Vec<Option<Reference>>>)> {
        match self {
            Reference::Array(object_array) => {
                Ok((&object_array.class, object_array.elements.as_mut()?))
            }
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns the reference as an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_ref(&self) -> Result<&Object> {
        match self {
            Reference::Object(object) => Ok(object),
            _ => Err(InvalidValueType("Expected object".to_string())),
        }
    }

    /// Returns hash code implementation based on memory address.
    #[must_use]
    pub fn hash_code(&self) -> usize {
        match self {
            Reference::ByteArray(reference) => reference.hash_code(),
            Reference::CharArray(reference) => reference.hash_code(),
            Reference::ShortArray(reference) => reference.hash_code(),
            Reference::IntArray(reference) => reference.hash_code(),
            Reference::LongArray(reference) => reference.hash_code(),
            Reference::FloatArray(reference) => reference.hash_code(),
            Reference::DoubleArray(reference) => reference.hash_code(),
            Reference::Array(reference) => reference.elements.hash_code(),
            Reference::Object(reference) => reference.hash_code(),
        }
    }

    /// Check if two references point to the same memory location.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::ByteArray(a), Reference::ByteArray(b)) => a.ptr_eq(b),
            (Reference::CharArray(a), Reference::CharArray(b)) => a.ptr_eq(b),
            (Reference::ShortArray(a), Reference::ShortArray(b)) => a.ptr_eq(b),
            (Reference::IntArray(a), Reference::IntArray(b)) => a.ptr_eq(b),
            (Reference::LongArray(a), Reference::LongArray(b)) => a.ptr_eq(b),
            (Reference::FloatArray(a), Reference::FloatArray(b)) => a.ptr_eq(b),
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => a.ptr_eq(b),
            (Reference::Array(a), Reference::Array(b)) => {
                Arc::ptr_eq(&a.class, &b.class) && a.elements.ptr_eq(&b.elements)
            }
            (Reference::Object(a), Reference::Object(b)) => a.ptr_eq(b),
            _ => false,
        }
    }

    /// Returns a deep clone of the reference.
    ///
    /// # Errors
    ///
    /// if the reference cannot be cloned.
    pub fn deep_clone(&self) -> Result<Reference> {
        let value = match self {
            Reference::ByteArray(value) => Reference::ByteArray(value.deep_clone()?),
            Reference::CharArray(value) => Reference::CharArray(value.deep_clone()?),
            Reference::ShortArray(value) => Reference::ShortArray(value.deep_clone()?),
            Reference::IntArray(value) => Reference::IntArray(value.deep_clone()?),
            Reference::LongArray(value) => Reference::LongArray(value.deep_clone()?),
            Reference::FloatArray(value) => Reference::FloatArray(value.deep_clone()?),
            Reference::DoubleArray(value) => Reference::DoubleArray(value.deep_clone()?),
            Reference::Array(object_array) => {
                let values = object_array.elements.to_vec()?;
                let mut cloned_values = Vec::with_capacity(values.len());
                for value in values {
                    match value {
                        Some(reference) => cloned_values.push(Some(reference.deep_clone()?)),
                        None => cloned_values.push(value),
                    }
                }
                let object_array = ObjectArray {
                    class: object_array.class.clone(),
                    elements: ConcurrentVec::from(cloned_values),
                };
                Reference::Array(object_array)
            }
            Reference::Object(value) => Reference::Object(value.deep_clone()?),
        };
        Ok(value)
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::ByteArray(value) => write!(f, "byte{value}"),
            Reference::CharArray(value) => write!(f, "char{value}"),
            Reference::ShortArray(value) => write!(f, "short{value}"),
            Reference::IntArray(value) => write!(f, "int{value}"),
            Reference::LongArray(value) => write!(f, "long{value}"),
            Reference::FloatArray(value) => write!(f, "float{value}"),
            Reference::DoubleArray(value) => write!(f, "double{value}"),
            Reference::Array(object_array) => {
                let length = object_array.elements.len().unwrap_or_default();
                write!(f, "{}[{length}]", object_array.class.array_component_type())
            }
            Reference::Object(value) => {
                write!(f, "{value}")
            }
        }
    }
}

impl Trace for Reference {
    fn trace(&self, collector: &GarbageCollector) {
        match self {
            Reference::Array(object_array) => {
                let references = object_array
                    .elements
                    .as_ref()
                    .expect("object_array.elements");
                for reference in references.iter().flatten() {
                    reference.trace(collector);
                }
            }
            Reference::Object(object) => object.trace(collector),
            _ => {}
        }
    }
}

impl Eq for Reference {}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::ByteArray(a), Reference::ByteArray(b)) => a == b,
            (Reference::CharArray(a), Reference::CharArray(b)) => a == b,
            (Reference::ShortArray(a), Reference::ShortArray(b)) => a == b,
            (Reference::IntArray(a), Reference::IntArray(b)) => a == b,
            (Reference::LongArray(a), Reference::LongArray(b)) => a == b,
            (Reference::FloatArray(a), Reference::FloatArray(b)) => {
                let a = a.as_ref().expect("a.Vec<f32>");
                let b = b.as_ref().expect("b.Vec<f32>");
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(&a, &b)| a.to_bits() == b.to_bits())
            }
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => {
                let a = a.as_ref().expect("a.Vec<f64>");
                let b = b.as_ref().expect("b.Vec<f64>");
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(&a, &b)| a.to_bits() == b.to_bits())
            }
            (Reference::Array(a), Reference::Array(b)) => {
                a.class == b.class && a.elements == b.elements
            }
            (Reference::Object(a), Reference::Object(b)) => a == b,
            _ => false,
        }
    }
}

impl From<Vec<bool>> for Reference {
    fn from(value: Vec<bool>) -> Self {
        let value: Vec<i8> = value.into_iter().map(i8::from).collect();
        Reference::ByteArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i8>> for Reference {
    fn from(value: Vec<i8>) -> Self {
        Reference::ByteArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u8>> for Reference {
    fn from(value: Vec<u8>) -> Self {
        let value: &[i8] = transmute_ref!(value.as_slice());
        Reference::ByteArray(ConcurrentVec::from(value.to_vec()))
    }
}

impl From<Vec<char>> for Reference {
    fn from(value: Vec<char>) -> Self {
        let value: Vec<u16> = value.into_iter().map(|v| v as u16).collect();
        Reference::CharArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i16>> for Reference {
    fn from(value: Vec<i16>) -> Self {
        Reference::ShortArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u16>> for Reference {
    fn from(value: Vec<u16>) -> Self {
        let value: &[i16] = transmute_ref!(value.as_slice());
        Reference::ShortArray(ConcurrentVec::from(value.to_vec()))
    }
}

impl From<Vec<i32>> for Reference {
    fn from(value: Vec<i32>) -> Self {
        Reference::IntArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u32>> for Reference {
    fn from(value: Vec<u32>) -> Self {
        let value: &[i32] = transmute_ref!(value.as_slice());
        Reference::IntArray(ConcurrentVec::from(value.to_vec()))
    }
}

impl From<Vec<i64>> for Reference {
    fn from(value: Vec<i64>) -> Self {
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u64>> for Reference {
    fn from(value: Vec<u64>) -> Self {
        let value: &[i64] = transmute_ref!(value.as_slice());
        Reference::LongArray(ConcurrentVec::from(value.to_vec()))
    }
}

impl From<Vec<isize>> for Reference {
    fn from(value: Vec<isize>) -> Self {
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<usize>> for Reference {
    fn from(value: Vec<usize>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<f32>> for Reference {
    fn from(value: Vec<f32>) -> Self {
        Reference::FloatArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<f64>> for Reference {
    fn from(value: Vec<f64>) -> Self {
        Reference::DoubleArray(ConcurrentVec::from(value))
    }
}

impl From<(Arc<Class>, Vec<Option<Reference>>)> for Reference {
    fn from((class, value): (Arc<Class>, Vec<Option<Reference>>)) -> Self {
        let object_array = ObjectArray {
            class: class.clone(),
            elements: ConcurrentVec::from(value),
        };
        Reference::Array(object_array)
    }
}

impl TryFrom<(Arc<Class>, Vec<Value>)> for Reference {
    type Error = crate::Error;

    fn try_from(value: (Arc<Class>, Vec<Value>)) -> Result<Self> {
        let (class, values) = value;
        let mut references = Vec::with_capacity(values.len());

        for value in values {
            let Value::Object(reference) = value else {
                return Err(InvalidValueType("Expected object".to_string()));
            };
            references.push(reference);
        }

        let object_array = ObjectArray {
            class,
            elements: ConcurrentVec::from(references),
        };
        Ok(Reference::Array(object_array))
    }
}

impl From<Object> for Reference {
    fn from(value: Object) -> Self {
        Reference::Object(value)
    }
}

impl TryInto<Vec<bool>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<bool>> {
        let value = self.as_byte_vec_ref()?.to_vec();
        let value = value.into_iter().map(|v| v != 0).collect();
        Ok(value)
    }
}

impl TryInto<Vec<char>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<char>> {
        let values = self.as_char_vec_ref()?.to_vec();
        let mut result = Vec::with_capacity(values.len());
        for value in values {
            let value = u32::from(value);
            let value = char::from_u32(value)
                .ok_or(InvalidValueType(format!("Invalid char value {value}")))?;
            result.push(value);
        }
        Ok(result)
    }
}

impl TryInto<Vec<i8>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i8>> {
        Ok(self.as_byte_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<u8>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u8>> {
        let value = self.as_byte_vec_ref()?.to_vec();
        let value: &[u8] = transmute_ref!(value.as_slice());
        Ok(value.to_vec())
    }
}

impl TryInto<Vec<i16>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i16>> {
        Ok(self.as_short_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<u16>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u16>> {
        let value = self.as_short_vec_ref()?.to_vec();
        let value: &[u16] = transmute_ref!(value.as_slice());
        Ok(value.to_vec())
    }
}

impl TryInto<Vec<i32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i32>> {
        Ok(self.as_int_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<u32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u32>> {
        let value = self.as_int_vec_ref()?.to_vec();
        let value: &[u32] = transmute_ref!(value.as_slice());
        Ok(value.to_vec())
    }
}

impl TryInto<Vec<i64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i64>> {
        Ok(self.as_long_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<u64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u64>> {
        let value = self.as_long_vec_ref()?.to_vec();
        let value: &[u64] = transmute_ref!(value.as_slice());
        Ok(value.to_vec())
    }
}

impl TryInto<Vec<isize>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<isize>> {
        let value: Vec<i64> = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        let value = value.into_iter().map(|v| v as isize).collect();
        Ok(value)
    }
}

impl TryInto<Vec<usize>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<usize>> {
        let value: Vec<u64> = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        let value = value.into_iter().map(|v| v as usize).collect();
        Ok(value)
    }
}

impl TryInto<Vec<f32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f32>> {
        Ok(self.as_float_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<f64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f64>> {
        Ok(self.as_double_vec_ref()?.to_vec())
    }
}

impl TryInto<Vec<Value>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<Value>> {
        let (_class, values) = self.as_class_vec_ref()?;
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

impl TryInto<(Arc<Class>, Vec<Option<Reference>>)> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<(Arc<Class>, Vec<Option<Reference>>)> {
        let (class, references) = self.as_class_vec_ref()?;
        Ok((class.clone(), references.to_vec()))
    }
}

impl TryInto<bool> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<bool> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<char> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<char> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<i8> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i8> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<u8> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u8> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<i16> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i16> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<u16> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u16> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<i32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i32> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<u32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u32> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<i64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i64> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<u64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u64> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<isize> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<isize> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<usize> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<usize> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<f32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<f32> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<f64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<f64> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

impl TryInto<Object> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Object> {
        match self {
            Reference::Object(object) => Ok(object),
            _ => Err(InvalidValueType("Expected object".to_string())),
        }
    }
}

impl TryInto<String> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<String> {
        let object: Object = self.try_into()?;
        object.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Result, Value, runtime};
    use std::sync::Arc;

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    #[test]
    fn test_display_byte_array() {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        assert_eq!(reference.class_name(), "[B");
        assert_eq!(reference.to_string(), "byte[1, 2, 3]");
    }

    #[test]
    fn test_trace_byte_array() {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        let collector = GarbageCollector::new();
        reference.trace(&collector);
    }

    #[tokio::test]
    async fn test_trace_class_vec() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class.clone())?;
        let values = vec![Some(Reference::Object(object))];
        let reference = Reference::from((class, values));
        let collector = GarbageCollector::new();
        reference.trace(&collector);
        Ok(())
    }

    #[tokio::test]
    async fn test_trace_object() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        let collector = GarbageCollector::new();
        reference.trace(&collector);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref() -> Result<()> {
        let original_value = vec![42i8];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_byte_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_byte_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_byte_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42i8]);
        {
            let mut mutable_reference = reference.as_byte_vec_mut()?;
            mutable_reference.push(3i8);
        }
        assert_eq!(reference.as_byte_vec_ref()?.to_vec(), vec![42i8, 3i8]);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_mut_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.as_byte_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_char_array() {
        let reference = Reference::from(vec![1 as char, 2 as char, 3 as char]);
        assert_eq!(reference.class_name(), "[C");
        assert_eq!(reference.to_string(), "char[1, 2, 3]");
    }

    #[test]
    fn test_as_char_vec_ref() -> Result<()> {
        let reference = Reference::from(vec!['*']);
        assert_eq!(vec![42u16], reference.as_char_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_char_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_char_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_char_vec_mut() -> Result<()> {
        let reference = Reference::from(vec!['*']);
        {
            let mut mutable_reference = reference.as_char_vec_mut()?;
            mutable_reference.push(50u16);
        }
        assert_eq!(reference.as_char_vec_ref()?.to_vec(), vec![42u16, 50u16]);
        Ok(())
    }

    #[test]
    fn test_as_char_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_char_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_short_array() {
        let reference = Reference::from(vec![1i16, 2i16, 3i16]);
        assert_eq!(reference.class_name(), "[S");
        assert_eq!(reference.to_string(), "short[1, 2, 3]");
    }

    #[test]
    fn test_as_short_vec_ref() -> Result<()> {
        let original_value = vec![42i16];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_short_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_short_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_short_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_short_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42i16]);
        {
            let mut mutable_reference = reference.as_short_vec_mut()?;
            mutable_reference.push(3i16);
        }
        assert_eq!(reference.as_short_vec_ref()?.to_vec(), vec![42i16, 3i16]);
        Ok(())
    }

    #[test]
    fn test_as_short_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_short_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_int_array() {
        let reference = Reference::from(vec![1i32, 2i32, 3i32]);
        assert_eq!(reference.class_name(), "[I");
        assert_eq!(reference.to_string(), "int[1, 2, 3]");
    }

    #[test]
    fn test_as_int_vec_ref() -> Result<()> {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_int_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_int_vec_ref_error() {
        let reference = Reference::from(vec![42i8]);
        let result = reference.as_int_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_int_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42i32]);
        {
            let mut mutable_reference = reference.as_int_vec_mut()?;
            mutable_reference.push(3i32);
        }
        assert_eq!(reference.as_int_vec_ref()?.to_vec(), vec![42i32, 3i32]);
        Ok(())
    }

    #[test]
    fn test_as_int_vec_mut_error() {
        let reference = Reference::from(vec![42i8]);
        let result = reference.as_int_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_long_array() {
        let reference = Reference::from(vec![1i64, 2i64, 3i64]);
        assert_eq!(reference.class_name(), "[J");
        assert_eq!(reference.to_string(), "long[1, 2, 3]");
    }

    #[test]
    fn test_as_long_vec_ref() -> Result<()> {
        let original_value = vec![42i64];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_long_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_long_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_long_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_long_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42i64]);
        {
            let mut mutable_reference = reference.as_long_vec_mut()?;
            mutable_reference.push(3i64);
        }
        assert_eq!(reference.as_long_vec_ref()?.to_vec(), vec![42i64, 3i64]);
        Ok(())
    }

    #[test]
    fn test_as_long_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_long_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_float_array() {
        let reference = Reference::from(vec![1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(reference.class_name(), "[F");
        assert_eq!(reference.to_string(), "float[1.0, 2.0, 3.0]");
    }

    #[test]
    fn test_as_float_vec_ref() -> Result<()> {
        let original_value = vec![42.1f32];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_float_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_float_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_float_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_float_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42.1f32]);
        {
            let mut mutable_reference = reference.as_float_vec_mut()?;
            mutable_reference.push(3.45f32);
        }
        assert_eq!(
            reference.as_float_vec_ref()?.to_vec(),
            vec![42.1f32, 3.45f32]
        );
        Ok(())
    }

    #[test]
    fn test_as_float_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_float_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_double_array() {
        let reference = Reference::from(vec![1.0f64, 2.0f64, 3.0f64]);
        assert_eq!(reference.class_name(), "[D");
        assert_eq!(reference.to_string(), "double[1.0, 2.0, 3.0]");
    }

    #[test]
    fn test_as_double_vec_ref() -> Result<()> {
        let original_value = vec![42.1f64];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.as_double_vec_ref()?.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_double_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_double_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_as_double_vec_mut() -> Result<()> {
        let reference = Reference::from(vec![42.1f64]);
        {
            let mut mutable_reference = reference.as_double_vec_mut()?;
            mutable_reference.push(3.45f64);
        }
        assert_eq!(
            reference.as_double_vec_ref()?.to_vec(),
            vec![42.1f64, 3.45f64]
        );
        Ok(())
    }

    #[test]
    fn test_as_double_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_double_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_display_reference_array() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let reference = Reference::from((class, vec![None]));
        assert_eq!(reference.class_name(), "[Ljava/lang/Object;");
        assert_eq!(reference.to_string(), "java/lang/Object[1]");
        Ok(())
    }

    #[tokio::test]
    async fn test_as_class_vec_ref() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![None];
        let reference = Reference::from((original_class.clone(), original_value.clone()));
        let (class, value) = reference.as_class_vec_ref()?;
        assert_eq!(&original_class, class);
        assert_eq!(original_value, value.to_vec());
        Ok(())
    }

    #[test]
    fn test_as_class_vec_ref_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_class_vec_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_as_class_vec_mut() -> Result<()> {
        let object_class = load_class("[Ljava/lang/Object;").await?;
        let reference = Reference::from((object_class.clone(), vec![]));
        {
            let (_class, mut mutable_reference) = reference.as_class_vec_mut()?;
            mutable_reference.push(None);
        }
        let (_class, array) = reference.as_class_vec_ref()?;
        assert_eq!(array.to_vec(), vec![None]);
        Ok(())
    }

    #[test]
    fn test_as_class_vec_mut_error() {
        let reference = Reference::from(vec![42i32]);
        let result = reference.as_class_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_display_reference() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert_eq!(reference.class_name(), "java/lang/Object");
        assert!(
            reference
                .to_string()
                .starts_with("Object(class java/lang/Object)")
        );
        assert!(reference.to_string().contains("java/lang/Object"));
        Ok(())
    }

    #[tokio::test]
    async fn test_as_object_ref() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object.clone());
        let result = reference.as_object_ref()?;
        assert_eq!(&object, result);
        Ok(())
    }

    #[test]
    fn test_as_object_ref_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.as_object_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_hash_code_byte_array() {
        let reference = Reference::from(vec![1i8]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_byte_array() {
        let reference1 = Reference::from(vec![1i8]);
        let reference2 = Reference::from(vec![1i8]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::ByteArray(ref array) = clone else {
            unreachable!("Expected byte array");
        };
        array.set(0, 2i8)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::ByteArray(ref array) = clone else {
            unreachable!("Expected byte array");
        };
        array.set(0, 2i8)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_char_array() {
        let reference = Reference::from(vec![1 as char]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_char_array() {
        let reference1 = Reference::from(vec![1 as char]);
        let reference2 = Reference::from(vec![1 as char]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::CharArray(ref array) = clone else {
            unreachable!("Expected char array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::CharArray(ref array) = clone else {
            unreachable!("Expected char array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_short_array() {
        let reference = Reference::from(vec![1i16]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_short_array() {
        let reference1 = Reference::from(vec![1i16]);
        let reference2 = Reference::from(vec![1i16]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::ShortArray(ref array) = clone else {
            unreachable!("Expected short array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::ShortArray(ref array) = clone else {
            unreachable!("Expected short array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_int_array() {
        let reference = Reference::from(vec![1i32]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_int_array() {
        let reference1 = Reference::from(vec![1i32]);
        let reference2 = Reference::from(vec![1i32]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::IntArray(ref array) = clone else {
            unreachable!("Expected int array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::IntArray(ref array) = clone else {
            unreachable!("Expected int array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_long_array() {
        let reference = Reference::from(vec![1i64]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_long_array() {
        let reference1 = Reference::from(vec![1i64]);
        let reference2 = Reference::from(vec![1i64]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::LongArray(ref array) = clone else {
            unreachable!("Expected long array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::LongArray(ref array) = clone else {
            unreachable!("Expected long array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_float_array() {
        let reference = Reference::from(vec![1.0f32]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_float_array() {
        let reference1 = Reference::from(vec![1.0f32]);
        let reference2 = Reference::from(vec![1.0f32]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::FloatArray(ref array) = clone else {
            unreachable!("Expected float array");
        };
        array.set(0, 2.0)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::FloatArray(ref array) = clone else {
            unreachable!("Expected float array");
        };
        array.set(0, 2.0)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_double_array() {
        let reference = Reference::from(vec![1.0f64]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_ptr_eq_double_array() {
        let reference1 = Reference::from(vec![1.0f64]);
        let reference2 = Reference::from(vec![1.0f64]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::DoubleArray(ref array) = clone else {
            unreachable!("Expected double array");
        };
        array.set(0, 2.0)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::DoubleArray(ref array) = clone else {
            unreachable!("Expected double array");
        };
        array.set(0, 2.0)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_code_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::from((class.clone(), vec![None]));
        assert_ne!(0, reference.hash_code());
        Ok(())
    }

    #[tokio::test]
    async fn test_ptr_eq_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from((class.clone(), vec![None]));
        let reference2 = Reference::from((class.clone(), vec![None]));
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::from((class.clone(), vec![None]));
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::Array(ref object_array) = clone else {
            unreachable!("Expected reference array");
        };
        object_array
            .elements
            .set(0, Some(Reference::from(vec![1i8])))?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::from((class.clone(), vec![None]));
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::Array(ref object_array) = clone else {
            unreachable!("Expected reference array");
        };
        object_array
            .elements
            .set(0, Some(Reference::from(vec![1i8])))?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_code_object() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::from(Object::new(class.clone())?);
        assert_ne!(0, reference.hash_code());
        Ok(())
    }

    #[tokio::test]
    async fn test_ptr_eq_object() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from(Object::new(class.clone())?);
        let reference2 = Reference::from(Object::new(class)?);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(reference1.ptr_eq(&reference3));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::Object(ref cloned_object) = clone else {
            unreachable!("Expected object");
        };
        cloned_object.set_value("value", Value::Int(2))?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::Object(ref cloned_object) = clone else {
            unreachable!("Expected object");
        };
        cloned_object.set_value("value", Value::Int(2))?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from((class.clone(), vec![]));
        let reference2 = Reference::from((class, vec![]));
        assert_eq!(reference1, reference2);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq_class_ne() -> Result<()> {
        let object_class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from((object_class, vec![]));
        let string_class = load_class("java.lang.String").await?;
        let reference2 = Reference::from((string_class, vec![]));
        assert_ne!(reference1, reference2);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq_value_ne() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from((class.clone(), vec![]));
        let reference2 = Reference::from((class, vec![None]));
        assert_ne!(reference1, reference2);
        Ok(())
    }

    #[test]
    fn test_byte_array_eq() {
        let reference1 = Reference::from(vec![42i8]);
        let reference2 = Reference::from(vec![42i8]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_byte_array_ne() {
        let reference1 = Reference::from(vec![3i8]);
        let reference2 = Reference::from(vec![42i8]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_char_array_eq() {
        let reference1 = Reference::from(vec![42 as char]);
        let reference2 = Reference::from(vec![42 as char]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_char_array_ne() {
        let reference1 = Reference::from(vec![3 as char]);
        let reference2 = Reference::from(vec![42 as char]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_double_array_eq() {
        let reference1 = Reference::from(vec![42.1f64]);
        let reference2 = Reference::from(vec![42.1f64]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_double_array_ne() {
        let reference1 = Reference::from(vec![3.1f64]);
        let reference2 = Reference::from(vec![42.1f64]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_float_array_eq() {
        let reference1 = Reference::from(vec![42.1f32]);
        let reference2 = Reference::from(vec![42.1f32]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_float_array_ne() {
        let reference1 = Reference::from(vec![3.1f32]);
        let reference2 = Reference::from(vec![42.1f32]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_int_array_eq() {
        let reference1 = Reference::from(vec![42i32]);
        let reference2 = Reference::from(vec![42i32]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_int_array_ne() {
        let reference1 = Reference::from(vec![3i32]);
        let reference2 = Reference::from(vec![42i32]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_long_array_eq() {
        let reference1 = Reference::from(vec![42i64]);
        let reference2 = Reference::from(vec![42i64]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_long_array_ne() {
        let reference1 = Reference::from(vec![3i64]);
        let reference2 = Reference::from(vec![42i64]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_short_array_eq() {
        let reference1 = Reference::from(vec![42i16]);
        let reference2 = Reference::from(vec![42i16]);
        assert_eq!(reference1, reference2);
    }

    #[test]
    fn test_short_array_ne() {
        let reference1 = Reference::from(vec![3i16]);
        let reference2 = Reference::from(vec![42i16]);
        assert_ne!(reference1, reference2);
    }

    #[tokio::test]
    async fn test_object_eq() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from(Object::new(class.clone())?);
        let reference2 = Reference::from(Object::new(class)?);
        assert_eq!(reference1, reference2);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_ne() -> Result<()> {
        let object_class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from(Object::new(object_class)?);
        let string_class = load_class("java.lang.String").await?;
        let reference2 = Reference::from(Object::new(string_class)?);
        assert_ne!(reference1, reference2);
        Ok(())
    }

    #[test]
    fn test_different_types_ne() {
        let reference1 = Reference::from(vec![42i32]);
        let reference2 = Reference::from(vec![42i64]);
        assert_ne!(reference1, reference2);
    }

    #[test]
    fn test_from_vec_bool() {
        let reference = Reference::from(vec![true]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_i8() {
        let reference = Reference::from(vec![0i8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_u8() {
        let reference = Reference::from(vec![0u8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_char() {
        let reference = Reference::from(vec!['a']);
        assert!(matches!(reference, Reference::CharArray(_)));
    }

    #[test]
    fn test_from_vec_i16() {
        let reference = Reference::from(vec![0i16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_vec_u16() {
        let reference = Reference::from(vec![0u16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_vec_i32() {
        let reference = Reference::from(vec![0i32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_vec_u32() {
        let reference = Reference::from(vec![0u32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_vec_i64() {
        let reference = Reference::from(vec![0i64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_u64() {
        let reference = Reference::from(vec![0u64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_isize() {
        let reference = Reference::from(vec![0isize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_usize() {
        let reference = Reference::from(vec![0usize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_f32() {
        let reference = Reference::from(vec![0.0f32]);
        assert!(matches!(reference, Reference::FloatArray(_)));
    }

    #[test]
    fn test_from_vec_f64() {
        let reference = Reference::from(vec![0.0f64]);
        assert!(matches!(reference, Reference::DoubleArray(_)));
    }

    #[tokio::test]
    async fn test_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![None];
        let reference = Reference::from((original_class.clone(), original_value.clone()));
        assert!(matches!(reference, Reference::Array(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java/lang/Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        assert!(matches!(reference, Reference::Array(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec_error() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let value = Value::from(42);
        let original_value = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_value.clone()));
        assert!(matches!(reference, Err(InvalidValueType(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_from_vec_object() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert!(matches!(reference, Reference::Object(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_bool() -> Result<()> {
        let original_value = vec![true];
        let reference = Reference::from(original_value.clone());
        let value: Vec<bool> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_char() -> Result<()> {
        let original_value = vec!['*'];
        let reference = Reference::from(original_value.clone());
        let value: Vec<char> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i8() -> Result<()> {
        let original_value = vec![42i8];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i8> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u8() -> Result<()> {
        let original_value = vec![42u8];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u8> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i16() -> Result<()> {
        let original_value = vec![42i16];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i16> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u16() -> Result<()> {
        let original_value = vec![42u16];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u16> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i32() -> Result<()> {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u32() -> Result<()> {
        let original_value = vec![42u32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i64() -> Result<()> {
        let original_value = vec![42i64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u64() -> Result<()> {
        let original_value = vec![42u64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_isize() -> Result<()> {
        let original_value = vec![42isize];
        let reference = Reference::from(original_value.clone());
        let value: Vec<isize> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_usize() -> Result<()> {
        let original_value = vec![42usize];
        let reference = Reference::from(original_value.clone());
        let value: Vec<usize> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f32() -> Result<()> {
        let original_value = vec![42.1f32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<f32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f64() -> Result<()> {
        let original_value = vec![42.1f64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<f64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_value() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class.clone())?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        let values: Vec<Value> = reference.try_into()?;
        assert_eq!(original_values, values);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class.clone())?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        let (reference_class, reference_values) = reference.try_into()?;
        assert_eq!(original_class.name(), reference_class.name());
        assert_eq!(original_values.len(), reference_values.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_bool() -> Result<()> {
        let class = load_class("java/lang/Boolean").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let value: bool = reference.try_into()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char() -> Result<()> {
        let class = load_class("java/lang/Character").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: char = reference.try_into()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i8 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u8 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i16 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u16 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i32 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u32 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: i64 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: u64 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_isize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: isize = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_usize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: usize = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f32() -> Result<()> {
        let class = load_class("java/lang/Float").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let reference = Reference::from(object);
        let value: f32 = reference.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f64() -> Result<()> {
        let class = load_class("java/lang/Double").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let reference = Reference::from(object);
        let value: f64 = reference.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_object() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let reference = Reference::from(Object::new(class)?);
        let object: Object = reference.try_into()?;
        assert_eq!("java/lang/Object", object.class().name());
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_object_error() -> Result<()> {
        let array = vec![42];
        let reference = Reference::from(array);
        let result: Result<Object> = reference.try_into();
        assert!(matches!(result, Err(InvalidValueType(_))));
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_try_into_string() -> Result<()> {
        let class = load_class("java/lang/String").await?;
        let object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }
}

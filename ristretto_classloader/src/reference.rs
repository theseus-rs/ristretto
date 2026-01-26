use crate::Error::InvalidValueType;
use crate::{Class, Object, Result, Value};
use ristretto_gc::{GarbageCollector, Trace};
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use zerocopy::transmute_ref;

/// Represents an array of objects in the Ristretto VM.
///
/// `ObjectArray` groups the array's class `Arc<Class>` and its elements
/// `Vec<Value>` together in order to reduce the amount of memory required by
/// values in the Reference enum.
#[derive(Clone, Debug)]
pub struct ObjectArray {
    pub class: Arc<Class>,
    pub elements: Box<[Value]>,
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
    BooleanArray(Box<[i8]>),
    ByteArray(Box<[i8]>),
    CharArray(Box<[u16]>),
    ShortArray(Box<[i16]>),
    IntArray(Box<[i32]>),
    LongArray(Box<[i64]>),
    FloatArray(Box<[f32]>),
    DoubleArray(Box<[f64]>),
    Array(ObjectArray),
    Object(Object),
}

impl Reference {
    /// Get the class name of the reference
    ///
    /// # Errors
    ///
    /// if the `Object` read lock is poisoned
    pub fn class_name(&self) -> Result<String> {
        let class_name = match self {
            Reference::BooleanArray(_) => "[Z".to_string(),
            Reference::ByteArray(_) => "[B".to_string(),
            Reference::CharArray(_) => "[C".to_string(),
            Reference::ShortArray(_) => "[S".to_string(),
            Reference::IntArray(_) => "[I".to_string(),
            Reference::LongArray(_) => "[J".to_string(),
            Reference::FloatArray(_) => "[F".to_string(),
            Reference::DoubleArray(_) => "[D".to_string(),
            Reference::Array(object_array) => object_array.class.name().to_string(),
            Reference::Object(object) => object.class().name().to_string(),
        };
        Ok(class_name)
    }

    /// Returns the reference to `Vec<bool>` (stored as i8).
    ///
    /// # Errors
    ///
    /// if the value is not a `BooleanArray`.
    pub fn as_bool_vec_ref(&self) -> Result<&[i8]> {
        match self {
            Reference::BooleanArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected boolean array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<bool>` (stored as i8).
    ///
    /// # Errors
    ///
    /// if the value is not a `BooleanArray`.
    pub fn as_bool_vec_mut(&mut self) -> Result<&mut [i8]> {
        match self {
            Reference::BooleanArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected boolean array".to_string())),
        }
    }

    /// Returns the reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_ref(&self) -> Result<&[i8]> {
        match self {
            Reference::ByteArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected byte array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_mut(&mut self) -> Result<&mut [i8]> {
        match self {
            Reference::ByteArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected byte array".to_string())),
        }
    }

    /// Returns a reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_ref(&self) -> Result<&[u16]> {
        match self {
            Reference::CharArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected char array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<char>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `CharArray`.
    pub fn as_char_vec_mut(&mut self) -> Result<&mut [u16]> {
        match self {
            Reference::CharArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected char array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_ref(&self) -> Result<&[i16]> {
        match self {
            Reference::ShortArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected short array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i16>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ShortArray`.
    pub fn as_short_vec_mut(&mut self) -> Result<&mut [i16]> {
        match self {
            Reference::ShortArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected short array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_ref(&self) -> Result<&[i32]> {
        match self {
            Reference::IntArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected int array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `IntArray`.
    pub fn as_int_vec_mut(&mut self) -> Result<&mut [i32]> {
        match self {
            Reference::IntArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected int array".to_string())),
        }
    }

    /// Returns a reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_ref(&self) -> Result<&[i64]> {
        match self {
            Reference::LongArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected long array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<i64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `LongArray`.
    pub fn as_long_vec_mut(&mut self) -> Result<&mut [i64]> {
        match self {
            Reference::LongArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected long array".to_string())),
        }
    }

    /// Returns a reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_ref(&self) -> Result<&[f32]> {
        match self {
            Reference::FloatArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected float array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<f32>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `FloatArray`.
    pub fn as_float_vec_mut(&mut self) -> Result<&mut [f32]> {
        match self {
            Reference::FloatArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected float array".to_string())),
        }
    }

    /// Returns a reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_ref(&self) -> Result<&[f64]> {
        match self {
            Reference::DoubleArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected double array".to_string())),
        }
    }

    /// Returns a mutable reference to`Vec<f64>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `DoubleArray`.
    pub fn as_double_vec_mut(&mut self) -> Result<&mut [f64]> {
        match self {
            Reference::DoubleArray(array) => Ok(array),
            _ => Err(InvalidValueType("Expected double array".to_string())),
        }
    }

    /// Returns a reference to `Vec<Option<Reference>>`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Array`.
    pub fn as_class_vec_ref(&self) -> Result<(&Arc<Class>, &[Value])> {
        match self {
            Reference::Array(object_array) => Ok((&object_array.class, &object_array.elements)),
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns a mutable reference to `Vec<Value>`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Array`.
    pub fn as_class_vec_mut(&mut self) -> Result<(&Arc<Class>, &mut [Value])> {
        match self {
            Reference::Array(object_array) => Ok((&object_array.class, &mut object_array.elements)),
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

    /// Returns a mutable reference to the `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Object`.
    pub fn as_object_mut(&mut self) -> Result<&mut Object> {
        match self {
            Reference::Object(object) => Ok(object),
            _ => Err(InvalidValueType("Expected object".to_string())),
        }
    }

    /// Returns a read-only byte slice view of the underlying primitive array data. This provides
    /// raw byte access to primitive arrays for low-level memory operations.
    ///
    /// Returns `None` if the reference is not a primitive array type (i.e., it's an `Object` or
    /// object `Array`).
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Reference::BooleanArray(array) | Reference::ByteArray(array) => {
                Some(bytemuck::cast_slice(array.as_ref()))
            }
            Reference::CharArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::ShortArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::IntArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::LongArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::FloatArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::DoubleArray(array) => Some(bytemuck::cast_slice(array.as_ref())),
            Reference::Array(_) | Reference::Object(_) => None,
        }
    }

    /// Returns a mutable byte slice view of the underlying primitive array data. This provides raw
    /// byte access to primitive arrays for low-level memory operations.
    ///
    /// Returns `None` if the reference is not a primitive array type (i.e., it's an `Object`
    /// or object `Array`).
    #[must_use]
    pub fn as_bytes_mut(&mut self) -> Option<&mut [u8]> {
        match self {
            Reference::BooleanArray(array) | Reference::ByteArray(array) => {
                Some(bytemuck::cast_slice_mut(array.as_mut()))
            }
            Reference::CharArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::ShortArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::IntArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::LongArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::FloatArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::DoubleArray(array) => Some(bytemuck::cast_slice_mut(array.as_mut())),
            Reference::Array(_) | Reference::Object(_) => None,
        }
    }

    /// Returns hash code implementation based on memory address. This is used by the Java
    /// `Object.hash_code()` implementation.
    #[must_use]
    pub fn hash_code(&self) -> usize {
        match self {
            Reference::BooleanArray(array) | Reference::ByteArray(array) => array.as_ptr() as usize,
            Reference::CharArray(array) => array.as_ptr() as usize,
            Reference::ShortArray(array) => array.as_ptr() as usize,
            Reference::IntArray(array) => array.as_ptr() as usize,
            Reference::LongArray(array) => array.as_ptr() as usize,
            Reference::FloatArray(array) => array.as_ptr() as usize,
            Reference::DoubleArray(array) => array.as_ptr() as usize,
            Reference::Array(object_array) => object_array.elements.as_ptr() as usize,
            Reference::Object(object) => std::ptr::from_ref::<Object>(object) as usize,
        }
    }

    /// Check if two references point to the same memory location.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::BooleanArray(a), Reference::BooleanArray(b))
            | (Reference::ByteArray(a), Reference::ByteArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::CharArray(a), Reference::CharArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::ShortArray(a), Reference::ShortArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::IntArray(a), Reference::IntArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::LongArray(a), Reference::LongArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::FloatArray(a), Reference::FloatArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => a.as_ptr() == b.as_ptr(),
            (Reference::Array(a), Reference::Array(b)) => {
                Arc::ptr_eq(&a.class, &b.class) && a.elements.as_ptr() == b.elements.as_ptr()
            }
            (Reference::Object(a), Reference::Object(b)) => a.ptr_eq(b),
            _ => false,
        }
    }

    /// Convert the object to a bool value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Boolean`.
    /// - if the value cannot be converted to a boolean.
    pub fn as_bool(&self) -> Result<bool> {
        let object = self.as_object_ref()?;
        object.as_bool()
    }

    /// Convert the object to a character value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Character`.
    /// - if the value cannot be converted to a character.
    pub fn as_char(&self) -> Result<char> {
        let object = self.as_object_ref()?;
        object.as_char()
    }

    /// Convert the object to a signed byte value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Byte`.
    /// - if the value cannot be converted to a signed byte.
    pub fn as_i8(&self) -> Result<i8> {
        let object = self.as_object_ref()?;
        object.as_i8()
    }

    /// Convert the object to an unsigned byte value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Byte`.
    /// - if the value cannot be converted to an unsigned byte.
    pub fn as_u8(&self) -> Result<u8> {
        let object = self.as_object_ref()?;
        object.as_u8()
    }

    /// Convert the object to a signed short value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Short`.
    /// - if the value cannot be converted to a signed short.
    pub fn as_i16(&self) -> Result<i16> {
        let object = self.as_object_ref()?;
        object.as_i16()
    }

    /// Convert the object to an unsigned short value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Short`.
    /// - if the value cannot be converted to an unsigned short.
    pub fn as_u16(&self) -> Result<u16> {
        let object = self.as_object_ref()?;
        object.as_u16()
    }

    /// Convert the object to a signed integer value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Integer`.
    /// - if the value cannot be converted to a signed integer.
    pub fn as_i32(&self) -> Result<i32> {
        let object = self.as_object_ref()?;
        object.as_i32()
    }

    /// Convert the object to an unsigned integer value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Integer`.
    /// - if the value cannot be converted to an unsigned integer.
    pub fn as_u32(&self) -> Result<u32> {
        let object = self.as_object_ref()?;
        object.as_u32()
    }

    /// Convert the object to a signed long value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to a signed long.
    pub fn as_i64(&self) -> Result<i64> {
        let object = self.as_object_ref()?;
        object.as_i64()
    }

    /// Convert the object to an unsigned long value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to an unsigned long.
    pub fn as_u64(&self) -> Result<u64> {
        let object = self.as_object_ref()?;
        object.as_u64()
    }

    /// Convert the object to a signed isize value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to a signed isize.
    pub fn as_isize(&self) -> Result<isize> {
        let object = self.as_object_ref()?;
        object.as_isize()
    }

    /// Convert the object to an unsigned usize value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Long`.
    /// - if the value cannot be converted to an unsigned usize.
    pub fn as_usize(&self) -> Result<usize> {
        let object = self.as_object_ref()?;
        object.as_usize()
    }

    /// Convert the object to a 32-bit floating point value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Float`.
    /// - if the value cannot be converted to a 32-bit floating point value.
    pub fn as_f32(&self) -> Result<f32> {
        let object = self.as_object_ref()?;
        object.as_f32()
    }

    /// Convert the object to a 64-bit floating point value.
    ///
    /// # Errors
    ///
    /// - if the reference is not an `Object`.
    /// - if the object is not an instance of `java/lang/Double`.
    /// - if the value cannot be converted to a 64-bit floating point value.
    pub fn as_f64(&self) -> Result<f64> {
        let object = self.as_object_ref()?;
        object.as_f64()
    }

    /// Attempts to convert the reference into a `String`.
    ///
    /// # Errors
    ///
    /// if the reference is not an `Object` or if the object cannot be converted to a `String`.
    pub fn as_string(&self) -> Result<String> {
        let object = self.as_object_ref()?;
        object.as_string()
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_vec<T: Debug>(f: &mut fmt::Formatter<'_>, name: &str, value: &[T]) -> fmt::Result {
            let slice = value;
            let mut values = Vec::with_capacity(slice.len());
            for value in slice {
                let value = format!("{value:?}");
                if value.len() > 100 {
                    values.push(format!("{}...", &value[..97]));
                } else {
                    values.push(value);
                }
            }
            write!(f, "{name}[{}]", values.join(", "))
        }

        match self {
            Reference::BooleanArray(array) => fmt_vec(f, "boolean", array),
            Reference::ByteArray(array) => fmt_vec(f, "byte", array),
            Reference::CharArray(array) => fmt_vec(f, "char", array),
            Reference::ShortArray(array) => fmt_vec(f, "short", array),
            Reference::IntArray(array) => fmt_vec(f, "int", array),
            Reference::LongArray(array) => fmt_vec(f, "long", array),
            Reference::FloatArray(array) => fmt_vec(f, "float", array),
            Reference::DoubleArray(array) => fmt_vec(f, "double", array),
            Reference::Array(object_array) => {
                write!(
                    f,
                    "{}[{}]",
                    object_array.class.array_component_type(),
                    object_array.elements.len()
                )
            }
            Reference::Object(object) => {
                write!(f, "{object}")
            }
        }
    }
}

impl Eq for Reference {}

impl Hash for Reference {
    /// Computes a hash for the `Reference` instance.  Handles the following cases:
    ///
    /// - `ByteArray`: hashes the byte array.
    /// - `CharArray`: hashes the character array.
    /// - `ShortArray`: hashes the short array.
    /// - `IntArray`: hashes the integer array.
    /// - `LongArray`: hashes the long array.
    /// - `FloatArray`: hashes the float array by converting each float to its bit representation.
    /// - `DoubleArray`: hashes the double array by converting each double to its bit representation.
    /// - `Array`: hashes the object array by hashing its class name and elements.
    /// - `Object`: hashes the object directly.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Reference::BooleanArray(array) | Reference::ByteArray(array) => {
                array.hash(state);
            }
            Reference::CharArray(array) => {
                array.hash(state);
            }
            Reference::ShortArray(array) => {
                array.hash(state);
            }
            Reference::IntArray(array) => {
                array.hash(state);
            }
            Reference::LongArray(array) => {
                array.hash(state);
            }
            Reference::FloatArray(array) => {
                for value in array {
                    value.to_bits().hash(state);
                }
            }
            Reference::DoubleArray(array) => {
                for value in array {
                    value.to_bits().hash(state);
                }
            }
            Reference::Array(object_array) => {
                object_array.class.name().hash(state);
                object_array.elements.hash(state);
            }
            Reference::Object(object) => {
                object.hash(state);
            }
        }
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::BooleanArray(a), Reference::BooleanArray(b))
            | (Reference::ByteArray(a), Reference::ByteArray(b)) => a == b,
            (Reference::CharArray(a), Reference::CharArray(b)) => a == b,
            (Reference::ShortArray(a), Reference::ShortArray(b)) => a == b,
            (Reference::IntArray(a), Reference::IntArray(b)) => a == b,
            (Reference::LongArray(a), Reference::LongArray(b)) => a == b,
            (Reference::FloatArray(a), Reference::FloatArray(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(&a, &b)| a.to_bits() == b.to_bits())
            }
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => {
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

impl Trace for Reference {
    fn trace(&self, collector: &GarbageCollector) {
        match self {
            Reference::Array(object_array) => {
                for value in &object_array.elements {
                    if let Value::Object(Some(reference)) = value {
                        reference.trace(collector);
                    }
                }
            }
            Reference::Object(object) => object.trace(collector),
            _ => {}
        }
    }
}

impl From<Vec<bool>> for Reference {
    fn from(value: Vec<bool>) -> Self {
        let value: Vec<i8> = value.into_iter().map(i8::from).collect();
        Reference::BooleanArray(value.into_boxed_slice())
    }
}

impl From<Vec<i8>> for Reference {
    fn from(value: Vec<i8>) -> Self {
        Reference::ByteArray(value.into_boxed_slice())
    }
}

impl From<Vec<u8>> for Reference {
    fn from(value: Vec<u8>) -> Self {
        let value: &[i8] = transmute_ref!(value.as_slice());
        Reference::ByteArray(value.to_vec().into_boxed_slice())
    }
}

impl From<Vec<char>> for Reference {
    fn from(value: Vec<char>) -> Self {
        let value: Vec<u16> = value.into_iter().map(|v| v as u16).collect();
        Reference::CharArray(value.into_boxed_slice())
    }
}

impl From<Vec<i16>> for Reference {
    fn from(value: Vec<i16>) -> Self {
        Reference::ShortArray(value.into_boxed_slice())
    }
}

impl From<Vec<u16>> for Reference {
    fn from(value: Vec<u16>) -> Self {
        let value: &[i16] = transmute_ref!(value.as_slice());
        Reference::ShortArray(value.to_vec().into_boxed_slice())
    }
}

impl From<Vec<i32>> for Reference {
    fn from(value: Vec<i32>) -> Self {
        Reference::IntArray(value.into_boxed_slice())
    }
}

impl From<Vec<u32>> for Reference {
    fn from(value: Vec<u32>) -> Self {
        let value: &[i32] = transmute_ref!(value.as_slice());
        Reference::IntArray(value.to_vec().into_boxed_slice())
    }
}

impl From<Vec<i64>> for Reference {
    fn from(value: Vec<i64>) -> Self {
        Reference::LongArray(value.into_boxed_slice())
    }
}

impl From<Vec<u64>> for Reference {
    fn from(value: Vec<u64>) -> Self {
        let value: &[i64] = transmute_ref!(value.as_slice());
        Reference::LongArray(value.to_vec().into_boxed_slice())
    }
}

impl From<Vec<isize>> for Reference {
    fn from(value: Vec<isize>) -> Self {
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(value.into_boxed_slice())
    }
}

impl From<Vec<usize>> for Reference {
    fn from(value: Vec<usize>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(value.into_boxed_slice())
    }
}

impl From<Vec<f32>> for Reference {
    fn from(value: Vec<f32>) -> Self {
        Reference::FloatArray(value.into_boxed_slice())
    }
}

impl From<Vec<f64>> for Reference {
    fn from(value: Vec<f64>) -> Self {
        Reference::DoubleArray(value.into_boxed_slice())
    }
}

impl Reference {
    /// Create a new object array reference.
    pub fn new_array(
        collector: &GarbageCollector,
        class: Arc<Class>,
        value: Vec<Option<Reference>>,
    ) -> Self {
        let elements = value
            .into_iter()
            .map(|opt_ref| Value::new_opt_object(collector, opt_ref))
            .collect::<Vec<_>>();
        let object_array = ObjectArray {
            class,
            elements: elements.into_boxed_slice(),
        };
        Reference::Array(object_array)
    }
}

impl TryFrom<(Arc<Class>, Vec<Value>)> for Reference {
    type Error = crate::Error;

    fn try_from(value: (Arc<Class>, Vec<Value>)) -> Result<Self> {
        let (class, values) = value;

        for value in &values {
            if !matches!(value, Value::Object(_)) {
                return Err(InvalidValueType("Expected object".to_string()));
            }
        }

        let object_array = ObjectArray {
            class,
            elements: values.into_boxed_slice(),
        };
        Ok(Reference::Array(object_array))
    }
}

impl From<Object> for Reference {
    fn from(value: Object) -> Self {
        Reference::Object(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Class, Result, Value, runtime};
    use ristretto_gc::GarbageCollector;
    use std::hash::{DefaultHasher, Hasher};
    use std::sync::Arc;

    fn test_ref(reference: impl Into<Reference>) -> Value {
        Value::new_object(&GarbageCollector::new(), reference.into())
    }

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    #[test]
    fn test_display_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        assert_eq!(reference.class_name()?, "[B");
        assert_eq!(reference.to_string(), "byte[1, 2, 3]");
        Ok(())
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
        let values = vec![Some(Reference::from(object))];
        let reference = Reference::new_array(&GarbageCollector::new(), class, values);
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
        let mut reference = Reference::from(vec![42i8]);
        {
            let mutable_reference = reference.as_byte_vec_mut()?;
            mutable_reference[0] = 3i8;
        }
        assert_eq!(reference.as_byte_vec_ref()?.to_vec(), vec![3i8]);
        Ok(())
    }

    #[test]
    fn test_as_byte_vec_mut_error() {
        let original_value = vec![42i32];
        let mut reference = Reference::from(original_value.clone());
        let result = reference.as_byte_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char, 2 as char, 3 as char]);
        assert_eq!(reference.class_name()?, "[C");
        assert_eq!(reference.to_string(), "char[1, 2, 3]");
        Ok(())
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
        let mut reference = Reference::from(vec!['*']);
        {
            let mutable_reference = reference.as_char_vec_mut()?;
            mutable_reference[0] = 50u16;
        }
        assert_eq!(reference.as_char_vec_ref()?.to_vec(), vec![50u16]);
        Ok(())
    }

    #[test]
    fn test_as_char_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_char_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16, 2i16, 3i16]);
        assert_eq!(reference.class_name()?, "[S");
        assert_eq!(reference.to_string(), "short[1, 2, 3]");
        Ok(())
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
        let mut reference = Reference::from(vec![42i16]);
        {
            let mutable_reference = reference.as_short_vec_mut()?;
            mutable_reference[0] = 3i16;
        }
        assert_eq!(reference.as_short_vec_ref()?.to_vec(), vec![3i16]);
        Ok(())
    }

    #[test]
    fn test_as_short_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_short_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32, 2i32, 3i32]);
        assert_eq!(reference.class_name()?, "[I");
        assert_eq!(reference.to_string(), "int[1, 2, 3]");
        Ok(())
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
        let mut reference = Reference::from(vec![42i32]);
        {
            let mutable_reference = reference.as_int_vec_mut()?;
            mutable_reference[0] = 3i32;
        }
        assert_eq!(reference.as_int_vec_ref()?.to_vec(), vec![3i32]);
        Ok(())
    }

    #[test]
    fn test_as_int_vec_mut_error() {
        let mut reference = Reference::from(vec![42i8]);
        let result = reference.as_int_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64, 2i64, 3i64]);
        assert_eq!(reference.class_name()?, "[J");
        assert_eq!(reference.to_string(), "long[1, 2, 3]");
        Ok(())
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
        let mut reference = Reference::from(vec![42i64]);
        {
            let mutable_reference = reference.as_long_vec_mut()?;
            mutable_reference[0] = 3i64;
        }
        assert_eq!(reference.as_long_vec_ref()?.to_vec(), vec![3i64]);
        Ok(())
    }

    #[test]
    fn test_as_long_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_long_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(reference.class_name()?, "[F");
        assert_eq!(reference.to_string(), "float[1.0, 2.0, 3.0]");
        Ok(())
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
        let mut reference = Reference::from(vec![42.1f32]);
        {
            let mutable_reference = reference.as_float_vec_mut()?;
            mutable_reference[0] = 3.45f32;
        }
        assert_eq!(reference.as_float_vec_ref()?.to_vec(), vec![3.45f32]);
        Ok(())
    }

    #[test]
    fn test_as_float_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_float_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64, 2.0f64, 3.0f64]);
        assert_eq!(reference.class_name()?, "[D");
        assert_eq!(reference.to_string(), "double[1.0, 2.0, 3.0]");
        Ok(())
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
        let mut reference = Reference::from(vec![42.1f64]);
        {
            let mutable_reference = reference.as_double_vec_mut()?;
            mutable_reference[0] = 3.45f64;
        }
        assert_eq!(reference.as_double_vec_ref()?.to_vec(), vec![3.45f64]);
        Ok(())
    }

    #[test]
    fn test_as_double_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_double_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_display_reference_array() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let reference = Reference::new_array(&GarbageCollector::new(), class, vec![None]);
        assert_eq!(reference.class_name()?, "[Ljava/lang/Object;");
        assert_eq!(reference.to_string(), "java/lang/Object[1]");
        Ok(())
    }

    #[tokio::test]
    async fn test_as_class_vec_ref() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let original_value = vec![Value::Object(None)];
        let reference =
            Reference::new_array(&GarbageCollector::new(), original_class.clone(), vec![None]);
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
        let mut reference =
            Reference::new_array(&GarbageCollector::new(), object_class.clone(), vec![None]);
        {
            let (_class, mutable_reference) = reference.as_class_vec_mut()?;
            mutable_reference[0] = Value::Int(42);
        }
        let (_class, array) = reference.as_class_vec_ref()?;
        assert_eq!(array.to_vec(), vec![Value::Int(42)]);
        Ok(())
    }

    #[test]
    fn test_as_class_vec_mut_error() {
        let mut reference = Reference::from(vec![42i32]);
        let result = reference.as_class_vec_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_display_reference() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert_eq!(reference.class_name()?, "java/lang/Object");
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
        let result = reference.as_object_ref()?.clone();
        assert_eq!(object, result);
        Ok(())
    }

    #[test]
    fn test_as_object_ref_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.as_object_ref();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_as_object_mut() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let object = Object::new(class)?;
        let mut reference = Reference::from(object.clone());
        let result = reference.as_object_mut()?;
        assert_eq!(object, result.clone());
        {
            let object_mut = result;
            object_mut.set_value("value", Value::Int(42))?;
        }
        Ok(())
    }

    #[test]
    fn test_as_object_mut_error() {
        let original_value = vec![42i32];
        let mut reference = Reference::from(original_value.clone());
        let result = reference.as_object_mut();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_hash_code_byte_array() {
        let reference = Reference::from(vec![1i8]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_byte_array() {
        let reference1 = Reference::from(vec![42i8]);
        let reference2 = Reference::from(vec![42i8]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_byte_array() {
        let reference1 = Reference::from(vec![1i8]);
        let reference2 = Reference::from(vec![1i8]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_byte_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_char_array() {
        let reference = Reference::from(vec![1 as char]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_char_array() {
        let reference1 = Reference::from(vec![42 as char]);
        let reference2 = Reference::from(vec![42 as char]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_char_array() {
        let reference1 = Reference::from(vec![1 as char]);
        let reference2 = Reference::from(vec![1 as char]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_char_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_short_array() {
        let reference = Reference::from(vec![1i16]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_short_array() {
        let reference1 = Reference::from(vec![42i16]);
        let reference2 = Reference::from(vec![42i16]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_short_array() {
        let reference1 = Reference::from(vec![1i16]);
        let reference2 = Reference::from(vec![1i16]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_short_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_int_array() {
        let reference = Reference::from(vec![1i32]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_int_array() {
        let reference1 = Reference::from(vec![42i32]);
        let reference2 = Reference::from(vec![42i32]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_int_array() {
        let reference1 = Reference::from(vec![1i32]);
        let reference2 = Reference::from(vec![1i32]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_int_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_long_array() {
        let reference = Reference::from(vec![1i64]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_long_array() {
        let reference1 = Reference::from(vec![42i64]);
        let reference2 = Reference::from(vec![42i64]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_long_array() {
        let reference1 = Reference::from(vec![1i64]);
        let reference2 = Reference::from(vec![1i64]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_long_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_float_array() {
        let reference = Reference::from(vec![1.0f32]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_float_array() {
        let reference1 = Reference::from(vec![42.1f32]);
        let reference2 = Reference::from(vec![42.1f32]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_float_array() {
        let reference1 = Reference::from(vec![1.0f32]);
        let reference2 = Reference::from(vec![1.0f32]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_float_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2.0;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code_double_array() {
        let reference = Reference::from(vec![1.0f64]);
        assert_ne!(0, reference.hash_code());
    }

    #[test]
    fn test_hash_double_array() {
        let reference1 = Reference::from(vec![42.1f64]);
        let reference2 = Reference::from(vec![42.1f64]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ptr_eq_double_array() {
        let reference1 = Reference::from(vec![1.0f64]);
        let reference2 = Reference::from(vec![1.0f64]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let array = clone.as_double_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2.0;
            }
        }
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_code_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        assert_ne!(0, reference.hash_code());
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        let reference2 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
        Ok(())
    }

    #[tokio::test]
    async fn test_ptr_eq_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        let reference2 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        let reference3 = reference1.clone();
        assert!(reference1.ptr_eq(&reference1));
        assert!(!reference1.ptr_eq(&reference2));
        assert!(!reference1.ptr_eq(&reference3));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![None]);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let (_class, array) = clone.as_class_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = test_ref(vec![1i8]);
            }
        }
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
    async fn test_hash_object() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from(Object::new(class.clone())?);
        let reference2 = Reference::from(Object::new(class.clone())?);
        let mut hasher1 = DefaultHasher::new();
        reference1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        reference2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
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
        assert!(!reference1.ptr_eq(&reference3));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let mut clone = reference.clone();
        assert_eq!(reference, clone);

        let cloned_object = clone.as_object_mut()?;
        cloned_object.set_value("value", Value::Int(2))?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![]);
        let reference2 = Reference::new_array(&GarbageCollector::new(), class, vec![]);
        assert_eq!(reference1, reference2);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq_class_ne() -> Result<()> {
        let object_class = load_class("java.lang.Object").await?;
        let reference1 = Reference::new_array(&GarbageCollector::new(), object_class, vec![]);
        let string_class = load_class("java.lang.String").await?;
        let reference2 = Reference::new_array(&GarbageCollector::new(), string_class, vec![]);
        assert_ne!(reference1, reference2);
        Ok(())
    }

    #[tokio::test]
    async fn test_array_eq_value_ne() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::new_array(&GarbageCollector::new(), class.clone(), vec![]);
        let reference2 = Reference::new_array(&GarbageCollector::new(), class, vec![None]);
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
        assert!(matches!(reference, Reference::BooleanArray(_)));
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
        let reference = Reference::new_array(
            &GarbageCollector::new(),
            original_class.clone(),
            original_value.clone(),
        );
        assert!(matches!(reference, Reference::Array(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let class_name = "java/lang/Integer";
        let class = load_class(class_name).await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = test_ref(object);
        let original_values = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_values.clone()))?;
        assert!(matches!(reference, Reference::Array(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec_error() -> Result<()> {
        let original_class = load_class("[Ljava/lang/Object;").await?;
        let value = Value::Int(42);
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
    async fn test_as_bool() -> Result<()> {
        let class = load_class("java/lang/Boolean").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let value = reference.as_bool()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_char() -> Result<()> {
        let class = load_class("java/lang/Character").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_char()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_i8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_u8()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_i16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_u16()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value = reference.as_u32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_i64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value = reference.as_i64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_u64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value = reference.as_u64()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_isize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value = reference.as_isize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_usize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value = reference.as_usize()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f32() -> Result<()> {
        let class = load_class("java/lang/Float").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let reference = Reference::from(object);
        let value = reference.as_f32()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_f64() -> Result<()> {
        let class = load_class("java/lang/Double").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let reference = Reference::from(object);
        let value = reference.as_f64()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_as_string() -> Result<()> {
        let class = load_class("java/lang/String").await?;
        let mut object = Object::new(class)?;
        let bytes = "foo".as_bytes();
        let string_bytes: &[i8] = zerocopy::transmute_ref!(bytes);
        let string_value = test_ref(string_bytes.to_vec());
        object.set_value("value", string_value)?;
        let reference = Reference::from(object);
        let result = reference.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }

    // Tests for as_bytes()

    #[test]
    fn test_as_bytes_byte_array() {
        let reference = Reference::from(vec![1i8, 2i8, 3i8, 4i8]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), 4);
        // i8 values cast to u8
        assert_eq!(bytes, &[1u8, 2u8, 3u8, 4u8]);
    }

    #[test]
    fn test_as_bytes_byte_array_negative_values() {
        let reference = Reference::from(vec![-1i8, -128i8, 127i8]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes, &[255u8, 128u8, 127u8]);
    }

    #[test]
    fn test_as_bytes_char_array() {
        let reference = Reference::from(vec!['A', 'B']); // 65 and 66 as u16
        let bytes = reference.as_bytes().expect("should return bytes");
        // 2 chars * 2 bytes each
        assert_eq!(bytes.len(), 4);
    }

    #[test]
    fn test_as_bytes_short_array() {
        let reference = Reference::from(vec![0x0102i16]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), 2);
        #[cfg(target_endian = "little")]
        assert_eq!(bytes, [0x02, 0x01]);
        #[cfg(target_endian = "big")]
        assert_eq!(bytes, [0x01, 0x02]);
    }

    #[test]
    fn test_as_bytes_int_array() {
        let reference = Reference::from(vec![0x0102_0304_i32]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), 4);
    }

    #[test]
    fn test_as_bytes_long_array() {
        let reference = Reference::from(vec![0x0102_0304_0506_0708_i64]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_as_bytes_float_array() {
        let reference = Reference::from(vec![1.0f32, 2.0f32]);
        let bytes = reference.as_bytes().expect("should return bytes");
        // 2 floats * 4 bytes each
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_as_bytes_double_array() {
        let reference = Reference::from(vec![1.0f64, 2.0f64]);
        let bytes = reference.as_bytes().expect("should return bytes");
        // 2 doubles * 8 bytes each
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_as_bytes_empty_array() {
        let reference = Reference::from(Vec::<i32>::new());
        let bytes = reference.as_bytes().expect("should return bytes");
        assert!(bytes.is_empty());
    }

    #[tokio::test]
    async fn test_as_bytes_object_array_returns_none() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let reference = Reference::new_array(&GarbageCollector::new(), class, vec![None]);
        assert!(reference.as_bytes().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_bytes_object_returns_none() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert!(reference.as_bytes().is_none());
        Ok(())
    }

    // Tests for as_bytes_mut()

    #[test]
    fn test_as_bytes_mut_byte_array() {
        let mut reference = Reference::from(vec![1i8, 2i8, 3i8, 4i8]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            bytes[0] = 10;
            bytes[1] = 20;
        }
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes[0], 10);
        assert_eq!(bytes[1], 20);
    }

    #[test]
    fn test_as_bytes_mut_char_array() {
        let mut reference = Reference::from(vec!['A', 'B']);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            // Modify the raw bytes
            bytes[0] = 0xFF;
        }
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes[0], 0xFF);
    }

    #[test]
    fn test_as_bytes_mut_short_array() {
        let mut reference = Reference::from(vec![0i16, 0i16]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            assert_eq!(bytes.len(), 4);
            bytes.fill(0xFF);
        }
        let shorts = reference.as_short_vec_ref().expect("should return shorts");
        assert_eq!(shorts[0], -1i16);
        assert_eq!(shorts[1], -1i16);
    }

    #[test]
    fn test_as_bytes_mut_int_array() {
        let mut reference = Reference::from(vec![0i32]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            assert_eq!(bytes.len(), 4);
            bytes.fill(0xFF);
        }
        let ints = reference.as_int_vec_ref().expect("should return ints");
        assert_eq!(ints[0], -1i32);
    }

    #[test]
    fn test_as_bytes_mut_long_array() {
        let mut reference = Reference::from(vec![0i64]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            assert_eq!(bytes.len(), 8);
            bytes.fill(0xFF);
        }
        let longs = reference.as_long_vec_ref().expect("should return longs");
        assert_eq!(longs[0], -1i64);
    }

    #[test]
    fn test_as_bytes_mut_float_array() {
        let mut reference = Reference::from(vec![0.0f32]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            assert_eq!(bytes.len(), 4);
            // Set to IEEE 754 representation of 1.0f32 (0x3F800000)
            bytes.copy_from_slice(&1.0f32.to_ne_bytes());
        }
        let floats = reference.as_float_vec_ref().expect("should return floats");
        assert!((floats[0] - 1.0f32).abs() < f32::EPSILON);
    }

    #[test]
    fn test_as_bytes_mut_double_array() {
        let mut reference = Reference::from(vec![0.0f64]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            assert_eq!(bytes.len(), 8);
            // Set to IEEE 754 representation of 1.0f64
            bytes.copy_from_slice(&1.0f64.to_ne_bytes());
        }
        let doubles = reference
            .as_double_vec_ref()
            .expect("should return doubles");
        assert!((doubles[0] - 1.0f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_as_bytes_mut_empty_array() {
        let mut reference = Reference::from(Vec::<i32>::new());
        let bytes = reference.as_bytes_mut().expect("should return bytes");
        assert!(bytes.is_empty());
    }

    #[tokio::test]
    async fn test_as_bytes_mut_object_array_returns_none() -> Result<()> {
        let class = load_class("[Ljava/lang/Object;").await?;
        let mut reference = Reference::new_array(&GarbageCollector::new(), class, vec![None]);
        assert!(reference.as_bytes_mut().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_as_bytes_mut_object_returns_none() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let object = Object::new(class)?;
        let mut reference = Reference::from(object);
        assert!(reference.as_bytes_mut().is_none());
        Ok(())
    }

    #[test]
    fn test_as_bytes_roundtrip_preserves_data() {
        // Test that reading bytes and writing them back preserves the array
        let original = vec![1i32, 2i32, 3i32, 4i32];
        let reference = Reference::from(original.clone());
        let bytes = reference.as_bytes().expect("should return bytes");
        let bytes_copy = bytes.to_vec();

        let mut new_reference = Reference::from(vec![0i32; 4]);
        {
            let new_bytes = new_reference.as_bytes_mut().expect("should return bytes");
            new_bytes.copy_from_slice(&bytes_copy);
        }

        let result = new_reference.as_int_vec_ref().expect("should return ints");
        assert_eq!(result.to_vec(), original);
    }

    #[test]
    fn test_as_bytes_mut_partial_write() {
        let mut reference = Reference::from(vec![0i8; 8]);
        {
            let bytes = reference.as_bytes_mut().expect("should return bytes");
            // Only write to first 4 bytes
            bytes[0..4].fill(0xAB);
        }
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(&bytes[0..4], &[0xAB, 0xAB, 0xAB, 0xAB]);
        assert_eq!(&bytes[4..8], &[0, 0, 0, 0]);
    }

    #[test]
    fn test_as_bytes_large_array() {
        let size = 10000;
        let reference = Reference::from(vec![42i64; size]);
        let bytes = reference.as_bytes().expect("should return bytes");
        assert_eq!(bytes.len(), size * 8); // 8 bytes per i64
    }
}

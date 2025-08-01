use crate::Error::{InvalidValueType, PoisonedLock};
use crate::{Class, Object, Result, Value};
use ristretto_gc::{GarbageCollector, Gc, Trace};
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use zerocopy::transmute_ref;

/// Represents an array of objects in the Ristretto VM.
///
/// `ObjectArray` groups the array's class `Arc<Class>` and its elements
/// `Gc<RwLock<<Option<Reference>>>>` together in order to reduce the amount of memory required by
/// values in the Reference enum.
#[derive(Clone, Debug)]
pub struct ObjectArray {
    pub class: Arc<Class>,
    pub elements: Gc<RwLock<Vec<Option<Reference>>>>,
}

impl Eq for ObjectArray {}

impl PartialEq for ObjectArray {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && Gc::ptr_eq(&self.elements, &other.elements)
    }
}

/// Represents a reference to an object in the Ristretto VM.
#[derive(Clone, Debug)]
pub enum Reference {
    ByteArray(Gc<RwLock<Vec<i8>>>),
    CharArray(Gc<RwLock<Vec<u16>>>),
    ShortArray(Gc<RwLock<Vec<i16>>>),
    IntArray(Gc<RwLock<Vec<i32>>>),
    LongArray(Gc<RwLock<Vec<i64>>>),
    FloatArray(Gc<RwLock<Vec<f32>>>),
    DoubleArray(Gc<RwLock<Vec<f64>>>),
    Array(ObjectArray),
    Object(Gc<RwLock<Object>>),
}

impl Reference {
    /// Get the class name of the reference
    ///
    /// # Errors
    ///
    /// if the `Object` read lock is poisoned
    pub fn class_name(&self) -> Result<String> {
        let class_name = match self {
            Reference::ByteArray(_) => "[B".to_string(),
            Reference::CharArray(_) => "[C".to_string(),
            Reference::ShortArray(_) => "[S".to_string(),
            Reference::IntArray(_) => "[I".to_string(),
            Reference::LongArray(_) => "[J".to_string(),
            Reference::FloatArray(_) => "[F".to_string(),
            Reference::DoubleArray(_) => "[D".to_string(),
            Reference::Array(object_array) => object_array.class.name().to_string(),
            Reference::Object(object) => {
                let object = object
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                object.class().name().to_string()
            }
        };
        Ok(class_name)
    }

    /// Returns the reference to `Vec<i8>`.
    ///
    /// # Errors
    ///
    /// if the value is not a `ByteArray`.
    pub fn as_byte_vec_ref(&self) -> Result<RwLockReadGuard<'_, Vec<i8>>> {
        match self {
            Reference::ByteArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::ByteArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::CharArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::CharArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::ShortArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::ShortArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::IntArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::IntArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::LongArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::LongArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::FloatArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::FloatArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::DoubleArray(array) => array
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
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
            Reference::DoubleArray(array) => array
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
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
                let array = object_array
                    .elements
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Ok((&object_array.class, array))
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
                let array = object_array
                    .elements
                    .write()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Ok((&object_array.class, array))
            }
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns the reference as an `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an Object.
    pub fn as_object_ref(&self) -> Result<RwLockReadGuard<'_, Object>> {
        match self {
            Reference::Object(object) => object
                .read()
                .map_err(|error| PoisonedLock(error.to_string())),
            _ => Err(InvalidValueType("Expected object".to_string())),
        }
    }

    /// Returns a mutable reference to the `Object`.
    ///
    /// # Errors
    ///
    /// if the value is not an `Object`.
    pub fn as_object_mut(&self) -> Result<RwLockWriteGuard<'_, Object>> {
        match self {
            Reference::Object(object) => object
                .write()
                .map_err(|error| PoisonedLock(error.to_string())),
            _ => Err(InvalidValueType("Expected object".to_string())),
        }
    }

    /// Returns hash code implementation based on memory address. This is used by the Java
    /// `Object.hash_code()` implementation.
    #[must_use]
    pub fn hash_code(&self) -> usize {
        match self {
            Reference::ByteArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<i8>>>() as usize,
            Reference::CharArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<u16>>>() as usize,
            Reference::ShortArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<i16>>>() as usize,
            Reference::IntArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<i32>>>() as usize,
            Reference::LongArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<i64>>>() as usize,
            Reference::FloatArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<f32>>>() as usize,
            Reference::DoubleArray(array) => Gc::as_ptr(array).cast::<Vec<RwLock<f64>>>() as usize,
            Reference::Array(object_array) => {
                Gc::as_ptr(&object_array.elements).cast::<Vec<RwLock<Option<Reference>>>>() as usize
            }
            Reference::Object(object) => Gc::as_ptr(object).cast::<RwLock<Object>>() as usize,
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
            Reference::ByteArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::ByteArray(Gc::new(RwLock::new(array.clone())))
            }
            Reference::CharArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::CharArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::ShortArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::ShortArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::IntArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::IntArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::LongArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::LongArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::FloatArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::FloatArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::DoubleArray(array) => {
                let array = array
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Reference::DoubleArray(Gc::new(RwLock::new(array.to_vec())))
            }
            Reference::Array(object_array) => {
                let array = object_array
                    .elements
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                let array = array.to_vec();
                let mut cloned_values = Vec::with_capacity(array.len());
                for value in array {
                    match value {
                        Some(reference) => cloned_values.push(Some(reference.deep_clone()?)),
                        None => cloned_values.push(value),
                    }
                }
                let object_array = ObjectArray {
                    class: object_array.class.clone(),
                    elements: Gc::new(RwLock::new(cloned_values)),
                };
                Reference::Array(object_array)
            }
            Reference::Object(object) => {
                let object = object
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                if object.class().name() == "java/lang/Class" {
                    // Special case for Class objects, which should not be deep cloned.
                    self.clone()
                } else {
                    Reference::Object(Gc::new(RwLock::new(object.deep_clone()?)))
                }
            }
        };
        Ok(value)
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
        fn fmt_vec<T: Debug>(
            f: &mut fmt::Formatter<'_>,
            name: &str,
            value: &Gc<RwLock<Vec<T>>>,
        ) -> fmt::Result {
            let guard = value.read().map_err(|_| fmt::Error)?;
            let slice = guard.as_slice();
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
            Reference::ByteArray(array) => fmt_vec(f, "byte", array),
            Reference::CharArray(array) => fmt_vec(f, "char", array),
            Reference::ShortArray(array) => fmt_vec(f, "short", array),
            Reference::IntArray(array) => fmt_vec(f, "int", array),
            Reference::LongArray(array) => fmt_vec(f, "long", array),
            Reference::FloatArray(array) => fmt_vec(f, "float", array),
            Reference::DoubleArray(array) => fmt_vec(f, "double", array),
            Reference::Array(object_array) => {
                let guard = object_array.elements.read().map_err(|_| fmt::Error)?;
                write!(
                    f,
                    "{}[{}]",
                    object_array.class.array_component_type(),
                    guard.len()
                )
            }
            Reference::Object(object) => match object.read() {
                Ok(object) => {
                    write!(f, "{object}")
                }
                Err(error) => {
                    write!(f, "{error:?}")
                }
            },
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
            Reference::ByteArray(array) => {
                let array = array.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::CharArray(array) => {
                let array = array.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::ShortArray(array) => {
                let array = array.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::IntArray(array) => {
                let array = array.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::LongArray(array) => {
                let array = array.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::FloatArray(array) => {
                let array = array.read().expect("poisoned lock");
                for value in array.iter() {
                    value.to_bits().hash(state);
                }
            }
            Reference::DoubleArray(array) => {
                let array = array.read().expect("poisoned lock");
                for value in array.iter() {
                    value.to_bits().hash(state);
                }
            }
            Reference::Array(object_array) => {
                object_array.class.name().hash(state);
                let array = object_array.elements.read().expect("poisoned lock");
                array.hash(state);
            }
            Reference::Object(object) => {
                let object = object.read().expect("poisoned lock");
                object.hash(state);
            }
        }
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        fn vec_eq<T: PartialEq>(a: &Gc<RwLock<Vec<T>>>, b: &Gc<RwLock<Vec<T>>>) -> bool {
            if Gc::ptr_eq(a, b) {
                return true;
            }
            // Use try_read to avoid blocking if locks are contended
            if let (Ok(a_guard), Ok(b_guard)) = (a.try_read(), b.try_read()) {
                *a_guard == *b_guard
            } else {
                let a_guard = a.read().expect("poisoned lock");
                let b_guard = b.read().expect("poisoned lock");
                *a_guard == *b_guard
            }
        }

        match (self, other) {
            (Reference::ByteArray(a), Reference::ByteArray(b)) => vec_eq(a, b),
            (Reference::CharArray(a), Reference::CharArray(b)) => vec_eq(a, b),
            (Reference::ShortArray(a), Reference::ShortArray(b)) => vec_eq(a, b),
            (Reference::IntArray(a), Reference::IntArray(b)) => vec_eq(a, b),
            (Reference::LongArray(a), Reference::LongArray(b)) => vec_eq(a, b),
            (Reference::FloatArray(a), Reference::FloatArray(b)) => {
                if Gc::ptr_eq(a, b) {
                    return true;
                }
                // Use try_read to avoid blocking if locks are contended
                let (a, b) = if let (Ok(a_guard), Ok(b_guard)) = (a.try_read(), b.try_read()) {
                    (a_guard, b_guard)
                } else {
                    let a_guard = a.read().expect("poisoned lock");
                    let b_guard = b.read().expect("poisoned lock");
                    (a_guard, b_guard)
                };

                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(&a, &b)| a.to_bits() == b.to_bits())
            }
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => {
                if Gc::ptr_eq(a, b) {
                    return true;
                }
                // Use try_read to avoid blocking if locks are contended
                let (a, b) = if let (Ok(a_guard), Ok(b_guard)) = (a.try_read(), b.try_read()) {
                    (a_guard, b_guard)
                } else {
                    let a_guard = a.read().expect("poisoned lock");
                    let b_guard = b.read().expect("poisoned lock");
                    (a_guard, b_guard)
                };

                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(&a, &b)| a.to_bits() == b.to_bits())
            }
            (Reference::Array(a), Reference::Array(b)) => {
                a.class == b.class && vec_eq(&a.elements, &b.elements)
            }
            (Reference::Object(a), Reference::Object(b)) => {
                if Gc::ptr_eq(a, b) {
                    return true;
                }
                // Use try_read to avoid blocking if locks are contended
                let a_guard = a.try_read().expect("poisoned lock");
                let b_guard = b.try_read().expect("poisoned lock");
                *a_guard == *b_guard
            }
            _ => false,
        }
    }
}

impl Trace for Reference {
    fn trace(&self, collector: &GarbageCollector) {
        match self {
            Reference::Array(object_array) => {
                let references = object_array.elements.read().expect("object_array.elements");
                for reference in references.iter().flatten() {
                    reference.trace(collector);
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
        Reference::ByteArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<i8>> for Reference {
    fn from(value: Vec<i8>) -> Self {
        Reference::ByteArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<u8>> for Reference {
    fn from(value: Vec<u8>) -> Self {
        let value: &[i8] = transmute_ref!(value.as_slice());
        Reference::ByteArray(Gc::new(RwLock::new(value.to_vec())))
    }
}

impl From<Vec<char>> for Reference {
    fn from(value: Vec<char>) -> Self {
        let value: Vec<u16> = value.into_iter().map(|v| v as u16).collect();
        Reference::CharArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<i16>> for Reference {
    fn from(value: Vec<i16>) -> Self {
        Reference::ShortArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<u16>> for Reference {
    fn from(value: Vec<u16>) -> Self {
        let value: &[i16] = transmute_ref!(value.as_slice());
        Reference::ShortArray(Gc::new(RwLock::new(value.to_vec())))
    }
}

impl From<Vec<i32>> for Reference {
    fn from(value: Vec<i32>) -> Self {
        Reference::IntArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<u32>> for Reference {
    fn from(value: Vec<u32>) -> Self {
        let value: &[i32] = transmute_ref!(value.as_slice());
        Reference::IntArray(Gc::new(RwLock::new(value.to_vec())))
    }
}

impl From<Vec<i64>> for Reference {
    fn from(value: Vec<i64>) -> Self {
        Reference::LongArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<u64>> for Reference {
    fn from(value: Vec<u64>) -> Self {
        let value: &[i64] = transmute_ref!(value.as_slice());
        Reference::LongArray(Gc::new(RwLock::new(value.to_vec())))
    }
}

impl From<Vec<isize>> for Reference {
    fn from(value: Vec<isize>) -> Self {
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<usize>> for Reference {
    fn from(value: Vec<usize>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<f32>> for Reference {
    fn from(value: Vec<f32>) -> Self {
        Reference::FloatArray(Gc::new(RwLock::new(value)))
    }
}

impl From<Vec<f64>> for Reference {
    fn from(value: Vec<f64>) -> Self {
        Reference::DoubleArray(Gc::new(RwLock::new(value)))
    }
}

impl From<(Arc<Class>, Vec<Option<Reference>>)> for Reference {
    fn from((class, value): (Arc<Class>, Vec<Option<Reference>>)) -> Self {
        let object_array = ObjectArray {
            class: class.clone(),
            elements: Gc::new(RwLock::new(value)),
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
            elements: Gc::new(RwLock::new(references)),
        };
        Ok(Reference::Array(object_array))
    }
}

impl From<Object> for Reference {
    fn from(value: Object) -> Self {
        Reference::Object(Gc::new(RwLock::new(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Result, Value, runtime};
    use std::hash::{DefaultHasher, Hasher};
    use std::sync::Arc;

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
        assert_eq!(reference.class_name()?, "[Ljava/lang/Object;");
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
        let reference = Reference::from(object.clone());
        let result = reference.as_object_mut()?;
        assert_eq!(object, result.clone());
        {
            let mut object_mut = result;
            object_mut.set_value("value", Value::Int(42))?;
        }
        Ok(())
    }

    #[test]
    fn test_as_object_mut_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_byte_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_byte_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_char_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_char_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_short_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_short_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_int_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_int_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_long_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_long_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_float_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2.0;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_float_vec_mut()?;
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
        assert!(reference1.ptr_eq(&reference3));
    }

    #[test]
    fn test_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_double_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = 2.0;
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut array = clone.as_double_vec_mut()?;
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
        let reference = Reference::from((class.clone(), vec![None]));
        assert_ne!(0, reference.hash_code());
        Ok(())
    }

    #[tokio::test]
    async fn test_hash_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference1 = Reference::from((class.clone(), vec![None]));
        let reference2 = Reference::from((class.clone(), vec![None]));
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

        {
            let (_class, mut array) = clone.as_class_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = Some(Reference::from(vec![1i8]));
            }
        }
        assert_eq!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_reference_array() -> Result<()> {
        let class = load_class("java.lang.Object").await?;
        let reference = Reference::from((class.clone(), vec![None]));
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let (_class, mut array) = clone.as_class_vec_mut()?;
            if let Some(element) = array.get_mut(0) {
                *element = Some(Reference::from(vec![1i8]));
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
        assert!(reference1.ptr_eq(&reference3));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let mut cloned_object = clone.as_object_mut()?;
        cloned_object.set_value("value", Value::Int(2))?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_object() -> Result<()> {
        let class = load_class("java.lang.Integer").await?;
        let mut object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        {
            let mut cloned_object = clone.as_object_mut()?;
            cloned_object.set_value("value", Value::Int(2))?;
        }
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
        let mut object = Object::new(class)?;
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

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_as_string() -> Result<()> {
        let class = load_class("java/lang/String").await?;
        let mut object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let reference = Reference::from(object);
        let result = reference.as_string()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }
}

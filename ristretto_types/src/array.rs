//! Fallible backing storage allocation for Java arrays.

use crate::{JavaError, Result};
use ristretto_classfile::BaseType;
use ristretto_classloader::Reference;

/// Allocates and initializes Java array elements without aborting on allocation failure.
///
/// Reserve before initializing so neither capacity overflow nor allocator failure reaches
/// Rust's infallible allocation handler. Callers can fill a multidimensional array's slots
/// in place without further growth of its backing storage.
///
/// # Errors
///
/// Returns `OutOfMemoryError` if the backing storage cannot be allocated.
pub fn allocate_array<T: Clone>(length: usize, value: T) -> Result<Vec<T>> {
    let mut elements = Vec::new();
    elements
        .try_reserve_exact(length)
        .map_err(|_| JavaError::OutOfMemoryError("Java heap space".to_string()))?;
    elements.resize(length, value);
    Ok(elements)
}

/// Allocates a zero-initialized primitive array using its Java storage representation.
///
/// Booleans use bytes and chars use UTF-16 code units. Constructing these variants directly
/// avoids a second, infallible allocation when converting from Rust `bool` or `char` vectors.
///
/// # Errors
///
/// Returns `OutOfMemoryError` if the backing storage cannot be allocated.
pub fn allocate_primitive_array(base_type: &BaseType, length: usize) -> Result<Reference> {
    Ok(match base_type {
        BaseType::Boolean => Reference::BooleanArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Byte => Reference::ByteArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Char => Reference::CharArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Short => Reference::ShortArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Int => Reference::IntArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Long => Reference::LongArray(allocate_array(length, 0)?.into_boxed_slice()),
        BaseType::Float => Reference::FloatArray(allocate_array(length, 0.0)?.into_boxed_slice()),
        BaseType::Double => Reference::DoubleArray(allocate_array(length, 0.0)?.into_boxed_slice()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use ristretto_classloader::Value;

    #[test]
    fn test_array_capacity_overflow() {
        for base_type in [
            BaseType::Boolean,
            BaseType::Byte,
            BaseType::Char,
            BaseType::Short,
            BaseType::Int,
            BaseType::Long,
            BaseType::Float,
            BaseType::Double,
        ] {
            assert!(matches!(
                allocate_primitive_array(&base_type, usize::MAX),
                Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
            ));
        }
        assert!(matches!(
            allocate_array(usize::MAX, Value::Object(None)),
            Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
        ));
    }

    // Fits Vec's capacity arithmetic but cannot fit in a 64-bit process's address space.
    // This exercises allocator failure without exhausting physical memory.
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_array_allocator_failure() {
        assert!(matches!(
            allocate_array(isize::MAX as usize, 0_u8),
            Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
        ));
    }

    #[test]
    fn test_array_initialization() -> Result<()> {
        assert_eq!(allocate_array(0, Value::Object(None))?, Vec::<Value>::new());
        assert_eq!(
            allocate_array(3, Value::Object(None))?,
            vec![Value::Object(None); 3]
        );
        for base_type in [
            BaseType::Boolean,
            BaseType::Byte,
            BaseType::Char,
            BaseType::Short,
            BaseType::Int,
            BaseType::Long,
            BaseType::Float,
            BaseType::Double,
        ] {
            let array = allocate_primitive_array(&base_type, 3)?;
            match array {
                Reference::BooleanArray(values) | Reference::ByteArray(values) => {
                    assert_eq!(&*values, &[0; 3]);
                }
                Reference::CharArray(values) => assert_eq!(&*values, &[0; 3]),
                Reference::ShortArray(values) => assert_eq!(&*values, &[0; 3]),
                Reference::IntArray(values) => assert_eq!(&*values, &[0; 3]),
                Reference::LongArray(values) => assert_eq!(&*values, &[0; 3]),
                Reference::FloatArray(values) => assert_eq!(&*values, &[0.0; 3]),
                Reference::DoubleArray(values) => assert_eq!(&*values, &[0.0; 3]),
                _ => return Err(Error::InternalError("Expected primitive array".to_string())),
            }
        }
        Ok(())
    }
}

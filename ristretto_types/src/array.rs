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

/// Allocates array storage directly in the collector's selected backing heap.
/// # Errors
/// Returns `OutOfMemoryError` if storage or the managed budget is exhausted.
pub fn allocate_array_in<T: Clone>(
    gc: &ristretto_gc::GarbageCollector,
    length: usize,
    value: T,
) -> Result<ristretto_gc::HeapBuffer<T>> {
    check_array_allocation(gc, length, size_of::<T>())?;
    Ok(ristretto_gc::HeapBuffer::try_with_fn_in(
        gc,
        length,
        |_| Ok(value.clone()),
    )?)
}

/// Allocates direct array storage with asynchronous recovery and bounded initialization batches.
/// # Errors
/// Returns `OutOfMemoryError` when bounded recovery cannot satisfy the request.
pub async fn allocate_array_in_async<T: Clone>(
    gc: &ristretto_gc::GarbageCollector,
    length: usize,
    value: T,
) -> Result<ristretto_gc::HeapBuffer<T>> {
    gc.make_room_async(array_allocation_size(length, size_of::<T>())?)
        .await?;
    Ok(ristretto_gc::HeapBuffer::try_with_fn_in_async(gc, length, |_| Ok(value.clone())).await?)
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
        BaseType::Boolean => Reference::BooleanArray(allocate_array(length, 0)?.into()),
        BaseType::Byte => Reference::ByteArray(allocate_array(length, 0)?.into()),
        BaseType::Char => Reference::CharArray(allocate_array(length, 0)?.into()),
        BaseType::Short => Reference::ShortArray(allocate_array(length, 0)?.into()),
        BaseType::Int => Reference::IntArray(allocate_array(length, 0)?.into()),
        BaseType::Long => Reference::LongArray(allocate_array(length, 0)?.into()),
        BaseType::Float => Reference::FloatArray(allocate_array(length, 0.0)?.into()),
        BaseType::Double => Reference::DoubleArray(allocate_array(length, 0.0)?.into()),
    })
}

/// Checks the managed budget before committing array backing storage.
/// # Errors
/// Returns `OutOfMemoryError` if the length overflows or exceeds available heap space.
pub fn check_array_allocation(
    gc: &ristretto_gc::GarbageCollector,
    length: usize,
    element_size: usize,
) -> Result<()> {
    gc.make_room(array_allocation_size(length, element_size)?)?;
    Ok(())
}

fn array_allocation_size(length: usize, element_size: usize) -> Result<usize> {
    let bytes = length
        .checked_mul(element_size)
        .and_then(|bytes| {
            bytes.checked_add(
                size_of::<ristretto_gc::sync::RwLock<Reference>>()
                    + align_of::<ristretto_gc::sync::RwLock<Reference>>(),
            )
        })
        .ok_or_else(|| JavaError::OutOfMemoryError("Java heap space".into()))?;
    Ok(bytes)
}

/// Allocates primitive storage after checking the managed heap budget.
/// # Errors
/// Returns `OutOfMemoryError` if either the budget or backing allocator is exhausted.
pub fn allocate_primitive_array_in(
    gc: &ristretto_gc::GarbageCollector,
    base_type: &BaseType,
    length: usize,
) -> Result<Reference> {
    Ok(match base_type {
        BaseType::Boolean => Reference::BooleanArray(allocate_array_in(gc, length, 0)?),
        BaseType::Byte => Reference::ByteArray(allocate_array_in(gc, length, 0)?),
        BaseType::Char => Reference::CharArray(allocate_array_in(gc, length, 0)?),
        BaseType::Short => Reference::ShortArray(allocate_array_in(gc, length, 0)?),
        BaseType::Int => Reference::IntArray(allocate_array_in(gc, length, 0)?),
        BaseType::Long => Reference::LongArray(allocate_array_in(gc, length, 0)?),
        BaseType::Float => Reference::FloatArray(allocate_array_in(gc, length, 0.0)?),
        BaseType::Double => Reference::DoubleArray(allocate_array_in(gc, length, 0.0)?),
    })
}

/// Allocates zeroed primitive storage without completing full GC cycles inline.
/// # Errors
/// Returns `OutOfMemoryError` or a collection error after bounded recovery.
pub async fn allocate_primitive_array_in_async(
    gc: &ristretto_gc::GarbageCollector,
    base_type: &BaseType,
    length: usize,
) -> Result<Reference> {
    Ok(match base_type {
        BaseType::Boolean => Reference::BooleanArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Byte => Reference::ByteArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Char => Reference::CharArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Short => Reference::ShortArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Int => Reference::IntArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Long => Reference::LongArray(allocate_array_in_async(gc, length, 0).await?),
        BaseType::Float => Reference::FloatArray(allocate_array_in_async(gc, length, 0.0).await?),
        BaseType::Double => Reference::DoubleArray(allocate_array_in_async(gc, length, 0.0).await?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use ristretto_classloader::Value;

    #[tokio::test]
    async fn async_arrays_recover_without_blocking_other_executor_tasks() -> Result<()> {
        use std::cell::Cell;
        let gc = ristretto_gc::GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .compact_heap(true)
                .diagnostics(true)
                .region_layout(128 * 1024, 4096)
                .max_heap_bytes(512 * 1024)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let done = Cell::new(false);
        let progress = Cell::new(0);
        let allocations = async {
            let result: Result<()> = async {
                for _ in 0..20 {
                    let array =
                        allocate_primitive_array_in_async(&gc, &BaseType::Byte, 128 * 1024).await?;
                    let value = Value::try_new_object_async(&gc, array).await?;
                    assert!(value.as_byte_vec_ref()?.iter().all(|value| *value == 0));
                }
                Ok(())
            }
            .await;
            done.set(true);
            result
        };
        let companion = async {
            while !done.get() {
                progress.set(progress.get() + 1);
                tokio::task::yield_now().await;
            }
        };
        let (result, ()) = tokio::join!(allocations, companion);
        result?;
        assert!(progress.get() >= 20);
        let diagnostic = gc.diagnostics();
        assert!(diagnostic.async_allocation_recovery.count > 0);
        assert!(diagnostic.async_assists > 0);
        assert_eq!(diagnostic.allocation_recovery.count, 0);
        assert_eq!(diagnostic.full_collection_calls, 0);
        gc.collect_full_async().await?;
        gc.collect_full_async().await?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(
            gc.region_heap()
                .expect("region heap")
                .statistics()
                .occupied_bytes,
            0
        );
        Ok(())
    }

    #[test]
    fn direct_arrays_use_the_collector_heap_without_staging() -> Result<()> {
        let gc = ristretto_gc::GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .compact_heap(true)
                .build(),
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
            let array = allocate_primitive_array_in(&gc, &base_type, 17)?;
            let expected = allocate_primitive_array(&base_type, 17)?;
            assert_eq!(array, expected);
            let value = Value::try_new_object(&gc, array)?;
            let heap = gc.region_heap().expect("region heap");
            assert!(heap.statistics().occupied_bytes > 0);
            assert_eq!(heap.statistics().bytes_staged, 0);
            drop(value);
            gc.collect_full()?;
            assert_eq!(heap.statistics().occupied_bytes, 0);
        }
        Ok(())
    }

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

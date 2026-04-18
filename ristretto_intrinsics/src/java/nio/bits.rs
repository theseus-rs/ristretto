use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::{ArrayIndexOutOfBoundsException, IllegalArgumentException};
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Implementation of `java.nio.Bits.copySwapMemory0`.
///
/// Copies `bytes` bytes from the source to the destination, swapping the byte order of each
/// element of size `element_size`. The source and destination may either be a Java byte array or
/// a native memory address (when the corresponding object reference is `null`).
///
/// # JVM specification
///
/// `element_size` must be `2`, `4`, or `8`. `bytes` must be a non-negative multiple of
/// `element_size`. If either constraint is violated an `IllegalArgumentException` is thrown.
#[intrinsic_method(
    "java/nio/Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn copy_swap_memory_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let element_size = parameters.pop_long()?;
    let bytes = parameters.pop_long()?;
    let dest_offset = parameters.pop_long()?;
    let dest = parameters.pop()?;
    let src_offset = parameters.pop_long()?;
    let src = parameters.pop()?;

    if element_size != 2 && element_size != 4 && element_size != 8 {
        return Err(
            IllegalArgumentException(format!("Invalid element size: {element_size}")).into(),
        );
    }
    if bytes < 0 || bytes % element_size != 0 {
        return Err(IllegalArgumentException(format!(
            "Invalid byte count: {bytes} (element size {element_size})"
        ))
        .into());
    }

    let element_size = usize::try_from(element_size)?;
    let bytes = usize::try_from(bytes)?;
    if bytes == 0 {
        return Ok(None);
    }

    let vm = thread.vm()?;
    let native_mem = vm.native_memory();

    // Read all source bytes into a single buffer.
    let src_buf: Vec<u8> = if src.is_null() {
        native_mem.read_bytes(src_offset, bytes)
    } else {
        let src_off = usize::try_from(src_offset)?;
        let src_ref = src.as_reference()?;
        let Some(src_bytes) = src_ref.as_bytes() else {
            return Err(InternalError(
                "copySwapMemory0: invalid source type".to_string(),
            ));
        };
        if src_off.saturating_add(bytes) > src_bytes.len() {
            return Err(ArrayIndexOutOfBoundsException {
                index: i32::try_from(src_off + bytes).unwrap_or(i32::MAX),
                length: src_bytes.len(),
            }
            .into());
        }
        src_bytes[src_off..src_off + bytes].to_vec()
    };

    // Swap byte order per element.
    let mut swapped = Vec::with_capacity(bytes);
    for chunk in src_buf.chunks(element_size) {
        swapped.extend(chunk.iter().rev());
    }

    // Write swapped bytes to destination.
    if dest.is_null() {
        native_mem.write_bytes(dest_offset, &swapped);
    } else {
        let dest_off = usize::try_from(dest_offset)?;
        let mut dest_ref = dest.as_reference_mut()?;
        let Some(dest_bytes) = dest_ref.as_bytes_mut() else {
            return Err(InternalError(
                "copySwapMemory0: invalid destination type".to_string(),
            ));
        };
        if dest_off.saturating_add(bytes) > dest_bytes.len() {
            return Err(ArrayIndexOutOfBoundsException {
                index: i32::try_from(dest_off + bytes).unwrap_or(i32::MAX),
                length: dest_bytes.len(),
            }
            .into());
        }
        dest_bytes[dest_off..dest_off + bytes].copy_from_slice(&swapped);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    fn byte_array_value<V: VM>(vm: &Arc<V>, data: &[u8]) -> Value {
        let signed: Vec<i8> = data.iter().map(|&b| b.cast_signed()).collect();
        Value::new_object(vm.garbage_collector(), Reference::from(signed))
    }

    fn into_bytes(value: &Value) -> Vec<u8> {
        let r = value.as_reference().expect("array");
        let bytes = r.as_bytes().expect("byte array");
        bytes.to_vec()
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_array_to_array_size_2() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        let dst = byte_array_value(&vm, &[0u8; 6]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst.clone());
        params.push_long(0);
        params.push_long(6);
        params.push_long(2);
        let result = copy_swap_memory_0(thread, params).await?;
        assert!(result.is_none());
        assert_eq!(into_bytes(&dst), vec![0x02, 0x01, 0x04, 0x03, 0x06, 0x05]);
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_array_to_array_size_4() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let dst = byte_array_value(&vm, &[0u8; 8]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst.clone());
        params.push_long(0);
        params.push_long(8);
        params.push_long(4);
        copy_swap_memory_0(thread, params).await?;
        assert_eq!(into_bytes(&dst), vec![4, 3, 2, 1, 8, 7, 6, 5]);
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_array_to_array_size_8() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let dst = byte_array_value(&vm, &[0u8; 8]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst.clone());
        params.push_long(0);
        params.push_long(8);
        params.push_long(8);
        copy_swap_memory_0(thread, params).await?;
        assert_eq!(into_bytes(&dst), vec![8, 7, 6, 5, 4, 3, 2, 1]);
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_zero_bytes() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[1, 2, 3, 4]);
        let dst = byte_array_value(&vm, &[9, 9, 9, 9]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst.clone());
        params.push_long(0);
        params.push_long(0);
        params.push_long(2);
        let result = copy_swap_memory_0(thread, params).await?;
        assert!(result.is_none());
        assert_eq!(into_bytes(&dst), vec![9, 9, 9, 9]);
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_invalid_element_size() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[1, 2, 3]);
        let dst = byte_array_value(&vm, &[0u8; 3]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst);
        params.push_long(0);
        params.push_long(3);
        params.push_long(3); // invalid
        let result = copy_swap_memory_0(thread, params).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_misaligned_bytes() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let src = byte_array_value(&vm, &[1, 2, 3, 4, 5]);
        let dst = byte_array_value(&vm, &[0u8; 5]);
        let mut params = Parameters::default();
        params.push(src);
        params.push_long(0);
        params.push(dst);
        params.push_long(0);
        params.push_long(5); // not multiple of 2
        params.push_long(2);
        let result = copy_swap_memory_0(thread, params).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0_native_to_native() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let mem = vm.native_memory();
        let src_addr = mem.allocate(8);
        let dst_addr = mem.allocate(8);
        mem.write_bytes(src_addr, &[1u8, 2, 3, 4, 5, 6, 7, 8]);
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        params.push_long(src_addr);
        params.push(Value::Object(None));
        params.push_long(dst_addr);
        params.push_long(8);
        params.push_long(4);
        copy_swap_memory_0(thread, params).await?;
        let result = mem.read_bytes(dst_addr, 8);
        assert_eq!(result, vec![4, 3, 2, 1, 8, 7, 6, 5]);
        Ok(())
    }
}

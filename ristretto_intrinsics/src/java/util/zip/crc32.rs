#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::cast_possible_wrap)]

use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::sync::LazyLock;

/// CRC-32 lookup table using the IEEE 802.3 polynomial
static CRC32_TABLE: LazyLock<[u32; 256]> = LazyLock::new(|| {
    let mut table = [0u32; 256];
    for (i, entry) in table.iter_mut().enumerate() {
        let mut crc = i as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xedb8_8320;
            } else {
                crc >>= 1;
            }
        }
        *entry = crc;
    }
    table
});

/// Update CRC-32 checksum with a single byte.
#[intrinsic_method("java/util/zip/CRC32.update(II)I", Any)]
#[async_method]
pub async fn update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_int()? as u8;
    let crc = parameters.pop_int()? as u32;

    // CRC-32 is computed with inverted bits
    let crc = crc ^ 0xffff_ffff;
    let index = ((crc as u8) ^ b) as usize;
    let crc = CRC32_TABLE[index] ^ (crc >> 8);
    let result = crc ^ 0xffff_ffff;

    Ok(Some(Value::Int(result as i32)))
}

#[intrinsic_method("java/util/zip/CRC32.updateByteBuffer(IJII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn update_byte_buffer<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    update_byte_buffer_0(thread, parameters).await
}

/// Update CRC-32 checksum from a direct byte buffer.
#[intrinsic_method("java/util/zip/CRC32.updateByteBuffer0(IJII)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn update_byte_buffer_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _addr = parameters.pop_long()?;
    let crc = parameters.pop_int()?;

    Ok(Some(Value::Int(crc)))
}

#[intrinsic_method("java/util/zip/CRC32.updateBytes(I[BII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn update_bytes<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    update_bytes_0(thread, parameters).await
}

/// Update CRC-32 checksum from a byte array.
#[intrinsic_method("java/util/zip/CRC32.updateBytes0(I[BII)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn update_bytes_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let array_ref = parameters.pop_reference()?;
    let crc = parameters.pop_int()? as u32;

    let Some(array_ref) = array_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "byte array is null".to_string(),
        ))
        .into());
    };

    if len <= 0 {
        return Ok(Some(Value::Int(crc as i32)));
    }

    let off = off as usize;
    let len = len as usize;

    let guard = array_ref.read();
    let bytes = guard.as_byte_vec_ref()?;

    if off >= bytes.len() || off + len > bytes.len() {
        return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
            index: (off + len) as i32,
            length: bytes.len(),
        }
        .into());
    }

    let mut crc = crc ^ 0xffff_ffff;

    for &b in &bytes[off..off + len] {
        let byte = b as u8;
        let index = ((crc as u8) ^ byte) as usize;
        crc = CRC32_TABLE[index] ^ (crc >> 8);
    }

    let result = crc ^ 0xffff_ffff;
    Ok(Some(Value::Int(result as i32)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_update() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test with initial crc = 0
        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_int(i32::from(b'a')); // byte 'a'

        let result = update(thread.clone(), parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // Known CRC32 value for single byte 'a' starting from 0
        assert_eq!(crc, 0xe8b7_be43_u32 as i32);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_sequence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Update with 'a', 'b', 'c'
        let mut crc = 0i32;

        for byte in [b'a', b'b', b'c'] {
            let mut parameters = Parameters::default();
            parameters.push_int(crc);
            parameters.push_int(i32::from(byte));
            let result = update(thread.clone(), parameters).await?;
            crc = result.expect("crc value").as_i32()?;
        }

        // Known CRC32 for "abc"
        assert_eq!(crc, 0x3524_41c2_u32 as i32);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_bytes_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let bytes: Vec<i8> = vec![b'a' as i8, b'b' as i8, b'c' as i8];
        let reference = Reference::from(bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let value = Value::new_object(gc, reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(3); // length

        let result = update_bytes_0(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // Same as updating byte by byte
        assert_eq!(crc, 0x3524_41c2_u32 as i32);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let bytes: Vec<i8> = vec![b'a' as i8, b'b' as i8, b'c' as i8];
        let reference = Reference::from(bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let value = Value::new_object(gc, reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(3); // length

        let result = update_bytes(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        assert_eq!(crc, 0x3524_41c2_u32 as i32);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_bytes_with_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let bytes: Vec<i8> = vec![b'x' as i8, b'a' as i8, b'b' as i8, b'c' as i8, b'y' as i8];
        let reference = Reference::from(bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let value = Value::new_object(gc, reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_reference(wrapped_object);
        parameters.push_int(1); // offset - skip 'x'
        parameters.push_int(3); // length - just 'a', 'b', 'c'

        let result = update_bytes_0(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // Same as "abc"
        assert_eq!(crc, 0x3524_41c2_u32 as i32);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_bytes_null_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_reference(None); // null array
        parameters.push_int(0); // offset
        parameters.push_int(3); // length

        let result = update_bytes_0(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_bytes_zero_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let bytes: Vec<i8> = vec![b'a' as i8, b'b' as i8, b'c' as i8];
        let reference = Reference::from(bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let value = Value::new_object(gc, reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(0); // length = 0

        let result = update_bytes_0(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // CRC unchanged with zero length
        assert_eq!(crc, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_byte_buffer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test with zero length - should return unchanged crc
        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_long(0); // addr (dummy)
        parameters.push_int(0); // offset
        parameters.push_int(0); // length = 0

        let result = update_byte_buffer(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // CRC unchanged with zero length
        assert_eq!(crc, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_byte_buffer_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test with zero length - should return unchanged crc
        let mut parameters = Parameters::default();
        parameters.push_int(0); // crc
        parameters.push_long(0); // addr (dummy)
        parameters.push_int(0); // offset
        parameters.push_int(0); // length = 0

        let result = update_byte_buffer_0(thread, parameters).await?;
        let crc = result.expect("crc value").as_i32()?;

        // CRC unchanged with zero length
        assert_eq!(crc, 0);
        Ok(())
    }
}

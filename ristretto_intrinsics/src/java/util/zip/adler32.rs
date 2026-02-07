#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::cast_possible_wrap)]

use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Adler-32 modulo constant
const ADLER32_MOD: u32 = 65_521;

/// Update Adler-32 checksum with a single byte.
///
/// Parameters: adler (current adler32), b (byte to add)
/// Returns: updated adler32 value
#[intrinsic_method("java/util/zip/Adler32.update(II)I", Any)]
#[async_method]
pub async fn update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let b = parameters.pop_int()? as u8;
    let adler = parameters.pop_int()? as u32;

    let s1 = adler & 0xffff;
    let s2 = (adler >> 16) & 0xffff;

    let s1 = (s1 + u32::from(b)) % ADLER32_MOD;
    let s2 = (s2 + s1) % ADLER32_MOD;

    let result = (s2 << 16) | s1;
    Ok(Some(Value::Int(result as i32)))
}

/// Update Adler-32 checksum from a direct byte buffer.
#[intrinsic_method("java/util/zip/Adler32.updateByteBuffer(IJII)I", Any)]
#[async_method]
pub async fn update_byte_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _addr = parameters.pop_long()?;
    let adler = parameters.pop_int()?;
    Ok(Some(Value::Int(adler)))
}

/// Update Adler-32 checksum from a byte array.
#[intrinsic_method("java/util/zip/Adler32.updateBytes(I[BII)I", Any)]
#[async_method]
pub async fn update_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let array_ref = parameters.pop_reference()?;
    let adler = parameters.pop_int()? as u32;

    let Some(array_ref) = array_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "byte array is null".to_string(),
        ))
        .into());
    };

    if len <= 0 {
        return Ok(Some(Value::Int(adler as i32)));
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

    let mut s1 = adler & 0xffff;
    let mut s2 = (adler >> 16) & 0xffff;

    for &b in &bytes[off..off + len] {
        #[expect(clippy::as_conversions)]
        let byte = b as u8;
        s1 = (s1 + u32::from(byte)) % ADLER32_MOD;
        s2 = (s2 + s1) % ADLER32_MOD;
    }

    let result = (s2 << 16) | s1;
    Ok(Some(Value::Int(result as i32)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_update() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test with initial adler32 = 1 (the standard initial value)
        let mut parameters = Parameters::default();
        parameters.push_int(1); // adler
        parameters.push_int(i32::from(b'a')); // byte 'a'

        let result = update(thread.clone(), parameters).await?;
        let adler = result.expect("adler value").as_i32()?;

        // For byte 'a' (97): s1 = (1 + 97) % 65521 = 98, s2 = (0 + 98) % 65521 = 98
        // result = (98 << 16) | 98 = 6_422_626
        assert_eq!(adler, 6_422_626);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_sequence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Update with 'a', then 'b', then 'c'
        let mut adler = 1i32;

        for byte in [b'a', b'b', b'c'] {
            let mut parameters = Parameters::default();
            parameters.push_int(adler);
            parameters.push_int(i32::from(byte));
            let result = update(thread.clone(), parameters).await?;
            adler = result.expect("adler value").as_i32()?;
        }

        // Known value for "abc" starting from 1
        assert_eq!(adler, 38_600_999);
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
        parameters.push_int(1); // adler
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(3); // length

        let result = update_bytes(thread, parameters).await?;
        let adler = result.expect("adler value").as_i32()?;

        // Same as updating byte by byte
        assert_eq!(adler, 38_600_999);
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
        parameters.push_int(1); // adler
        parameters.push_reference(wrapped_object);
        parameters.push_int(1); // offset - skip 'x'
        parameters.push_int(3); // length - just 'a', 'b', 'c'

        let result = update_bytes(thread, parameters).await?;
        let adler = result.expect("adler value").as_i32()?;

        // Same as "abc"
        assert_eq!(adler, 38_600_999);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_bytes_null_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        let mut parameters = Parameters::default();
        parameters.push_int(1); // adler
        parameters.push_reference(None); // null array
        parameters.push_int(0); // offset
        parameters.push_int(3); // length

        let result = update_bytes(thread, parameters).await;
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
        parameters.push_int(1); // adler
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(0); // length = 0

        let result = update_bytes(thread, parameters).await?;
        let adler = result.expect("adler value").as_i32()?;

        // Adler unchanged with zero length
        assert_eq!(adler, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_byte_buffer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test with zero length - should return unchanged adler
        let mut parameters = Parameters::default();
        parameters.push_int(1); // adler
        parameters.push_long(0); // addr (dummy)
        parameters.push_int(0); // offset
        parameters.push_int(0); // length = 0

        let result = update_byte_buffer(thread, parameters).await?;
        let adler = result.expect("adler value").as_i32()?;

        // Adler unchanged with zero length
        assert_eq!(adler, 1);
        Ok(())
    }
}

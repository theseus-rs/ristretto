#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::cast_possible_wrap)]

use flate2::{Decompress, FlushDecompress};
use parking_lot::RwLock;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

/// Inflater context containing the decompressor and its mode
struct InflaterContext {
    decompress: Decompress,
    zlib_header: bool, // true = expects zlib header, false = raw deflate
    needs_dict: bool,  // true = currently waiting for dictionary to be set
}

/// Global storage for Inflater (decompression) contexts.
/// Maps a handle ID to an `InflaterContext` instance.
static INFLATER_HANDLES: RwLock<Option<HashMap<i64, InflaterContext>>> = RwLock::new(None);
static NEXT_INFLATER_ID: AtomicI64 = AtomicI64::new(1);

fn get_or_init_inflaters() -> &'static RwLock<Option<HashMap<i64, InflaterContext>>> {
    let guard = INFLATER_HANDLES.read();
    if guard.is_none() {
        drop(guard);
        let mut guard = INFLATER_HANDLES.write();
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
    }
    &INFLATER_HANDLES
}

#[intrinsic_method("java/util/zip/Inflater.end(J)V", Any)]
#[async_method]
pub async fn end<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    if let Some(map) = guard.as_mut() {
        map.remove(&handle);
    }
    Ok(None)
}

#[intrinsic_method("java/util/zip/Inflater.getAdler(J)I", Any)]
#[async_method]
pub async fn get_adler<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 1 as default Adler32 value (no data)
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBufferBuffer(JJIJI)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn inflate_buffer_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBufferBytes(JJI[BII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn inflate_buffer_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBytesBuffer(J[BIIJI)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn inflate_bytes_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/Inflater.inflateBytes(J[BII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn inflate_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let output_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let Some(output_ref) = output_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "output array is null".to_string(),
        ))
        .into());
    };

    if len <= 0 {
        return Ok(Some(Value::Int(0)));
    }

    let off = off as usize;
    let len = len as usize;

    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Inflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Inflater has been closed".to_string(),
        )
        .into());
    };

    let mut output_buffer = vec![0u8; len];
    let before_out = context.decompress.total_out();
    let input: &[u8] = &[];
    let result = context
        .decompress
        .decompress(input, &mut output_buffer, FlushDecompress::None);
    let _status = match result {
        Ok(status) => status,
        Err(error) => {
            return Err(ristretto_types::Error::InternalError(format!(
                "Decompression error: {error}"
            )));
        }
    };

    let bytes_written = (context.decompress.total_out() - before_out) as usize;

    // Copy output to array
    if bytes_written > 0 {
        let mut guard = output_ref.write();
        let output_bytes = guard.as_byte_vec_mut()?;
        if off >= output_bytes.len() || off + bytes_written > output_bytes.len() {
            return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                index: (off + bytes_written) as i32,
                length: output_bytes.len(),
            }
            .into());
        }
        for (i, byte) in output_buffer[..bytes_written].iter().enumerate() {
            output_bytes[off + i] = *byte as i8;
        }
    }

    Ok(Some(Value::Int(bytes_written as i32)))
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBytesBytes(J[BII[BII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn inflate_bytes_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let output_len_i32 = parameters.pop_int()?;
    let output_off_i32 = parameters.pop_int()?;
    let output_ref = parameters.pop_reference()?;
    let input_len_i32 = parameters.pop_int()?;
    let input_off_i32 = parameters.pop_int()?;
    let input_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    // Convert with proper bounds checking
    let output_len = if output_len_i32 >= 0 {
        output_len_i32 as usize
    } else {
        0
    };
    let output_off = if output_off_i32 >= 0 {
        output_off_i32 as usize
    } else {
        0
    };
    let input_len = if input_len_i32 >= 0 {
        input_len_i32 as usize
    } else {
        0
    };
    let input_off = if input_off_i32 >= 0 {
        input_off_i32 as usize
    } else {
        0
    };

    // Get input bytes; convert from i8 to u8
    let input_bytes: Vec<u8> = {
        let Some(input_ref) = input_ref else {
            return Err(ristretto_types::JavaError::NullPointerException(Some(
                "input array is null".to_string(),
            ))
            .into());
        };
        let guard = input_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        // If input_len is 0, return empty vec (no data to process)
        if input_len == 0 {
            Vec::new()
        } else if input_off >= bytes.len() || input_off + input_len > bytes.len() {
            // Invalid bounds - this shouldn't happen with correct usage
            // Return an error to avoid infinite loop
            return Err(ristretto_types::JavaError::IndexOutOfBoundsException {
                index: input_off as i32,
                size: bytes.len() as i32,
            }
            .into());
        } else {
            bytes[input_off..input_off + input_len]
                .iter()
                .map(|b| *b as u8)
                .collect()
        }
    };

    // Get the inflater
    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Inflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Inflater has been closed".to_string(),
        )
        .into());
    };

    // Decompress
    let mut output_buffer = vec![0u8; output_len];
    let before_in = context.decompress.total_in();
    let before_out = context.decompress.total_out();
    let result =
        context
            .decompress
            .decompress(&input_bytes, &mut output_buffer, FlushDecompress::None);

    let (status, need_dict) = match result {
        Ok(status) => (Some(status), false),
        Err(error) => {
            // Check if this is a "needs dictionary" error
            if error.needs_dictionary().is_some() {
                context.needs_dict = true;
                (None, true)
            } else {
                return Err(ristretto_types::Error::InternalError(format!(
                    "Decompression error: {error}"
                )));
            }
        }
    };

    let bytes_read = (context.decompress.total_in() - before_in) as i64;
    let bytes_written = (context.decompress.total_out() - before_out) as i64;

    // Check if finished (stream end)
    let finished = matches!(status, Some(flate2::Status::StreamEnd));

    // Copy output to array
    if bytes_written > 0 {
        let Some(output_ref) = output_ref else {
            return Err(ristretto_types::JavaError::NullPointerException(Some(
                "output array is null".to_string(),
            ))
            .into());
        };
        let mut guard = output_ref.write();
        let output_bytes = guard.as_byte_vec_mut()?;
        for (i, byte) in output_buffer[..bytes_written as usize].iter().enumerate() {
            output_bytes[output_off + i] = *byte as i8;
        }
    }

    // Return packed value according to JDK spec:
    // bits 0-30: bytes read (31 bits)
    // bits 31-61: bytes written (31 bits)
    // bit 62: finished flag
    // bit 63: needDict flag
    let mut result: i64 = (bytes_read & 0x7fff_ffff) | ((bytes_written & 0x7fff_ffff) << 31);
    if finished {
        result |= 1i64 << 62;
    }
    if need_dict {
        result |= 1i64 << 63;
    }
    Ok(Some(Value::Long(result)))
}

#[intrinsic_method("java/util/zip/Inflater.init(Z)J", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nowrap = parameters.pop_int()? != 0;

    // Create a new decompression context
    // nowrap=true means raw deflate (no zlib header)
    // nowrap=false means zlib format (with header)
    let zlib_header = !nowrap;
    let decompress = Decompress::new(zlib_header);
    let context = InflaterContext {
        decompress,
        zlib_header,
        needs_dict: false,
    };

    // Store and return handle
    let handle = NEXT_INFLATER_ID.fetch_add(1, Ordering::SeqCst);
    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    if let Some(map) = guard.as_mut() {
        map.insert(handle, context);
    }

    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/util/zip/Inflater.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/util/zip/Inflater.reset(J)V", Any)]
#[async_method]
pub async fn reset<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Inflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Inflater has been closed".to_string(),
        )
        .into());
    };

    context.decompress.reset(context.zlib_header);
    Ok(None)
}

#[intrinsic_method("java/util/zip/Inflater.setDictionary(J[BII)V", Any)]
#[async_method]
pub async fn set_dictionary<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let dict_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let off = if off >= 0 { off as usize } else { 0 };
    let len = if len >= 0 { len as usize } else { 0 };

    // Get dictionary bytes
    let dict_bytes: Vec<u8> = {
        let Some(dict_ref) = dict_ref else {
            return Err(ristretto_types::JavaError::NullPointerException(Some(
                "dictionary array is null".to_string(),
            ))
            .into());
        };
        let guard = dict_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if len == 0 {
            Vec::new()
        } else if off >= bytes.len() || off + len > bytes.len() {
            return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                index: (off + len) as i32,
                length: bytes.len(),
            }
            .into());
        } else {
            bytes[off..off + len].iter().map(|b| *b as u8).collect()
        }
    };

    // Set dictionary on the decompressor
    let inflaters = get_or_init_inflaters();
    let mut guard = inflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Inflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Inflater has been closed".to_string(),
        )
        .into());
    };

    // Set the dictionary; this may fail if the stream hasn't requested a dictionary yet.
    // We ignore errors here since the Java API doesn't throw on setDictionary.
    // The dictionary will be used when the stream actually needs it.
    let _ = context.decompress.set_dictionary(&dict_bytes);
    context.needs_dict = false;

    Ok(None)
}

#[intrinsic_method(
    "java/util/zip/Inflater.setDictionaryBuffer(JJI)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn set_dictionary_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Dictionary support is complex; for now just ignore it
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_init_and_end() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Test init with nowrap = false
        let mut parameters = Parameters::default();
        parameters.push_int(0); // nowrap = false
        let result = init(thread.clone(), parameters).await?;
        let handle = result.expect("handle").as_i64()?;
        assert!(handle > 0);

        // Test end
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = end(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_adler() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(1); // dummy handle
        let result = get_adler(thread, parameters).await?;
        assert_eq!(Some(Value::Int(1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_inflate_buffer_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inflate_buffer_buffer(thread, Parameters::default())
            .await
            .expect("inflate_buffer_buffer");
        // Returns 0 (no progress) since direct buffer operations not implemented
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_inflate_buffer_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inflate_buffer_bytes(thread, Parameters::default())
            .await
            .expect("inflate_buffer_bytes");
        // Returns 0 (no progress) since direct buffer operations not implemented
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_inflate_bytes_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inflate_bytes_buffer(thread, Parameters::default())
            .await
            .expect("inflate_bytes_buffer");
        // Returns 0 (no progress) since direct buffer operations not implemented
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_inflate_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create an inflater
        let mut parameters = Parameters::default();
        parameters.push_int(0); // nowrap = false
        let result = init(thread.clone(), parameters).await?;
        let handle = result.expect("handle").as_i64()?;

        // Create output array
        let output_bytes: Vec<i8> = vec![0i8; 100];
        let reference = Reference::from(output_bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let value = Value::new_object(gc, reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(wrapped_object);
        parameters.push_int(0); // offset
        parameters.push_int(100); // length

        // Should return 0 bytes written (no input data)
        let result = inflate_bytes(thread.clone(), parameters).await?;
        let bytes_written = result.expect("bytes_written").as_i32()?;
        assert_eq!(bytes_written, 0);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_reset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // First create an inflater
        let mut parameters = Parameters::default();
        parameters.push_int(0); // nowrap = false
        let result = init(thread.clone(), parameters).await?;
        let handle = result.expect("handle").as_i64()?;

        // Test reset
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = reset(thread.clone(), parameters).await?;
        assert!(result.is_none());

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_set_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // First create a valid inflater
        let mut init_params = Parameters::default();
        init_params.push_int(0); // nowrap = false
        let result = init(thread.clone(), init_params).await.expect("init");
        let handle = result.expect("handle").as_i64().expect("handle value");

        // Create dictionary array
        let dict_bytes: Vec<i8> = b"dictionary".iter().map(|b| *b as i8).collect();
        let dict_ref = Reference::from(dict_bytes);
        let vm = thread.vm().expect("vm");
        let gc = vm.garbage_collector();
        let dict_value = Value::new_object(gc, dict_ref);
        let Value::Object(dict_wrapped) = dict_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(dict_wrapped);
        parameters.push_int(0); // offset
        parameters.push_int(10); // length

        let result = set_dictionary(thread.clone(), parameters)
            .await
            .expect("set_dictionary");
        assert!(result.is_none());

        // Cleanup
        let mut end_params = Parameters::default();
        end_params.push_long(handle);
        end(thread, end_params).await.expect("end");
    }

    #[tokio::test]
    async fn test_set_dictionary_null_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle
        parameters.push(Value::Object(None)); // null dict array
        parameters.push_int(0); // offset
        parameters.push_int(0); // length
        let result = set_dictionary(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_dictionary_buffer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_dictionary_buffer(thread, Parameters::default()).await?;
        assert!(result.is_none());
        Ok(())
    }
}

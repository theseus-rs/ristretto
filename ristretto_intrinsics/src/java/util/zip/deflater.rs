#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::cast_possible_wrap)]

use flate2::{Compress, Compression, FlushCompress, Status};
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

/// Deflater context containing the compressor and its configuration
struct DeflaterContext {
    compress: Compress,
    level: i32,
    strategy: i32,
    zlib_header: bool, // true = zlib header, false = raw deflate
    finished: bool,
    input_consumed: u64,
    output_produced: u64,
    adler32: u32, // Running Adler32 checksum of input data
}

/// Global storage for Deflater (compression) contexts.
static DEFLATER_HANDLES: RwLock<Option<HashMap<i64, DeflaterContext>>> = RwLock::new(None);
static NEXT_DEFLATER_ID: AtomicI64 = AtomicI64::new(1);

fn get_or_init_deflaters() -> &'static RwLock<Option<HashMap<i64, DeflaterContext>>> {
    let guard = DEFLATER_HANDLES.read();
    if guard.is_none() {
        drop(guard);
        let mut guard = DEFLATER_HANDLES.write();
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
    }
    &DEFLATER_HANDLES
}

/// Adler-32 modulo constant
const ADLER32_MOD: u32 = 65_521;

/// Update an Adler32 checksum with a slice of bytes
fn update_adler32(adler: u32, data: &[u8]) -> u32 {
    let mut s1 = adler & 0xffff;
    let mut s2 = (adler >> 16) & 0xffff;

    for &byte in data {
        s1 = (s1 + u32::from(byte)) % ADLER32_MOD;
        s2 = (s2 + s1) % ADLER32_MOD;
    }

    (s2 << 16) | s1
}

fn level_to_compression(level: i32) -> Compression {
    match level {
        0 => Compression::none(),
        1..=9 => Compression::new(level as u32),
        _ => Compression::default(), // DEFAULT_COMPRESSION = -1 and any other value
    }
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytes(J[BIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
#[expect(clippy::match_same_arms)]
pub async fn deflate_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flush_mode = parameters.pop_int()?;
    let out_len = parameters.pop_int()?;
    let out_off = parameters.pop_int()?;
    let output_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let Some(output_ref) = output_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "output array is null".to_string(),
        ))
        .into());
    };

    if out_len <= 0 {
        return Ok(Some(Value::Int(0)));
    }

    let out_off = out_off as usize;
    let out_len = out_len as usize;

    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Deflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Deflater has been closed".to_string(),
        )
        .into());
    };

    let flush = match flush_mode {
        0 => FlushCompress::None,
        2 => FlushCompress::Sync,
        3 => FlushCompress::Full,
        4 => FlushCompress::Finish,
        _ => FlushCompress::None,
    };

    let mut output_buffer = vec![0u8; out_len];
    let before_out = context.compress.total_out();
    let input: &[u8] = &[];
    let status = context.compress.compress(input, &mut output_buffer, flush);

    let status = match status {
        Ok(s) => s,
        Err(e) => {
            return Err(ristretto_types::Error::InternalError(format!(
                "Compression error: {e}"
            )));
        }
    };

    let bytes_written = (context.compress.total_out() - before_out) as usize;
    context.output_produced += bytes_written as u64;

    if matches!(status, Status::StreamEnd) {
        context.finished = true;
    }

    // Copy output to array
    if bytes_written > 0 {
        let mut guard = output_ref.write();
        let output_bytes = guard.as_byte_vec_mut()?;
        for (i, byte) in output_buffer[..bytes_written].iter().enumerate() {
            output_bytes[out_off + i] = *byte as i8;
        }
    }

    Ok(Some(Value::Int(bytes_written as i32)))
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBufferBuffer(JJIJIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn deflate_buffer_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flush_mode = parameters.pop_int()?;
    let _params = parameters.pop_int()?;
    let _out_len = parameters.pop_int()?;
    let _out_addr = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_addr = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;

    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBufferBytes(JJI[BIIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn deflate_buffer_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flush_mode = parameters.pop_int()?;
    let _params = parameters.pop_int()?;
    let _out_len = parameters.pop_int()?;
    let _out_off = parameters.pop_int()?;
    let _output_ref = parameters.pop_reference()?;
    let _in_len = parameters.pop_int()?;
    let _in_addr = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;

    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytesBuffer(J[BIIJIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn deflate_bytes_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flush_mode = parameters.pop_int()?;
    let _params = parameters.pop_int()?;
    let _out_len = parameters.pop_int()?;
    let _out_addr = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_off = parameters.pop_int()?;
    let _input_ref = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;

    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytesBytes(J[BII[BIIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
#[expect(clippy::match_same_arms)]
#[expect(clippy::too_many_lines)]
pub async fn deflate_bytes_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _params = parameters.pop_int()?;
    let flush_mode = parameters.pop_int()?;
    let out_len = parameters.pop_int()?;
    let out_off = parameters.pop_int()?;
    let output_ref = parameters.pop_reference()?;
    let in_len = parameters.pop_int()?;
    let in_off = parameters.pop_int()?;
    let input_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let Some(input_ref) = input_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "input array is null".to_string(),
        ))
        .into());
    };
    let Some(output_ref) = output_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "output array is null".to_string(),
        ))
        .into());
    };

    if out_len <= 0 {
        return Ok(Some(Value::Long(0)));
    }

    let in_off = in_off as usize;
    let in_len = in_len as usize;
    let out_off = out_off as usize;
    let out_len = out_len as usize;

    // Get input bytes
    let input_bytes: Vec<u8> = {
        let guard = input_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if in_len == 0 {
            Vec::new()
        } else if in_off >= bytes.len() || in_off + in_len > bytes.len() {
            return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                index: (in_off + in_len) as i32,
                length: bytes.len(),
            }
            .into());
        } else {
            bytes[in_off..in_off + in_len]
                .iter()
                .map(|b| *b as u8)
                .collect()
        }
    };

    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Deflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Deflater has been closed".to_string(),
        )
        .into());
    };

    let flush = match flush_mode {
        0 => FlushCompress::None,
        2 => FlushCompress::Sync,
        3 => FlushCompress::Full,
        4 => FlushCompress::Finish,
        _ => FlushCompress::None,
    };

    let mut output_buffer = vec![0u8; out_len];
    let before_in = context.compress.total_in();
    let before_out = context.compress.total_out();

    let status = context
        .compress
        .compress(&input_bytes, &mut output_buffer, flush);
    let status = match status {
        Ok(s) => s,
        Err(e) => {
            return Err(ristretto_types::Error::InternalError(format!(
                "Compression error: {e}"
            )));
        }
    };

    let bytes_read = (context.compress.total_in() - before_in) as i64;
    let bytes_written = (context.compress.total_out() - before_out) as i64;
    context.input_consumed += bytes_read as u64;
    context.output_produced += bytes_written as u64;

    // Update Adler32 checksum with the input data that was consumed
    if bytes_read > 0 {
        let consumed_input = &input_bytes[..bytes_read as usize];
        context.adler32 = update_adler32(context.adler32, consumed_input);
    }

    let finished = matches!(status, Status::StreamEnd);
    if finished {
        context.finished = true;
    }

    // Copy output to array
    if bytes_written > 0 {
        let mut guard = output_ref.write();
        let output_bytes = guard.as_byte_vec_mut()?;
        for (i, byte) in output_buffer[..bytes_written as usize].iter().enumerate() {
            output_bytes[out_off + i] = *byte as i8;
        }
    }

    // Return packed value
    let mut result: i64 = (bytes_read & 0x7fff_ffff) | ((bytes_written & 0x7fff_ffff) << 31);
    if finished {
        result |= 1i64 << 62;
    }
    Ok(Some(Value::Long(result)))
}

#[intrinsic_method("java/util/zip/Deflater.end(J)V", Any)]
#[async_method]
pub async fn end<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    if let Some(map) = guard.as_mut() {
        map.remove(&handle);
    }
    Ok(None)
}

#[intrinsic_method("java/util/zip/Deflater.getAdler(J)I", Any)]
#[async_method]
pub async fn get_adler<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let deflaters = get_or_init_deflaters();
    let guard = deflaters.read();
    let Some(map) = guard.as_ref() else {
        return Err(ristretto_types::Error::InternalError(
            "Deflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Deflater has been closed".to_string(),
        )
        .into());
    };

    // Return the running Adler32 checksum of all input data
    Ok(Some(Value::Int(context.adler32 as i32)))
}

/// Initialize a new Deflater.
#[intrinsic_method("java/util/zip/Deflater.init(IIZ)J", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nowrap = parameters.pop_int()? != 0;
    let strategy = parameters.pop_int()?;
    let level = parameters.pop_int()?;

    let compression = level_to_compression(level);
    let zlib_header = !nowrap;
    let compress = Compress::new(compression, zlib_header);

    let context = DeflaterContext {
        compress,
        level,
        strategy,
        zlib_header,
        finished: false,
        input_consumed: 0,
        output_produced: 0,
        adler32: 1, // Initial Adler32 value
    };

    let handle = NEXT_DEFLATER_ID.fetch_add(1, Ordering::SeqCst);
    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    if let Some(map) = guard.as_mut() {
        map.insert(handle, context);
    }

    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/util/zip/Deflater.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/util/zip/Deflater.reset(J)V", Any)]
#[async_method]
pub async fn reset<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Deflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Deflater has been closed".to_string(),
        )
        .into());
    };

    // Reset the compressor
    context.compress.reset();
    context.finished = false;
    context.input_consumed = 0;
    context.output_produced = 0;
    context.adler32 = 1; // Reset Adler32 to initial value
    Ok(None)
}

#[intrinsic_method("java/util/zip/Deflater.setDictionary(J[BII)V", Any)]
#[async_method]
pub async fn set_dictionary<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let dict_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let Some(dict_ref) = dict_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "dictionary array is null".to_string(),
        ))
        .into());
    };

    if len <= 0 {
        return Ok(None);
    }

    let off = off as usize;
    let len = len as usize;

    // Get dictionary bytes
    let dict_bytes: Vec<u8> = {
        let guard = dict_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if off >= bytes.len() || off + len > bytes.len() {
            return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                index: (off + len) as i32,
                length: bytes.len(),
            }
            .into());
        }
        bytes[off..off + len].iter().map(|b| *b as u8).collect()
    };

    let deflaters = get_or_init_deflaters();
    let mut guard = deflaters.write();
    let Some(map) = guard.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "Deflater handles not initialized".to_string(),
        ));
    };
    let Some(context) = map.get_mut(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Deflater has been closed".to_string(),
        )
        .into());
    };

    // flate2's set_dictionary is available but requires careful usage
    if let Err(e) = context.compress.set_dictionary(&dict_bytes) {
        return Err(ristretto_types::Error::InternalError(format!(
            "Failed to set dictionary: {e}"
        )));
    }

    Ok(None)
}

#[intrinsic_method(
    "java/util/zip/Deflater.setDictionaryBuffer(JJI)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn set_dictionary_buffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _addr = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;

    // Direct buffer operations require unsafe code which is not allowed.
    // This is a no-op; Java code should use setDictionary with byte arrays.
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    async fn create_deflater<T: ristretto_types::Thread + 'static>(
        thread: &Arc<T>,
        level: i32,
        nowrap: bool,
    ) -> Result<i64> {
        let mut parameters = Parameters::default();
        parameters.push_int(level); // level
        parameters.push_int(0); // strategy = DEFAULT_STRATEGY
        parameters.push_int(i32::from(nowrap)); // nowrap
        let result = init(thread.clone(), parameters).await?;
        Ok(result.expect("handle").as_i64()?)
    }

    #[test]
    fn test_flate2_behavior() {
        // Test that flate2 compresses correctly
        let input = b"Hello World Hello World Hello World";
        let mut compress = Compress::new(Compression::default(), true);
        let mut output = vec![0u8; 200];

        let status = compress
            .compress(input, &mut output, FlushCompress::Finish)
            .unwrap();

        let bytes_read = compress.total_in();
        let bytes_written = compress.total_out();

        assert_eq!(bytes_read, 35, "Should read all input");
        assert!(
            bytes_written > 2,
            "Should write more than just header: {bytes_written}"
        );
        assert!(
            matches!(status, Status::StreamEnd),
            "Should finish: {status:?}"
        );
    }

    #[tokio::test]
    async fn test_init_and_end() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test init with default compression
        let handle = create_deflater(&thread, -1, false).await?;
        assert!(handle > 0);

        // Test end
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = end(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_init_nowrap() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Test init with nowrap = true (raw deflate)
        let handle = create_deflater(&thread, 6, true).await?;
        assert!(handle > 0);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_adler() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, -1, false).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_adler(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(1)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_reset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, -1, false).await?;

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
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_deflate_bytes_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        // Create input array
        let input_bytes: Vec<i8> = b"Hello, World!".iter().map(|b| *b as i8).collect();
        let input_ref = Reference::from(input_bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let input_value = Value::new_object(gc, input_ref);
        let Value::Object(input_wrapped) = input_value else {
            panic!("expected object");
        };

        // Create output array (larger to accommodate compression overhead)
        let output_bytes: Vec<i8> = vec![0i8; 100];
        let output_ref = Reference::from(output_bytes);
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(output_wrapped) = output_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(input_wrapped);
        parameters.push_int(0); // in_off
        parameters.push_int(13); // in_len
        parameters.push_reference(output_wrapped);
        parameters.push_int(0); // out_off
        parameters.push_int(100); // out_len
        parameters.push_int(4); // flush = FINISH
        parameters.push_int(0); // params (unused)

        let result = deflate_bytes_bytes(thread.clone(), parameters).await?;
        let packed = result.expect("result").as_i64()?;

        // Extract bytes_read from bits 0-30
        let bytes_read = packed & 0x7fff_ffff;
        // Extract bytes_written from bits 31-61
        let bytes_written = (packed >> 31) & 0x7fff_ffff;
        // Check finished flag (bit 62)
        let finished = (packed >> 62) & 1;

        // Should have read all 13 input bytes
        assert_eq!(bytes_read, 13, "Should have read all 13 input bytes");
        // Should have written some output
        assert!(
            bytes_written > 0,
            "Should have written compressed output, got {bytes_written}"
        );
        // Should be finished since we used FINISH flush
        assert_eq!(finished, 1, "Should be finished");

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_deflate_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        // Create output array
        let output_bytes: Vec<i8> = vec![0i8; 100];
        let output_ref = Reference::from(output_bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(output_wrapped) = output_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(output_wrapped);
        parameters.push_int(0); // out_off
        parameters.push_int(100); // out_len
        parameters.push_int(0); // flush

        let result = deflate_bytes(thread.clone(), parameters).await?;
        // Should return number of bytes written (possibly 0 with no input)
        let bytes_written = result.expect("result").as_i32()?;
        assert!(bytes_written >= 0);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_set_dictionary() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        // Create dictionary array
        let dict_bytes: Vec<i8> = b"dictionary".iter().map(|b| *b as i8).collect();
        let dict_ref = Reference::from(dict_bytes);
        let vm = thread.vm()?;
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

        let result = set_dictionary(thread.clone(), parameters).await?;
        assert!(result.is_none());

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_deflate_buffer_buffer_zero_len() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(0); // in_addr
        parameters.push_int(0); // in_len
        parameters.push_long(0); // out_addr
        parameters.push_int(0); // out_len = 0
        parameters.push_int(0); // params
        parameters.push_int(0); // flush

        let result = deflate_buffer_buffer(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_deflate_buffer_bytes_zero_len() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        let output_bytes: Vec<i8> = vec![0i8; 100];
        let output_ref = Reference::from(output_bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(output_wrapped) = output_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(0); // in_addr
        parameters.push_int(0); // in_len
        parameters.push_reference(output_wrapped);
        parameters.push_int(0); // out_off
        parameters.push_int(0); // out_len = 0
        parameters.push_int(0); // params
        parameters.push_int(0); // flush

        let result = deflate_buffer_bytes(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_deflate_bytes_buffer_zero_len() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        let input_bytes: Vec<i8> = vec![b'a' as i8];
        let input_ref = Reference::from(input_bytes);
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let input_value = Value::new_object(gc, input_ref);
        let Value::Object(input_wrapped) = input_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(input_wrapped);
        parameters.push_int(0); // in_off
        parameters.push_int(1); // in_len
        parameters.push_long(0); // out_addr
        parameters.push_int(0); // out_len = 0
        parameters.push_int(0); // params
        parameters.push_int(0); // flush

        let result = deflate_bytes_buffer(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_set_dictionary_buffer_zero_len() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let handle = create_deflater(&thread, 6, false).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(0); // addr
        parameters.push_int(0); // len = 0

        let result = set_dictionary_buffer(thread.clone(), parameters).await?;
        assert!(result.is_none());

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        end(thread, parameters).await?;
        Ok(())
    }
}

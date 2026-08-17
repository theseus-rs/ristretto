use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{JavaError, JavaObject, Parameters, Result, Thread, VM as _};
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::{Arc, Mutex};
#[cfg(not(target_family = "wasm"))]
use std::{io::Read, process::Command};
#[cfg(not(target_family = "wasm"))]
use zip::ZipArchive;

const MAX_PACKED_BYTES: usize = 512 * 1024 * 1024;
const MAX_EXPANDED_BYTES: usize = 1024 * 1024 * 1024;
const MAX_ENTRIES: usize = 100_000;

#[derive(Debug)]
struct UnpackedEntry {
    name: String,
    data: Vec<u8>,
    modification_time: i32,
    deflated: bool,
}

#[derive(Debug, Default)]
struct UnpackContext {
    entries: VecDeque<UnpackedEntry>,
    options: HashMap<String, String>,
    consumed: i64,
}

#[derive(Debug, Default)]
struct NativeUnpackState {
    contexts: Mutex<HashMap<usize, UnpackContext>>,
}

fn receiver_key(receiver: &Value) -> usize {
    match receiver {
        Value::Object(Some(reference)) => reference.as_ptr() as usize,
        _ => 0,
    }
}

fn state<T: Thread + 'static>(thread: &T) -> Result<Arc<NativeUnpackState>> {
    thread
        .vm()?
        .resource_manager()
        .get_or_init(NativeUnpackState::default)
}

async fn byte_buffer_bytes<T: Thread + 'static>(
    thread: &Arc<T>,
    buffer: &Value,
) -> Result<Vec<u8>> {
    let remaining = thread
        .try_invoke(
            "java/nio/Buffer",
            "remaining()I",
            std::slice::from_ref(buffer),
        )
        .await?
        .as_i32()?;
    let remaining = usize::try_from(remaining)?;
    if remaining > MAX_PACKED_BYTES {
        return Err(JavaError::IoException(format!(
            "Pack200 input exceeds the {MAX_PACKED_BYTES}-byte limit"
        ))
        .into());
    }
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let mut byte_array = Vec::new();
    byte_array.try_reserve_exact(remaining).map_err(|error| {
        ristretto_types::Error::InternalError(format!(
            "Cannot allocate Pack200 input ({remaining} bytes): {error}"
        ))
    })?;
    byte_array.resize(remaining, 0_i8);
    let bytes = Value::new_object(collector, Reference::from(byte_array));
    thread
        .try_invoke(
            "java/nio/ByteBuffer",
            "get([B)Ljava/nio/ByteBuffer;",
            &[buffer.clone(), bytes.clone()],
        )
        .await?;
    let bytes = bytes.as_byte_vec_ref()?;
    let mut result = Vec::new();
    result.try_reserve_exact(remaining).map_err(|error| {
        ristretto_types::Error::InternalError(format!(
            "Cannot copy Pack200 input ({remaining} bytes): {error}"
        ))
    })?;
    result.extend(bytes.iter().map(|byte| byte.cast_unsigned()));
    Ok(result)
}

async fn input_stream_bytes<T: Thread + 'static>(
    thread: &Arc<T>,
    receiver: &Value,
) -> Result<Vec<u8>> {
    let input = receiver.as_object_ref()?.value("in")?;
    if input.is_null() {
        return Ok(Vec::new());
    }
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let buffer = Value::new_object(collector, Reference::from(vec![0_i8; 16 * 1024]));
    let mut result = Vec::new();
    loop {
        let count = thread
            .try_invoke(
                "java/io/InputStream",
                "read([BII)I",
                &[
                    input.clone(),
                    buffer.clone(),
                    Value::Int(0),
                    Value::Int(16 * 1024),
                ],
            )
            .await?
            .as_i32()?;
        if count <= 0 {
            break;
        }
        let count = usize::try_from(count)?;
        let new_length = result
            .len()
            .checked_add(count)
            .ok_or_else(|| JavaError::IoException("Pack200 input size overflow".to_string()))?;
        if new_length > MAX_PACKED_BYTES {
            return Err(JavaError::IoException(format!(
                "Pack200 input exceeds the {MAX_PACKED_BYTES}-byte limit"
            ))
            .into());
        }
        let bytes = buffer.as_byte_vec_ref()?;
        let bytes = bytes.get(..count).ok_or_else(|| {
            JavaError::IoException("Pack200 stream returned an invalid byte count".to_string())
        })?;
        result.try_reserve(count).map_err(|error| {
            ristretto_types::Error::InternalError(format!(
                "Cannot buffer Pack200 input ({new_length} bytes): {error}"
            ))
        })?;
        result.extend(bytes.iter().map(|byte| byte.cast_unsigned()));
    }
    Ok(result)
}

fn unpack200_executable(java_home: &Path) -> std::path::PathBuf {
    let executable = java_home.join("bin").join("unpack200");
    if executable.is_file() {
        return executable;
    }
    executable.with_extension("exe")
}

fn modification_time(date_time: Option<zip::DateTime>) -> i32 {
    let Some(date_time) = date_time else {
        return 0;
    };
    let Ok(date_time) = jiff::civil::DateTime::new(
        i16::try_from(date_time.year()).unwrap_or_default(),
        i8::try_from(date_time.month()).unwrap_or_default(),
        i8::try_from(date_time.day()).unwrap_or_default(),
        i8::try_from(date_time.hour()).unwrap_or_default(),
        i8::try_from(date_time.minute()).unwrap_or_default(),
        i8::try_from(date_time.second()).unwrap_or_default(),
        0,
    ) else {
        return 0;
    };
    date_time
        .to_zoned(jiff::tz::TimeZone::system())
        .ok()
        .and_then(|date_time| i32::try_from(date_time.timestamp().as_second()).ok())
        .unwrap_or_default()
}

#[cfg(not(target_family = "wasm"))]
fn decode_pack200(java_home: &Path, packed: &[u8]) -> Result<VecDeque<UnpackedEntry>> {
    let executable = unpack200_executable(java_home);
    if !executable.is_file() {
        return Err(JavaError::IoException(format!(
            "Pack200 decoder is unavailable at {}",
            executable.display()
        ))
        .into());
    }

    let directory = tempfile::tempdir().map_err(|error| {
        JavaError::IoException(format!(
            "Could not create Pack200 temporary directory: {error}"
        ))
    })?;
    let input_path = directory.path().join("input.pack");
    let output_path = directory.path().join("output.jar");
    std::fs::write(&input_path, packed).map_err(|error| {
        JavaError::IoException(format!("Could not write Pack200 input: {error}"))
    })?;
    let output = Command::new(&executable)
        .arg(&input_path)
        .arg(&output_path)
        .output()
        .map_err(|error| {
            JavaError::IoException(format!("Could not run {}: {error}", executable.display()))
        })?;
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        return Err(
            JavaError::IoException(format!("Pack200 decoding failed: {}", message.trim())).into(),
        );
    }

    let jar = std::fs::File::open(&output_path).map_err(|error| {
        JavaError::IoException(format!("Could not open decoded Pack200 archive: {error}"))
    })?;
    let mut archive = ZipArchive::new(jar).map_err(|error| {
        JavaError::IoException(format!("Invalid decoded Pack200 archive: {error}"))
    })?;
    if archive.len() > MAX_ENTRIES {
        return Err(JavaError::IoException(format!(
            "Pack200 archive exceeds the {MAX_ENTRIES}-entry limit"
        ))
        .into());
    }

    let mut entries = VecDeque::with_capacity(archive.len());
    let mut expanded_bytes = 0_usize;
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).map_err(|error| {
            JavaError::IoException(format!("Could not read Pack200 entry {index}: {error}"))
        })?;
        let size = usize::try_from(file.size()).map_err(|_| {
            JavaError::IoException("Pack200 entry is too large for this platform".to_string())
        })?;
        expanded_bytes = expanded_bytes
            .checked_add(size)
            .ok_or_else(|| JavaError::IoException("Pack200 expanded size overflow".to_string()))?;
        if expanded_bytes > MAX_EXPANDED_BYTES {
            return Err(JavaError::IoException(format!(
                "Pack200 output exceeds the {MAX_EXPANDED_BYTES}-byte limit"
            ))
            .into());
        }
        let mut data = Vec::new();
        data.try_reserve_exact(size).map_err(|error| {
            ristretto_types::Error::InternalError(format!(
                "Could not allocate Pack200 entry ({size} bytes): {error}"
            ))
        })?;
        file.read_to_end(&mut data).map_err(|error| {
            JavaError::IoException(format!("Could not extract Pack200 entry {index}: {error}"))
        })?;
        let modification_time = modification_time(file.last_modified());
        entries.push_back(UnpackedEntry {
            name: file.name().to_string(),
            data,
            modification_time,
            deflated: file.compression() == zip::CompressionMethod::Deflated,
        });
    }
    Ok(entries)
}

#[cfg(target_family = "wasm")]
fn decode_pack200(_java_home: &Path, _packed: &[u8]) -> Result<VecDeque<UnpackedEntry>> {
    Err(JavaError::IoException("Pack200 decoding is unavailable on WebAssembly".to_string()).into())
}

async fn wrap_byte_buffer<T: Thread + 'static>(thread: &Arc<T>, data: Vec<u8>) -> Result<Value> {
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let length = data.len();
    let mut signed_data = Vec::new();
    signed_data.try_reserve_exact(length).map_err(|error| {
        ristretto_types::Error::InternalError(format!(
            "Cannot allocate Pack200 Java byte buffer ({length} bytes): {error}"
        ))
    })?;
    signed_data.extend(data.into_iter().map(u8::cast_signed));
    let array = Value::new_object(collector, Reference::from(signed_data));
    thread
        .try_invoke(
            "java/nio/ByteBuffer",
            "wrap([B)Ljava/nio/ByteBuffer;",
            &[array],
        )
        .await
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.finish()J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn finish<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let receiver = parameters.pop().unwrap_or(Value::Object(None));
    let key = receiver_key(&receiver);
    let consumed = state(thread.as_ref())?
        .contexts
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remove(&key)
        .map_or(0, |context| context.consumed);
    if !receiver.is_null() {
        receiver
            .as_object_mut()?
            .set_value("unpackerPtr", Value::Long(0))?;
    }
    Ok(Some(Value::Long(consumed)))
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getNextFile([Ljava/lang/Object;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_next_file<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let parts = parameters.pop()?;
    let receiver = parameters.pop().unwrap_or(Value::Object(None));
    let entry = state(thread.as_ref())?
        .contexts
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get_mut(&receiver_key(&receiver))
        .and_then(|context| context.entries.pop_front());
    let Some(entry) = entry else {
        return Ok(Some(Value::from(false)));
    };

    let size = u64::try_from(entry.data.len())?;
    let name = entry.name.to_object(&thread).await?;
    let data = wrap_byte_buffer(&thread, entry.data).await?;
    let int_parts = {
        let (_class, values) = parts.as_class_vec_ref()?;
        values
            .first()
            .cloned()
            .ok_or(JavaError::ArrayIndexOutOfBoundsException {
                index: 0,
                length: 0,
            })?
    };
    {
        let mut values = int_parts.as_int_vec_mut()?;
        let values_length = values.len();
        let [size_high, size_low, modification_time, deflated, ..] = &mut *values else {
            return Err(JavaError::ArrayIndexOutOfBoundsException {
                index: 3,
                length: values_length,
            }
            .into());
        };
        *size_high = i32::try_from(size >> 32)?;
        *size_low = u32::try_from(size & u64::from(u32::MAX))?.cast_signed();
        *modification_time = entry.modification_time;
        *deflated = i32::from(entry.deflated);
    }
    {
        let (_class, mut values) = parts.as_class_vec_mut()?;
        let values_length = values.len();
        let [_, name_slot, data_slot, trailing_data_slot, ..] = &mut *values else {
            return Err(JavaError::ArrayIndexOutOfBoundsException {
                index: 3,
                length: values_length,
            }
            .into());
        };
        *name_slot = name;
        *data_slot = data;
        *trailing_data_slot = Value::Object(None);
    }
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let property = parameters.pop()?;
    let receiver = parameters.pop().unwrap_or(Value::Object(None));
    if property.is_null() {
        return Ok(Some(Value::Object(None)));
    }
    let property = property.as_string()?;
    let value = state(thread.as_ref())?
        .contexts
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&receiver_key(&receiver))
        .and_then(|context| context.options.get(&property).cloned());
    match value {
        Some(value) => Ok(Some(value.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_unused_input<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // The runtime decoder consumes the complete supplied Pack200 stream in one operation.
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.initIDs()V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let property = parameters.pop()?;
    let receiver = parameters.pop().unwrap_or(Value::Object(None));
    if property.is_null() || value.is_null() {
        return Ok(Some(Value::from(false)));
    }
    let property = property.as_string()?;
    let value = value.as_string()?;
    if !property.starts_with("com.sun.java.util.jar.pack.") {
        return Ok(Some(Value::from(false)));
    }
    state(thread.as_ref())?
        .contexts
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .entry(receiver_key(&receiver))
        .or_default()
        .options
        .insert(property, value);
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.start(Ljava/nio/ByteBuffer;J)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn start<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let offset = usize::try_from(parameters.pop_long()?)?;
    let buffer = parameters.pop()?;
    let receiver = parameters.pop().unwrap_or(Value::Object(None));
    let mut packed = if buffer.is_null() {
        input_stream_bytes(&thread, &receiver).await?
    } else {
        byte_buffer_bytes(&thread, &buffer).await?
    };
    if offset > packed.len() {
        return Err(JavaError::IllegalArgumentException(format!(
            "Pack200 offset {offset} exceeds input length {}",
            packed.len()
        ))
        .into());
    }
    if offset != 0 {
        packed.drain(..offset);
    }
    if packed.is_empty() {
        return Err(JavaError::IoException("Empty Pack200 input".to_string()).into());
    }

    let java_home = thread.vm()?.java_home().clone();
    let packed_length = packed.len();
    #[cfg(not(target_family = "wasm"))]
    let entries = {
        tokio::task::spawn_blocking(move || decode_pack200(&java_home, &packed))
            .await
            .map_err(|error| {
                ristretto_types::Error::InternalError(format!(
                    "Pack200 decoder task failed: {error}"
                ))
            })??
    };
    #[cfg(target_family = "wasm")]
    let entries = decode_pack200(&java_home, &packed)?;
    let file_count = u32::try_from(entries.len())?;
    let key = receiver_key(&receiver);
    let unpack_state = state(thread.as_ref())?;
    let mut contexts = unpack_state
        .contexts
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let options = contexts
        .remove(&key)
        .map_or_else(HashMap::new, |context| context.options);
    contexts.insert(
        key,
        UnpackContext {
            entries,
            options,
            consumed: i64::try_from(packed_length)?,
        },
    );
    drop(contexts);
    if !receiver.is_null() {
        receiver
            .as_object_mut()?
            .set_value("unpackerPtr", Value::Long(i64::try_from(key.max(1))?))?;
    }
    Ok(Some(Value::Long(i64::from(file_count))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn options_round_trip() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let property = "com.sun.java.util.jar.pack.verbose"
            .to_object(&thread)
            .await?;
        let value = "2".to_object(&thread).await?;
        let result = set_option(
            thread.clone(),
            Parameters::new(vec![Value::Object(None), property.clone(), value.clone()]),
        )
        .await?;
        assert_eq!(Some(Value::from(true)), result);
        let result = get_option(thread, Parameters::new(vec![Value::Object(None), property]))
            .await?
            .expect("option");
        assert_eq!("2", result.as_string()?);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn runtime_unpack200_decodes_archive() -> Result<()> {
        use std::io::Write;
        let (vm, _thread) = crate::test::java11_thread().await?;
        let java_home = vm.java_home();
        if !unpack200_executable(java_home).is_file()
            || !java_home.join("bin").join("pack200").is_file()
        {
            return Ok(());
        }
        let directory = tempfile::tempdir()?;
        let jar_path = directory.path().join("input.jar");
        let pack_path = directory.path().join("input.pack");
        {
            let file = std::fs::File::create(&jar_path)?;
            let mut writer = zip::ZipWriter::new(file);
            writer
                .start_file("hello.txt", zip::write::SimpleFileOptions::default())
                .map_err(|error| ristretto_types::Error::InternalError(error.to_string()))?;
            writer.write_all(b"hello Pack200")?;
            writer
                .finish()
                .map_err(|error| ristretto_types::Error::InternalError(error.to_string()))?;
        }
        let packer_executable = java_home.join("bin").join("pack200");
        let output = Command::new(packer_executable)
            .arg("--no-gzip")
            .arg(&pack_path)
            .arg(&jar_path)
            .output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let packed = std::fs::read(pack_path)?;
        let entries = decode_pack200(java_home, &packed)?;
        let entry = entries
            .into_iter()
            .find(|entry| entry.name == "hello.txt")
            .expect("hello entry");
        assert_eq!(b"hello Pack200", entry.data.as_slice());
        Ok(())
    }
}

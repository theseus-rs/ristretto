#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::JavaError::{AccessControlException, IllegalArgumentException};
use crate::JavaError::{FileNotFoundException, IoException};
use crate::Result;
use crate::handle::{FileModeFlags, Handle};
use crate::intrinsic_methods::java::io::filedescriptor;
use crate::intrinsic_methods::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::intrinsic_methods::java::io::fileoutputstream::file_handle_identifier;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::intrinsic_methods::java::io::fileoutputstream::raw_file_descriptor;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_17};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "wasi")]
use std::fs::OpenOptions;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::io::{ErrorKind, SeekFrom};
#[cfg(target_os = "wasi")]
use std::io::{Read, Seek, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::OpenOptions;
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[intrinsic_method("java/io/RandomAccessFile.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    filedescriptor::close_0(thread, parameters).await
}

#[intrinsic_method("java/io/RandomAccessFile.getFilePointer()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_file_pointer(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref_mut();
    let position: u64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
        position = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        position = file.stream_position()?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        position = file.stream_position().await?;
    }

    let position = i64::try_from(position)?;
    Ok(Some(Value::Long(position)))
}

#[intrinsic_method("java/io/RandomAccessFile.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/RandomAccessFile.length()J", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    length_0(thread, parameters).await
}

#[intrinsic_method("java/io/RandomAccessFile.length0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let handle_guard = handles
        .get(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref();
    let length: u64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
        length = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        let metadata = file.metadata()?;
        length = metadata.len();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let metadata = file.metadata().await?;
        length = metadata.len();
    }

    let length = i64::try_from(length)?;
    Ok(Some(Value::Long(length)))
}

#[intrinsic_method("java/io/RandomAccessFile.open0(Ljava/lang/String;I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = u16::try_from(parameters.pop_int()?)?;
    let mode = FileModeFlags::from_bits_truncate(mode);
    let path: String = parameters.pop()?.try_into()?;
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;

    if path.is_empty() {
        return Err(FileNotFoundException("File path is empty".to_string()).into());
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = thread;
        let _ = mode;
        let _ = file_descriptor;
        return Err(crate::java_error::JavaError::RuntimeException(
            "java.io.RandomAccessFile.open0(Ljava/lang/String;I)V is not supported on WebAssembly"
                .to_string(),
        )
        .into());
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let file_result;

        #[cfg(target_os = "wasi")]
        {
            file_result = match mode {
                FileModeFlags::READ_ONLY => OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&path),
                _ => OpenOptions::new()
                    .create(true)
                    .truncate(false)
                    .read(true)
                    .write(true)
                    .open(&path),
            };
        }

        #[cfg(not(target_family = "wasm"))]
        {
            file_result = match mode {
                FileModeFlags::READ_ONLY => {
                    OpenOptions::new()
                        .read(true)
                        .write(false)
                        .create(false)
                        .open(&path)
                        .await
                }
                _ => {
                    OpenOptions::new()
                        .create(true)
                        .truncate(false)
                        .read(true)
                        .write(true)
                        .open(&path)
                        .await
                }
            };
        }

        match file_result {
            Ok(file) => {
                let fd = raw_file_descriptor(&file)?;
                let vm = thread.vm()?;
                let handles = vm.handles();
                let handle: Handle = (file, false).into();
                let handle_identifier = file_handle_identifier(fd);
                handles.insert(handle_identifier, handle).await?;

                file_descriptor.set_value("fd", Value::Int(i32::try_from(fd)?))?;
                if vm.java_class_file_version() >= &JAVA_11 {
                    file_descriptor.set_value("handle", Value::Long(fd))?;
                }
                let read_and_write = mode != FileModeFlags::READ_ONLY;
                random_access_file.set_value("rw", Value::from(read_and_write))?;
                Ok(None)
            }
            Err(error) => {
                let error = match error.kind() {
                    ErrorKind::NotFound => FileNotFoundException(format!("File not found: {path}")),
                    ErrorKind::PermissionDenied => {
                        AccessControlException(format!("Access denied: {path}"))
                    }
                    ErrorKind::AlreadyExists => {
                        IoException(format!("File exists and cannot be overwritten: {path}"))
                    }
                    ErrorKind::InvalidInput => {
                        IllegalArgumentException(format!("Invalid file path: {path}"))
                    }
                    ErrorKind::InvalidFilename => IoException(format!("Invalid filename: {path}")),
                    _ => IoException(format!("IO error opening file '{path}': {error}")),
                };
                Err(error.into())
            }
        }
    }
}

#[intrinsic_method("java/io/RandomAccessFile.read0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let random_access_fie = parameters.pop()?;
    let bytes = Value::from(vec![0i8; 1]);
    let mut parameters = Parameters::default();
    parameters.push(random_access_fie);
    parameters.push(bytes.clone());
    parameters.push_int(0); // offset
    parameters.push_int(1); // length

    let mut result = -1; // Default to -1 if no byte is read
    if let Some(Value::Int(read_result)) = read_bytes_0(thread, parameters).await? {
        result = read_result;
    }
    if result > 0 {
        let bytes: Vec<i8> = bytes.try_into()?;
        let byte = bytes.first().cloned().unwrap_or_default();
        let byte = byte as u8;
        result = i32::from(byte);
    }

    Ok(Some(Value::Int(result)))
}

#[intrinsic_method("java/io/RandomAccessFile.readBytes([BII)I", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    read_bytes_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/RandomAccessFile.readBytes0([BII)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes = parameters.pop_reference()?;
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;

    let handles = vm.handles();
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref_mut();

    let capacity = length.saturating_sub(offset);
    let mut buffer = vec![0u8; capacity];
    let bytes_read: usize;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
        bytes_read = 0;
        buffer.resize(bytes_read, 0);
    }

    #[cfg(target_os = "wasi")]
    {
        bytes_read = file
            .read(&mut buffer[0..length])
            .map_err(|error| crate::java_error::JavaError::IoException(error.to_string()))?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        bytes_read = file
            .read(&mut buffer[0..length])
            .await
            .map_err(|error| IoException(error.to_string()))?;
    }

    let Some(Reference::ByteArray(bytes)) = bytes else {
        return Err(IoException("Cannot read bytes from reference".to_string()).into());
    };
    let mut bytes = bytes.as_mut()?;
    if bytes_read > 0 {
        for i in 0..bytes_read {
            bytes[offset + i] = buffer[i] as i8;
        }
    }

    let bytes_read = if bytes_read == 0 && length > 0 {
        -1
    } else {
        i32::try_from(bytes_read)?
    };
    Ok(Some(Value::Int(bytes_read)))
}

#[intrinsic_method("java/io/RandomAccessFile.seek0(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn seek_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let position = u64::try_from(parameters.pop_long()?)?;
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref_mut();

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = position;
        let _ = file;
    }

    #[cfg(target_os = "wasi")]
    let _ = file.seek(SeekFrom::Start(position))?;

    #[cfg(not(target_family = "wasm"))]
    let _ = file.seek(SeekFrom::Start(position)).await?;

    Ok(None)
}

#[intrinsic_method("java/io/RandomAccessFile.setLength(J)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn set_length(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_length_0(thread, parameters).await
}

#[intrinsic_method("java/io/RandomAccessFile.setLength0(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn set_length_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = u64::try_from(parameters.pop_int()?)?;
    let random_access_file = parameters.pop_object()?;
    let file_descriptor: Object = random_access_file.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref_mut();

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = length;
        let _ = file;
    }

    #[cfg(target_os = "wasi")]
    file.set_len(length)
        .map_err(|error| IoException(error.to_string()))?;

    #[cfg(not(target_family = "wasm"))]
    file.set_len(length)
        .await
        .map_err(|error| IoException(error.to_string()))?;

    Ok(None)
}

#[intrinsic_method("java/io/RandomAccessFile.write0(I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let byte = i8::try_from(parameters.pop_int()?)?;
    let random_access_file = parameters.pop()?;
    let bytes = Value::from(vec![byte]);
    let mut parameters = Parameters::default();
    parameters.push(random_access_file);
    parameters.push(bytes);
    parameters.push_int(0); // offset
    parameters.push_int(1); // length
    parameters.push_bool(append);
    write_bytes(thread, parameters).await
}

#[intrinsic_method("java/io/RandomAccessFile.writeBytes([BII)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    write_bytes_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/RandomAccessFile.writeBytes0([BII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes: Vec<u8> = parameters.pop()?.try_into()?;
    let file_output_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_output_stream.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;

    let handles = vm.handles();
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, mode, .. } = handle_guard.deref_mut();

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = length;
        let _ = offset;
        let _ = bytes;
        let _ = file;
    }

    #[cfg(target_os = "wasi")]
    file.write_all(&bytes[offset..offset + length])
        .map_err(|error| IoException(error.to_string()))?;

    #[cfg(not(target_family = "wasm"))]
    file.write_all(&bytes[offset..offset + length])
        .await
        .map_err(|error| IoException(error.to_string()))?;

    match *mode {
        FileModeFlags::READ_ONLY => {
            return Err(IoException(format!(
                "Cannot read from file opened in read-only mode: {fd}"
            ))
            .into());
        }
        FileModeFlags::SYNCHRONIZE_ALL => {
            #[cfg(target_os = "wasi")]
            file.sync_all()
                .map_err(|error| IoException(error.to_string()))?;

            #[cfg(not(target_family = "wasm"))]
            file.sync_all()
                .await
                .map_err(|error| IoException(error.to_string()))?;
        }
        FileModeFlags::SYNCHRONIZE_DATA => {
            #[cfg(target_os = "wasi")]
            file.sync_data()
                .map_err(|error| IoException(error.to_string()))?;

            #[cfg(not(target_family = "wasm"))]
            file.sync_data()
                .await
                .map_err(|error| IoException(error.to_string()))?;
        }
        _ => {
            // Nothing required for other modes
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

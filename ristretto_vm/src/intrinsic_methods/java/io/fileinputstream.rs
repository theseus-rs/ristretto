#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::JavaError::{AccessControlException, IllegalArgumentException};
use crate::JavaError::{FileNotFoundException, IoException};
use crate::Result;
use crate::handle::Handle;
use crate::intrinsic_methods::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::intrinsic_methods::java::io::fileoutputstream::file_handle_identifier;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::intrinsic_methods::java::io::fileoutputstream::raw_file_descriptor;
use crate::intrinsic_methods::java::io::{filedescriptor, randomaccessfile};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_24};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "wasi")]
use std::fs::OpenOptions;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::io::{ErrorKind, SeekFrom};
#[cfg(target_os = "wasi")]
use std::io::{Read, Seek};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::OpenOptions;
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[intrinsic_method("java/io/FileInputStream.available0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn available_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_input_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_input_stream.value("fd")?.try_into()?;
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
    let current_position: u64;
    let available_bytes: u64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
        position = 0;
        current_position = 0;
        available_bytes = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        current_position = file.stream_position()?;
        let end_position = file.seek(SeekFrom::End(0))?;
        available_bytes = end_position.saturating_sub(current_position);
        position = file.seek(SeekFrom::Start(current_position))?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        current_position = file.stream_position().await?;
        let end_position = file.seek(SeekFrom::End(0)).await?;
        available_bytes = end_position.saturating_sub(current_position);
        position = file.seek(SeekFrom::Start(current_position)).await?;
    }

    if position != current_position {
        return Err(IoException(format!(
            "Failed to seek back to original position: {current_position} != {position}"
        ))
        .into());
    }
    let available_bytes = i32::try_from(available_bytes)?;
    Ok(Some(Value::Int(available_bytes)))
}

#[intrinsic_method("java/io/FileInputStream.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    filedescriptor::close_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileInputStream.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/io/FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_24)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_regular_file_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_input_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_input_stream.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let handle_guard = handles
        .get(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref();

    let is_regular_file: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
        is_regular_file = false;
    }

    #[cfg(target_os = "wasi")]
    {
        let metadata = file.metadata()?;
        is_regular_file = metadata.is_file();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let metadata = file.metadata().await?;
        is_regular_file = metadata.is_file();
    }

    Ok(Some(Value::from(is_regular_file)))
}

#[intrinsic_method("java/io/FileInputStream.length0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    randomaccessfile::length_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileInputStream.open0(Ljava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path: String = parameters.pop()?.try_into()?;
    let file_input_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_input_stream.value("fd")?.try_into()?;

    if path.is_empty() {
        return Err(FileNotFoundException("File path is empty".to_string()).into());
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = thread;
        let _ = file_descriptor;
        Err(crate::java_error::JavaError::RuntimeException(
            "java.io.FileInputStream.open0(Ljava/lang/String;)V is not supported on WebAssembly"
                .to_string(),
        )
        .into())
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let file_result;

        #[cfg(target_os = "wasi")]
        {
            file_result = OpenOptions::new()
                .create(false)
                .read(true)
                .write(false)
                .open(&path);
        }

        #[cfg(not(target_family = "wasm"))]
        {
            file_result = OpenOptions::new()
                .create(false)
                .read(true)
                .write(false)
                .open(&path)
                .await;
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
                Ok(None)
            }
            Err(error) => {
                let error = match error.kind() {
                    ErrorKind::NotFound => FileNotFoundException(format!("File not found: {path}")),
                    ErrorKind::PermissionDenied => {
                        AccessControlException(format!("Access denied: {path}"))
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

#[intrinsic_method("java/io/FileInputStream.position0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn position_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    randomaccessfile::get_file_pointer(thread, parameters).await
}

#[intrinsic_method("java/io/FileInputStream.read0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_input_stream = parameters.pop()?;
    let bytes = Value::from(vec![0i8; 1]);
    let mut parameters = Parameters::default();
    parameters.push(file_input_stream);
    parameters.push(bytes.clone());
    parameters.push_int(0); // offset
    parameters.push_int(1); // length

    let mut result = -1; // Default to -1 if no byte is read
    if let Some(Value::Int(read_result)) = read_bytes(thread, parameters).await? {
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

#[intrinsic_method("java/io/FileInputStream.readBytes([BII)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes = parameters.pop_reference()?;
    let file_input_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_input_stream.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let capacity = length.saturating_sub(offset);
    let mut buffer = vec![0u8; capacity];

    let bytes_read = match fd {
        0 => {
            let configuration = vm.configuration();
            let stdin_lock = configuration.stdin();
            let mut stdin = stdin_lock.lock().await;
            stdin
                .read(&mut buffer[0..length])
                .map_err(|error| IoException(error.to_string()))?
        }
        _ => {
            let handles = vm.handles();
            let handle_identifier = file_handle_identifier(fd);
            let mut handle_guard = handles.get_mut(&handle_identifier).await.ok_or_else(|| {
                IoException(format!("File handle not found: {handle_identifier}"))
            })?;
            let Handle::File { file, .. } = handle_guard.deref_mut();

            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            {
                let _ = file;
                0
            }

            #[cfg(target_os = "wasi")]
            {
                file.read(&mut buffer[0..length])
                    .map_err(|error| IoException(error.to_string()))?
            }

            #[cfg(not(target_family = "wasm"))]
            {
                file.read(&mut buffer[0..length])
                    .await
                    .map_err(|error| IoException(error.to_string()))?
            }
        }
    };

    let Some(Reference::ByteArray(bytes)) = bytes else {
        return Err(IoException("Cannot read bytes from reference".to_string()).into());
    };

    let bytes_read = if bytes_read == 0 && length > 0 {
        -1
    } else {
        let mut bytes = bytes.as_mut()?;
        if bytes_read > 0 {
            for i in 0..bytes_read {
                bytes[offset + i] = buffer[i] as i8;
            }
        }
        i32::try_from(bytes_read)?
    };
    Ok(Some(Value::Int(bytes_read)))
}

#[intrinsic_method("java/io/FileInputStream.skip0(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn skip_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let skip_bytes = parameters.pop_long()?;
    let file_input_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_input_stream.value("fd")?.try_into()?;
    let vm = thread.vm()?;
    let handles = vm.handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let mut handle_guard = handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let Handle::File { file, .. } = handle_guard.deref_mut();
    let original_position: u64;
    let current_position: u64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = skip_bytes;
        let _ = file;
        original_position = 0;
        current_position = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        original_position = file.stream_position()?;
        current_position = file.seek(SeekFrom::Current(skip_bytes))?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        original_position = file.stream_position().await?;
        current_position = file.seek(SeekFrom::Current(skip_bytes)).await?;
    }

    let skipped_bytes = current_position.saturating_sub(original_position);
    let skipped_bytes = i64::try_from(skipped_bytes)?;
    Ok(Some(Value::Long(skipped_bytes)))
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

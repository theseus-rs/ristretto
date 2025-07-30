#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::JavaError::{AccessControlException, IllegalArgumentException};
use crate::JavaError::{FileNotFoundException, IoException};
use crate::Result;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::handles::FileHandle;
use crate::intrinsic_methods::java::io::filedescriptor;
use crate::intrinsic_methods::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "wasi")]
use std::fs::{File, OpenOptions};
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::io::{ErrorKind, SeekFrom};
#[cfg(target_os = "wasi")]
use std::io::{Seek, Write};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::{File, OpenOptions};
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use zerocopy::transmute_ref;

/// Generates a unique identifier for a file handle based on its integer value.  This is used to
/// track file handles in the VM's handles table.
pub(crate) fn file_handle_identifier(handle: i64) -> String {
    format!("file:{handle}")
}

/// Returns a raw file descriptor for the current platform. On WebAssembly, it returns -1 as
/// WebAssembly does not support file descriptors in the same way as traditional operating systems.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
pub(crate) fn raw_file_descriptor() -> i64 {
    -1
}

/// Converts a `File` into its corresponding file descriptor, which is an integer value that
/// represents the file handle in the operating system. This is used to interact with the file
/// system at a lower level.
#[cfg(any(not(target_family = "wasm"), target_os = "wasi"))]
pub(crate) fn raw_file_descriptor(file: &File) -> Result<i64> {
    #[cfg(target_os = "wasi")]
    let file_descriptor = {
        use std::os::wasi::io::AsRawFd;
        let fd = file.as_raw_fd();
        i64::from(fd)
    };

    #[cfg(target_os = "windows")]
    let file_descriptor = {
        use std::os::windows::io::AsRawHandle;
        let fd = file.as_raw_handle() as usize;
        i64::try_from(fd)?
    };

    #[cfg(not(any(target_family = "wasm", target_os = "windows")))]
    let file_descriptor = {
        use std::os::unix::io::AsRawFd;
        let fd = file.as_raw_fd();
        i64::from(fd)
    };

    Ok(file_descriptor)
}

#[intrinsic_method("java/io/FileOutputStream.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    filedescriptor::close_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileOutputStream.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileOutputStream.open0(Ljava/lang/String;Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let path = parameters.pop()?;
    let path = path.as_string()?;
    let file_output_stream = parameters.pop_object()?;
    let file_descriptor = file_output_stream.value("fd")?;
    let file_descriptor = file_descriptor.as_object_ref()?;

    if path.is_empty() {
        return Err(FileNotFoundException("File path is empty".to_string()).into());
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = thread;
        let _ = append;
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
            file_result = if append {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .append(true)
                    .open(&path)
            } else {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .truncate(true)
                    .open(&path)
            };
        }

        #[cfg(not(target_family = "wasm"))]
        {
            file_result = if append {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .append(true)
                    .open(&path)
                    .await
            } else {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .truncate(true)
                    .open(&path)
                    .await
            };
        }

        match file_result {
            Ok(file) => {
                let fd = raw_file_descriptor(&file)?;
                let vm = thread.vm()?;
                let file_handles = vm.file_handles();
                let file_handle: FileHandle = (file, false).into();
                let handle_identifier = file_handle_identifier(fd);
                file_handles.insert(handle_identifier, file_handle).await?;

                file_descriptor.set_value("fd", Value::Int(i32::try_from(fd)?))?;
                if vm.java_class_file_version() >= &JAVA_11 {
                    file_descriptor.set_value("handle", Value::Long(fd))?;
                }
                file_descriptor.set_value("append", Value::from(append))?;
                Ok(None)
            }
            Err(error) => {
                let error = match error.kind() {
                    ErrorKind::NotFound => FileNotFoundException(format!("File not found: {path}")),
                    ErrorKind::PermissionDenied => {
                        AccessControlException(format!("Access denied: {path}"))
                    }
                    ErrorKind::AlreadyExists if !append => {
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

#[intrinsic_method("java/io/FileOutputStream.write(IZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let byte = i8::try_from(parameters.pop_int()?)?;
    let file_output_stream = parameters.pop()?;
    let bytes = Value::from(vec![byte]);
    let mut parameters = Parameters::default();
    parameters.push(file_output_stream);
    parameters.push(bytes);
    parameters.push_int(0); // offset
    parameters.push_int(1); // length
    parameters.push_bool(append);
    write_bytes(thread, parameters).await
}

#[intrinsic_method("java/io/FileOutputStream.writeBytes([BIIZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(bytes.as_slice());
        bytes.to_vec()
    };
    let file_output_stream = parameters.pop_object()?;
    let file_descriptor = file_output_stream.value("fd")?;
    let file_descriptor = file_descriptor.as_object_ref()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, file_descriptor)?;

    match fd {
        1 => {
            let configuration = vm.configuration();
            let stdout_lock = configuration.stdout();
            let mut stdout = stdout_lock.lock().await;
            stdout
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| IoException(error.to_string()))?;
        }
        2 => {
            let configuration = vm.configuration();
            let stderr_lock = configuration.stderr();
            let mut stderr = stderr_lock.lock().await;
            stderr
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| IoException(error.to_string()))?;
        }
        _ => {
            let file_handles = vm.file_handles();
            let handle_identifier = file_handle_identifier(fd);
            let mut file_handle =
                file_handles
                    .get_mut(&handle_identifier)
                    .await
                    .ok_or_else(|| {
                        IoException(format!("File handle not found: {handle_identifier}"))
                    })?;
            let file = &mut file_handle.file;

            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            {
                let _ = append;
                let _ = file;
            }

            #[cfg(target_os = "wasi")]
            {
                if append {
                    file.seek(SeekFrom::End(0))?;
                }
                file.write_all(&bytes[offset..offset + length])
                    .map_err(|error| IoException(error.to_string()))?;
            }

            #[cfg(not(target_family = "wasm"))]
            {
                if append {
                    file.seek(SeekFrom::End(0)).await?;
                }
                file.write_all(&bytes[offset..offset + length])
                    .await
                    .map_err(|error| IoException(error.to_string()))?;
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[test]
    fn test_file_handle_identifier() {
        let handle = 12345;
        let identifier = file_handle_identifier(handle);
        assert_eq!(identifier, "file:12345");
    }

    #[tokio::test]
    async fn test_raw_file_descriptor() -> Result<()> {
        let file_name = "test_raw_file_descriptor.txt";
        let file = File::create(file_name).await?;
        let descriptor = raw_file_descriptor(&file)?;
        assert!(descriptor > 0);
        remove_file(file_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

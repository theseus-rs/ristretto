use crate::java::io::filedescriptor;
use crate::java::io::filedescriptor::file_descriptor_from_java_object;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::java::io::filedescriptor::raw_file_descriptor;
use ristretto_classfile::JAVA_8;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
use ristretto_types::JavaError::RuntimeException;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_types::JavaError::{AccessControlException, IllegalArgumentException};
use ristretto_types::JavaError::{FileNotFoundException, IoException};
use ristretto_types::Thread;
use ristretto_types::VM;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_types::handles::FileHandle;
use ristretto_types::{Parameters, Result};
#[cfg(target_os = "wasi")]
use std::fs::OpenOptions;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::io::ErrorKind;
#[cfg(target_os = "wasi")]
use std::io::Write;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::path::PathBuf;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::OpenOptions;
#[cfg(not(target_family = "wasm"))]
use tokio::io::AsyncWriteExt;
use zerocopy::transmute_ref;

#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
fn resolve_path<T: Thread + 'static>(thread: &Arc<T>, path: &str) -> Result<PathBuf> {
    let path_buf = PathBuf::from(path);
    if path_buf.is_absolute() {
        return Ok(path_buf);
    }
    let vm = thread.vm()?;
    if let Some(user_dir) = vm.system_properties().get("user.dir") {
        Ok(PathBuf::from(user_dir).join(path_buf))
    } else {
        Ok(path_buf)
    }
}

#[intrinsic_method("java/io/FileOutputStream.close0()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filedescriptor::close_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileOutputStream.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileOutputStream.open0(Ljava/lang/String;Z)V", Any)]
#[expect(clippy::too_many_lines)]
#[async_method]
pub async fn open_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let path = parameters.pop()?;
    let path = path.as_string()?;
    let file_output_stream = parameters.pop()?;
    let file_descriptor = {
        let file_output_stream = file_output_stream.as_object_ref()?;
        file_output_stream.value("fd")?
    };

    if path.is_empty() {
        return Err(FileNotFoundException("File path is empty".to_string()).into());
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = thread;
        let _ = append;
        let _ = file_descriptor;
        Err(RuntimeException(
            "java.io.FileInputStream.open0(Ljava/lang/String;)V is not supported on WebAssembly"
                .to_string(),
        )
        .into())
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let file_result;
        let resolved_path = resolve_path(&thread, &path)?;

        #[cfg(target_os = "wasi")]
        {
            file_result = if append {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .append(true)
                    .open(&resolved_path)
            } else {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .truncate(true)
                    .open(&resolved_path)
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
                    .open(&resolved_path)
                    .await
            } else {
                OpenOptions::new()
                    .create(true)
                    .read(false)
                    .write(true)
                    .truncate(true)
                    .open(&resolved_path)
                    .await
            };
        }

        match file_result {
            Ok(file) => {
                let fd = raw_file_descriptor(&file)?;
                let vm = thread.vm()?;
                let file_handles = vm.file_handles();
                let file_handle: FileHandle = (file, append).into();
                file_handles.insert(fd, file_handle).await?;

                {
                    let mut file_descriptor = file_descriptor.as_object_mut()?;
                    file_descriptor.set_value("fd", Value::Int(i32::try_from(fd)?))?;
                    if vm.java_class_file_version() >= &JAVA_11 {
                        file_descriptor.set_value("handle", Value::Long(fd))?;
                    }
                    file_descriptor.set_value("append", Value::from(append))?;
                }
                Ok(None)
            }
            Err(error) => {
                let error = match error.kind() {
                    ErrorKind::NotFound => {
                        FileNotFoundException(format!("{path} (No such file or directory)"))
                    }
                    ErrorKind::PermissionDenied => {
                        AccessControlException(format!("Access denied: {path}"))
                    }
                    ErrorKind::IsADirectory => {
                        FileNotFoundException(format!("{path} (Is a directory)"))
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
#[async_method]
pub async fn write<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let append = parameters.pop_bool()?;
    let byte = i8::from_ne_bytes([parameters.pop_int()?.to_le_bytes()[0]]);
    let file_output_stream = parameters.pop()?;
    let bytes = Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::from(vec![byte]),
    );
    let mut parameters = Parameters::default();
    parameters.push(file_output_stream);
    parameters.push(bytes);
    parameters.push_int(0); // offset
    parameters.push_int(1); // length
    parameters.push_bool(append);
    write_bytes(thread, parameters).await
}

#[intrinsic_method("java/io/FileOutputStream.writeBytes([BIIZ)V", Any)]
#[async_method]
pub async fn write_bytes<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _append = parameters.pop_bool()?;
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let file_output_stream = parameters.pop()?;
    let file_descriptor = {
        let file_output_stream = file_output_stream.as_object_ref()?;
        file_output_stream.value("fd")?
    };
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;

    match fd {
        1 => {
            let stdout_lock = vm.stdout();
            let data = bytes[offset..offset + length].to_vec();
            let mut stdout = stdout_lock.lock().await;
            stdout
                .write_all(&data)
                .map_err(|error| IoException(error.to_string()))?;
        }
        2 => {
            let stderr_lock = vm.stderr();
            let data = bytes[offset..offset + length].to_vec();
            let mut stderr = stderr_lock.lock().await;
            stderr
                .write_all(&data)
                .map_err(|error| IoException(error.to_string()))?;
        }
        _ => {
            let file_handles = vm.file_handles();
            let mut file_handle = file_handles
                .get_mut(&fd)
                .await
                .ok_or_else(|| IoException(format!("File handle not found: {fd}")))?;
            let file = &mut file_handle.file;

            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            {
                let _ = file;
            }

            #[cfg(target_os = "wasi")]
            {
                file.write_all(&bytes[offset..offset + length])
                    .map_err(|error| IoException(error.to_string()))?;
            }

            #[cfg(not(target_family = "wasm"))]
            {
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

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

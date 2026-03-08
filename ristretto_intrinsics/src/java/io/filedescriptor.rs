use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError::IoException;
use ristretto_types::Parameters;
use ristretto_types::{Result, VM};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::File;
#[cfg(not(target_family = "wasm"))]
use tokio::io::AsyncWriteExt;

/// Returns the file descriptor for a given Java `java.io.FileDescriptor` object taking into account
/// the Java version. For Java 11 and later, it uses the `handle` field for windows, while for
/// earlier versions, or on non-windows platforms, it uses the `fd` field.
pub(crate) fn file_descriptor_from_java_object<V: VM>(
    #[cfg_attr(not(target_os = "windows"), expect(unused_variables))] vm: &Arc<V>,
    file_descriptor: &Value,
) -> Result<i64> {
    let file_descriptor = file_descriptor.as_object_ref()?;

    #[cfg(not(target_os = "windows"))]
    let fd = {
        let fd = file_descriptor.value("fd")?.as_i32()?;
        i64::from(fd)
    };

    #[cfg(target_os = "windows")]
    let fd = if vm.java_class_file_version() >= &JAVA_11 {
        file_descriptor.value("handle")?.as_i64()?
    } else {
        let fd = file_descriptor.value("fd")?.as_i32()?;
        i64::from(fd)
    };

    Ok(fd)
}

/// Per-VM counter for generating synthetic file descriptors on WebAssembly.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
struct WasmFdCounter(std::sync::atomic::AtomicI64);

/// Returns a raw file descriptor for the current platform. On WebAssembly, it returns a negative
/// counter as WebAssembly does not support file descriptors in the same way as traditional
/// operating systems.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
pub(crate) fn raw_file_descriptor(
    _file: &str,
    resource_manager: &ristretto_types::ResourceManager,
) -> Result<i64> {
    use std::sync::atomic::Ordering;
    let counter =
        resource_manager.get_or_init(|| WasmFdCounter(std::sync::atomic::AtomicI64::new(-1000)))?;
    Ok(counter.0.fetch_sub(1, Ordering::Relaxed))
}

/// Converts a `File` into its corresponding file descriptor, which is an integer value that
/// represents the file handle in the operating system.
#[cfg(target_os = "wasi")]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn raw_file_descriptor(file: &std::fs::File) -> Result<i64> {
    use std::os::wasi::io::AsRawFd;
    let fd = file.as_raw_fd();
    Ok(i64::from(fd))
}

/// Converts a `File` into its corresponding file descriptor, which is an integer value that
/// represents the file handle in the operating system.
#[cfg(target_os = "windows")]
pub(crate) fn raw_file_descriptor(file: &tokio::fs::File) -> Result<i64> {
    use std::os::windows::io::AsRawHandle;
    let fd = file.as_raw_handle() as usize;
    let file_descriptor = i64::try_from(fd)?;
    Ok(file_descriptor)
}

/// Converts a `File` into its corresponding file descriptor, which is an integer value that
/// represents the file handle in the operating system.
#[cfg(not(any(target_family = "wasm", target_os = "windows", target_os = "wasi")))]
#[expect(clippy::unnecessary_wraps)]
pub(crate) fn raw_file_descriptor(file: &tokio::fs::File) -> Result<i64> {
    use std::os::unix::io::AsRawFd;
    let fd = file.as_raw_fd();
    let file_descriptor = i64::from(fd);
    Ok(file_descriptor)
}

#[intrinsic_method("java/io/FileDescriptor.close0()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_descriptor = parameters.pop()?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;

    {
        let mut file_descriptor = file_descriptor.as_object_mut()?;
        file_descriptor.set_value("fd", Value::Int(-1))?;
        if vm.java_class_file_version() >= &JAVA_11 {
            file_descriptor.set_value("handle", Value::Long(-1))?;
        }
    }

    let Some(handle) = file_handles.remove(&fd).await else {
        return Ok(None);
    };

    #[cfg(target_family = "wasm")]
    {
        let _ = handle;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let mut file_handle: File = handle.try_into()?;
        file_handle.shutdown().await?;
    }

    Ok(None)
}

#[intrinsic_method("java/io/FileDescriptor.getAppend(I)Z", GreaterThanOrEqual(JAVA_11))]
#[expect(clippy::match_same_arms)]
#[async_method]
pub async fn get_append<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let append = match handle {
        0 => {
            // true if stdin is in append mode
            false
        }
        1 => {
            // true if stdout is in append mode
            false
        }
        2 => {
            // true if stderr is in append mode
            false
        }
        _ => {
            let vm = thread.vm()?;
            let file_handles = vm.file_handles();
            let fd = i64::from(handle);
            let file_handle = file_handles
                .get(&fd)
                .await
                .ok_or_else(|| IoException(format!("File handle not found: {fd}")))?;
            file_handle.append
        }
    };
    Ok(Some(Value::from(append)))
}

#[intrinsic_method("java/io/FileDescriptor.getHandle(I)J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_handle<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/io/FileDescriptor.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileDescriptor.sync()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn sync<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    sync_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileDescriptor.sync0()V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn sync_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_descriptor = parameters.pop()?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let file_handle = file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {fd}")))?;
    let file = &file_handle.file;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
    }

    #[cfg(target_os = "wasi")]
    {
        file.sync_all()?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        file.sync_all().await?;
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_append() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_handles = [0, 1, 2];

        for handle in file_handles {
            let result =
                get_append(thread.clone(), Parameters::new(vec![Value::Int(handle)])).await?;
            assert_eq!(Some(Value::from(false)), result);
        }

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

use crate::sun::nio::ch::iocp::{
    CompletionTarget, begin_operation, io_error_code, is_associated, mark_closed,
    operation_is_open, post_operation,
};
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, Equal};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

const IO_UNAVAILABLE: i32 = -2;
const ERROR_HANDLE_EOF: i32 = 38;
const ERROR_OPERATION_ABORTED: i32 = 995;

fn io_exception(operation: &str, error: impl std::fmt::Display) -> ristretto_types::Error {
    JavaError::IoException(format!("{operation} failed: {error}")).into()
}

async fn validate_file_operation<V: VM + ?Sized>(
    vm: &V,
    handle: i64,
    address: Option<(i64, usize)>,
    operation: &str,
) -> Result<()> {
    if vm.file_handles().get(&handle).await.is_none() {
        return Err(io_exception(operation, "invalid file handle"));
    }
    if !is_associated(vm, handle)? {
        return Err(io_exception(
            operation,
            "file handle is not associated with an I/O completion port",
        ));
    }
    if let Some((address, length)) = address
        && length > 0
        && vm
            .native_memory()
            .read_with(address, length, |_| ())
            .is_none()
    {
        return Err(io_exception(operation, "invalid native buffer"));
    }
    Ok(())
}

fn post_if_open<V: VM + ?Sized>(
    vm: &V,
    target: CompletionTarget,
    error: i32,
    bytes: usize,
    overlapped: i64,
) {
    let error = if operation_is_open(vm, target).unwrap_or(false) {
        error
    } else {
        ERROR_OPERATION_ABORTED
    };
    let _ = post_operation(vm, target, error, bytes, overlapped);
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousFileChannelImpl.close0(J)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    if vm.file_handles().get(&handle).await.is_none() {
        return Err(io_exception("CloseHandle", "invalid file handle"));
    }
    mark_closed(vm.as_ref(), handle);
    managed_files::close(vm.file_handles(), handle).await;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.lockFile(JJJZJ)I", Any)]
#[async_method]
pub async fn lock_file<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let shared = parameters.pop_bool()?;
    let size = parameters.pop_long()?;
    let position = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    if position < 0 || size < 0 {
        return Err(io_exception("LockFile", "negative position or size"));
    }

    let vm = thread.vm()?;
    validate_file_operation(vm.as_ref(), handle, None, "LockFile").await?;
    let file = managed_files::clone_file(vm.file_handles(), handle)
        .await
        .map_err(|error| io_exception("LockFile", error))?;

    // LockFileEx completes synchronously when an uncontended lock can be acquired.
    let result = managed_files::lock_range_file(
        &file,
        position.cast_unsigned(),
        size.cast_unsigned(),
        shared,
        false,
    )
    .await
    .map_err(|error| io_exception("LockFile", error))?;
    if result == 0 {
        return Ok(Some(Value::Int(0)));
    }
    let target = begin_operation(vm.as_ref(), handle)?;

    // A contended LockFileEx on an overlapped handle reports ERROR_IO_PENDING. Retry the
    // non-blocking operation so closing the channel can cancel the pending request promptly.
    tokio::spawn(async move {
        loop {
            if !operation_is_open(vm.as_ref(), target).unwrap_or(false) {
                let _ = post_operation(vm.as_ref(), target, ERROR_OPERATION_ABORTED, 0, overlapped);
                break;
            }

            match managed_files::lock_range_file(
                &file,
                position.cast_unsigned(),
                size.cast_unsigned(),
                shared,
                false,
            )
            .await
            {
                Ok(0) => {
                    post_if_open(vm.as_ref(), target, 0, 0, overlapped);
                    break;
                }
                Ok(_) => tokio::time::sleep(std::time::Duration::from_millis(10)).await,
                Err(error) => {
                    post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
                    break;
                }
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.readFile(JJIJJ)I", Any)]
#[async_method]
pub async fn read_file<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let offset = parameters.pop_long()?;
    let length = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let length =
        usize::try_from(length).map_err(|_| io_exception("ReadFile", "negative buffer length"))?;
    let offset = u64::try_from(offset).map_err(|_| io_exception("ReadFile", "negative offset"))?;
    let vm = thread.vm()?;
    validate_file_operation(vm.as_ref(), handle, Some((address, length)), "ReadFile").await?;
    let file = managed_files::clone_file(vm.file_handles(), handle)
        .await
        .map_err(|error| io_exception("ReadFile", error))?;

    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(length)
        .map_err(|error| io_exception("ReadFile", error))?;
    buffer.resize(length, 0);
    let target = begin_operation(vm.as_ref(), handle)?;
    tokio::spawn(async move {
        match managed_files::read_at_file(file, buffer, offset).await {
            Ok((0, _)) if length > 0 => {
                post_if_open(vm.as_ref(), target, ERROR_HANDLE_EOF, 0, overlapped);
            }
            Ok((bytes_read, buffer)) => {
                if let Some(bytes) = buffer.get(..bytes_read) {
                    vm.native_memory().write_bytes(address, bytes);
                }
                post_if_open(vm.as_ref(), target, 0, bytes_read, overlapped);
            }
            Err(error) => {
                post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.writeFile(JJIJJ)I", Any)]
#[async_method]
pub async fn write_file<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let offset = parameters.pop_long()?;
    let length = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let length =
        usize::try_from(length).map_err(|_| io_exception("WriteFile", "negative buffer length"))?;
    let offset = u64::try_from(offset).map_err(|_| io_exception("WriteFile", "negative offset"))?;
    let vm = thread.vm()?;
    validate_file_operation(vm.as_ref(), handle, Some((address, length)), "WriteFile").await?;
    let file = managed_files::clone_file(vm.file_handles(), handle)
        .await
        .map_err(|error| io_exception("WriteFile", error))?;
    let data = if length == 0 {
        Vec::new()
    } else {
        vm.native_memory()
            .try_read_bytes(address, length)
            .ok_or_else(|| io_exception("WriteFile", "invalid native buffer"))?
    };
    let target = begin_operation(vm.as_ref(), handle)?;

    tokio::spawn(async move {
        match managed_files::write_at_file(file, data, offset).await {
            Ok(bytes_written) => {
                post_if_open(vm.as_ref(), target, 0, bytes_written, overlapped);
            }
            Err(error) => {
                post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid file handle")
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lock_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid file handle")
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid file handle")
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid file handle")
        );
    }
}

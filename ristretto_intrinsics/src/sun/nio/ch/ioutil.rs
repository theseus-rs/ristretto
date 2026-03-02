use crate::sun::nio::fs::managed_files;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

/// No-op: regular files are always blocking on POSIX; managed VM files don't
/// support non-blocking mode, so this is a safe no-op.
#[intrinsic_method("sun/nio/ch/IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V", Any)]
#[async_method]
pub async fn configure_blocking<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

/// Drain bytes from fd. Returns false (nothing to drain for managed files).
#[intrinsic_method("sun/nio/ch/IOUtil.drain(I)Z", Any)]
#[async_method]
pub async fn drain<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

/// Drain a single byte from fd. Returns 0 (nothing drained).
#[intrinsic_method("sun/nio/ch/IOUtil.drain1(I)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn drain_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

/// Return the maximum number of file descriptors.
#[intrinsic_method("sun/nio/ch/IOUtil.fdLimit()I", Any)]
#[async_method]
pub async fn fd_limit<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1024)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.fdVal(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn fd_val<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = {
        let guard = fd_value.as_reference()?;
        let Reference::Object(object) = &*guard else {
            return Err(InternalError("fdVal: not an object".to_string()));
        };
        object.value("fd")?.as_i32()?
    };
    Ok(Some(Value::Int(fd)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/IOUtil.iovMax()I", Any)]
#[async_method]
pub async fn iov_max<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(16)))
}

/// Create a pipe. Returns a long encoding two fds: `(read_fd << 32) | write_fd`.
#[intrinsic_method("sun/nio/ch/IOUtil.makePipe(Z)J", Any)]
#[async_method]
pub async fn make_pipe<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocking = parameters.pop_int()?;

    #[cfg(not(target_family = "wasm"))]
    {
        #[cfg(target_family = "unix")]
        let (read_file, write_file) = {
            let (sender, receiver) = tokio::net::unix::pipe::pipe().map_err(|e| {
                ristretto_types::Error::JavaError(ristretto_types::JavaError::IoException(
                    e.to_string(),
                ))
            })?;
            let read_fd = receiver.into_blocking_fd().map_err(|e| {
                ristretto_types::Error::JavaError(ristretto_types::JavaError::IoException(
                    e.to_string(),
                ))
            })?;
            let write_fd = sender.into_blocking_fd().map_err(|e| {
                ristretto_types::Error::JavaError(ristretto_types::JavaError::IoException(
                    e.to_string(),
                ))
            })?;
            (std::fs::File::from(read_fd), std::fs::File::from(write_fd))
        };

        #[cfg(target_family = "windows")]
        let (read_file, write_file) = {
            use std::os::windows::io::OwnedHandle;
            let (read_pipe, write_pipe) = os_pipe::pipe().map_err(|e| {
                ristretto_types::Error::JavaError(ristretto_types::JavaError::IoException(
                    e.to_string(),
                ))
            })?;
            (
                std::fs::File::from(OwnedHandle::from(read_pipe)),
                std::fs::File::from(OwnedHandle::from(write_pipe)),
            )
        };

        let read_tokio = tokio::fs::File::from_std(read_file);
        let write_tokio = tokio::fs::File::from_std(write_file);

        let vm = thread.vm()?;
        let file_handles = vm.file_handles();

        let read_handle = ristretto_types::handles::FileHandle::from((read_tokio, false));
        let write_handle = ristretto_types::handles::FileHandle::from((write_tokio, false));

        let read_fd = crate::java::io::filedescriptor::raw_file_descriptor(&read_handle.file)?;
        let write_fd = crate::java::io::filedescriptor::raw_file_descriptor(&write_handle.file)?;

        file_handles
            .insert(read_fd, read_handle)
            .await
            .map_err(|e| InternalError(e.to_string()))?;
        file_handles
            .insert(write_fd, write_handle)
            .await
            .map_err(|e| InternalError(e.to_string()))?;

        let result = (read_fd << 32) | (write_fd & 0xFFFF_FFFF);
        Ok(Some(Value::Long(result)))
    }

    #[cfg(target_family = "wasm")]
    {
        let _ = thread;
        Err(ristretto_types::Error::JavaError(
            ristretto_types::JavaError::IoException("Pipes not supported on WASM".to_string()),
        ))
    }
}

/// Fill the provided byte array with random bytes.
#[intrinsic_method("sun/nio/ch/IOUtil.randomBytes([B)Z", Any)]
#[async_method]
pub async fn random_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let array_value = parameters.pop()?;
    let mut guard = array_value.as_reference_mut()?;
    let Reference::ByteArray(bytes) = &mut *guard else {
        return Err(InternalError("randomBytes: not a byte array".to_string()));
    };

    let mut buf = vec![0u8; bytes.len()];
    getrandom::fill(&mut buf)
        .map_err(|e| InternalError(format!("randomBytes: failed to generate random bytes: {e}")))?;

    for (i, &b) in buf.iter().enumerate() {
        #[expect(clippy::cast_possible_wrap)]
        {
            bytes[i] = b as i8;
        }
    }

    Ok(Some(Value::Int(1)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn setfd_val<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_int = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let mut guard = fd_value.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError("setfdVal: not an object".to_string()));
    };
    object.set_value("fd", Value::Int(fd_int))?;
    Ok(None)
}

/// Write a single byte to a file descriptor.
#[intrinsic_method("sun/nio/ch/IOUtil.write1(IB)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn write_1<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let byte_val = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let data = [u8::try_from(byte_val & 0xFF).unwrap_or(0)];
    let n = managed_files::write(file_handles, i64::from(fd), &data)
        .await
        .map_err(|e| {
            ristretto_types::Error::JavaError(ristretto_types::JavaError::IoException(
                e.to_string(),
            ))
        })?;
    Ok(Some(Value::Int(i32::try_from(n).unwrap_or(i32::MAX))))
}

#[intrinsic_method("sun/nio/ch/IOUtil.writevMax()J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn writev_max<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Some(Value::Long(i64::MAX)))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Some(Value::Long(i64::from(i32::MAX))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::consts::OS;

    #[tokio::test]
    async fn test_configure_blocking() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = configure_blocking(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_drain() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = drain(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fd_limit() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = fd_limit(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1024)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fd_val(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_iov_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = iov_max(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(16)));
        Ok(())
    }

    #[tokio::test]
    async fn test_make_pipe() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Int(1)]);
        let result = make_pipe(thread, parameters).await?;
        let value = result.expect("expected a long value");
        let Value::Long(packed) = value else {
            panic!("Expected long value, got {value:?}");
        };
        // Extract read_fd and write_fd from packed long
        let read_fd = packed >> 32;
        let write_fd = packed & 0xFFFF_FFFF;
        // Both fds should be valid (non-negative on Unix)
        assert!(read_fd >= 0, "read_fd should be non-negative");
        assert!(write_fd >= 0, "write_fd should be non-negative");
        assert_ne!(read_fd, write_fd, "read and write fds should differ");
        Ok(())
    }

    #[tokio::test]
    async fn test_random_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = random_bytes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_setfd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setfd_val(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_drain_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = drain_1(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_write_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_writev_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writev_max(thread, Parameters::default()).await?;
        let expected = match OS {
            "windows" => i64::MAX,
            _ => i64::from(i32::MAX),
        };
        assert_eq!(result, Some(Value::Long(expected)));
        Ok(())
    }
}

use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

/// No-op: regular files are always blocking on POSIX; managed VM files don't
/// support non-blocking mode, so this is a safe no-op.
#[intrinsic_method("sun/nio/ch/IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V", Any)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let blocking = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = {
        let guard = fd_value.as_reference()?;
        let Reference::Object(object) = &*guard else {
            return Err(InternalError(
                "configureBlocking: not an object".to_string(),
            ));
        };
        object.value("fd")?.as_i32()?
    };
    let vm = thread.vm()?;
    // For tokio types, non-blocking is always set; track mode but don't error
    if let Some(guard) = vm.socket_handles().get(&fd).await {
        if let Some(socket) = guard.socket_type.as_raw() {
            socket
                .set_nonblocking(!blocking)
                .map_err(|e| InternalError(e.to_string()))?;
        }
        // For TcpStream/TcpListener/UdpSocket, tokio manages blocking mode
        drop(guard);
        if let Some(mut guard) = vm.socket_handles().get_mut(&fd).await {
            guard.non_blocking = !blocking;
        }
    }
    Ok(None)
}

/// Drain bytes from fd. Returns false (nothing to drain for managed files).
#[intrinsic_method("sun/nio/ch/IOUtil.drain(I)Z", Any)]
#[async_method]
pub async fn drain<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;

    let Some(guard) = vm.socket_handles().get(&fd).await else {
        return Ok(Some(Value::from(false)));
    };

    if let Some(stream) = guard.socket_type.as_tcp_stream() {
        // Drain from TcpStream
        let mut buf = [0u8; 128];
        let mut any = false;
        loop {
            match stream.try_read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(_) => any = true,
            }
        }
        return Ok(Some(Value::from(any)));
    }

    let Some(socket) = guard.socket_type.as_raw() else {
        return Ok(Some(Value::from(false)));
    };
    let cloned = socket
        .try_clone()
        .map_err(|e| InternalError(format!("drain: clone: {e}")))?;
    drop(guard);

    let drained = tokio::task::spawn_blocking(move || {
        let mut buf = [0u8; 128];
        let mut any = false;
        loop {
            match std::io::Read::read(&mut &cloned, &mut buf) {
                Ok(0) | Err(_) => break,
                Ok(_) => any = true,
            }
        }
        any
    })
    .await
    .map_err(|e| InternalError(format!("drain: spawn: {e}")))?;

    Ok(Some(Value::from(drained)))
}

/// Drain a single byte from fd. Returns 0 (nothing drained).
#[intrinsic_method("sun/nio/ch/IOUtil.drain1(I)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn drain_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;

    let Some(guard) = vm.socket_handles().get(&fd).await else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    let cloned = socket
        .try_clone()
        .map_err(|e| InternalError(format!("drain1: clone: {e}")))?;
    drop(guard);

    let result = tokio::task::spawn_blocking(move || {
        let mut buf = [0u8; 1];
        match std::io::Read::read(&mut &cloned, &mut buf) {
            Ok(0) => -1,
            Ok(_) => i32::from(buf[0]),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => -2,
            Err(_) => -1,
        }
    })
    .await
    .map_err(|e| InternalError(format!("drain1: spawn: {e}")))?;

    Ok(Some(Value::Int(result)))
}

/// Return the maximum number of file descriptors.
#[intrinsic_method("sun/nio/ch/IOUtil.fdLimit()I", Any)]
#[async_method]
pub async fn fd_limit<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    {
        let limit = std::process::Command::new("sh")
            .arg("-c")
            .arg("ulimit -n")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.trim().parse::<i32>().ok());
        if let Some(n) = limit {
            return Ok(Some(Value::Int(n)));
        }
    }
    Ok(Some(Value::Int(1024)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.fdVal(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn fd_val<T: Thread + 'static>(
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
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/IOUtil.iovMax()I", Any)]
#[async_method]
pub async fn iov_max<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(16)))
}

/// Create a pipe. Returns a long encoding two fds: `(read_fd << 32) | write_fd`.
#[intrinsic_method("sun/nio/ch/IOUtil.makePipe(Z)J", Any)]
#[async_method]
pub async fn make_pipe<T: Thread + 'static>(
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
pub async fn random_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let buf = parameters.pop_reference()?;
    let Some(buf) = buf else {
        return Err(InternalError("randomBytes: null array".to_string()));
    };
    let mut guard = buf.write();
    let bytes = guard.as_byte_vec_mut()?;
    #[expect(clippy::cast_possible_truncation)]
    let mut seed: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42);
    for b in bytes.iter_mut() {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[expect(clippy::cast_possible_truncation)]
        {
            *b = (seed >> 33) as i8;
        }
    }
    Ok(Some(Value::from(true)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn setfd_val<T: Thread + 'static>(
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
pub async fn write_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let byte_val = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;

    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("write1: socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    let cloned = socket
        .try_clone()
        .map_err(|e| InternalError(format!("write1: clone: {e}")))?;
    drop(guard);

    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let buf = [byte_val as u8];
    let result = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned, &buf))
        .await
        .map_err(|e| InternalError(format!("write1: spawn: {e}")))?
        .map_err(|e| InternalError(format!("write1: {e}")))?;

    let n = i32::try_from(result).map_err(|e| InternalError(e.to_string()))?;
    Ok(Some(Value::Int(n)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.writevMax()J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn writev_max<T: Thread + 'static>(
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
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = configure_blocking(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_drain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drain(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fd_limit() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = fd_limit(thread, Parameters::default()).await?;
        let Some(Value::Int(limit)) = result else {
            panic!("expected Int value");
        };
        assert!(limit > 0, "fd limit should be positive, got {limit}");
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
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drain_1(thread, Parameters::default()).await;
        assert!(result.is_err());
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

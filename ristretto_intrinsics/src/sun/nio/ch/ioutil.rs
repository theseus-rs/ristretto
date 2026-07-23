#[cfg(target_family = "windows")]
use crate::net_helpers::socket_from_type;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

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
    let mut managed_socket = false;
    // Tokio socket types stay non-blocking at the OS level; their Java blocking
    // semantics are emulated by the dispatcher readiness loops.
    if let Some(guard) = vm.socket_handles().get(&fd).await {
        managed_socket = true;
        if let Some(socket) = guard.socket_type.as_raw() {
            socket.set_nonblocking(!blocking).map_err(|error| {
                ristretto_types::JavaError::IoException(format!("configure blocking: {error}"))
            })?;
        }
        // For TcpStream/TcpListener/UdpSocket, tokio manages blocking mode
        drop(guard);
        if let Some(mut guard) = vm.socket_handles().get_mut(&fd).await {
            guard.non_blocking = !blocking;
        }
    }
    #[cfg(target_family = "unix")]
    if !managed_socket {
        #[expect(unsafe_code)]
        // SAFETY: fd is supplied by NIO and F_GETFL/F_SETFL have no pointer arguments.
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL);
            if flags == -1 {
                return Err(super::posix::last_io_exception("configure blocking"));
            }
            let new_flags = if blocking {
                flags & !libc::O_NONBLOCK
            } else {
                flags | libc::O_NONBLOCK
            };
            if new_flags != flags && libc::fcntl(fd, libc::F_SETFL, new_flags) == -1 {
                return Err(super::posix::last_io_exception("configure blocking"));
            }
        }
    }
    #[cfg(target_family = "windows")]
    let _ = managed_socket;
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

    #[cfg(target_family = "unix")]
    {
        let fd = super::posix::raw_descriptor(&*vm, fd).await;
        let mut any = false;
        let mut bytes = [0_u8; 128];
        while super::posix::descriptor_readable(fd)? {
            match super::posix::read_descriptor(fd, &mut bytes)? {
                1.. => any = true,
                -2 | 0 => break,
                super::posix::IOS_INTERRUPTED => {}
                _ => break,
            }
        }
        return Ok(Some(Value::from(any)));
    }

    #[cfg(target_family = "windows")]
    {
        let Some(guard) = vm.socket_handles().get(&fd).await else {
            return Ok(Some(Value::from(false)));
        };

        if let Some(stream) = guard.socket_type.as_tcp_stream() {
            // Drain from TcpStream
            let mut buf = [0u8; 128];
            let mut any = false;
            loop {
                match stream.try_read(&mut buf) {
                    Ok(0) => break,
                    Ok(_) => any = true,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(error) => {
                        return Err(
                            ristretto_types::JavaError::IoException(error.to_string()).into()
                        );
                    }
                }
            }
            return Ok(Some(Value::from(any)));
        }

        let Some(socket) = guard.socket_type.as_raw() else {
            return Ok(Some(Value::from(false)));
        };
        let restore_blocking = !guard.non_blocking;
        if restore_blocking {
            socket.set_nonblocking(true).map_err(|error| {
                ristretto_types::JavaError::IoException(format!(
                    "drain configure non-blocking: {error}"
                ))
            })?;
        }
        let drained = {
            let mut buf = [0u8; 128];
            let mut any = false;
            loop {
                match std::io::Read::read(&mut &*socket, &mut buf) {
                    Ok(0) => break Ok(any),
                    Ok(_) => any = true,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break Ok(any),
                    Err(error) => break Err(error),
                }
            }
        };
        let restore_result = if restore_blocking {
            socket.set_nonblocking(false)
        } else {
            Ok(())
        };
        let drained =
            drained.map_err(|error| ristretto_types::JavaError::IoException(error.to_string()))?;
        restore_result.map_err(|error| {
            ristretto_types::JavaError::IoException(format!("drain restore blocking mode: {error}"))
        })?;

        Ok(Some(Value::from(drained)))
    }
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

    #[cfg(target_family = "unix")]
    {
        let fd = super::posix::raw_descriptor(&*vm, fd).await;
        let mut byte = [0_u8; 1];
        let result = super::posix::read_descriptor(fd, &mut byte)?;
        // OpenJDK's drain1 maps EAGAIN/EWOULDBLOCK to zero, unlike normal
        // reads where IOS_UNAVAILABLE (-2) is returned.
        return Ok(Some(Value::Int(if result == -2 { 0 } else { result })));
    }

    #[cfg(target_family = "windows")]
    {
        let Some(guard) = vm.socket_handles().get(&fd).await else {
            return Ok(Some(Value::Int(-1)));
        };
        let socket = socket_from_type(&guard.socket_type);
        let cloned = socket
            .try_clone()
            .map_err(|e| InternalError(format!("drain1: clone: {e}")))?;
        drop(guard);

        let result = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 1];
            match std::io::Read::read(&mut &cloned, &mut buf) {
                Ok(0) => Ok(-1),
                Ok(_) => Ok(1),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    Ok(super::IOS_INTERRUPTED)
                }
                Err(error) => Err(error),
            }
        })
        .await
        .map_err(|e| InternalError(format!("drain1: spawn: {e}")))?
        .map_err(|error| ristretto_types::JavaError::IoException(error.to_string()))?;

        Ok(Some(Value::Int(result)))
    }
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
        let mut limit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        #[expect(unsafe_code)]
        // SAFETY: limit points to a writable rlimit structure.
        if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &raw mut limit) } == -1 {
            return Err(super::posix::last_io_exception("getrlimit"));
        }
        return Ok(Some(Value::Int(
            i32::try_from(limit.rlim_max).unwrap_or(i32::MAX),
        )));
    }
    #[cfg(windows)]
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
    #[cfg(unix)]
    {
        #[expect(unsafe_code)]
        // SAFETY: sysconf has no pointer arguments.
        let value = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };
        return Ok(Some(Value::Int(if value > 0 {
            i32::try_from(value).unwrap_or(i32::MAX)
        } else {
            16
        })));
    }
    #[cfg(windows)]
    Ok(Some(Value::Int(16)))
}

/// Create a pipe. Returns a long encoding two fds: `(read_fd << 32) | write_fd`.
#[intrinsic_method("sun/nio/ch/IOUtil.makePipe(Z)J", Any)]
#[async_method]
#[cfg_attr(target_family = "unix", expect(unsafe_code))]
pub async fn make_pipe<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let blocking = parameters.pop_int()? != 0;
    #[cfg(any(target_family = "wasm", target_family = "windows"))]
    let _ = blocking;

    #[cfg(not(target_family = "wasm"))]
    {
        #[cfg(target_family = "unix")]
        let (read_file, write_file) = {
            use std::os::fd::FromRawFd;
            let mut descriptors = [0_i32; 2];
            #[cfg(target_os = "linux")]
            let result = {
                let flags = libc::O_CLOEXEC | if blocking { 0 } else { libc::O_NONBLOCK };
                #[expect(unsafe_code)]
                // SAFETY: descriptors points to two writable integers.
                unsafe {
                    libc::pipe2(descriptors.as_mut_ptr(), flags)
                }
            };
            #[cfg(not(target_os = "linux"))]
            let result = {
                #[expect(unsafe_code)]
                // SAFETY: descriptors points to two writable integers.
                let result = unsafe { libc::pipe(descriptors.as_mut_ptr()) };
                if result == 0 {
                    for fd in descriptors {
                        #[expect(unsafe_code)]
                        // SAFETY: fd was returned by pipe and F_GETFD has no pointer argument.
                        let fd_flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
                        #[expect(unsafe_code)]
                        // SAFETY: fd is live and the flags are valid for F_SETFD.
                        unsafe {
                            libc::fcntl(fd, libc::F_SETFD, fd_flags | libc::FD_CLOEXEC)
                        };
                        if !blocking {
                            #[expect(unsafe_code)]
                            // SAFETY: fd was returned by pipe and F_GETFL has no pointer argument.
                            let status = unsafe { libc::fcntl(fd, libc::F_GETFL) };
                            #[expect(unsafe_code)]
                            // SAFETY: fd is live and the flags are valid for F_SETFL.
                            unsafe {
                                libc::fcntl(fd, libc::F_SETFL, status | libc::O_NONBLOCK)
                            };
                        }
                    }
                }
                result
            };
            if result == -1 {
                return Err(super::posix::last_io_exception("pipe"));
            }
            // SAFETY: pipe returned ownership of both live descriptors.
            let read_file = unsafe { std::fs::File::from_raw_fd(descriptors[0]) };
            // SAFETY: pipe returned ownership of both live descriptors.
            let write_file = unsafe { std::fs::File::from_raw_fd(descriptors[1]) };
            (read_file, write_file)
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
    #[cfg(target_family = "unix")]
    {
        let _ = parameters.pop_reference()?;
        return Err(
            ristretto_types::JavaError::UnsupportedOperationException(String::new()).into(),
        );
    }
    #[cfg(target_family = "windows")]
    {
        let buf = parameters.pop_reference()?;
        let Some(buf) = buf else {
            return Err(InternalError("randomBytes: null array".to_string()));
        };
        let mut guard = buf.write();
        let bytes = guard.as_byte_vec_mut()?;
        let mut random = vec![0_u8; bytes.len()];
        if getrandom::fill(&mut random).is_err() {
            return Ok(Some(Value::from(false)));
        }
        for (destination, source) in bytes.iter_mut().zip(random) {
            *destination = source.cast_signed();
        }
        Ok(Some(Value::from(true)))
    }
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
#[cfg_attr(target_family = "windows", expect(unsafe_code))]
pub async fn write_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let byte_val = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;

    #[cfg(target_family = "unix")]
    {
        let fd = super::posix::raw_descriptor(&*vm, fd).await;
        let byte = byte_val.to_ne_bytes()[0];
        return Ok(Some(Value::Int(super::posix::write_descriptor(
            fd,
            &[byte],
        )?)));
    }

    #[cfg(target_family = "windows")]
    {
        use windows_sys::Win32::Networking::WinSock::{SOCKET_ERROR, WSAGetLastError, send};

        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("write1: socket not found for fd {fd}")))?;
        let raw_socket = usize::try_from(guard.socket_type.raw_socket()).unwrap_or(usize::MAX);
        drop(guard);

        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let buf = [byte_val as u8];
        let result = unsafe { send(raw_socket, buf.as_ptr(), 1, 0) };
        if result == SOCKET_ERROR {
            let error = std::io::Error::from_raw_os_error(unsafe { WSAGetLastError() });
            if error.kind() == std::io::ErrorKind::WouldBlock {
                super::wepoll::notify(&*vm)?;
                return Ok(Some(Value::Int(0)));
            }
            return Err(ristretto_types::JavaError::IoException(error.to_string()).into());
        }
        super::wepoll::notify(&*vm)?;
        Ok(Some(Value::Int(result)))
    }
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
        assert!(matches!(result, Some(Value::Int(max)) if max >= 16));
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

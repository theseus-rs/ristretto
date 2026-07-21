use crate::sun::nio::fs::managed_files;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::handles::{FileHandle, SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use socket2::{Domain, Socket};
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::Arc;

fn io_exception(operation: &str, error: &std::io::Error) -> ristretto_types::Error {
    ristretto_types::JavaError::IoException(format!("{operation}: {error}")).into()
}

#[expect(unsafe_code)]
fn borrowed_socket(fd: RawFd) -> ManuallyDrop<Socket> {
    // SAFETY: ManuallyDrop prevents the borrowed descriptor from being closed.
    ManuallyDrop::new(unsafe { Socket::from_raw_fd(fd) })
}

fn socket_family(fd: RawFd) -> i32 {
    if fd < 0 {
        return -1;
    }
    match borrowed_socket(fd)
        .local_addr()
        .map(|address| address.domain())
    {
        Ok(Domain::IPV4) => 1,
        Ok(Domain::IPV6) => 2,
        Ok(Domain::UNIX) => 3,
        _ => -1,
    }
}

async fn inet_peer_address<T: Thread + 'static>(thread: &Arc<T>, fd: RawFd) -> Result<Value> {
    if fd < 0 {
        return Ok(Value::Object(None));
    }
    let Ok(address) = borrowed_socket(fd).peer_addr() else {
        return Ok(Value::Object(None));
    };
    let bytes = if let Some(address) = address.as_socket_ipv4() {
        address.ip().octets().to_vec()
    } else if let Some(address) = address.as_socket_ipv6() {
        address.ip().octets().to_vec()
    } else {
        return Ok(Value::Object(None));
    };
    let vm = thread.vm()?;
    let bytes: Box<[i8]> = bytes.iter().map(|byte| byte.cast_signed()).collect();
    let bytes = Value::new_object(vm.garbage_collector(), Reference::ByteArray(bytes));
    thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[Value::Object(None), bytes],
        )
        .await?
        .ok_or_else(|| InternalError("InetAddress.getByAddress returned null".to_string()))
}

#[expect(unsafe_code)]
async fn adopt_descriptor<V: VM>(vm: &V, fd: RawFd) -> Result<()> {
    let socket = borrowed_socket(fd);
    if socket.r#type().is_ok() {
        let family = socket_family(fd);
        // SAFETY: fd is newly duplicated and ownership is transferred to Socket.
        let socket = unsafe { Socket::from_raw_fd(fd) };
        let mut handle = SocketHandle::new(SocketType::Raw(socket));
        handle.is_ipv6 = family == 2;
        handle.is_unix = family == 3;
        vm.socket_handles().insert(fd, handle).await?;
    } else {
        // SAFETY: fd is newly opened/duplicated and ownership is transferred to File.
        let file = unsafe { std::fs::File::from_raw_fd(fd) };
        let file = tokio::fs::File::from_std(file);
        vm.file_handles()
            .insert(i64::from(fd), FileHandle::from((file, false)))
            .await?;
    }
    Ok(())
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.addressFamily(I)I", GreaterThan(JAVA_11))]
#[async_method]
pub async fn address_family<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    Ok(Some(Value::Int(socket_family(fd))))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.close0(I)V", Any)]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    if vm.socket_handles().remove(&fd).await.is_some() {
        return Ok(None);
    }
    if vm.file_handles().get(&i64::from(fd)).await.is_some() {
        managed_files::close(vm.file_handles(), i64::from(fd)).await;
        return Ok(None);
    }
    #[expect(unsafe_code)]
    // SAFETY: fd is supplied by InheritedChannel and is no longer VM-owned.
    if unsafe { libc::close(fd) } == -1 {
        return Err(io_exception("close", &std::io::Error::last_os_error()));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup(I)I", Any)]
#[async_method]
pub async fn dup<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[expect(unsafe_code)]
    // SAFETY: dup has no pointer arguments and returns a new owned descriptor.
    let new_fd = unsafe { libc::dup(fd) };
    if new_fd == -1 {
        return Err(io_exception("dup", &std::io::Error::last_os_error()));
    }
    let vm = thread.vm()?;
    adopt_descriptor(&*vm, new_fd).await?;
    Ok(Some(Value::Int(new_fd)))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup2(II)V", Any)]
#[async_method]
pub async fn dup_2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd2 = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    #[expect(unsafe_code)]
    // SAFETY: dup2 has no pointer arguments; both descriptors are provided by Java.
    if unsafe { libc::dup2(fd, fd2) } == -1 {
        return Err(io_exception("dup2", &std::io::Error::last_os_error()));
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn inet_peer_address_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    Ok(Some(inet_peer_address(&thread, fd).await?))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.initIDs()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.isConnected(I)Z", GreaterThan(JAVA_11))]
#[async_method]
pub async fn is_connected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    Ok(Some(Value::from(
        fd >= 0 && borrowed_socket(fd).peer_addr().is_ok(),
    )))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.open0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn open_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let oflag = parameters.pop_int()?;
    let path = parameters.pop()?;
    let path = path.as_string()?;
    let path = CString::new(path)
        .map_err(|_| InternalError("InheritedChannel.open0 path contains NUL".to_string()))?;
    #[expect(unsafe_code)]
    // SAFETY: path is a NUL-terminated string and the flags are provided by OpenJDK.
    let fd = unsafe { libc::open(path.as_ptr(), oflag) };
    if fd == -1 {
        return Err(io_exception("open", &std::io::Error::last_os_error()));
    }
    let vm = thread.vm()?;
    adopt_descriptor(&*vm, fd).await?;
    Ok(Some(Value::Int(fd)))
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn peer_address_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    Ok(Some(inet_peer_address(&thread, fd).await?))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.peerPort0(I)I", Any)]
#[async_method]
pub async fn peer_port_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let port = if fd < 0 {
        -1
    } else {
        borrowed_socket(fd)
            .peer_addr()
            .ok()
            .and_then(|address| address.as_socket())
            .map_or(-1, |address| i32::from(address.port()))
    };
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.soType0(I)I", Any)]
#[async_method]
pub async fn so_type_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let socket_type = if fd < 0 {
        None
    } else {
        borrowed_socket(fd).r#type().ok()
    }
    .map_or(0, |kind| {
        if kind == socket2::Type::STREAM {
            libc::SOCK_STREAM
        } else if kind == socket2::Type::DGRAM {
            libc::SOCK_DGRAM
        } else {
            0
        }
    });
    Ok(Some(Value::Int(socket_type)))
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.unixPeerAddress0(I)[B",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn unix_peer_address_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    if fd < 0 {
        return Ok(Some(Value::Object(None)));
    }
    let Ok(address) = borrowed_socket(fd).peer_addr() else {
        return Ok(Some(Value::Object(None)));
    };
    let bytes = address.as_pathname().map_or_else(Vec::new, |path| {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    });
    let vm = thread.vm()?;
    let bytes: Box<[i8]> = bytes.iter().map(|byte| byte.cast_signed()).collect();
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::ByteArray(bytes),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::new(vec![Value::Int(-1)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup(thread, Parameters::new(vec![Value::Int(-1)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dup_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup_2(
            thread,
            Parameters::new(vec![Value::Int(-1), Value::Int(-1)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peer_port_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peer_port_0(thread, Parameters::new(vec![Value::Int(-1)]))
            .await
            .expect("peer port");
        assert_eq!(Some(Value::Int(-1)), result);
    }

    #[tokio::test]
    async fn test_so_type_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = so_type_0(thread, Parameters::new(vec![Value::Int(-1)]))
            .await
            .expect("socket type");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_inet_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inet_peer_address_0(thread, Parameters::new(vec![Value::Int(-1)])).await;
        assert_eq!(Some(Value::Object(None)), result.expect("peer address"));
    }

    #[tokio::test]
    async fn test_address_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = address_family(thread, Parameters::new(vec![Value::Int(-1)]))
            .await
            .expect("family");
        assert_eq!(Some(Value::Int(-1)), result);
    }

    #[tokio::test]
    async fn test_is_connected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_connected(thread, Parameters::new(vec![Value::Int(-1)]))
            .await
            .expect("connected");
        assert_eq!(Some(Value::from(false)), result);
    }

    #[tokio::test]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peer_address_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = peer_address_0(thread, Parameters::new(vec![Value::Int(-1)])).await;
        assert_eq!(Some(Value::Object(None)), result.expect("peer address"));
    }

    #[tokio::test]
    async fn test_unix_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unix_peer_address_0(thread, Parameters::new(vec![Value::Int(-1)])).await;
        assert_eq!(Some(Value::Object(None)), result.expect("peer address"));
    }
}

use crate::java::io::socketfiledescriptor::{get_fd, set_fd};
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use socket2::{Domain, SockAddr, Socket, Type};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg(windows)]
fn microsoft_unix_socket() -> std::io::Result<Socket> {
    use std::os::windows::io::FromRawSocket;
    use windows_sys::Win32::Foundation::{HANDLE_FLAG_INHERIT, SetHandleInformation};
    use windows_sys::Win32::Networking::WinSock::{
        AF_UNIX, INVALID_SOCKET, SOCK_STREAM, SOCKET_ERROR, WSA_FLAG_OVERLAPPED, WSAENOBUFS,
        WSAEnumProtocolsW, WSAGetLastError, WSAPROTOCOL_INFOW, WSASocketW,
    };
    use windows_sys::core::GUID;

    const MICROSOFT_UNIX_PROVIDER: GUID =
        GUID::from_u128(0xa009_43d9_9c2e_4633_9b59_0057_a316_0994);

    fn same_guid(left: &GUID, right: &GUID) -> bool {
        left.data1 == right.data1
            && left.data2 == right.data2
            && left.data3 == right.data3
            && left.data4 == right.data4
    }

    // socket2 initializes Winsock on first use. Open an IPv4 socket solely to
    // ensure that initialization has happened before enumerating providers.
    drop(Socket::new(Domain::IPV4, Type::STREAM, None)?);

    let mut buffer_length = 0_u32;
    #[expect(unsafe_code)]
    // SAFETY: Null buffers are accepted to query the required byte length.
    let initial_result = unsafe {
        WSAEnumProtocolsW(
            std::ptr::null(),
            std::ptr::null_mut(),
            &raw mut buffer_length,
        )
    };
    if initial_result != SOCKET_ERROR {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Microsoft AF_UNIX Winsock provider was not found",
        ));
    }
    #[expect(unsafe_code)]
    // SAFETY: WSAGetLastError has no pointer arguments.
    let initial_error = unsafe { WSAGetLastError() };
    if initial_error != WSAENOBUFS {
        return Err(std::io::Error::from_raw_os_error(initial_error));
    }

    let entry_size = u32::try_from(size_of::<WSAPROTOCOL_INFOW>()).unwrap_or(u32::MAX);
    let entry_count = usize::try_from(buffer_length.div_ceil(entry_size)).unwrap_or_default();
    let mut protocols = vec![WSAPROTOCOL_INFOW::default(); entry_count];
    #[expect(unsafe_code)]
    // SAFETY: protocols has at least buffer_length writable bytes and its pointer
    // has the alignment required by WSAPROTOCOL_INFOW.
    let protocol_count = unsafe {
        WSAEnumProtocolsW(
            std::ptr::null(),
            protocols.as_mut_ptr(),
            &raw mut buffer_length,
        )
    };
    if protocol_count == SOCKET_ERROR {
        #[expect(unsafe_code)]
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { WSAGetLastError() };
        return Err(std::io::Error::from_raw_os_error(code));
    }
    let protocol_count = usize::try_from(protocol_count).unwrap_or_default();
    let provider = protocols
        .get(..protocol_count)
        .unwrap_or_default()
        .iter()
        .find(|protocol| {
            protocol.iAddressFamily == i32::from(AF_UNIX)
                && same_guid(&protocol.ProviderId, &MICROSOFT_UNIX_PROVIDER)
        })
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Microsoft AF_UNIX Winsock provider was not found",
            )
        })?;

    #[expect(unsafe_code)]
    // SAFETY: provider points into the live protocols buffer, and Winsock returns
    // either an owned SOCKET or INVALID_SOCKET.
    let raw_socket = unsafe {
        WSASocketW(
            i32::from(AF_UNIX),
            SOCK_STREAM,
            0,
            std::ptr::from_ref(provider),
            0,
            WSA_FLAG_OVERLAPPED,
        )
    };
    if raw_socket == INVALID_SOCKET {
        #[expect(unsafe_code)]
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { WSAGetLastError() };
        return Err(std::io::Error::from_raw_os_error(code));
    }

    let raw_socket = u64::try_from(raw_socket).unwrap_or(u64::MAX);
    #[expect(unsafe_code)]
    // SAFETY: WSASocketW returned a new owned socket, which is transferred to Socket.
    let socket = unsafe { Socket::from_raw_socket(raw_socket) };
    let handle = std::ptr::with_exposed_provenance_mut(usize::try_from(raw_socket).unwrap_or(0));
    #[expect(unsafe_code)]
    // SAFETY: handle is the live socket handle owned by socket. A zero flags value
    // clears HANDLE_FLAG_INHERIT without changing ownership.
    let inherited = unsafe { SetHandleInformation(handle, HANDLE_FLAG_INHERIT, 0) };
    if inherited == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(socket)
}

fn socket_error(operation: &str, error: &std::io::Error) -> ristretto_types::Error {
    ristretto_types::JavaError::IoException(format!("{operation}: {error}")).into()
}

fn connect_pending(error: &std::io::Error) -> bool {
    if error.kind() == std::io::ErrorKind::WouldBlock {
        return true;
    }
    #[cfg(unix)]
    return error.raw_os_error() == Some(libc::EINPROGRESS);
    #[cfg(windows)]
    return error.raw_os_error() == Some(windows_sys::Win32::Networking::WinSock::WSAEINPROGRESS);
    #[cfg(not(any(unix, windows)))]
    false
}

fn path_from_value(value: &Value) -> Result<PathBuf> {
    let bytes = value.as_byte_vec_ref()?;
    let bytes: Vec<u8> = bytes.iter().map(|byte| byte.cast_unsigned()).collect();
    #[cfg(unix)]
    {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        Ok(PathBuf::from(OsString::from_vec(bytes)))
    }
    #[cfg(windows)]
    {
        Ok(PathBuf::from(String::from_utf8_lossy(&bytes).into_owned()))
    }
}

#[cfg(not(windows))]
fn unix_address(path: &Path) -> std::io::Result<SockAddr> {
    SockAddr::unix(path)
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn unix_address(path: &Path) -> std::io::Result<SockAddr> {
    use socket2::SockAddrStorage;
    use windows_sys::Win32::Networking::WinSock::{AF_UNIX, SOCKADDR_UN};

    let bytes = path
        .to_str()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unix-domain path must be valid UTF-8",
            )
        })?
        .as_bytes();
    if bytes.len() > SOCKADDR_UN::default().sun_path.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unix-domain path is too long",
        ));
    }

    // OpenJDK's Windows implementation passes the address length as the
    // offset of sun_path plus the path bytes, without a trailing NUL. The
    // Microsoft AF_UNIX provider rejects the extra byte used by socket2's
    // generic Windows constructor with WSAEINVAL on the hosted runners.
    let ((), address) = unsafe {
        SockAddr::try_init(|storage: *mut SockAddrStorage, length| {
            let address = storage.cast::<SOCKADDR_UN>();
            address.write(SOCKADDR_UN::default());
            (*address).sun_family = AF_UNIX;
            for (destination, source) in (*address).sun_path.iter_mut().zip(bytes) {
                *destination = source.cast_signed();
            }
            let address_length = std::mem::offset_of!(SOCKADDR_UN, sun_path)
                .checked_add(bytes.len())
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Unix-domain address length overflow",
                    )
                })?;
            *length = i32::try_from(address_length)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
            Ok(())
        })?
    };
    Ok(address)
}

#[cfg(unix)]
fn path_bytes(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().to_vec()
}

#[cfg(windows)]
fn path_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}

#[cfg(windows)]
fn address_path_bytes(address: &SockAddr) -> Vec<u8> {
    let length = usize::try_from(address.len()).unwrap_or_default();
    if length <= size_of::<u16>() {
        return Vec::new();
    }
    #[expect(unsafe_code)]
    // SAFETY: SockAddr owns an initialized address buffer of address.len() bytes.
    let bytes = unsafe { std::slice::from_raw_parts(address.as_ptr().cast::<u8>(), length) };
    let path = bytes.get(size_of::<u16>()..).unwrap_or_default();
    let end = path
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(path.len());
    path.get(..end).unwrap_or_default().to_vec()
}

fn byte_array(vm: &impl VM, bytes: &[u8]) -> Value {
    let bytes: Box<[i8]> = bytes.iter().map(|byte| byte.cast_signed()).collect();
    Value::new_object(vm.garbage_collector(), Reference::ByteArray(bytes))
}

async fn restore_blocking<V: VM>(vm: &V, fd: i32) -> Result<()> {
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("UnixDomainSockets.connect0: invalid fd {fd}")))?;
    let socket = handle
        .socket_type
        .as_raw()
        .ok_or_else(|| InternalError("Unix-domain socket is not raw".to_string()))?;
    socket
        .set_nonblocking(false)
        .map_err(|error| socket_error("Unix connect configure blocking", &error))
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn accept_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let array = parameters.pop()?;
    let new_fd_value = parameters.pop()?;
    let fd_value = parameters.pop()?;
    if array.as_class_vec_ref()?.1.is_empty() {
        return Err(InternalError("accept0 address array is empty".to_string()));
    }
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let (socket, non_blocking) = {
        let handle =
            vm.socket_handles().get(&fd).await.ok_or_else(|| {
                InternalError(format!("UnixDomainSockets.accept0: invalid fd {fd}"))
            })?;
        let socket = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| InternalError("Unix-domain listener is not a raw socket".to_string()))?
            .try_clone()
            .map_err(|error| socket_error("accept clone", &error))?;
        (socket, handle.non_blocking)
    };
    let accepted = if non_blocking {
        socket.accept()
    } else {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, false).await?;
        if status < 0 {
            return Ok(Some(Value::Int(i32::try_from(status)?)));
        }
        socket.accept()
    };
    let (accepted, peer) = match accepted {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            return Ok(Some(Value::Int(-2)));
        }
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            return Ok(Some(Value::Int(-3)));
        }
        Err(error) => return Err(socket_error("accept", &error)),
    };
    accepted
        .set_nonblocking(false)
        .map_err(|error| socket_error("accept configure", &error))?;
    let new_fd = vm.next_nio_fd();
    set_fd(&new_fd_value, new_fd)?;
    let mut handle = SocketHandle::new(SocketType::Raw(accepted));
    handle.is_unix = true;
    handle.non_blocking = false;
    vm.socket_handles().insert(new_fd, handle).await?;

    #[cfg(unix)]
    let peer_path = peer.as_pathname().map_or_else(Vec::new, path_bytes);
    #[cfg(windows)]
    let peer_path = address_path_bytes(&peer);
    let (_, mut values) = array.as_class_vec_mut()?;
    let first = values
        .first_mut()
        .ok_or_else(|| InternalError("accept0 address array is empty".to_string()))?;
    *first = byte_array(&*vm, &peer_path);
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let path = path_from_value(&path)?;
    let address = unix_address(&path).map_err(|error| socket_error("Unix bind address", &error))?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("UnixDomainSockets.bind0: invalid fd {fd}")))?;
    let socket = handle
        .socket_type
        .as_raw()
        .ok_or_else(|| InternalError("Unix-domain socket is not raw".to_string()))?;
    socket
        .bind(&address)
        .map_err(|error| socket_error("Unix bind", &error))?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let path = path_from_value(&path)?;
    let address =
        unix_address(&path).map_err(|error| socket_error("Unix connect address", &error))?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    #[cfg(not(windows))]
    let (result, non_blocking, restore_socket_mode) = {
        let handle =
            vm.socket_handles().get(&fd).await.ok_or_else(|| {
                InternalError(format!("UnixDomainSockets.connect0: invalid fd {fd}"))
            })?;
        let socket = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| InternalError("Unix-domain socket is not raw".to_string()))?;
        let non_blocking = handle.non_blocking;
        if !non_blocking {
            socket
                .set_nonblocking(true)
                .map_err(|error| socket_error("Unix connect configure non-blocking", &error))?;
        }
        (socket.connect(&address), non_blocking, !non_blocking)
    };
    #[cfg(windows)]
    let (result, non_blocking, restore_socket_mode) = {
        use std::os::windows::io::AsRawSocket;

        let handle =
            vm.socket_handles().get(&fd).await.ok_or_else(|| {
                InternalError(format!("UnixDomainSockets.connect0: invalid fd {fd}"))
            })?;
        let socket = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| InternalError("Unix-domain socket is not raw".to_string()))?;
        let raw_socket = socket.as_raw_socket();
        let non_blocking = handle.non_blocking;
        drop(handle);
        let result = if non_blocking {
            super::net::connect_windows_socket(raw_socket, &address)
        } else {
            tokio::task::spawn_blocking(move || {
                super::net::connect_windows_socket(raw_socket, &address)
            })
            .await
            .map_err(|error| InternalError(format!("Unix connect task failed: {error}")))?
        };
        (result, non_blocking, false)
    };
    match result {
        Ok(()) => {
            if restore_socket_mode {
                restore_blocking(&*vm, fd).await?;
            }
            Ok(Some(Value::Int(1)))
        }
        Err(error) if connect_pending(&error) && non_blocking => Ok(Some(Value::Int(-2))),
        Err(error) if connect_pending(&error) => {
            let status = super::net::check_connect(&*vm, fd, true).await;
            match status {
                Ok(status) => {
                    if restore_socket_mode {
                        restore_blocking(&*vm, fd).await?;
                    }
                    Ok(Some(Value::Int(status)))
                }
                Err(error) => {
                    if restore_socket_mode {
                        let _ = restore_blocking(&*vm, fd).await;
                    }
                    Err(error)
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            if restore_socket_mode {
                let _ = restore_blocking(&*vm, fd).await;
            }
            Ok(Some(Value::Int(-3)))
        }
        Err(error) => {
            if restore_socket_mode {
                let _ = restore_blocking(&*vm, fd).await;
            }
            Err(socket_error("Unix connect", &error))
        }
    }
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.init()Z", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(windows)]
    let supported = microsoft_unix_socket().is_ok();
    #[cfg(not(windows))]
    let supported = Socket::new(Domain::UNIX, Type::STREAM, None).is_ok();
    Ok(Some(Value::from(supported)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn local_address_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(|| {
        InternalError(format!("UnixDomainSockets.localAddress0: invalid fd {fd}"))
    })?;
    let socket = handle
        .socket_type
        .as_raw()
        .ok_or_else(|| InternalError("Unix-domain socket is not raw".to_string()))?;
    let address = socket
        .local_addr()
        .map_err(|error| socket_error("Unix local address", &error))?;
    #[cfg(unix)]
    let path = address.as_pathname().map_or_else(Vec::new, path_bytes);
    #[cfg(windows)]
    let path = address_path_bytes(&address);
    Ok(Some(byte_array(&*vm, &path)))
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.socket0()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(windows)]
    let socket = microsoft_unix_socket().map_err(|error| socket_error("Unix socket", &error))?;
    #[cfg(not(windows))]
    let socket = Socket::new(Domain::UNIX, Type::STREAM, None)
        .map_err(|error| socket_error("Unix socket", &error))?;
    let vm = thread.vm()?;
    let fd = vm.next_nio_fd();
    let mut handle = SocketHandle::new(SocketType::Raw(socket));
    handle.is_unix = true;
    vm.socket_handles().insert(fd, handle).await?;
    Ok(Some(Value::Int(fd)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await.expect("init");
        assert_eq!(Some(Value::from(true)), result);
    }

    #[tokio::test]
    async fn test_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_address_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_0(thread, Parameters::default())
            .await
            .expect("socket");
        assert!(matches!(result, Some(Value::Int(fd)) if fd >= 0));
    }
}

use crate::java::io::socketfiledescriptor::get_fd;
use crate::net_helpers::inet_socket_address;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
#[cfg(all(unix, not(target_os = "macos")))]
use std::mem::size_of;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;

const MAX_PACKET_LEN: usize = 65_536;

#[cfg(windows)]
#[expect(
    unsafe_code,
    clippy::needless_pass_by_value,
    reason = "the blocking worker owns its cloned socket for the full receive operation"
)]
fn receive_datagram(
    socket: socket2::Socket,
    count: usize,
) -> std::io::Result<(usize, Vec<u8>, SocketAddr)> {
    let mut data = vec![0_u8; count];
    // SAFETY: `MaybeUninit<u8>` has the same layout as `u8`; the backing bytes
    // are already initialized and remain owned by `data` for the entire call.
    let uninitialized = unsafe {
        std::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<std::mem::MaybeUninit<u8>>(), count)
    };
    let mut buffers = [socket2::MaybeUninitSlice::new(uninitialized)];
    let (received, _flags, sender) = socket.recv_from_vectored(&mut buffers)?;
    let received = received.min(count);
    data.truncate(received);
    let sender = sender.as_socket().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "datagram sender is not an Internet socket address",
        )
    })?;
    Ok((received, data, sender))
}

#[cfg(not(windows))]
fn receive_datagram(
    socket: socket2::Socket,
    count: usize,
) -> std::io::Result<(usize, Vec<u8>, SocketAddr)> {
    let udp: std::net::UdpSocket = socket.into();
    let mut data = vec![0_u8; count];
    let (received, sender) = udp.recv_from(&mut data)?;
    data.truncate(received);
    Ok((received, data, sender))
}

#[cfg(windows)]
#[expect(unsafe_code)]
pub(super) fn receive_connected_datagram(
    socket: &socket2::Socket,
    count: usize,
) -> std::io::Result<(usize, Vec<u8>)> {
    let mut data = vec![0_u8; count];
    // SAFETY: `MaybeUninit<u8>` has the same layout as `u8`; the backing bytes
    // are initialized and remain valid for the duration of `recv_vectored`.
    let uninitialized = unsafe {
        std::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<std::mem::MaybeUninit<u8>>(), count)
    };
    let mut buffers = [socket2::MaybeUninitSlice::new(uninitialized)];
    let (received, _flags) = socket.recv_vectored(&mut buffers)?;
    let received = received.min(count);
    data.truncate(received);
    Ok((received, data))
}

#[cfg(not(windows))]
pub(super) fn receive_connected_datagram(
    socket: &socket2::Socket,
    count: usize,
) -> std::io::Result<(usize, Vec<u8>)> {
    let mut data = vec![0_u8; count];
    let mut socket = socket;
    let received = std::io::Read::read(&mut socket, &mut data)?;
    data.truncate(received);
    Ok((received, data))
}

#[cfg(unix)]
#[expect(clippy::cast_possible_truncation)]
const AF_INET6_VALUE: u16 = libc::AF_INET6 as u16;
#[cfg(target_os = "windows")]
const AF_INET6_VALUE: u16 = 23;

fn native_bytes(
    memory: &ristretto_types::NativeMemory,
    address: i64,
    length: usize,
    operation: &str,
) -> Result<Vec<u8>> {
    if length == 0 {
        return Ok(Vec::new());
    }
    memory.try_read_bytes(address, length).ok_or_else(|| {
        InternalError(format!(
            "{operation}: invalid native memory range at {address} with length {length}"
        ))
    })
}

#[expect(
    clippy::indexing_slicing,
    reason = "the sockaddr length is checked before its fixed-layout fields are accessed"
)]
fn read_sockaddr(
    memory: &ristretto_types::NativeMemory,
    address: i64,
    length: usize,
) -> Result<socket2::SockAddr> {
    if length < 2 {
        return Err(InternalError(
            "socket address is shorter than its family".to_string(),
        ));
    }
    let family_bytes = native_bytes(memory, address, 2, "DatagramChannelImpl.send0")?;
    #[cfg(target_os = "macos")]
    let family = u16::from(family_bytes[1]);
    #[cfg(not(target_os = "macos"))]
    let family = u16::from_ne_bytes(
        family_bytes
            .try_into()
            .map_err(|_| InternalError("invalid socket family".to_string()))?,
    );
    let required = if family == AF_INET6_VALUE { 28 } else { 16 };
    if length < required {
        return Err(InternalError(format!(
            "socket address length {length} is less than {required}"
        )));
    }
    let bytes = native_bytes(memory, address, required, "DatagramChannelImpl.send0")?;
    let port = u16::from_be_bytes([bytes[2], bytes[3]]);
    if family == AF_INET6_VALUE {
        let octets: [u8; 16] = bytes[8..24]
            .try_into()
            .map_err(|_| InternalError("invalid IPv6 socket address".to_string()))?;
        let scope_id = u32::from_ne_bytes(
            bytes[24..28]
                .try_into()
                .map_err(|_| InternalError("invalid IPv6 scope id".to_string()))?,
        );
        Ok(socket2::SockAddr::from(SocketAddrV6::new(
            Ipv6Addr::from(octets),
            port,
            0,
            scope_id,
        )))
    } else if family == 2 {
        Ok(socket2::SockAddr::from(SocketAddrV4::new(
            Ipv4Addr::new(bytes[4], bytes[5], bytes[6], bytes[7]),
            port,
        )))
    } else {
        Err(ristretto_types::JavaError::SocketException(format!(
            "unsupported socket address family {family}"
        ))
        .into())
    }
}

#[expect(
    clippy::indexing_slicing,
    reason = "the buffers are created at the exact sockaddr layout sizes"
)]
fn write_sockaddr(
    memory: &ristretto_types::NativeMemory,
    address: i64,
    socket_address: &SocketAddr,
) -> Result<()> {
    let bytes = match socket_address {
        SocketAddr::V4(address) => {
            let mut bytes = vec![0_u8; 16];
            #[cfg(target_os = "macos")]
            {
                bytes[0] = 16;
                bytes[1] = 2;
            }
            #[cfg(not(target_os = "macos"))]
            bytes[..2].copy_from_slice(&2_u16.to_ne_bytes());
            bytes[2..4].copy_from_slice(&address.port().to_be_bytes());
            bytes[4..8].copy_from_slice(&address.ip().octets());
            bytes
        }
        SocketAddr::V6(address) => {
            let mut bytes = vec![0_u8; 28];
            #[cfg(target_os = "macos")]
            {
                bytes[0] = 28;
                bytes[1] = u8::try_from(AF_INET6_VALUE).unwrap_or_default();
            }
            #[cfg(not(target_os = "macos"))]
            bytes[..2].copy_from_slice(&AF_INET6_VALUE.to_ne_bytes());
            bytes[2..4].copy_from_slice(&address.port().to_be_bytes());
            bytes[4..8].copy_from_slice(&address.flowinfo().to_ne_bytes());
            bytes[8..24].copy_from_slice(&address.ip().octets());
            bytes[24..28].copy_from_slice(&address.scope_id().to_ne_bytes());
            bytes
        }
    };
    if memory.try_write_bytes(address, &bytes) {
        Ok(())
    } else {
        Err(InternalError("invalid native sender address".to_string()))
    }
}

pub(super) fn datagram_io_status(error: &std::io::Error) -> Result<i64> {
    match error.kind() {
        std::io::ErrorKind::WouldBlock => Ok(i64::from(super::IOS_UNAVAILABLE)),
        std::io::ErrorKind::Interrupted => Ok(i64::from(super::IOS_INTERRUPTED)),
        std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionReset => {
            Err(ristretto_types::JavaError::PortUnreachableException(error.to_string()).into())
        }
        _ => Err(ristretto_types::JavaError::SocketException(error.to_string()).into()),
    }
}

async fn inet_socket_address_value<T: Thread + 'static>(
    thread: &Arc<T>,
    address: SocketAddr,
) -> Result<(Value, Value)> {
    let vm = thread.vm()?;
    let (octets, scope_id, port) = match address {
        SocketAddr::V4(address) => (address.ip().octets().to_vec(), 0, address.port()),
        SocketAddr::V6(address) => (
            address.ip().octets().to_vec(),
            address.scope_id(),
            address.port(),
        ),
    };
    #[expect(clippy::cast_possible_wrap)]
    let octets: Box<[i8]> = octets.into_iter().map(|byte| byte as i8).collect();
    let bytes = Value::new_object(vm.garbage_collector(), Reference::ByteArray(octets));
    let inet_address = if scope_id == 0 {
        thread
            .invoke(
                "java.net.InetAddress",
                "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
                &[Value::Object(None), bytes],
            )
            .await?
    } else {
        thread
            .invoke(
                "java.net.Inet6Address",
                "getByAddress(Ljava/lang/String;[BI)Ljava/net/Inet6Address;",
                &[
                    Value::Object(None),
                    bytes,
                    Value::Int(i32::try_from(scope_id).unwrap_or(i32::MAX)),
                ],
            )
            .await?
    }
    .ok_or_else(|| InternalError("InetAddress.getByAddress returned null".to_string()))?;
    let socket_address = thread
        .object(
            "java.net.InetSocketAddress",
            "Ljava/net/InetAddress;I",
            &[inet_address.clone(), Value::Int(i32::from(port))],
        )
        .await?;
    Ok((inet_address, socket_address))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V",
    Any
)]
#[async_method]
pub async fn disconnect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_ipv6 = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;

    // Disconnect by connecting to AF_UNSPEC, matching the real JDK native implementation.
    // Using AF_INET6/AF_INET with unspecified address does NOT disconnect on Linux;
    // the kernel only disconnects UDP sockets when sa_family == AF_UNSPEC.
    let cloned_socket = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("disconnect: clone: {e}")))?
    };

    let result = tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "macos")]
        {
            use std::os::fd::AsRawFd;
            #[expect(unsafe_code)]
            // SAFETY: the cloned socket is live and the wildcard association and
            // connection identifiers request a datagram disconnect.
            let result = unsafe {
                libc::disconnectx(
                    cloned_socket.as_raw_fd(),
                    libc::SAE_ASSOCID_ANY,
                    libc::SAE_CONNID_ANY,
                )
            };
            if result == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            use std::os::fd::AsRawFd;
            #[expect(unsafe_code, clippy::cast_possible_truncation)]
            // SAFETY: cloned_socket is a valid socket fd; we connect to AF_UNSPEC to
            // disconnect the UDP socket, matching the JDK native implementation.
            unsafe {
                let mut addr: libc::sockaddr_storage = std::mem::zeroed();
                addr.ss_family = libc::AF_UNSPEC as libc::sa_family_t;
                // On macOS, connect(AF_UNSPEC) may return EAFNOSUPPORT; ignore it
                let result = libc::connect(
                    cloned_socket.as_raw_fd(),
                    std::ptr::from_ref(&addr).cast::<libc::sockaddr>(),
                    size_of::<libc::sockaddr>() as libc::socklen_t,
                );
                if result == -1 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(())
                }
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawSocket;

            #[expect(unsafe_code)]
            // SAFETY: cloned_socket is a valid socket handle; we connect to AF_UNSPEC
            // using a zeroed SOCKETADDRESS-sized buffer, matching OpenJDK.
            unsafe {
                let address = [0_u8; 28];
                let result = windows_sys::Win32::Networking::WinSock::connect(
                    usize::try_from(cloned_socket.as_raw_socket()).unwrap_or(usize::MAX),
                    address.as_ptr().cast(),
                    i32::try_from(address.len()).unwrap_or(28),
                );
                if result == -1 {
                    let code = windows_sys::Win32::Networking::WinSock::WSAGetLastError();
                    Err(std::io::Error::from_raw_os_error(code))
                } else {
                    Ok(())
                }
            }
        }
    })
    .await
    .map_err(|e| InternalError(format!("disconnect: spawn: {e}")))?;
    if let Err(error) = result {
        #[cfg(all(unix, not(target_os = "macos")))]
        if error.kind() == std::io::ErrorKind::AddrNotAvailable {
            return Ok(None);
        }
        return Err(ristretto_types::JavaError::SocketException(error.to_string()).into());
    }

    Ok(None)
}

#[intrinsic_method("sun/nio/ch/DatagramChannelImpl.initIDs()V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn receive_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let connected = parameters.pop_bool()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let receiver = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len)
        .map_err(|e| InternalError(e.to_string()))?
        .min(MAX_PACKET_LEN);

    let vm = thread.vm()?;
    let non_blocking = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        guard.non_blocking
    };

    let result = loop {
        if !non_blocking {
            let status = super::socketdispatcher::wait_raw_socket(&thread, fd, false).await?;
            if status < 0 {
                return Ok(Some(Value::Int(i32::try_from(status)?)));
            }
        }
        let cloned_socket = {
            let guard = vm
                .socket_handles()
                .get(&fd)
                .await
                .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
            let socket = guard
                .socket_type
                .as_raw()
                .ok_or_else(|| InternalError("expected raw socket".to_string()))?;
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("receive: clone: {e}")))?
        };
        let result = tokio::task::spawn_blocking(move || receive_datagram(cloned_socket, count))
            .await
            .map_err(|e| InternalError(format!("receive: spawn: {e}")))?;
        match result {
            Err(error) if !connected && error.kind() == std::io::ErrorKind::ConnectionRefused => {}
            result => break result,
        }
    };

    let (n, data, sender) = match result {
        Ok(result) => result,
        Err(error) => {
            return Ok(Some(Value::Int(i32::try_from(datagram_io_status(
                &error,
            )?)?)));
        }
    };
    if !vm.native_memory().try_write_bytes(address, &data) {
        return Err(InternalError("receive0: invalid native buffer".to_string()));
    }
    let (inet_address, socket_address) = inet_socket_address_value(&thread, sender).await?;
    let mut receiver = receiver.as_reference_mut()?;
    let Reference::Object(receiver) = &mut *receiver else {
        return Err(InternalError(
            "DatagramChannelImpl receiver is not an object".to_string(),
        ));
    };
    receiver.set_value("cachedSenderInetAddress", inet_address)?;
    receiver.set_value("cachedSenderPort", Value::Int(i32::from(sender.port())))?;
    receiver.set_value("sender", socket_address)?;
    Ok(Some(Value::Int(i32::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIJZ)I",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn receive_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let connected = parameters.pop_bool()?;
    let sender_addr_ptr = parameters.pop_long()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len)
        .map_err(|e| InternalError(e.to_string()))?
        .min(MAX_PACKET_LEN);

    let vm = thread.vm()?;
    let non_blocking = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        guard.non_blocking
    };

    let result = loop {
        if !non_blocking {
            let status = super::socketdispatcher::wait_raw_socket(&thread, fd, false).await?;
            if status < 0 {
                return Ok(Some(Value::Int(i32::try_from(status)?)));
            }
        }
        let cloned_socket = {
            let guard = vm
                .socket_handles()
                .get(&fd)
                .await
                .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
            let socket = guard
                .socket_type
                .as_raw()
                .ok_or_else(|| InternalError("expected raw socket".to_string()))?;
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("receive: clone: {e}")))?
        };
        let result = tokio::task::spawn_blocking(move || receive_datagram(cloned_socket, count))
            .await
            .map_err(|e| InternalError(format!("receive: spawn: {e}")))?;
        match result {
            Err(error) if !connected && error.kind() == std::io::ErrorKind::ConnectionRefused => {}
            result => break result,
        }
    };

    let (n, data, sender) = match result {
        Ok(result) => result,
        Err(error) => {
            return Ok(Some(Value::Int(i32::try_from(datagram_io_status(
                &error,
            )?)?)));
        }
    };
    if !vm.native_memory().try_write_bytes(address, &data) {
        return Err(InternalError("receive0: invalid native buffer".to_string()));
    }
    write_sockaddr(vm.native_memory(), sender_addr_ptr, &sender)?;
    Ok(Some(Value::Int(i32::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let addr_value = parameters.pop()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let prefer_ipv6 = parameters.pop_bool()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len)
        .map_err(|e| InternalError(e.to_string()))?
        .min(MAX_PACKET_LEN);

    let vm = thread.vm()?;
    let data = native_bytes(
        vm.native_memory(),
        address,
        count,
        "DatagramChannelImpl.send0",
    )?;
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let target = inet_socket_address(&addr_value, prefer_ipv6, port as u16)?;

    let (cloned_socket, non_blocking) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("send: clone: {e}")))?,
            guard.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, true).await?;
        if status < 0 {
            return Ok(Some(Value::Int(i32::try_from(status)?)));
        }
    }

    let result = tokio::task::spawn_blocking(move || cloned_socket.send_to(&data, &target))
        .await
        .map_err(|e| InternalError(format!("send: spawn: {e}")))?;

    match result {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(error) => Ok(Some(Value::Int(i32::try_from(datagram_io_status(
            &error,
        )?)?))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.send0(Ljava/io/FileDescriptor;JIJI)I",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn send_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let target_addr_len = parameters.pop_int()?;
    let target_addr_ptr = parameters.pop_long()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len)
        .map_err(|e| InternalError(e.to_string()))?
        .min(MAX_PACKET_LEN);
    let target_len = usize::try_from(target_addr_len).map_err(|e| InternalError(e.to_string()))?;

    let vm = thread.vm()?;
    let data = native_bytes(
        vm.native_memory(),
        address,
        count,
        "DatagramChannelImpl.send0",
    )?;
    let target = read_sockaddr(vm.native_memory(), target_addr_ptr, target_len)?;

    let (cloned_socket, non_blocking) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("send: clone: {e}")))?,
            guard.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, true).await?;
        if status < 0 {
            return Ok(Some(Value::Int(i32::try_from(status)?)));
        }
    }

    let result = tokio::task::spawn_blocking(move || cloned_socket.send_to(&data, &target))
        .await
        .map_err(|e| InternalError(format!("send: spawn: {e}")))?;

    match result {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(error) => Ok(Some(Value::Int(i32::try_from(datagram_io_status(
            &error,
        )?)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disconnect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_receive_0_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = receive_0_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_receive_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive_0_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_0_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = send_0_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_0_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

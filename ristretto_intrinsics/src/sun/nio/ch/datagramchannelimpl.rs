use crate::java::io::socketfiledescriptor::get_fd;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;

/// Read an IPv4 or IPv6 sockaddr from native memory and return `(ip_bytes, port)`.
/// Returns 4 bytes for IPv4, 16 bytes for IPv6.
#[expect(clippy::unnecessary_wraps)]
fn read_sockaddr(native_mem: &ristretto_types::NativeMemory, addr: i64) -> Result<(Vec<u8>, u16)> {
    // Detect address family from native sockaddr
    #[cfg(target_os = "macos")]
    let family = native_mem.read_bytes(addr + 1, 1)[0];
    #[cfg(not(target_os = "macos"))]
    let family = {
        let fam_bytes = native_mem.read_bytes(addr, 2);
        u16::from_ne_bytes([fam_bytes[0], fam_bytes[1]])
    };

    #[cfg(target_os = "macos")]
    let is_ipv6 = family == 30; // AF_INET6 on macOS
    #[cfg(target_os = "linux")]
    let is_ipv6 = family == 10; // AF_INET6 on Linux
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let is_ipv6 = family == 23; // AF_INET6 on Windows and other platforms

    let port_bytes = native_mem.read_bytes(addr + 2, 2);
    let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);

    if is_ipv6 {
        // IPv6: skip flowinfo (4 bytes at offset 4), read 16 bytes at offset 8
        let ip_bytes = native_mem.read_bytes(addr + 8, 16);
        let ip = Ipv6Addr::from(<[u8; 16]>::try_from(ip_bytes.as_slice()).unwrap_or([0; 16]));
        // Convert IPv4-mapped IPv6 to IPv4
        if let Some(v4) = ip.to_ipv4_mapped() {
            Ok((v4.octets().to_vec(), port))
        } else {
            Ok((ip.octets().to_vec(), port))
        }
    } else {
        let ip_bytes = native_mem.read_bytes(addr + 4, 4);
        Ok((ip_bytes, port))
    }
}

/// Read an IPv4 sockaddr from native memory and return (ip, port).
fn read_sockaddr_v4(
    native_mem: &ristretto_types::NativeMemory,
    addr: i64,
) -> Result<(Ipv4Addr, u16)> {
    let (bytes, port) = read_sockaddr(native_mem, addr)?;
    if bytes.len() == 4 {
        Ok((Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]), port))
    } else {
        // IPv6 bytes, but caller wants IPv4; return unspecified
        Ok((Ipv4Addr::UNSPECIFIED, port))
    }
}

/// Write a sockaddr to native memory, choosing IPv4 or IPv6 format based on socket domain.
fn write_sockaddr(
    native_mem: &ristretto_types::NativeMemory,
    addr: i64,
    sock_addr: &std::net::SocketAddr,
) {
    match sock_addr {
        std::net::SocketAddr::V4(v4) => {
            write_sockaddr_v4(native_mem, addr, *v4.ip(), v4.port());
        }
        std::net::SocketAddr::V6(v6) => {
            // Check if it's an IPv4-mapped IPv6 address
            if let Some(v4) = v6.ip().to_ipv4_mapped() {
                write_sockaddr_v4(native_mem, addr, v4, v6.port());
            } else {
                write_sockaddr_v4(native_mem, addr, Ipv4Addr::UNSPECIFIED, v6.port());
            }
        }
    }
}

/// Write an IPv4 sockaddr to native memory.
fn write_sockaddr_v4(
    native_mem: &ristretto_types::NativeMemory,
    addr: i64,
    ip: Ipv4Addr,
    port: u16,
) {
    // sin_len + sin_family (AF_INET=2) on macOS; sin_family only on Linux
    #[cfg(target_os = "macos")]
    {
        native_mem.write_bytes(addr, &[16u8]); // sin_len = 16
        native_mem.write_bytes(addr + 1, &[2u8]); // sin_family = AF_INET
    }
    #[cfg(not(target_os = "macos"))]
    {
        native_mem.write_bytes(addr, &2u16.to_ne_bytes()); // sin_family = AF_INET
    }
    native_mem.write_bytes(addr + 2, &port.to_be_bytes());
    native_mem.write_bytes(addr + 4, &ip.octets());
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

    tokio::task::spawn_blocking(move || {
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            #[expect(unsafe_code, clippy::cast_possible_truncation)]
            // SAFETY: cloned_socket is a valid socket fd; we connect to AF_UNSPEC to
            // disconnect the UDP socket, matching the JDK native implementation.
            unsafe {
                let mut addr: libc::sockaddr_storage = std::mem::zeroed();
                addr.ss_family = libc::AF_UNSPEC as libc::sa_family_t;
                // On macOS, connect(AF_UNSPEC) may return EAFNOSUPPORT; ignore it
                let _ = libc::connect(
                    cloned_socket.as_raw_fd(),
                    std::ptr::from_ref(&addr).cast::<libc::sockaddr>(),
                    std::mem::size_of::<libc::sockaddr>() as libc::socklen_t,
                );
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawSocket;

            #[repr(C)]
            struct SockAddrStorage {
                ss_family: u16,
                _padding: [u8; 126],
            }

            #[expect(unsafe_code)]
            // SAFETY: ss_family is the first field of the sockaddr structure, and Windows expects
            // the address family to be in the first 2 bytes for connect() even when disconnecting.
            #[link(name = "ws2_32")]
            unsafe extern "system" {
                fn connect(s: u64, name: *const SockAddrStorage, namelen: i32) -> i32;
            }

            #[expect(unsafe_code)]
            // SAFETY: cloned_socket is a valid socket handle; we connect to AF_UNSPEC
            // to disconnect the UDP socket, matching the JDK native implementation.
            unsafe {
                let addr = SockAddrStorage {
                    ss_family: 0, // AF_UNSPEC
                    _padding: [0u8; 126],
                };
                #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                let _ = connect(
                    cloned_socket.as_raw_socket(),
                    std::ptr::from_ref(&addr),
                    std::mem::size_of::<SockAddrStorage>() as i32,
                );
            }
        }
    })
    .await
    .map_err(|e| InternalError(format!("disconnect: spawn: {e}")))?;

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
    let _connected = parameters.pop_bool()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;

    let vm = thread.vm()?;
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
            .map_err(|e| InternalError(format!("receive: clone: {e}")))?
    };

    let result = tokio::task::spawn_blocking(move || {
        let udp: std::net::UdpSocket = cloned_socket.into();
        let mut buf = vec![0u8; count];
        match udp.recv_from(&mut buf) {
            Ok((n, _from_addr)) => {
                buf.truncate(n);
                let n_val = i32::try_from(n).unwrap_or(i32::MAX);
                Ok((n_val, buf))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok((-2, Vec::new())),
            Err(e) => Err(e),
        }
    })
    .await
    .map_err(|e| InternalError(format!("receive: spawn: {e}")))?
    .map_err(|e| InternalError(format!("receive: {e}")))?;

    let (n, data) = result;
    if n > 0 {
        vm.native_memory().write_bytes(address, &data);
    }
    Ok(Some(Value::Int(n)))
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
    let _connected = parameters.pop_bool()?;
    let sender_addr_ptr = parameters.pop_long()?;
    let len = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;

    let vm = thread.vm()?;
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
            .map_err(|e| InternalError(format!("receive: clone: {e}")))?
    };

    let result = tokio::task::spawn_blocking(move || {
        let udp: std::net::UdpSocket = cloned_socket.into();
        let mut buf = vec![0u8; count];
        match udp.recv_from(&mut buf) {
            Ok((n, from_addr)) => {
                buf.truncate(n);
                let n_val = i32::try_from(n).unwrap_or(i32::MAX);
                Ok((n_val, buf, Some(from_addr)))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok((-2, Vec::new(), None)),
            Err(e) => Err(e),
        }
    })
    .await
    .map_err(|e| InternalError(format!("receive: spawn: {e}")))?
    .map_err(|e| InternalError(format!("receive: {e}")))?;

    let (n, data, sender) = result;
    if n > 0 {
        vm.native_memory().write_bytes(address, &data);
    }
    // Write sender address to native sockaddr
    if let Some(sender_addr) = sender {
        write_sockaddr(vm.native_memory(), sender_addr_ptr, &sender_addr);
    }
    Ok(Some(Value::Int(n)))
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
    let _prefer_ipv6 = parameters.pop_bool()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;

    // Extract IPv4 address from InetAddress holder
    let ip = {
        let holder_value = {
            let object = addr_value.as_object_ref()?;
            object.value("holder")?
        };
        let holder = holder_value.as_object_ref()?;
        let addr_int = holder.value("address")?.as_i32()?;
        #[expect(clippy::cast_sign_loss)]
        Ipv4Addr::from(addr_int as u32)
    };

    let vm = thread.vm()?;
    let data = vm.native_memory().read_bytes(address, count);
    let is_ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.is_ipv6);
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let target = if is_ipv6 {
        socket2::SockAddr::from(SocketAddrV6::new(ip.to_ipv6_mapped(), port as u16, 0, 0))
    } else {
        socket2::SockAddr::from(SocketAddrV4::new(ip, port as u16))
    };

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
            .map_err(|e| InternalError(format!("send: clone: {e}")))?
    };

    let n = tokio::task::spawn_blocking(move || cloned_socket.send_to(&data, &target))
        .await
        .map_err(|e| InternalError(format!("send: spawn: {e}")))?
        .map_err(|e| InternalError(format!("send: {e}")))?;

    #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    Ok(Some(Value::Int(n as i32)))
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
    let count = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;
    let _target_len = usize::try_from(target_addr_len).map_err(|e| InternalError(e.to_string()))?;

    let vm = thread.vm()?;
    let data = vm.native_memory().read_bytes(address, count);
    let (ip_bytes, port) = read_sockaddr(vm.native_memory(), target_addr_ptr)?;
    let is_ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.is_ipv6);
    let target = if ip_bytes.len() == 4 {
        let ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
        if is_ipv6 {
            socket2::SockAddr::from(SocketAddrV6::new(ip.to_ipv6_mapped(), port, 0, 0))
        } else {
            socket2::SockAddr::from(SocketAddrV4::new(ip, port))
        }
    } else {
        let ip = Ipv6Addr::from(<[u8; 16]>::try_from(ip_bytes.as_slice()).unwrap_or([0; 16]));
        socket2::SockAddr::from(SocketAddrV6::new(ip, port, 0, 0))
    };

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
            .map_err(|e| InternalError(format!("send: clone: {e}")))?
    };

    let n = tokio::task::spawn_blocking(move || cloned_socket.send_to(&data, &target))
        .await
        .map_err(|e| InternalError(format!("send: spawn: {e}")))?
        .map_err(|e| InternalError(format!("send: {e}")))?;

    let n = i32::try_from(n).map_err(|e| InternalError(e.to_string()))?;
    Ok(Some(Value::Int(n)))
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

use crate::java::io::socketfiledescriptor::{get_fd, set_fd};
use crate::net_helpers::{
    inet_socket_address, ipv4_from_java_int, ipv6_from_java_bytes, socket_from_type,
};
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::cmp::Ordering;
#[cfg(unix)]
use std::net::Ipv4Addr;
use std::net::Shutdown;
#[cfg(target_os = "linux")]
use std::net::{Ipv6Addr, SocketAddrV6};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(unix)]
#[expect(unsafe_code)]
fn poll_socket_once(raw: std::os::fd::RawFd, events: i16, timeout: i32) -> std::io::Result<i32> {
    let mut descriptor = libc::pollfd {
        fd: raw,
        events,
        revents: 0,
    };
    // SAFETY: descriptor points to one initialized pollfd value.
    let result = unsafe { libc::poll(&raw mut descriptor, 1, timeout) };
    match result.cmp(&0) {
        Ordering::Greater => Ok(i32::from(descriptor.revents)),
        Ordering::Equal => Ok(0),
        Ordering::Less => Err(std::io::Error::last_os_error()),
    }
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn poll_socket_once(
    raw: std::os::windows::io::RawSocket,
    events: i16,
    timeout: i32,
) -> std::io::Result<i32> {
    use windows_sys::Win32::Networking::WinSock::{WSAGetLastError, WSAPOLLFD, WSAPoll};
    let mut descriptor = WSAPOLLFD {
        fd: usize::try_from(raw).unwrap_or(usize::MAX),
        events,
        revents: 0,
    };
    // SAFETY: descriptor points to one initialized WSAPOLLFD value.
    let result = unsafe { WSAPoll(&raw mut descriptor, 1, timeout) };
    match result.cmp(&0) {
        Ordering::Greater => Ok(i32::from(descriptor.revents)),
        Ordering::Equal => Ok(0),
        Ordering::Less => {
            // SAFETY: WSAGetLastError has no pointer arguments.
            let code = unsafe { WSAGetLastError() };
            Err(std::io::Error::from_raw_os_error(code))
        }
    }
}

#[cfg(unix)]
#[repr(C)]
#[expect(clippy::struct_field_names)]
struct IpMreqSource {
    multi_address: libc::in_addr,
    interface_address: libc::in_addr,
    source_address: libc::in_addr,
}

#[cfg(unix)]
fn native_ipv4(address: Ipv4Addr) -> libc::in_addr {
    libc::in_addr {
        s_addr: u32::from_ne_bytes(address.octets()),
    }
}

#[cfg(unix)]
#[expect(unsafe_code)]
fn change_source_block(
    socket: &Socket,
    block: bool,
    group: Ipv4Addr,
    interface: Ipv4Addr,
    source: Ipv4Addr,
) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    let request = IpMreqSource {
        multi_address: native_ipv4(group),
        interface_address: native_ipv4(interface),
        source_address: native_ipv4(source),
    };
    let option = if block {
        libc::IP_BLOCK_SOURCE
    } else {
        libc::IP_UNBLOCK_SOURCE
    };
    // SAFETY: request is a correctly laid out ip_mreq_source value.
    let result = unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::IPPROTO_IP,
            option,
            std::ptr::from_ref(&request).cast(),
            libc::socklen_t::try_from(size_of::<IpMreqSource>()).unwrap_or_default(),
        )
    };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn change_source_block(
    socket: &Socket,
    block: bool,
    group: std::net::Ipv4Addr,
    interface: std::net::Ipv4Addr,
    source: std::net::Ipv4Addr,
) -> std::io::Result<()> {
    use std::os::windows::io::AsRawSocket;
    use windows_sys::Win32::Networking::WinSock::{
        IN_ADDR, IN_ADDR_0, IP_BLOCK_SOURCE, IP_MREQ_SOURCE, IP_UNBLOCK_SOURCE, IPPROTO_IP,
        SOCKET_ERROR, WSAGetLastError, setsockopt,
    };

    let address = |address: std::net::Ipv4Addr| IN_ADDR {
        S_un: IN_ADDR_0 {
            S_addr: u32::from_ne_bytes(address.octets()),
        },
    };
    let request = IP_MREQ_SOURCE {
        imr_multiaddr: address(group),
        imr_sourceaddr: address(source),
        imr_interface: address(interface),
    };
    // SAFETY: request is a correctly laid out IP_MREQ_SOURCE value and remains valid for the
    // duration of setsockopt.
    let result = unsafe {
        setsockopt(
            usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
            IPPROTO_IP,
            if block {
                IP_BLOCK_SOURCE
            } else {
                IP_UNBLOCK_SOURCE
            },
            std::ptr::from_ref(&request).cast(),
            i32::try_from(size_of::<IP_MREQ_SOURCE>()).unwrap_or(i32::MAX),
        )
    };
    if result == SOCKET_ERROR {
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { WSAGetLastError() };
        Err(std::io::Error::from_raw_os_error(code))
    } else {
        Ok(())
    }
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct GroupSourceRequest {
    interface: u32,
    group: libc::sockaddr_storage,
    source: libc::sockaddr_storage,
}

#[cfg(target_os = "linux")]
#[expect(unsafe_code)]
fn change_ipv6_source(
    socket: &Socket,
    option: i32,
    group: Ipv6Addr,
    interface: u32,
    source: Ipv6Addr,
) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    let group = SockAddr::from(SocketAddrV6::new(group, 0, 0, interface));
    let source = SockAddr::from(SocketAddrV6::new(source, 0, 0, interface));
    // SAFETY: sockaddr_storage is valid when zero initialized, and each SockAddr
    // is copied using its kernel-provided length into equally sized storage.
    let mut request: GroupSourceRequest = unsafe { std::mem::zeroed() };
    request.interface = interface;
    unsafe {
        std::ptr::copy_nonoverlapping(
            group.as_ptr().cast::<u8>(),
            std::ptr::from_mut(&mut request.group).cast::<u8>(),
            usize::try_from(group.len()).unwrap_or_default(),
        );
        std::ptr::copy_nonoverlapping(
            source.as_ptr().cast::<u8>(),
            std::ptr::from_mut(&mut request.source).cast::<u8>(),
            usize::try_from(source.len()).unwrap_or_default(),
        );
    }
    // SAFETY: request matches Linux's group_source_req ABI.
    let result = unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::IPPROTO_IPV6,
            option,
            std::ptr::from_ref(&request).cast(),
            libc::socklen_t::try_from(size_of::<GroupSourceRequest>()).unwrap_or_default(),
        )
    };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

async fn finish_connect<V: VM>(vm: &V, fd: i32) -> Result<bool> {
    let ready = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Ok(true);
        };
        if let Some(error) = socket
            .take_error()
            .map_err(|error| ristretto_types::JavaError::ConnectException(error.to_string()))?
        {
            return Err(ristretto_types::JavaError::ConnectException(error.to_string()).into());
        }
        socket.peer_addr().is_ok()
    };
    if !ready {
        return Ok(false);
    }

    let handle = vm
        .socket_handles()
        .remove(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let SocketHandle {
        socket_type,
        timeout,
        is_ipv6,
        is_unix,
        non_blocking,
        lifecycle,
    } = handle;
    let SocketType::Raw(socket) = socket_type else {
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle {
                    socket_type,
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            )
            .await?;
        return Ok(true);
    };
    if is_unix || socket.r#type().ok() != Some(Type::STREAM) {
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle {
                    socket_type: SocketType::Raw(socket),
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            )
            .await?;
        return Ok(true);
    }
    socket
        .set_nonblocking(true)
        .map_err(|error| InternalError(format!("connect: set_nonblocking: {error}")))?;
    let stream = tokio::net::TcpStream::from_std(socket.into())
        .map_err(|error| InternalError(format!("connect: from_std: {error}")))?;
    vm.socket_handles()
        .insert(
            fd,
            SocketHandle {
                socket_type: SocketType::TcpStream(Arc::new(stream)),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        )
        .await?;
    Ok(true)
}

pub(crate) async fn check_connect<V: VM>(vm: &V, fd: i32, block: bool) -> Result<i32> {
    if finish_connect(vm, fd).await? {
        return Ok(1);
    }
    if !block {
        return Ok(-2);
    }
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return Ok(super::IOS_INTERRUPTED);
        };
        #[cfg(unix)]
        let raw_fd = handle.socket_type.raw_fd();
        #[cfg(windows)]
        let raw_fd = usize::try_from(handle.socket_type.raw_socket()).unwrap_or(usize::MAX);
        drop(handle);
        let poll_result = tokio::task::spawn_blocking(move || {
            #[cfg(unix)]
            {
                let mut descriptor = libc::pollfd {
                    fd: raw_fd,
                    events: libc::POLLOUT,
                    revents: 0,
                };
                #[expect(unsafe_code)]
                // SAFETY: descriptor points to one live pollfd record.
                let result = unsafe { libc::poll(&raw mut descriptor, 1, 100) };
                if result == -1 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(result > 0)
                }
            }
            #[cfg(windows)]
            {
                let mut descriptor = windows_sys::Win32::Networking::WinSock::WSAPOLLFD {
                    fd: raw_fd,
                    events: windows_sys::Win32::Networking::WinSock::POLLOUT,
                    revents: 0,
                };
                #[expect(unsafe_code)]
                // SAFETY: descriptor points to one live WSAPOLLFD record.
                let result = unsafe {
                    windows_sys::Win32::Networking::WinSock::WSAPoll(&raw mut descriptor, 1, 100)
                };
                if result == -1 {
                    #[expect(unsafe_code)]
                    // SAFETY: WSAGetLastError has no pointer arguments.
                    let code =
                        unsafe { windows_sys::Win32::Networking::WinSock::WSAGetLastError() };
                    Err(std::io::Error::from_raw_os_error(code))
                } else {
                    Ok(result > 0)
                }
            }
        })
        .await
        .map_err(|error| InternalError(format!("connect poll task failed: {error}")))?;
        match poll_result {
            Ok(true) if finish_connect(vm, fd).await? => return Ok(1),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                return Ok(super::IOS_INTERRUPTED);
            }
            Err(error) => {
                return Err(ristretto_types::JavaError::ConnectException(error.to_string()).into());
            }
        }
    }
}

/// Platform-specific socket option constants (macOS / BSD / Windows).
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod sock_const {
    pub const SOL_SOCKET: i32 = 0xFFFF;
    pub const SO_REUSEADDR: i32 = 0x0004;
    pub const SO_KEEPALIVE: i32 = 0x0008;
    pub const SO_LINGER: i32 = 0x0080;
    pub const SO_OOBINLINE: i32 = 0x0100;
    pub const SO_SNDBUF: i32 = 0x1001;
    pub const SO_RCVBUF: i32 = 0x1002;
    pub const IPPROTO_TCP: i32 = 6;
    pub const TCP_NODELAY: i32 = 0x0001;
}

/// Platform-specific socket option constants (Linux; also used as default for other platforms).
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod sock_const {
    pub const SOL_SOCKET: i32 = 1;
    pub const SO_REUSEADDR: i32 = 2;
    pub const SO_KEEPALIVE: i32 = 9;
    pub const SO_OOBINLINE: i32 = 10;
    pub const SO_LINGER: i32 = 13;
    pub const SO_SNDBUF: i32 = 7;
    pub const SO_RCVBUF: i32 = 8;
    pub const IPPROTO_TCP: i32 = 6;
    pub const TCP_NODELAY: i32 = 1;
}

#[cfg(unix)]
#[expect(unsafe_code)]
fn get_raw_socket_option(socket: &Socket, level: i32, option: i32) -> std::io::Result<i32> {
    use std::os::fd::AsRawFd;
    let mut value = 0_i32;
    let mut length = libc::socklen_t::try_from(size_of::<i32>()).unwrap_or_default();
    // SAFETY: value and length point to writable storage of the declared size.
    let result = unsafe {
        libc::getsockopt(
            socket.as_raw_fd(),
            level,
            option,
            std::ptr::from_mut(&mut value).cast(),
            &raw mut length,
        )
    };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn get_raw_socket_option(socket: &Socket, level: i32, option: i32) -> std::io::Result<i32> {
    use std::os::windows::io::AsRawSocket;
    let mut value = 0_i32;
    let mut length = i32::try_from(size_of::<i32>()).unwrap_or_default();
    // SAFETY: value and length point to writable storage of the declared size.
    let result = unsafe {
        windows_sys::Win32::Networking::WinSock::getsockopt(
            usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
            level,
            option,
            std::ptr::from_mut(&mut value).cast(),
            &raw mut length,
        )
    };
    if result != 0 {
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { windows_sys::Win32::Networking::WinSock::WSAGetLastError() };
        Err(std::io::Error::from_raw_os_error(code))
    } else {
        Ok(value)
    }
}

#[cfg(unix)]
#[expect(unsafe_code)]
fn set_raw_socket_option(
    socket: &Socket,
    level: i32,
    option: i32,
    value: i32,
) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    // SAFETY: value points to a readable i32 of the declared size.
    let result = unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            level,
            option,
            std::ptr::from_ref(&value).cast(),
            libc::socklen_t::try_from(size_of::<i32>()).unwrap_or_default(),
        )
    };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn set_raw_socket_option(
    socket: &Socket,
    level: i32,
    option: i32,
    value: i32,
) -> std::io::Result<()> {
    use std::os::windows::io::AsRawSocket;
    // SAFETY: value points to a readable i32 of the declared size.
    let result = unsafe {
        windows_sys::Win32::Networking::WinSock::setsockopt(
            usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
            level,
            option,
            std::ptr::from_ref(&value).cast::<u8>(),
            i32::try_from(size_of::<i32>()).unwrap_or_default(),
        )
    };
    if result != 0 {
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { windows_sys::Win32::Networking::WinSock::WSAGetLastError() };
        Err(std::io::Error::from_raw_os_error(code))
    } else {
        Ok(())
    }
}

fn get_socket_option(socket: &Socket, level: i32, opt: i32) -> Result<i32> {
    use sock_const::{
        IPPROTO_TCP, SO_KEEPALIVE, SO_LINGER, SO_OOBINLINE, SO_RCVBUF, SO_REUSEADDR, SO_SNDBUF,
        SOL_SOCKET, TCP_NODELAY,
    };
    let result = match (level, opt) {
        (SOL_SOCKET, SO_REUSEADDR) => socket.reuse_address().map(i32::from),
        (SOL_SOCKET, SO_KEEPALIVE) => socket.keepalive().map(i32::from),
        #[expect(clippy::cast_possible_truncation)]
        (SOL_SOCKET, SO_LINGER) => socket
            .linger()
            .map(|value| value.map_or(-1, |duration| duration.as_secs() as i32)),
        (SOL_SOCKET, SO_OOBINLINE) => socket.out_of_band_inline().map(i32::from),
        #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        (SOL_SOCKET, SO_SNDBUF) => socket.send_buffer_size().map(|v| v as i32),
        #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        (SOL_SOCKET, SO_RCVBUF) => socket.recv_buffer_size().map(|v| v as i32),
        (IPPROTO_TCP, TCP_NODELAY) => socket.tcp_nodelay().map(i32::from),
        _ => get_raw_socket_option(socket, level, opt),
    };
    result.map_err(|error| {
        ristretto_types::JavaError::SocketException(format!(
            "get_socket_option(level={level}, opt={opt}): {error}"
        ))
        .into()
    })
}

#[expect(clippy::cast_sign_loss)]
fn set_socket_option(socket: &Socket, level: i32, opt: i32, value: i32) -> Result<()> {
    use sock_const::{
        IPPROTO_TCP, SO_KEEPALIVE, SO_LINGER, SO_OOBINLINE, SO_RCVBUF, SO_REUSEADDR, SO_SNDBUF,
        SOL_SOCKET, TCP_NODELAY,
    };
    let result = match (level, opt) {
        (SOL_SOCKET, SO_REUSEADDR) => socket.set_reuse_address(value != 0),
        (SOL_SOCKET, SO_KEEPALIVE) => socket.set_keepalive(value != 0),
        (SOL_SOCKET, SO_LINGER) => {
            if value >= 0 {
                socket.set_linger(Some(Duration::from_secs(u64::from(value as u32))))
            } else {
                socket.set_linger(None)
            }
        }
        (SOL_SOCKET, SO_OOBINLINE) => socket.set_out_of_band_inline(value != 0),
        (SOL_SOCKET, SO_SNDBUF) => socket.set_send_buffer_size(value as usize),
        (SOL_SOCKET, SO_RCVBUF) => socket.set_recv_buffer_size(value as usize),
        (IPPROTO_TCP, TCP_NODELAY) => socket.set_tcp_nodelay(value != 0),
        _ => set_raw_socket_option(socket, level, opt, value),
    };
    result.map_err(|error| {
        ristretto_types::JavaError::SocketException(format!(
            "set_socket_option(level={level}, opt={opt}, value={value}): {error}"
        ))
        .into()
    })
}

fn accept_status(error: &std::io::Error) -> Result<i32> {
    match error.kind() {
        std::io::ErrorKind::WouldBlock => Ok(super::IOS_UNAVAILABLE),
        std::io::ErrorKind::Interrupted => Ok(super::IOS_INTERRUPTED),
        _ => Err(ristretto_types::JavaError::SocketException(error.to_string()).into()),
    }
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

#[cfg(windows)]
#[expect(unsafe_code)]
pub(crate) fn connect_windows_socket(
    socket: std::os::windows::io::RawSocket,
    address: &SockAddr,
) -> std::io::Result<()> {
    use windows_sys::Win32::Networking::WinSock::{SOCKET_ERROR, WSAGetLastError, connect};

    // SAFETY: address owns a fully initialized socket address for address.len() bytes, and the
    // raw socket remains owned by the VM handle table for the duration of this call.
    let result = unsafe {
        connect(
            usize::try_from(socket).unwrap_or(usize::MAX),
            address.as_ptr().cast(),
            address.len(),
        )
    };
    if result == SOCKET_ERROR {
        // SAFETY: WSAGetLastError has no pointer arguments.
        let code = unsafe { WSAGetLastError() };
        Err(std::io::Error::from_raw_os_error(code))
    } else {
        Ok(())
    }
}

fn is_windows_invalid_argument(error: &std::io::Error) -> bool {
    #[cfg(windows)]
    return error.raw_os_error() == Some(windows_sys::Win32::Networking::WinSock::WSAEINVAL);
    #[cfg(not(windows))]
    {
        let _ = error;
        false
    }
}

fn multicast_unsupported(error: &std::io::Error) -> bool {
    if error.kind() == std::io::ErrorKind::Unsupported {
        return true;
    }
    let Some(code) = error.raw_os_error() else {
        return false;
    };
    #[cfg(unix)]
    if code == libc::ENOPROTOOPT || code == libc::EOPNOTSUPP {
        return true;
    }
    #[cfg(windows)]
    if code == windows_sys::Win32::Networking::WinSock::WSAENOPROTOOPT
        || code == windows_sys::Win32::Networking::WinSock::WSAEOPNOTSUPP
    {
        return true;
    }
    false
}

fn multicast_status(result: std::io::Result<()>, unavailable_is_status: bool) -> Result<i32> {
    match result {
        Ok(()) => Ok(0),
        Err(error) if unavailable_is_status && multicast_unsupported(&error) => {
            Ok(super::IOS_UNAVAILABLE)
        }
        Err(error) => Err(ristretto_types::JavaError::SocketException(error.to_string()).into()),
    }
}

async fn inet_address_value<T: Thread + 'static>(
    thread: &Arc<T>,
    address: &SockAddr,
) -> Result<Value> {
    let vm = thread.vm()?;
    let (bytes, scope_id) = if let Some(address) = address.as_socket_ipv4() {
        (address.ip().octets().to_vec(), 0)
    } else if let Some(address) = address.as_socket_ipv6() {
        if let Some(mapped) = address.ip().to_ipv4_mapped() {
            (mapped.octets().to_vec(), 0)
        } else {
            (address.ip().octets().to_vec(), address.scope_id())
        }
    } else {
        (vec![0, 0, 0, 0], 0)
    };
    #[expect(clippy::cast_possible_wrap)]
    let bytes: Box<[i8]> = bytes.into_iter().map(|byte| byte as i8).collect();
    let bytes = Value::new_object(vm.garbage_collector(), Reference::ByteArray(bytes));
    let result = if scope_id == 0 {
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
    };
    result.ok_or_else(|| InternalError("InetAddress.getByAddress returned null".to_string()))
}

async fn socket_operation_interrupted<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
) -> Result<bool> {
    let vm = thread.vm()?;
    if vm.socket_handles().get(&fd).await.is_none() {
        return Ok(true);
    }
    #[cfg(target_family = "unix")]
    if super::nativethread::take_signal(&**thread)? {
        return Ok(true);
    }
    Ok(false)
}

#[intrinsic_method(
    "sun/nio/ch/Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn accept<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let isas = parameters.pop()?;
    let new_fd_value = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;

    // Check what type of socket this is and its non-blocking mode, then release the lock
    let (is_tcp_listener, is_nonblocking, is_ipv6) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        (
            guard.socket_type.as_tcp_listener().is_some(),
            guard.non_blocking,
            guard.is_ipv6,
        )
    };

    let (tokio_stream, addr) = if is_tcp_listener {
        if is_nonblocking {
            // Non-blocking: use raw fd to do immediate accept without tokio
            let guard = vm
                .socket_handles()
                .get(&fd)
                .await
                .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
            let sock = socket_from_type(&guard.socket_type);
            drop(guard);

            let accepted = loop {
                match sock.accept() {
                    Err(error) if error.kind() == std::io::ErrorKind::ConnectionAborted => {}
                    result => break result,
                }
            };
            match accepted {
                Ok((new_socket, addr)) => {
                    new_socket
                        .set_nonblocking(true)
                        .map_err(|e| InternalError(format!("accept: set_nonblocking: {e}")))?;
                    let std_stream: std::net::TcpStream = new_socket.into();
                    let stream = tokio::net::TcpStream::from_std(std_stream)
                        .map_err(|e| InternalError(format!("accept: from_std: {e}")))?;
                    (Some(stream), addr)
                }
                Err(error) => return Ok(Some(Value::Int(accept_status(&error)?))),
            }
        } else {
            // Blocking: clone the Arc and accept without removing from the map
            let listener = {
                let guard = vm
                    .socket_handles()
                    .get(&fd)
                    .await
                    .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
                guard
                    .socket_type
                    .as_tcp_listener()
                    .ok_or_else(|| InternalError("expected TcpListener".to_string()))?
                    .clone()
            };

            let result = loop {
                match tokio::time::timeout(Duration::from_millis(100), listener.accept()).await {
                    Ok(Err(error)) if error.kind() == std::io::ErrorKind::ConnectionAborted => {}
                    Ok(result) => break result,
                    Err(_) => {
                        if socket_operation_interrupted(&thread, fd).await? {
                            return Ok(Some(Value::Int(super::IOS_INTERRUPTED)));
                        }
                    }
                }
            };

            match result {
                Ok((stream, peer_addr)) => {
                    let addr = SockAddr::from(peer_addr);
                    (Some(stream), addr)
                }
                Err(error) => return Ok(Some(Value::Int(accept_status(&error)?))),
            }
        }
    } else {
        // Raw socket fallback
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError(
                "expected TcpListener or Raw socket for accept".to_string(),
            ));
        };
        let cloned = socket
            .try_clone()
            .map_err(|e| InternalError(format!("accept: clone: {e}")))?;
        drop(guard);

        let accept_result = tokio::task::spawn_blocking(move || {
            loop {
                match cloned.accept() {
                    Err(error) if error.kind() == std::io::ErrorKind::ConnectionAborted => {}
                    result => break result,
                }
            }
        })
        .await
        .map_err(|e| InternalError(format!("accept: spawn: {e}")))?;

        match accept_result {
            Ok((new_socket, addr)) => {
                new_socket
                    .set_nonblocking(true)
                    .map_err(|e| InternalError(format!("accept: set_nonblocking: {e}")))?;
                let std_stream: std::net::TcpStream = new_socket.into();
                let stream = tokio::net::TcpStream::from_std(std_stream)
                    .map_err(|e| InternalError(format!("accept: from_std: {e}")))?;
                (Some(stream), addr)
            }
            Err(error) => return Ok(Some(Value::Int(accept_status(&error)?))),
        }
    };

    let new_fd = vm.next_nio_fd();
    set_fd(&new_fd_value, new_fd)?;

    if let Some(stream) = tokio_stream {
        let mut handle = SocketHandle::new(SocketType::TcpStream(Arc::new(stream)));
        handle.is_ipv6 = is_ipv6;
        vm.socket_handles().insert(new_fd, handle).await?;
    }

    // Fill in isas[0] with the remote address as InetSocketAddress
    let port = addr.as_socket().map_or(0, |address| address.port());
    let inet_addr = inet_address_value(&thread, &addr).await?;
    let port_value = Value::Int(i32::from(port));
    let isa = thread
        .object(
            "java.net.InetSocketAddress",
            "Ljava/net/InetAddress;I",
            &[inet_addr, port_value],
        )
        .await?;
    // Store in isas[0]
    let mut guard = isas.as_reference_mut()?;
    if let Reference::Array(object_array) = &mut *guard
        && let Some(first) = object_array.elements.first_mut()
    {
        *first = isa;
    }

    // The descriptor is returned through new_fd_value. The native status is
    // the number of accepted sockets, which is always one on success.
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.available(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn available<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    #[cfg(unix)]
    let available = {
        let mut available = 0_i32;
        #[expect(unsafe_code)]
        // SAFETY: available points to writable storage for FIONREAD.
        if unsafe {
            libc::ioctl(
                handle.socket_type.raw_fd(),
                libc::FIONREAD,
                &raw mut available,
            )
        } == -1
        {
            return Err(ristretto_types::JavaError::IoException(
                std::io::Error::last_os_error().to_string(),
            )
            .into());
        }
        available
    };
    #[cfg(windows)]
    let available = {
        let mut available = 0_u32;
        #[expect(unsafe_code)]
        // SAFETY: available points to writable storage for FIONREAD.
        if unsafe {
            windows_sys::Win32::Networking::WinSock::ioctlsocket(
                usize::try_from(handle.socket_type.raw_socket()).unwrap_or(usize::MAX),
                windows_sys::Win32::Networking::WinSock::FIONREAD,
                &raw mut available,
            )
        } != 0
        {
            #[expect(unsafe_code)]
            // SAFETY: WSAGetLastError has no pointer arguments.
            let code = unsafe { windows_sys::Win32::Networking::WinSock::WSAGetLastError() };
            return Err(ristretto_types::JavaError::IoException(
                std::io::Error::from_raw_os_error(code).to_string(),
            )
            .into());
        }
        i32::try_from(available).unwrap_or(i32::MAX)
    };
    Ok(Some(Value::Int(available)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V",
    Any
)]
#[async_method]
pub async fn bind_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let inet_addr_obj = parameters.pop()?;
    let _exclusive_bind = parameters.pop_bool()?;
    let prefer_ipv6 = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = inet_socket_address(&inet_addr_obj, prefer_ipv6, port as u16)?;
    if let Err(error) = socket.bind(&addr) {
        return Err(ristretto_types::JavaError::BindException(format!(
            "Net.bind0(addr={addr:?}): {error}"
        ))
        .into());
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn block_or_unblock_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop_int()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let block = parameters.pop_int()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    let _ = (source, interf, group, block, &socket);
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let result = change_source_block(
        &socket,
        block != 0,
        ipv4_from_java_int(group),
        ipv4_from_java_int(interf),
        ipv4_from_java_int(source),
    );
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    let result = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "IPv4 multicast source filtering is unavailable",
    ));
    Ok(Some(Value::Int(multicast_status(result, true)?)))
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn block_or_unblock_6<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let block = parameters.pop_int()?;
    let group = {
        let bytes = group.as_byte_vec_ref()?;
        ipv6_from_java_bytes(&bytes).ok_or_else(|| {
            InternalError("IPv6 multicast group must contain 16 bytes".to_string())
        })?
    };
    let source = {
        let bytes = source.as_byte_vec_ref()?;
        ipv6_from_java_bytes(&bytes).ok_or_else(|| {
            InternalError("IPv6 multicast source must contain 16 bytes".to_string())
        })?
    };
    let interface = u32::try_from(interf)
        .map_err(|_| InternalError("negative IPv6 interface index".to_string()))?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    #[cfg(not(target_os = "linux"))]
    let _ = (group, source, interface, block, &socket);
    #[cfg(target_os = "linux")]
    let result = change_ipv6_source(
        &socket,
        if block != 0 {
            libc::MCAST_BLOCK_SOURCE
        } else {
            libc::MCAST_UNBLOCK_SOURCE
        },
        group,
        interface,
        source,
    );
    #[cfg(not(target_os = "linux"))]
    let result = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "IPv6 multicast source filtering is unavailable",
    ));
    Ok(Some(Value::Int(multicast_status(result, true)?)))
}

#[intrinsic_method("sun/nio/ch/Net.canIPv6SocketJoinIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_ipv6_socket_join_ipv4_group_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(cfg!(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows"
    )))))
}

#[intrinsic_method("sun/nio/ch/Net.canJoin6WithIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_join_6_with_ipv4_group_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(cfg!(target_os = "macos"))))
}

#[intrinsic_method(
    "sun/nio/ch/Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn can_use_ipv6_options_with_ipv4_local_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(cfg!(target_family = "unix"))))
}

#[intrinsic_method(
    "sun/nio/ch/Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I",
    Any
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let inet_addr_obj = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let prefer_ipv6 = parameters.pop_bool()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = inet_socket_address(&inet_addr_obj, prefer_ipv6, port as u16)?;
    #[cfg(not(windows))]
    let (result, non_blocking) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let socket = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| InternalError("expected raw socket for connect".to_string()))?;
        let non_blocking = handle.non_blocking;
        if !non_blocking {
            // NIO's blocking semantics are implemented around a non-blocking native
            // connect so that closing the channel can interrupt the operation. Configure
            // and connect the managed socket itself: connecting a duplicated Winsock
            // handle does not reliably propagate the pending connection on Windows ARM.
            socket.set_nonblocking(true).map_err(|error| {
                ristretto_types::JavaError::SocketException(format!(
                    "connect configure non-blocking: {error}"
                ))
            })?;
        }
        (socket.connect(&addr), non_blocking)
    };
    #[cfg(windows)]
    let (result, non_blocking) = {
        use std::os::windows::io::AsRawSocket;

        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let socket = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| InternalError("expected raw socket for connect".to_string()))?;
        let raw_socket = socket.as_raw_socket();
        let non_blocking = handle.non_blocking;
        drop(handle);
        let result = if non_blocking {
            connect_windows_socket(raw_socket, &addr)
        } else {
            tokio::task::spawn_blocking(move || connect_windows_socket(raw_socket, &addr))
                .await
                .map_err(|error| InternalError(format!("connect task failed: {error}")))?
        };
        (result, non_blocking)
    };
    match result {
        Ok(()) => {
            if finish_connect(&*vm, fd).await? {
                Ok(Some(Value::Int(1)))
            } else if non_blocking {
                Ok(Some(Value::Int(super::IOS_UNAVAILABLE)))
            } else {
                Ok(Some(Value::Int(check_connect(&*vm, fd, true).await?)))
            }
        }
        Err(error) if connect_pending(&error) && non_blocking => {
            Ok(Some(Value::Int(super::IOS_UNAVAILABLE)))
        }
        Err(error) if connect_pending(&error) => {
            Ok(Some(Value::Int(check_connect(&*vm, fd, true).await?)))
        }
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            if !non_blocking
                && let Some(handle) = vm.socket_handles().get(&fd).await
                && let Some(socket) = handle.socket_type.as_raw()
            {
                let _ = socket.set_nonblocking(false);
            }
            Ok(Some(Value::Int(super::IOS_INTERRUPTED)))
        }
        Err(error) => {
            if !non_blocking
                && let Some(handle) = vm.socket_handles().get(&fd).await
                && let Some(socket) = handle.socket_type.as_raw()
            {
                let _ = socket.set_nonblocking(false);
            }
            Err(ristretto_types::JavaError::ConnectException(error.to_string()).into())
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/Net.discardOOB(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn discard_oob<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    let mut byte = [std::mem::MaybeUninit::<u8>::uninit(); 1];
    match socket.recv_out_of_band(&mut byte) {
        Ok(0) => Ok(Some(Value::from(false))),
        Ok(_) => Ok(Some(Value::from(true))),
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            Ok(Some(Value::from(false)))
        }
        Err(error) => Err(ristretto_types::JavaError::IoException(error.to_string()).into()),
    }
}

#[intrinsic_method("sun/nio/ch/Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I", Any)]
#[async_method]
pub async fn get_int_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let opt = parameters.pop_int()?;
    let level = parameters.pop_int()?;
    let _may_need_conversion = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);
    let value = get_socket_option(&socket, level, opt)?;
    Ok(Some(Value::Int(value)))
}

#[intrinsic_method("sun/nio/ch/Net.getInterface4(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn get_interface_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    let address = socket
        .multicast_if_v4()
        .map_err(|error| ristretto_types::JavaError::SocketException(error.to_string()))?;
    Ok(Some(Value::Int(i32::from_be_bytes(address.octets()))))
}

#[intrinsic_method("sun/nio/ch/Net.getInterface6(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn get_interface_6<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    let index = socket
        .multicast_if_v6()
        .map_err(|error| ristretto_types::JavaError::SocketException(error.to_string()))?;
    Ok(Some(Value::Int(i32::try_from(index).unwrap_or(i32::MAX))))
}

#[intrinsic_method("sun/nio/ch/Net.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.isExclusiveBindAvailable()I", Any)]
#[async_method]
pub async fn is_exclusive_bind_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Windows supports SO_EXCLUSIVEADDRUSE; on Unix-like systems exclusive bind is unavailable.
    let value = if cfg!(target_os = "windows") { 1 } else { -1 };
    Ok(Some(Value::Int(value)))
}

#[intrinsic_method("sun/nio/ch/Net.isIPv6Available0()Z", Any)]
#[async_method]
pub async fn is_ipv6_available_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(
        Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)).is_ok(),
    )))
}

#[intrinsic_method("sun/nio/ch/Net.isReusePortAvailable0()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_reuse_port_available_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let available = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .and_then(|socket| socket.set_reuse_port(true))
        .is_ok();
    #[cfg(not(unix))]
    let available = false;
    Ok(Some(Value::from(available)))
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn join_or_drop_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop_int()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let join = parameters.pop_int()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    let group = ipv4_from_java_int(group);
    let interface = ipv4_from_java_int(interf);
    let result = if source == 0 {
        if join != 0 {
            socket.join_multicast_v4(&group, &interface)
        } else {
            socket.leave_multicast_v4(&group, &interface)
        }
    } else {
        let source = ipv4_from_java_int(source);
        if join != 0 {
            socket.join_ssm_v4(&source, &group, &interface)
        } else {
            socket.leave_ssm_v4(&source, &group, &interface)
        }
    };
    Ok(Some(Value::Int(multicast_status(result, join != 0)?)))
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn join_or_drop_6<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let join = parameters.pop_int()?;
    let group = {
        let bytes = group.as_byte_vec_ref()?;
        ipv6_from_java_bytes(&bytes).ok_or_else(|| {
            InternalError("IPv6 multicast group must contain 16 bytes".to_string())
        })?
    };
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    let interface = u32::try_from(interf)
        .map_err(|_| InternalError("negative IPv6 interface index".to_string()))?;
    let result = if matches!(source, Value::Object(None)) {
        if join != 0 {
            socket.join_multicast_v6(&group, interface)
        } else {
            socket.leave_multicast_v6(&group, interface)
        }
    } else {
        let source = source.as_byte_vec_ref()?;
        let source = ipv6_from_java_bytes(&source).ok_or_else(|| {
            InternalError("IPv6 multicast source must contain 16 bytes".to_string())
        })?;
        #[cfg(target_os = "linux")]
        {
            change_ipv6_source(
                &socket,
                if join != 0 {
                    libc::MCAST_JOIN_SOURCE_GROUP
                } else {
                    libc::MCAST_LEAVE_SOURCE_GROUP
                },
                group,
                interface,
                source,
            )
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = source;
            Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "IPv6 source-specific multicast is unavailable on this platform backend",
            ))
        }
    };
    Ok(Some(Value::Int(multicast_status(result, join != 0)?)))
}

#[intrinsic_method("sun/nio/ch/Net.listen(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn listen<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let backlog = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;

    // Remove raw socket, listen, then convert to TcpListener
    let raw_handle = vm
        .socket_handles()
        .remove(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let SocketHandle {
        socket_type,
        timeout,
        is_ipv6,
        is_unix,
        non_blocking,
        lifecycle,
    } = raw_handle;
    let SocketType::Raw(socket) = socket_type else {
        return Err(InternalError("expected raw socket for listen".to_string()));
    };

    if let Err(error) = socket.listen(backlog) {
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle {
                    socket_type: SocketType::Raw(socket),
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            )
            .await?;
        return Err(ristretto_types::JavaError::SocketException(error.to_string()).into());
    }

    if is_unix {
        socket
            .set_nonblocking(non_blocking)
            .map_err(|e| InternalError(format!("listen: set_nonblocking: {e}")))?;
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle {
                    socket_type: SocketType::Raw(socket),
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            )
            .await?;
        return Ok(None);
    }

    // Convert Internet stream sockets to a Tokio listener.
    socket
        .set_nonblocking(true)
        .map_err(|e| InternalError(format!("listen: set_nonblocking: {e}")))?;
    let std_listener: std::net::TcpListener = socket.into();
    let tokio_listener = tokio::net::TcpListener::from_std(std_listener)
        .map_err(|e| InternalError(format!("listen: from_std: {e}")))?;
    vm.socket_handles()
        .insert(
            fd,
            SocketHandle {
                socket_type: SocketType::TcpListener(Arc::new(tokio_listener)),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        )
        .await?;

    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub async fn local_inet_address<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let addr = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let socket = socket_from_type(&guard.socket_type);
        socket
            .local_addr()
            .map_err(|error| ristretto_types::JavaError::SocketException(error.to_string()))?
    };
    Ok(Some(inet_address_value(&thread, &addr).await?))
}

#[intrinsic_method("sun/nio/ch/Net.localPort(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn local_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);
    let addr = match socket.local_addr() {
        Ok(a) => Some(a),
        // On Windows, getsockname() returns WSAEINVAL (10022) for unbound sockets.
        Err(error) if is_windows_invalid_argument(&error) => None,
        Err(error) => {
            return Err(ristretto_types::JavaError::SocketException(error.to_string()).into());
        }
    };
    let port = addr
        .as_ref()
        .and_then(SockAddr::as_socket)
        .map_or(0, |a: std::net::SocketAddr| i32::from(a.port()));
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method("sun/nio/ch/Net.poll(Ljava/io/FileDescriptor;IJ)I", Any)]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_long()?;
    let events = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;

    // Get the raw fd for the OS-level poll
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    #[cfg(unix)]
    let raw_handle = guard.socket_type.raw_fd();
    #[cfg(windows)]
    let raw_handle = guard.socket_type.raw_socket();
    drop(guard);

    // Use spawn_blocking to do OS-level poll syscall
    let timeout_ms = if timeout < -1 {
        -1
    } else {
        i32::try_from(timeout).unwrap_or(i32::MAX)
    };

    #[expect(clippy::cast_possible_truncation)]
    let poll_events = events as i16;
    let deadline = if timeout_ms < 0 {
        None
    } else {
        Instant::now().checked_add(Duration::from_millis(
            u64::try_from(timeout_ms).unwrap_or_default(),
        ))
    };
    loop {
        let interval = deadline.map_or(100, |deadline| {
            i32::try_from(
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis()
                    .min(100),
            )
            .unwrap_or(100)
        });
        let result = tokio::task::spawn_blocking(move || {
            poll_socket_once(raw_handle, poll_events, interval)
        })
        .await
        .map_err(|error| InternalError(format!("poll task failed: {error}")))?;
        match result {
            Ok(ready) if ready != 0 => return Ok(Some(Value::Int(ready))),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                return Ok(Some(Value::Int(0)));
            }
            Err(error) => {
                return Err(ristretto_types::JavaError::SocketException(error.to_string()).into());
            }
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline)
            || socket_operation_interrupted(&thread, fd).await?
        {
            return Ok(Some(Value::Int(0)));
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/Net.pollConnect(Ljava/io/FileDescriptor;J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn poll_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    if finish_connect(&*vm, fd).await? {
        return Ok(Some(Value::from(true)));
    }
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    #[cfg(unix)]
    let raw = handle.socket_type.raw_fd();
    #[cfg(windows)]
    let raw = handle.socket_type.raw_socket();
    drop(handle);
    let timeout = if timeout < -1 {
        -1
    } else {
        i32::try_from(timeout).unwrap_or(i32::MAX)
    };
    let deadline = if timeout < 0 {
        None
    } else {
        Instant::now().checked_add(Duration::from_millis(
            u64::try_from(timeout).unwrap_or_default(),
        ))
    };
    #[cfg(unix)]
    let events = libc::POLLOUT;
    #[cfg(windows)]
    let events = windows_sys::Win32::Networking::WinSock::POLLOUT;
    loop {
        let interval = deadline.map_or(100, |deadline| {
            i32::try_from(
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis()
                    .min(100),
            )
            .unwrap_or(100)
        });
        let result = tokio::task::spawn_blocking(move || poll_socket_once(raw, events, interval))
            .await
            .map_err(|error| InternalError(format!("pollConnect task failed: {error}")))?;
        match result {
            Ok(ready) if ready != 0 && finish_connect(&*vm, fd).await? => {
                return Ok(Some(Value::from(true)));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                return Ok(Some(Value::from(false)));
            }
            Err(error) => {
                return Err(ristretto_types::JavaError::IoException(error.to_string()).into());
            }
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline)
            || socket_operation_interrupted(&thread, fd).await?
        {
            return Ok(Some(Value::from(false)));
        }
    }
}

#[intrinsic_method("sun/nio/ch/Net.pollconnValue()S", Any)]
#[async_method]
pub async fn pollconn_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLOUT;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLOUT;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method("sun/nio/ch/Net.pollerrValue()S", Any)]
#[async_method]
pub async fn pollerr_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLERR;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLERR;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method("sun/nio/ch/Net.pollhupValue()S", Any)]
#[async_method]
pub async fn pollhup_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLHUP;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLHUP;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method("sun/nio/ch/Net.pollinValue()S", Any)]
#[async_method]
pub async fn pollin_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLIN;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLIN;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method("sun/nio/ch/Net.pollnvalValue()S", Any)]
#[async_method]
pub async fn pollnval_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLNVAL;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLNVAL;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method("sun/nio/ch/Net.polloutValue()S", Any)]
#[async_method]
pub async fn pollout_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    let value = libc::POLLOUT;
    #[cfg(windows)]
    let value = windows_sys::Win32::Networking::WinSock::POLLOUT;
    Ok(Some(Value::Int(i32::from(value))))
}

#[intrinsic_method(
    "sun/nio/ch/Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub async fn remote_inet_address<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let addr = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let socket = socket_from_type(&guard.socket_type);
        socket.peer_addr().map_err(|error: std::io::Error| {
            ristretto_types::JavaError::SocketException(error.to_string())
        })?
    };
    Ok(Some(inet_address_value(&thread, &addr).await?))
}

#[intrinsic_method("sun/nio/ch/Net.remotePort(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn remote_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);
    let addr = match socket.peer_addr() {
        Ok(address) => Some(address),
        Err(error) if is_windows_invalid_argument(&error) => None,
        Err(error) => {
            return Err(ristretto_types::JavaError::SocketException(error.to_string()).into());
        }
    };
    let port = addr
        .as_ref()
        .and_then(SockAddr::as_socket)
        .map_or(0, |a: std::net::SocketAddr| i32::from(a.port()));
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.sendOOB(Ljava/io/FileDescriptor;B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let byte = parameters.pop_int()?.to_ne_bytes()[0];
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    match socket.send_out_of_band(&[byte]) {
        Ok(count) => Ok(Some(Value::Int(i32::try_from(count)?))),
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => Ok(Some(Value::Int(-2))),
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            Ok(Some(Value::Int(super::IOS_INTERRUPTED)))
        }
        Err(error) => Err(ristretto_types::JavaError::IoException(error.to_string()).into()),
    }
}

#[intrinsic_method("sun/nio/ch/Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V", Any)]
#[async_method]
pub async fn set_int_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_ipv6 = parameters.pop_bool()?;
    let value = parameters.pop_int()?;
    let opt = parameters.pop_int()?;
    let level = parameters.pop_int()?;
    let _may_need_conversion = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);
    set_socket_option(&socket, level, opt, value)?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.setInterface4(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_interface_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interf = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    socket
        .set_multicast_if_v4(&ipv4_from_java_int(interf))
        .map_err(|error| ristretto_types::JavaError::SocketException(error.to_string()))?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.setInterface6(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_interface_6<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interf = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&handle.socket_type);
    socket
        .set_multicast_if_v6(
            u32::try_from(interf)
                .map_err(|_| InternalError("negative IPv6 interface index".to_string()))?,
        )
        .map_err(|error| ristretto_types::JavaError::SocketException(error.to_string()))?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/Net.shouldSetBothIPv4AndIPv6Options0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn should_set_both_ipv4_and_ipv6_options_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(cfg!(any(
        target_os = "linux",
        target_os = "windows"
    )))))
}

#[intrinsic_method("sun/nio/ch/Net.shutdown(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn shutdown<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let how = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = get_fd(&fd_value)?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);
    let how = match how {
        0 => Shutdown::Read,
        1 => Shutdown::Write,
        _ => Shutdown::Both,
    };
    socket.shutdown(how).or_else(|e: std::io::Error| {
        if e.kind() == std::io::ErrorKind::NotConnected {
            Ok(())
        } else {
            Err(ristretto_types::JavaError::SocketException(e.to_string()))
        }
    })?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.socket0(ZZZZ)I", Any)]
#[async_method]
pub async fn socket_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fast_loopback = parameters.pop_bool()?;
    let reuse_addr = parameters.pop_bool()?;
    let stream = parameters.pop_bool()?;
    let prefer_ipv6 = parameters.pop_bool()?;
    let (sock_type, protocol) = if stream {
        (Type::STREAM, Some(Protocol::TCP))
    } else {
        (Type::DGRAM, Some(Protocol::UDP))
    };
    let domain = if prefer_ipv6 {
        Domain::IPV6
    } else {
        Domain::IPV4
    };
    let socket = Socket::new(domain, sock_type, protocol).map_err(|error| {
        ristretto_types::JavaError::SocketException(format!(
            "Net.socket0(domain={domain:?}): {error}"
        ))
    })?;
    if prefer_ipv6 {
        // Allow IPv4-mapped IPv6 addresses (dual-stack)
        #[cfg(unix)]
        socket.set_only_v6(false).map_err(|error| {
            ristretto_types::JavaError::SocketException(format!(
                "Unable to set IPV6_V6ONLY: {error}"
            ))
        })?;
        #[cfg(windows)]
        let _ = socket.set_only_v6(false);
    }
    if !stream {
        // Linux otherwise delivers packets for groups joined by other sockets
        // on the host. OpenJDK disables that behavior for NIO datagram sockets.
        #[cfg(target_os = "linux")]
        {
            if prefer_ipv6 {
                socket.set_multicast_hops_v6(1).map_err(|error| {
                    ristretto_types::JavaError::SocketException(format!(
                        "Unable to set IPV6_MULTICAST_HOPS: {error}"
                    ))
                })?;
            }
            let (level, option) = if prefer_ipv6 {
                (libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_ALL)
            } else {
                (libc::IPPROTO_IP, libc::IP_MULTICAST_ALL)
            };
            if let Err(error) = set_raw_socket_option(&socket, level, option, 0)
                && error.raw_os_error() != Some(libc::ENOPROTOOPT)
            {
                return Err(ristretto_types::JavaError::SocketException(format!(
                    "Unable to set IP_MULTICAST_ALL: {error}"
                ))
                .into());
            }
        }
    }
    if reuse_addr {
        // On Windows, OpenJDK uses SO_EXCLUSIVEADDRUSE rather than SO_REUSEADDR; the latter
        // would allow another process to steal the port. Skip the option entirely; the
        // default Windows behavior already yields exclusive bind semantics for TCP.
        #[cfg(not(target_os = "windows"))]
        socket.set_reuse_address(true).map_err(|error| {
            ristretto_types::JavaError::SocketException(format!(
                "Unable to set SO_REUSEADDR: {error}"
            ))
        })?;
    }
    let vm = thread.vm()?;
    let fd = vm.next_nio_fd();
    let mut handle = SocketHandle::new(SocketType::Raw(socket));
    if prefer_ipv6 {
        handle.is_ipv6 = true;
    }
    vm.socket_handles().insert(fd, handle).await?;
    Ok(Some(Value::Int(fd)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.shouldShutdownWriteBeforeClose0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn should_shutdown_write_before_close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Winsock shuts down the write side before close; Unix does not.
    let value = cfg!(target_os = "windows");
    Ok(Some(Value::from(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_block_or_unblock_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = block_or_unblock_4(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_block_or_unblock_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = block_or_unblock_6(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_can_ipv6_socket_join_ipv4_group_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = can_ipv6_socket_join_ipv4_group_0(thread, Parameters::default()).await?;
        assert_eq!(
            Some(Value::from(cfg!(any(
                target_os = "linux",
                target_os = "macos",
                target_os = "windows"
            )))),
            result
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_can_join_6_with_ipv4_group_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = can_join_6_with_ipv4_group_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(cfg!(target_os = "macos"))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_can_use_ipv6_options_with_ipv4_local_address_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result =
            can_use_ipv6_options_with_ipv4_local_address_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(cfg!(target_family = "unix"))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discard_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = discard_oob(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_interface_4(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_interface_6(thread, Parameters::default()).await;
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
    async fn test_is_exclusive_bind_available() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_exclusive_bind_available(thread, Parameters::default()).await?;
        let expected = if cfg!(target_os = "windows") { 1 } else { -1 };
        assert_eq!(Some(Value::Int(expected)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_ipv6_available_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_ipv6_available_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(true)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_reuse_port_available_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_reuse_port_available_0(thread, Parameters::default()).await?;
        let expected = i32::from(cfg!(unix));
        assert_eq!(Some(Value::Int(expected)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_join_or_drop_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = join_or_drop_4(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_or_drop_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = join_or_drop_6(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = listen(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_local_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_inet_address(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_local_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_port(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll_connect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pollconn_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollconn_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLOUT;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLOUT;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollerr_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollerr_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLERR;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLERR;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollhup_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollhup_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLHUP;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLHUP;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollin_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollin_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLIN;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLIN;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollnval_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollnval_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLNVAL;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLNVAL;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollout_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollout_value(thread, Parameters::default()).await?;
        #[cfg(unix)]
        let expected = libc::POLLOUT;
        #[cfg(windows)]
        let expected = windows_sys::Win32::Networking::WinSock::POLLOUT;
        assert_eq!(Some(Value::Int(i32::from(expected))), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_remote_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_inet_address(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remote_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_port(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_oob(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_interface_4(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_interface_6(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_should_set_both_ipv4_and_ipv6_options_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = should_set_both_ipv4_and_ipv6_options_0(thread, Parameters::default()).await?;
        assert_eq!(
            Some(Value::from(cfg!(any(
                target_os = "linux",
                target_os = "windows"
            )))),
            result
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_should_shutdown_write_before_close_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = should_shutdown_write_before_close_0(thread, Parameters::default()).await?;
        let expected = cfg!(target_os = "windows");
        assert_eq!(Some(Value::from(expected)), result);
        Ok(())
    }
}

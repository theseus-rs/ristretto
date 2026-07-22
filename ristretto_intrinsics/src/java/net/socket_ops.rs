//! Shared implementation for the legacy `java.net` socket natives.

use crate::net_helpers::{close_socket, socket_available, socket_from_type, socket_io_error};
use ristretto_types::handles::{SocketHandle, SocketLifecycle, SocketType};
use ristretto_types::{JavaError, Result, VM};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
#[cfg(unix)]
use std::mem::size_of;
#[cfg(windows)]
use std::mem::size_of_val;
use std::net::{Shutdown, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

pub(crate) const IO_UNAVAILABLE: i32 = -2;

pub(crate) const TCP_NODELAY: i32 = 0x0001;
pub(crate) const IP_TOS: i32 = 0x0003;
pub(crate) const SO_REUSEADDR: i32 = 0x0004;
pub(crate) const SO_KEEPALIVE: i32 = 0x0008;
pub(crate) const SO_REUSEPORT: i32 = 0x000e;
pub(crate) const SO_LINGER: i32 = 0x0080;
pub(crate) const SO_SNDBUF: i32 = 0x1001;
pub(crate) const SO_RCVBUF: i32 = 0x1002;
pub(crate) const SO_OOBINLINE: i32 = 0x1003;

fn closed() -> ristretto_types::Error {
    JavaError::SocketException("Socket closed".to_string()).into()
}

fn is_connect_pending(error: &std::io::Error) -> bool {
    if matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
    ) {
        return true;
    }
    matches!(
        error.raw_os_error(),
        // Linux, macOS/BSD, Winsock WSAEWOULDBLOCK/WSAEINPROGRESS/
        // WSAEALREADY.
        Some(115 | 36 | 10035 | 10036 | 10037)
    )
}

#[cfg(unix)]
fn reuse_port(socket: &Socket) -> std::io::Result<bool> {
    socket.reuse_port()
}

#[cfg(windows)]
fn reuse_port(_socket: &Socket) -> std::io::Result<bool> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "SO_REUSEPORT is not supported by Winsock",
    ))
}

#[cfg(unix)]
fn set_reuse_port(socket: &Socket, enabled: bool) -> std::io::Result<()> {
    socket.set_reuse_port(enabled)
}

#[cfg(windows)]
fn set_reuse_port(_socket: &Socket, _enabled: bool) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "SO_REUSEPORT is not supported by Winsock",
    ))
}

pub(crate) fn reuse_port_available(stream: bool) -> bool {
    let socket_type = if stream { Type::STREAM } else { Type::DGRAM };
    let protocol = if stream { Protocol::TCP } else { Protocol::UDP };
    Socket::new(Domain::IPV4, socket_type, Some(protocol))
        .and_then(|socket| set_reuse_port(&socket, true))
        .is_ok()
}

fn traffic_class(socket: &Socket, ipv6: bool) -> std::io::Result<u32> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if ipv6 {
        return socket.tclass_v6();
    }
    let _ = ipv6;
    socket.tos_v4()
}

fn set_traffic_class(socket: &Socket, ipv6: bool, value: u32) -> std::io::Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if ipv6 {
        return socket.set_tclass_v6(value);
    }
    let _ = ipv6;
    socket.set_tos_v4(value)
}

pub(crate) async fn create<V: VM + ?Sized>(vm: &V, stream: bool, ipv6: bool) -> Result<i32> {
    let domain = if ipv6 { Domain::IPV6 } else { Domain::IPV4 };
    let (socket_type, protocol) = if stream {
        (Type::STREAM, Protocol::TCP)
    } else {
        (Type::DGRAM, Protocol::UDP)
    };
    let socket = Socket::new(domain, socket_type, Some(protocol))
        .map_err(|error| socket_io_error("socket", error))?;
    if ipv6 {
        socket
            .set_only_v6(false)
            .map_err(|error| socket_io_error("socket IPV6_V6ONLY", error))?;
    }
    let fd = vm.next_nio_fd();
    let mut handle = SocketHandle::new(SocketType::Raw(socket));
    handle.is_ipv6 = ipv6;
    vm.socket_handles().insert(fd, handle).await?;
    Ok(fd)
}

pub(crate) fn prefer_ipv4_stack<V: VM + ?Sized>(vm: &V) -> bool {
    vm.system_properties()
        .get("java.net.preferIPv4Stack")
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

pub(crate) async fn create_preferred<V: VM + ?Sized>(vm: &V, stream: bool) -> Result<i32> {
    if !prefer_ipv4_stack(vm)
        && let Ok(fd) = create(vm, stream, true).await
    {
        return Ok(fd);
    }
    create(vm, stream, false).await
}

#[cfg(windows)]
pub(crate) async fn set_only_v6<V: VM + ?Sized>(vm: &V, fd: i32, only_v6: bool) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    if !handle.is_ipv6 {
        return Ok(());
    }
    socket_from_type(&handle.socket_type)
        .set_only_v6(only_v6)
        .map_err(|error| socket_io_error("setsockopt IPV6_V6ONLY", error))
}

#[cfg(windows)]
#[expect(unsafe_code)]
pub(crate) async fn set_exclusive_bind<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    exclusive: bool,
) -> Result<()> {
    use std::os::windows::io::AsRawSocket;
    use windows_sys::Win32::Networking::WinSock::{
        SO_EXCLUSIVEADDRUSE, SOCKET_ERROR, SOL_SOCKET, setsockopt,
    };

    if !exclusive {
        return Ok(());
    }
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = socket_from_type(&handle.socket_type);
    let value = 1i32;
    let result = unsafe {
        setsockopt(
            usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
            SOL_SOCKET,
            SO_EXCLUSIVEADDRUSE,
            (&raw const value).cast(),
            i32::try_from(size_of_val(&value)).unwrap_or(i32::MAX),
        )
    };
    if result == SOCKET_ERROR {
        return Err(socket_io_error(
            "setsockopt SO_EXCLUSIVEADDRUSE",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}

pub(crate) async fn close<V: VM + ?Sized>(vm: &V, fd: i32) {
    close_socket(vm, fd).await;
}

pub(crate) async fn bind<V: VM + ?Sized>(vm: &V, fd: i32, address: SocketAddr) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = handle.socket_type.as_raw().ok_or_else(|| {
        JavaError::SocketException("Socket is already bound or connected".to_string())
    })?;
    socket
        .bind(&SockAddr::from(address))
        .map_err(|error| socket_io_error("bind", error))?;
    #[cfg(windows)]
    handle.lifecycle.set_bound_address(address);
    Ok(())
}

#[cfg(windows)]
pub(crate) async fn set_peer_address<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    address: Option<SocketAddr>,
) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    handle.lifecycle.set_peer_address(address);
    Ok(())
}

#[cfg(any(windows, test))]
fn reported_local_address(
    local_address: SocketAddr,
    bound_address: Option<SocketAddr>,
    peer_address: Option<SocketAddr>,
) -> SocketAddr {
    fn canonical_ip(address: SocketAddr) -> std::net::IpAddr {
        match address.ip() {
            std::net::IpAddr::V6(address) => address
                .to_ipv4_mapped()
                .map_or(std::net::IpAddr::V6(address), std::net::IpAddr::V4),
            address @ std::net::IpAddr::V4(_) => address,
        }
    }

    if let Some(mut bound_address) = bound_address
        && !canonical_ip(bound_address).is_unspecified()
    {
        bound_address.set_port(local_address.port());
        return bound_address;
    }
    if let Some(mut peer_address) = peer_address
        && canonical_ip(peer_address).is_loopback()
        && !canonical_ip(local_address).is_loopback()
    {
        peer_address.set_port(local_address.port());
        return peer_address;
    }
    local_address
}

pub(crate) async fn local_address<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<SocketAddr> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = socket_from_type(&handle.socket_type);
    let local_address = socket
        .local_addr()
        .and_then(|address| {
            address
                .as_socket()
                .ok_or_else(|| std::io::Error::other("not an internet socket"))
        })
        .map_err(|error| socket_io_error("getsockname", error))?;
    #[cfg(windows)]
    {
        let peer_address = handle.lifecycle.peer_address().or_else(|| {
            socket
                .peer_addr()
                .ok()
                .and_then(|address| address.as_socket())
        });
        Ok(reported_local_address(
            local_address,
            handle.lifecycle.bound_address(),
            peer_address,
        ))
    }
    #[cfg(not(windows))]
    {
        Ok(local_address)
    }
}

pub(crate) async fn peer_address<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<SocketAddr> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    socket_from_type(&handle.socket_type)
        .peer_addr()
        .and_then(|address| {
            address
                .as_socket()
                .ok_or_else(|| std::io::Error::other("not an internet socket"))
        })
        .map_err(|error| socket_io_error("getpeername", error))
}

/// Start a non-blocking TCP connection. Returns `1` if it completed
/// immediately or `IO_UNAVAILABLE` when completion must be awaited.
pub(crate) async fn connect_start<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    address: &SocketAddr,
) -> Result<i32> {
    // Keep the manager write lock for the short synchronous conversion so close cannot miss the
    // descriptor between a remove and reinsert.
    let mut handles = vm.socket_handles().write().await;
    let handle = handles.remove(&fd).ok_or_else(closed)?;
    let SocketHandle {
        socket_type,
        timeout,
        is_ipv6,
        is_unix,
        non_blocking,
        lifecycle,
    } = handle;
    let SocketType::Raw(socket) = socket_type else {
        handles.insert(
            fd,
            SocketHandle {
                socket_type,
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(JavaError::SocketException("Socket is already connected".to_string()).into());
    };
    if let Err(error) = socket.set_nonblocking(true) {
        handles.insert(
            fd,
            SocketHandle {
                socket_type: SocketType::Raw(socket),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(socket_io_error("connect", error));
    }
    let result = match socket.connect(&SockAddr::from(*address)) {
        Ok(()) => 1,
        Err(error) if is_connect_pending(&error) => IO_UNAVAILABLE,
        Err(error) => {
            handles.insert(
                fd,
                SocketHandle {
                    socket_type: SocketType::Raw(socket),
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            );
            return Err(socket_io_error("connect", error));
        }
    };
    let stream = tokio::net::TcpStream::from_std(socket.into())
        .map_err(|error| socket_io_error("connect", error))?;
    #[cfg(windows)]
    lifecycle.set_peer_address(Some(*address));
    handles.insert(
        fd,
        SocketHandle {
            socket_type: SocketType::TcpStream(Arc::new(stream)),
            timeout,
            is_ipv6,
            is_unix,
            non_blocking,
            lifecycle,
        },
    );
    Ok(result)
}

pub(crate) async fn wait_for_connect<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    timeout_millis: i32,
) -> Result<()> {
    let (stream, lifecycle) = {
        let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
        (
            handle
                .socket_type
                .as_tcp_stream()
                .ok_or_else(|| JavaError::SocketException("Not a TCP socket".to_string()))?
                .clone(),
            handle.lifecycle.clone(),
        )
    };
    let wait = async {
        stream
            .writable()
            .await
            .map_err(|error| socket_io_error("connect", error))?;
        if let Some(error) = stream
            .take_error()
            .map_err(|error| socket_io_error("connect", error))?
        {
            return Err(socket_io_error("connect", error));
        }
        Ok(())
    };
    if timeout_millis > 0 {
        let duration =
            Duration::from_millis(u64::try_from(timeout_millis).map_err(|_| {
                JavaError::IllegalArgumentException("negative timeout".to_string())
            })?);
        tokio::select! {
            result = tokio::time::timeout(duration, wait) => result.map_err(|_| ristretto_types::Error::JavaError(JavaError::SocketTimeoutException("connect timed out".to_string())))?,
            () = lifecycle.cancelled() => Err(closed()),
        }
    } else {
        tokio::select! {
            result = wait => result,
            () = lifecycle.cancelled() => Err(closed()),
        }
    }
}

pub(crate) async fn listen<V: VM + ?Sized>(vm: &V, fd: i32, backlog: i32) -> Result<()> {
    let mut handles = vm.socket_handles().write().await;
    let handle = handles.remove(&fd).ok_or_else(closed)?;
    let SocketHandle {
        socket_type,
        timeout,
        is_ipv6,
        is_unix,
        non_blocking,
        lifecycle,
    } = handle;
    let SocketType::Raw(socket) = socket_type else {
        handles.insert(
            fd,
            SocketHandle {
                socket_type,
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(JavaError::SocketException("Socket is not bindable".to_string()).into());
    };
    if let Err(error) = socket.listen(backlog.max(1)) {
        handles.insert(
            fd,
            SocketHandle {
                socket_type: SocketType::Raw(socket),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(socket_io_error("listen", error));
    }
    if let Err(error) = socket.set_nonblocking(true) {
        handles.insert(
            fd,
            SocketHandle {
                socket_type: SocketType::Raw(socket),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(socket_io_error("listen", error));
    }
    let listener = tokio::net::TcpListener::from_std(socket.into())
        .map_err(|error| socket_io_error("listen", error))?;
    handles.insert(
        fd,
        SocketHandle {
            socket_type: SocketType::TcpListener(Arc::new(listener)),
            timeout,
            is_ipv6,
            is_unix,
            non_blocking,
            lifecycle,
        },
    );
    Ok(())
}

pub(crate) async fn accept<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    timeout_millis: Option<i32>,
) -> Result<(tokio::net::TcpStream, SocketAddr)> {
    let (listener, lifecycle, handle_timeout) = {
        let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
        (
            handle
                .socket_type
                .as_tcp_listener()
                .ok_or_else(|| JavaError::SocketException("Not a listening socket".to_string()))?
                .clone(),
            handle.lifecycle.clone(),
            handle.timeout,
        )
    };
    let timeout = timeout_millis
        .filter(|timeout| *timeout > 0)
        .and_then(|timeout| u64::try_from(timeout).ok().map(Duration::from_millis))
        .or(handle_timeout);
    let wait = async {
        listener
            .accept()
            .await
            .map_err(|error| socket_io_error("accept", error))
    };
    if let Some(timeout) = timeout {
        tokio::select! {
            result = tokio::time::timeout(timeout, wait) => result.map_err(|_| ristretto_types::Error::JavaError(JavaError::SocketTimeoutException("Accept timed out".to_string())))?,
            () = lifecycle.cancelled() => Err(closed()),
        }
    } else {
        tokio::select! {
            result = wait => result,
            () = lifecycle.cancelled() => Err(closed()),
        }
    }
}

pub(crate) async fn insert_accepted<V: VM + ?Sized>(
    vm: &V,
    stream: tokio::net::TcpStream,
    ipv6: bool,
) -> Result<i32> {
    let fd = vm.next_nio_fd();
    let mut handle = SocketHandle::new(SocketType::TcpStream(Arc::new(stream)));
    handle.is_ipv6 = ipv6;
    vm.socket_handles().insert(fd, handle).await?;
    Ok(fd)
}

pub(crate) async fn available<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<i32> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    socket_available(&handle.socket_type)
}

pub(crate) async fn configure_blocking<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    blocking: bool,
) -> Result<()> {
    let mut handle = vm.socket_handles().get_mut(&fd).await.ok_or_else(closed)?;
    // Tokio descriptors must remain non-blocking; this records the Java/native
    // mode used by the wait functions.
    handle.non_blocking = !blocking;
    Ok(())
}

pub(crate) async fn shutdown<V: VM + ?Sized>(vm: &V, fd: i32, how: i32) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = socket_from_type(&handle.socket_type);
    let how = match how {
        0 => Shutdown::Read,
        1 => Shutdown::Write,
        _ => Shutdown::Both,
    };
    match socket.shutdown(how) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotConnected => Ok(()),
        Err(error) => Err(socket_io_error("shutdown", error)),
    }
}

pub(crate) async fn send_oob<V: VM + ?Sized>(vm: &V, fd: i32, data: i32) -> Result<()> {
    let (socket, lifecycle): (Socket, Arc<SocketLifecycle>) = {
        let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
        (
            socket_from_type(&handle.socket_type)
                .try_clone()
                .map_err(|error| socket_io_error("send OOB", error))?,
            handle.lifecycle.clone(),
        )
    };
    #[expect(clippy::cast_sign_loss)]
    let byte = [(data & 0xff) as u8];
    loop {
        match socket.send_out_of_band(&byte) {
            Ok(1) => return Ok(()),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if lifecycle.is_closed() {
                    return Err(closed());
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            Err(error) => return Err(socket_io_error("send OOB", error)),
        }
    }
}

pub(crate) async fn get_int_option<V: VM + ?Sized>(vm: &V, fd: i32, option: i32) -> Result<i32> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = socket_from_type(&handle.socket_type);
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let value = match option {
        TCP_NODELAY => i32::from(
            socket
                .tcp_nodelay()
                .map_err(|e| socket_io_error("getsockopt TCP_NODELAY", e))?,
        ),
        IP_TOS => traffic_class(&socket, handle.is_ipv6)
            .map_err(|e| socket_io_error("getsockopt IP_TOS", e))? as i32,
        SO_REUSEADDR => i32::from(
            socket
                .reuse_address()
                .map_err(|e| socket_io_error("getsockopt SO_REUSEADDR", e))?,
        ),
        SO_REUSEPORT => i32::from(
            reuse_port(&socket).map_err(|e| socket_io_error("getsockopt SO_REUSEPORT", e))?,
        ),
        SO_KEEPALIVE => i32::from(
            socket
                .keepalive()
                .map_err(|e| socket_io_error("getsockopt SO_KEEPALIVE", e))?,
        ),
        SO_LINGER => socket
            .linger()
            .map_err(|e| socket_io_error("getsockopt SO_LINGER", e))?
            .map_or(-1, |duration| duration.as_secs() as i32),
        SO_SNDBUF => socket
            .send_buffer_size()
            .map_err(|e| socket_io_error("getsockopt SO_SNDBUF", e))? as i32,
        SO_RCVBUF => socket
            .recv_buffer_size()
            .map_err(|e| socket_io_error("getsockopt SO_RCVBUF", e))? as i32,
        SO_OOBINLINE => i32::from(
            socket
                .out_of_band_inline()
                .map_err(|e| socket_io_error("getsockopt SO_OOBINLINE", e))?,
        ),
        _ => {
            return Err(
                JavaError::SocketException(format!("Invalid socket option: {option}")).into(),
            );
        }
    };
    Ok(value)
}

pub(crate) async fn set_int_option<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    option: i32,
    value: i32,
) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    let socket = socket_from_type(&handle.socket_type);
    let enabled = value != 0;
    #[expect(clippy::cast_sign_loss)]
    let result = match option {
        TCP_NODELAY => socket.set_tcp_nodelay(enabled),
        IP_TOS => set_traffic_class(&socket, handle.is_ipv6, value as u32),
        SO_REUSEADDR => socket.set_reuse_address(enabled),
        SO_REUSEPORT => set_reuse_port(&socket, enabled),
        SO_KEEPALIVE => socket.set_keepalive(enabled),
        SO_LINGER => socket.set_linger((value >= 0).then(|| Duration::from_secs(value as u64))),
        SO_SNDBUF => socket.set_send_buffer_size(value as usize),
        SO_RCVBUF => socket.set_recv_buffer_size(value as usize),
        SO_OOBINLINE => socket.set_out_of_band_inline(enabled),
        _ => {
            return Err(
                JavaError::SocketException(format!("Invalid socket option: {option}")).into(),
            );
        }
    };
    result.map_err(|error| socket_io_error("setsockopt", error))
}

pub(crate) async fn set_timeout<V: VM + ?Sized>(vm: &V, fd: i32, timeout: i32) -> Result<()> {
    if timeout < 0 {
        return Err(JavaError::IllegalArgumentException("negative timeout".to_string()).into());
    }
    let mut handle = vm.socket_handles().get_mut(&fd).await.ok_or_else(closed)?;
    handle.timeout =
        (timeout != 0).then(|| Duration::from_millis(u64::try_from(timeout).unwrap_or(0)));
    Ok(())
}

pub(crate) async fn ensure_udp<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
) -> Result<(Arc<tokio::net::UdpSocket>, Arc<SocketLifecycle>)> {
    let mut handles = vm.socket_handles().write().await;
    let handle = handles.remove(&fd).ok_or_else(closed)?;
    let SocketHandle {
        socket_type,
        timeout,
        is_ipv6,
        is_unix,
        non_blocking,
        lifecycle,
    } = handle;
    let SocketType::Raw(socket) = socket_type else {
        if let SocketType::UdpSocket(socket) = &socket_type {
            let result = (socket.clone(), lifecycle.clone());
            handles.insert(
                fd,
                SocketHandle {
                    socket_type,
                    timeout,
                    is_ipv6,
                    is_unix,
                    non_blocking,
                    lifecycle,
                },
            );
            return Ok(result);
        }
        handles.insert(
            fd,
            SocketHandle {
                socket_type,
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(JavaError::SocketException("Not a datagram socket".to_string()).into());
    };
    if let Err(error) = socket.set_nonblocking(true) {
        handles.insert(
            fd,
            SocketHandle {
                socket_type: SocketType::Raw(socket),
                timeout,
                is_ipv6,
                is_unix,
                non_blocking,
                lifecycle,
            },
        );
        return Err(socket_io_error("datagram", error));
    }
    let socket = Arc::new(
        tokio::net::UdpSocket::from_std(socket.into())
            .map_err(|error| socket_io_error("datagram", error))?,
    );
    let lifecycle_result = lifecycle.clone();
    handles.insert(
        fd,
        SocketHandle {
            socket_type: SocketType::UdpSocket(socket.clone()),
            timeout,
            is_ipv6,
            is_unix,
            non_blocking,
            lifecycle,
        },
    );
    Ok((socket, lifecycle_result))
}

/// Disconnect a UDP socket using the platform `AF_UNSPEC` operation. The
/// standard library does not expose this operation, but it is the behavior
/// required by `DatagramSocket.disconnect()`.
#[expect(unsafe_code)]
pub(crate) async fn disconnect_udp<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<()> {
    let handle = vm.socket_handles().get(&fd).await.ok_or_else(closed)?;
    #[cfg(unix)]
    let is_ipv6 = handle.is_ipv6;
    let socket = socket_from_type(&handle.socket_type);
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        #[cfg(target_os = "linux")]
        let local_address = socket.local_addr().ok();
        let address: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let address_length = if is_ipv6 {
            size_of::<libc::sockaddr_in6>()
        } else {
            size_of::<libc::sockaddr_in>()
        };
        let result = unsafe {
            libc::connect(
                socket.as_raw_fd(),
                (&raw const address).cast(),
                libc::socklen_t::try_from(address_length).unwrap_or(libc::socklen_t::MAX),
            )
        };
        if result != 0 {
            let error = std::io::Error::last_os_error();
            // Darwin can report EADDRNOTAVAIL (and some Unix variants report
            // EAFNOSUPPORT) even though AF_UNSPEC already removed the peer.
            // Check the resulting socket state before surfacing the syscall error.
            let disconnected = socket.peer_addr().is_err();
            if !disconnected && !matches!(error.raw_os_error(), Some(libc::EAFNOSUPPORT)) {
                return Err(socket_io_error("datagram disconnect", error));
            }
        }
        // Linux may discard an implicitly bound local port when disconnecting
        // a UDP socket. OpenJDK restores that binding so disconnect() does not
        // change DatagramSocket.getLocalPort().
        #[cfg(target_os = "linux")]
        if let Some(local_address) = local_address
            && local_address
                .as_socket()
                .is_some_and(|address| address.port() != 0)
            && socket
                .local_addr()
                .ok()
                .and_then(|address| address.as_socket())
                .is_some_and(|address| address.port() == 0)
        {
            socket
                .bind(&local_address)
                .map_err(|error| socket_io_error("datagram rebind", error))?;
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawSocket;
        use windows_sys::Win32::Networking::WinSock::{
            AF_UNSPEC, SOCKADDR, SOCKADDR_STORAGE, SOCKET_ERROR, connect,
        };
        let address = SOCKADDR_STORAGE {
            ss_family: AF_UNSPEC,
            ..Default::default()
        };
        let result = unsafe {
            connect(
                usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
                (&raw const address).cast::<SOCKADDR>(),
                i32::try_from(size_of_val(&address)).unwrap_or(i32::MAX),
            )
        };
        if result == SOCKET_ERROR {
            return Err(socket_io_error(
                "datagram disconnect",
                std::io::Error::last_os_error(),
            ));
        }
        handle.lifecycle.set_peer_address(None);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::Error;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

    #[test]
    fn reported_local_address_prefers_explicit_bind_address() {
        let local = SocketAddr::from((Ipv4Addr::new(192, 0, 2, 10), 4321));
        let bound = SocketAddr::from((Ipv4Addr::LOCALHOST, 0));
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST, 4321)),
            reported_local_address(local, Some(bound), None)
        );
    }

    #[test]
    fn reported_local_address_recovers_explicit_dual_stack_bind() {
        let local = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4321));
        let bound = SocketAddr::from((Ipv4Addr::LOCALHOST.to_ipv6_mapped(), 0));
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST.to_ipv6_mapped(), 4321)),
            reported_local_address(local, Some(bound), None)
        );
    }

    #[test]
    fn reported_local_address_recovers_implicit_loopback_bind() {
        let local = SocketAddr::from((Ipv4Addr::new(192, 0, 2, 10), 4321));
        let peer = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST, 4321)),
            reported_local_address(local, None, Some(peer))
        );
    }

    #[test]
    fn reported_local_address_preserves_any_local_for_non_loopback_peer() {
        let local = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4321));
        let peer = SocketAddr::from((Ipv4Addr::new(192, 0, 2, 10), 1234));
        assert_eq!(local, reported_local_address(local, None, Some(peer)));
    }

    #[test]
    fn connect_pending_recognizes_portable_error_forms() {
        assert!(is_connect_pending(&std::io::Error::from(
            std::io::ErrorKind::WouldBlock
        )));
        assert!(is_connect_pending(&std::io::Error::from(
            std::io::ErrorKind::Interrupted
        )));
        assert!(is_connect_pending(&std::io::Error::from_raw_os_error(115)));
        assert!(!is_connect_pending(&std::io::Error::from(
            std::io::ErrorKind::ConnectionRefused
        )));
    }

    #[tokio::test]
    async fn tcp_loopback_lifecycle_and_options() -> Result<()> {
        let (vm, _thread) = crate::test::java17_thread().await?;
        let _stream_reuse_port = reuse_port_available(true);
        let _datagram_reuse_port = reuse_port_available(false);
        assert!(!prefer_ipv4_stack(vm.as_ref()));

        let server_fd = create(vm.as_ref(), true, false).await?;
        set_int_option(vm.as_ref(), server_fd, SO_REUSEADDR, 1).await?;
        assert_eq!(
            1,
            get_int_option(vm.as_ref(), server_fd, SO_REUSEADDR).await?
        );
        #[cfg(unix)]
        {
            set_int_option(vm.as_ref(), server_fd, SO_REUSEPORT, 1).await?;
            assert_eq!(
                1,
                get_int_option(vm.as_ref(), server_fd, SO_REUSEPORT).await?
            );
        }
        bind(
            vm.as_ref(),
            server_fd,
            SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
        )
        .await?;
        let server_address = local_address(vm.as_ref(), server_fd).await?;
        assert!(server_address.ip().is_loopback());
        listen(vm.as_ref(), server_fd, 0).await?;

        let timeout = accept(vm.as_ref(), server_fd, Some(5))
            .await
            .expect_err("accept should time out");
        assert!(matches!(
            timeout,
            Error::JavaError(JavaError::SocketTimeoutException(_))
        ));
        assert!(listen(vm.as_ref(), server_fd, 1).await.is_err());
        assert!(ensure_udp(vm.as_ref(), server_fd).await.is_err());

        let client_fd = create(vm.as_ref(), true, false).await?;
        for (option, value) in [
            (TCP_NODELAY, 1),
            (IP_TOS, 0x10),
            (SO_KEEPALIVE, 1),
            (SO_LINGER, 1),
            (SO_SNDBUF, 8192),
            (SO_RCVBUF, 8192),
            (SO_OOBINLINE, 1),
        ] {
            set_int_option(vm.as_ref(), client_fd, option, value).await?;
            let actual = get_int_option(vm.as_ref(), client_fd, option).await?;
            if matches!(option, SO_SNDBUF | SO_RCVBUF) {
                assert!(actual >= value);
            } else {
                assert_eq!(value, actual);
            }
        }
        assert!(get_int_option(vm.as_ref(), client_fd, -1).await.is_err());
        assert!(set_int_option(vm.as_ref(), client_fd, -1, 0).await.is_err());
        assert!(set_timeout(vm.as_ref(), client_fd, -1).await.is_err());
        set_timeout(vm.as_ref(), client_fd, 50).await?;
        configure_blocking(vm.as_ref(), client_fd, false).await?;
        configure_blocking(vm.as_ref(), client_fd, true).await?;

        let started = connect_start(vm.as_ref(), client_fd, &server_address).await?;
        assert!(matches!(started, 1 | IO_UNAVAILABLE));
        wait_for_connect(vm.as_ref(), client_fd, 2_000).await?;
        assert_eq!(server_address, peer_address(vm.as_ref(), client_fd).await?);
        assert!(
            connect_start(vm.as_ref(), client_fd, &server_address)
                .await
                .is_err()
        );

        let (accepted, peer) = accept(vm.as_ref(), server_fd, None).await?;
        assert!(peer.ip().is_loopback());
        let accepted_fd = insert_accepted(vm.as_ref(), accepted, false).await?;
        assert!(
            local_address(vm.as_ref(), accepted_fd)
                .await?
                .ip()
                .is_loopback()
        );
        assert!(
            peer_address(vm.as_ref(), accepted_fd)
                .await?
                .ip()
                .is_loopback()
        );
        assert_eq!(0, available(vm.as_ref(), accepted_fd).await?);

        send_oob(vm.as_ref(), client_fd, 0x5a).await?;
        shutdown(vm.as_ref(), client_fd, 1).await?;
        shutdown(vm.as_ref(), accepted_fd, 0).await?;
        shutdown(vm.as_ref(), accepted_fd, 2).await?;

        close(vm.as_ref(), client_fd).await;
        close(vm.as_ref(), accepted_fd).await;
        close(vm.as_ref(), server_fd).await;
        close(vm.as_ref(), server_fd).await;
        assert!(available(vm.as_ref(), client_fd).await.is_err());
        assert!(peer_address(vm.as_ref(), accepted_fd).await.is_err());
        assert!(local_address(vm.as_ref(), server_fd).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn udp_conversion_and_disconnect() -> Result<()> {
        let (vm, _thread) = crate::test::java17_thread().await?;
        let fd = create(vm.as_ref(), false, false).await?;
        bind(vm.as_ref(), fd, SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).await?;
        let local = local_address(vm.as_ref(), fd).await?;
        let (socket, lifecycle) = ensure_udp(vm.as_ref(), fd).await?;
        assert_eq!(local, socket.local_addr()?);
        assert!(!lifecycle.is_closed());
        let (same_socket, same_lifecycle) = ensure_udp(vm.as_ref(), fd).await?;
        assert!(Arc::ptr_eq(&socket, &same_socket));
        assert!(Arc::ptr_eq(&lifecycle, &same_lifecycle));

        let peer = tokio::net::UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).await?;
        socket.connect(peer.local_addr()?).await?;
        assert!(peer_address(vm.as_ref(), fd).await?.ip().is_loopback());
        disconnect_udp(vm.as_ref(), fd).await?;
        assert!(socket.peer_addr().is_err());

        close(vm.as_ref(), fd).await;
        assert!(lifecycle.is_closed());
        assert!(disconnect_udp(vm.as_ref(), fd).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn invalid_socket_roles_and_ipv6_options() -> Result<()> {
        let (vm, _thread) = crate::test::java17_thread().await?;
        let datagram_fd = create(vm.as_ref(), false, false).await?;
        assert!(listen(vm.as_ref(), datagram_fd, 1).await.is_err());
        assert!(vm.socket_handles().get(&datagram_fd).await.is_some());
        shutdown(vm.as_ref(), datagram_fd, 2).await?;
        assert!(send_oob(vm.as_ref(), datagram_fd, 1).await.is_err());
        close(vm.as_ref(), datagram_fd).await;

        let unconnected_fd = create(vm.as_ref(), true, false).await?;
        assert!(send_oob(vm.as_ref(), unconnected_fd, 1).await.is_err());
        let invalid_peer = SocketAddr::from((Ipv4Addr::BROADCAST, 9));
        if connect_start(vm.as_ref(), unconnected_fd, &invalid_peer)
            .await
            .is_ok()
        {
            let _connect_error = wait_for_connect(vm.as_ref(), unconnected_fd, 10).await;
        }
        close(vm.as_ref(), unconnected_fd).await;

        if let Ok(ipv6_fd) = create(vm.as_ref(), true, true).await {
            set_int_option(vm.as_ref(), ipv6_fd, IP_TOS, 0x20).await?;
            assert_eq!(0x20, get_int_option(vm.as_ref(), ipv6_fd, IP_TOS).await?);
            close(vm.as_ref(), ipv6_fd).await;
        }
        if let (Ok(ipv6_fd), Ok(peer)) = (
            create(vm.as_ref(), false, true).await,
            tokio::net::UdpSocket::bind((Ipv6Addr::LOCALHOST, 0)).await,
        ) {
            bind(
                vm.as_ref(),
                ipv6_fd,
                SocketAddr::from((Ipv6Addr::LOCALHOST, 0)),
            )
            .await?;
            let (socket, _) = ensure_udp(vm.as_ref(), ipv6_fd).await?;
            socket.connect(peer.local_addr()?).await?;
            disconnect_udp(vm.as_ref(), ipv6_fd).await?;
            close(vm.as_ref(), ipv6_fd).await;
        }
        Ok(())
    }
}

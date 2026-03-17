use crate::java::io::socketfiledescriptor::{get_fd, set_fd};
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
use std::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;

/// Create a `socket2::Socket` from a `SocketType` for option operations.
/// The returned Socket is wrapped in `ManuallyDrop` so it won't close the handle.
#[expect(unsafe_code)]
fn socket_from_type(socket_type: &SocketType) -> std::mem::ManuallyDrop<Socket> {
    #[cfg(unix)]
    {
        use std::os::fd::FromRawFd;
        std::mem::ManuallyDrop::new(unsafe { Socket::from_raw_fd(socket_type.raw_fd()) })
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawSocket;
        std::mem::ManuallyDrop::new(unsafe { Socket::from_raw_socket(socket_type.raw_socket()) })
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

fn get_inet_address_int(inet_addr: &Value) -> Result<i32> {
    let holder_value = {
        let object = inet_addr.as_object_ref()?;
        object.value("holder")?
    };
    let holder = holder_value.as_object_ref()?;
    Ok(holder.value("address")?.as_i32()?)
}

fn ipv4_from_int(addr: i32) -> Ipv4Addr {
    #[expect(clippy::cast_sign_loss)]
    let bits = addr as u32;
    Ipv4Addr::new(
        ((bits >> 24) & 0xFF) as u8,
        ((bits >> 16) & 0xFF) as u8,
        ((bits >> 8) & 0xFF) as u8,
        (bits & 0xFF) as u8,
    )
}

/// Create a `SockAddr` for the given socket, converting IPv4 to IPv4-mapped IPv6 if needed.
fn make_sock_addr(prefer_ipv6: bool, ipv4: Ipv4Addr, port: u16) -> SockAddr {
    if prefer_ipv6 {
        if ipv4.is_unspecified() {
            // Use proper IPv6 wildcard (::) instead of IPv4-mapped (::ffff:0.0.0.0)
            SockAddr::from(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0))
        } else {
            SockAddr::from(SocketAddrV6::new(ipv4.to_ipv6_mapped(), port, 0, 0))
        }
    } else {
        SockAddr::from(SocketAddrV4::new(ipv4, port))
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
        (SOL_SOCKET, SO_LINGER) => socket.linger().map(|v| v.map_or(0, |d| d.as_secs() as i32)),
        (SOL_SOCKET, SO_OOBINLINE) => socket.out_of_band_inline().map(i32::from),
        #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        (SOL_SOCKET, SO_SNDBUF) => socket.send_buffer_size().map(|v| v as i32),
        #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        (SOL_SOCKET, SO_RCVBUF) => socket.recv_buffer_size().map(|v| v as i32),
        (IPPROTO_TCP, TCP_NODELAY) => socket.tcp_nodelay().map(i32::from),
        _ => Ok(0),
    };
    result.map_err(|e| InternalError(e.to_string()))
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
            if value > 0 {
                socket.set_linger(Some(Duration::from_secs(u64::from(value as u32))))
            } else {
                socket.set_linger(None)
            }
        }
        (SOL_SOCKET, SO_OOBINLINE) => socket.set_out_of_band_inline(value != 0),
        (SOL_SOCKET, SO_SNDBUF) => socket.set_send_buffer_size(value as usize),
        (SOL_SOCKET, SO_RCVBUF) => socket.set_recv_buffer_size(value as usize),
        (IPPROTO_TCP, TCP_NODELAY) => socket.set_tcp_nodelay(value != 0),
        _ => Ok(()),
    };
    result.map_err(|e| InternalError(e.to_string()))
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
    let (is_tcp_listener, is_nonblocking) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        (
            guard.socket_type.as_tcp_listener().is_some(),
            guard.non_blocking,
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

            match sock.accept() {
                Ok((new_socket, addr)) => {
                    new_socket
                        .set_nonblocking(true)
                        .map_err(|e| InternalError(format!("accept: set_nonblocking: {e}")))?;
                    let std_stream: std::net::TcpStream = new_socket.into();
                    let stream = tokio::net::TcpStream::from_std(std_stream)
                        .map_err(|e| InternalError(format!("accept: from_std: {e}")))?;
                    (Some(stream), addr)
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return Ok(Some(Value::Int(-2)));
                }
                Err(e) => return Err(InternalError(format!("accept: {e}"))),
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

            let result: std::result::Result<(tokio::net::TcpStream, std::net::SocketAddr), _> =
                listener.accept().await;

            match result {
                Ok((stream, peer_addr)) => {
                    let addr = SockAddr::from(peer_addr);
                    (Some(stream), addr)
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return Ok(Some(Value::Int(-2)));
                }
                Err(e) => return Err(InternalError(format!("accept: {e}"))),
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

        let accept_result = tokio::task::spawn_blocking(move || cloned.accept())
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
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return Ok(Some(Value::Int(-2)));
            }
            Err(e) => return Err(InternalError(format!("accept: {e}"))),
        }
    };

    let new_fd = vm.next_nio_fd();
    set_fd(&new_fd_value, new_fd)?;

    if let Some(stream) = tokio_stream {
        vm.socket_handles()
            .insert(
                new_fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(stream))),
            )
            .await?;
    }

    // Fill in isas[0] with the remote address as InetSocketAddress
    let (addr_bytes, port): (Vec<u8>, u16) = if let Some(v4) = addr.as_socket_ipv4() {
        (v4.ip().octets().to_vec(), v4.port())
    } else if let Some(v6) = addr.as_socket_ipv6() {
        let ip = v6.ip();
        if let Some(mapped) = ip.to_ipv4_mapped() {
            (mapped.octets().to_vec(), v6.port())
        } else {
            (ip.octets().to_vec(), v6.port())
        }
    } else {
        (vec![0, 0, 0, 0], 0)
    };

    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = addr_bytes.iter().map(|&b| b as i8).collect();
    let byte_array_value =
        Value::new_object(vm.garbage_collector(), Reference::ByteArray(byte_array));
    let null_string = Value::Object(None);
    let inet_addr = thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[null_string, byte_array_value],
        )
        .await?
        .ok_or_else(|| InternalError("getByAddress returned null".to_string()))?;
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
        && !object_array.elements.is_empty()
    {
        object_array.elements[0] = isa;
    }

    Ok(Some(Value::Int(new_fd)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.available(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd_value = parameters.pop()?;
    Ok(Some(Value::Int(0)))
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
    let address_int = get_inet_address_int(&inet_addr_obj)?;
    let ipv4 = ipv4_from_int(address_int);
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = make_sock_addr(prefer_ipv6, ipv4, port as u16);
    if let Err(e) = socket.bind(&addr) {
        if e.kind() == std::io::ErrorKind::AddrInUse {
            return Err(ristretto_types::JavaError::BindException(e.to_string()).into());
        }
        return Err(InternalError(e.to_string()));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn block_or_unblock_4<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop_int()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let block = parameters.pop_int()?;
    let _ = (source, interf, group, fd_value, block);
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn block_or_unblock_6<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let block = parameters.pop_int()?;
    let _ = (source, interf, group, fd_value, block);
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/ch/Net.canIPv6SocketJoinIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_ipv6_socket_join_ipv4_group_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/nio/ch/Net.canJoin6WithIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_join_6_with_ipv4_group_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn can_use_ipv6_options_with_ipv4_local_address_0<
    T: ristretto_types::Thread + 'static,
>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
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
    let address_int = get_inet_address_int(&inet_addr_obj)?;
    let ipv4 = ipv4_from_int(address_int);

    // Clone the raw socket for connecting (don't take ownership yet to avoid blocking)
    let cloned_socket = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket for connect".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("connect: clone: {e}")))?
    };

    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = make_sock_addr(prefer_ipv6, ipv4, port as u16);

    // Connect in spawn_blocking (connect itself is a one-shot blocking op)
    tokio::task::spawn_blocking(move || cloned_socket.connect(&addr))
        .await
        .map_err(|e| InternalError(format!("connect: spawn: {e}")))?
        .map_err(|e| -> ristretto_types::Error {
            ristretto_types::JavaError::ConnectException(e.to_string()).into()
        })?;

    // Now transition: remove Raw socket and convert to appropriate type
    let raw_socket = vm
        .socket_handles()
        .remove(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd} after connect")))?;
    let SocketType::Raw(socket) = raw_socket.socket_type else {
        return Err(InternalError(
            "expected raw socket after connect".to_string(),
        ));
    };

    // Check socket type to decide transition
    let sock_type = socket
        .r#type()
        .map_err(|e| InternalError(format!("connect: get type: {e}")))?;

    if sock_type == Type::DGRAM {
        // UDP: keep as Raw socket (connect just sets default destination)
        vm.socket_handles()
            .insert(fd, SocketHandle::new(SocketType::Raw(socket)))
            .await?;
    } else {
        // TCP: convert to TcpStream
        socket
            .set_nonblocking(true)
            .map_err(|e| InternalError(format!("connect: set_nonblocking: {e}")))?;
        let std_stream: std::net::TcpStream = socket.into();
        let tokio_stream = tokio::net::TcpStream::from_std(std_stream)
            .map_err(|e| InternalError(format!("connect: from_std: {e}")))?;

        vm.socket_handles()
            .insert(
                fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(tokio_stream))),
            )
            .await?;
    }

    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.discardOOB(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn discard_oob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd_value = parameters.pop()?;
    Ok(Some(Value::from(false)))
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let _ = fd_value;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/ch/Net.getInterface6(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn get_interface_6<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let _ = fd_value;
    Ok(Some(Value::Int(0)))
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
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method("sun/nio/ch/Net.isIPv6Available0()Z", Any)]
#[async_method]
pub async fn is_ipv6_available_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // IPv6 dual-stack support is implemented across all socket callsites
    Ok(Some(Value::from(true)))
}

#[intrinsic_method("sun/nio/ch/Net.isReusePortAvailable0()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_reuse_port_available_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn join_or_drop_4<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop_int()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let join = parameters.pop_int()?;
    let _ = (source, interf, group, fd_value, join);
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn join_or_drop_6<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source = parameters.pop()?;
    let interf = parameters.pop_int()?;
    let group = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let join = parameters.pop_int()?;
    let _ = (source, interf, group, fd_value, join);
    Ok(Some(Value::Int(0)))
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
    let raw_socket = vm
        .socket_handles()
        .remove(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let SocketType::Raw(socket) = raw_socket.socket_type else {
        return Err(InternalError("expected raw socket for listen".to_string()));
    };

    socket
        .listen(backlog)
        .map_err(|e| InternalError(e.to_string()))?;

    // Convert to tokio TcpListener
    socket
        .set_nonblocking(true)
        .map_err(|e| InternalError(format!("listen: set_nonblocking: {e}")))?;
    let std_listener: std::net::TcpListener = socket.into();
    let tokio_listener = tokio::net::TcpListener::from_std(std_listener)
        .map_err(|e| InternalError(format!("listen: from_std: {e}")))?;

    vm.socket_handles()
        .insert(
            fd,
            SocketHandle::new(SocketType::TcpListener(Arc::new(tokio_listener))),
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
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?
    };
    let addr_bytes: Vec<u8> = if let Some(v4) = addr.as_socket_ipv4() {
        v4.ip().octets().to_vec()
    } else if let Some(v6) = addr.as_socket_ipv6() {
        if let Some(mapped) = v6.ip().to_ipv4_mapped() {
            mapped.octets().to_vec()
        } else {
            v6.ip().octets().to_vec()
        }
    } else {
        vec![0, 0, 0, 0]
    };
    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = addr_bytes.iter().map(|&b| b as i8).collect();
    let byte_array_value =
        Value::new_object(vm.garbage_collector(), Reference::ByteArray(byte_array));
    let null_string = Value::Object(None);
    let result = thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[null_string, byte_array_value],
        )
        .await?;
    let Some(addr_value) = result else {
        return Err(InternalError("getByAddress returned null".to_string()));
    };
    Ok(Some(addr_value))
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
    let addr = socket
        .local_addr()
        .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
    let port = addr
        .as_socket()
        .map_or(0, |a: std::net::SocketAddr| i32::from(a.port()));
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method("sun/nio/ch/Net.poll(Ljava/io/FileDescriptor;IJ)I", Any)]
#[async_method]
#[expect(unsafe_code)]
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
    #[expect(clippy::cast_possible_truncation)]
    let timeout_ms = timeout as i32;

    let poll_result = tokio::task::spawn_blocking(move || {
        #[expect(clippy::cast_possible_truncation)]
        let poll_events: i16 = events as i16;

        #[cfg(unix)]
        {
            // SAFETY: raw_handle is a valid fd owned by our SocketHandle; poll only checks
            // readiness without consuming data.
            let mut pollfd = libc::pollfd {
                fd: raw_handle,
                events: poll_events,
                revents: 0,
            };
            let ret = unsafe { libc::poll(std::ptr::from_mut(&mut pollfd), 1, timeout_ms) };
            match ret.cmp(&0) {
                Ordering::Greater => Ok(i32::from(pollfd.revents)),
                Ordering::Equal => Ok(0),
                Ordering::Less => Err(std::io::Error::last_os_error()),
            }
        }

        #[cfg(windows)]
        {
            #[repr(C)]
            struct WsaPollFd {
                fd: u64,
                events: i16,
                revents: i16,
            }
            #[link(name = "ws2_32")]
            unsafe extern "system" {
                fn WSAPoll(fdarray: *mut WsaPollFd, nfds: u32, timeout: i32) -> i32;
            }
            // SAFETY: raw_handle is a valid socket owned by our SocketHandle; WSAPoll only
            // checks readiness without consuming data.
            let mut pollfd = WsaPollFd {
                fd: raw_handle,
                events: poll_events,
                revents: 0,
            };
            let ret = unsafe { WSAPoll(std::ptr::from_mut(&mut pollfd), 1, timeout_ms) };
            match ret.cmp(&0) {
                Ordering::Greater => Ok(i32::from(pollfd.revents)),
                Ordering::Equal => Ok(0),
                Ordering::Less => Err(std::io::Error::last_os_error()),
            }
        }
    })
    .await
    .map_err(|e| InternalError(format!("poll: spawn: {e}")))?
    .map_err(|e| InternalError(format!("poll: {e}")))?;

    Ok(Some(Value::Int(poll_result)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.pollConnect(Ljava/io/FileDescriptor;J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn poll_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    let _fd_value = parameters.pop()?;
    // With blocking I/O via spawn_blocking, connect is already complete
    Ok(Some(Value::from(true)))
}

#[intrinsic_method("sun/nio/ch/Net.pollconnValue()S", Any)]
#[async_method]
pub async fn pollconn_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0004)))
}

#[intrinsic_method("sun/nio/ch/Net.pollerrValue()S", Any)]
#[async_method]
pub async fn pollerr_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0008)))
}

#[intrinsic_method("sun/nio/ch/Net.pollhupValue()S", Any)]
#[async_method]
pub async fn pollhup_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0010)))
}

#[intrinsic_method("sun/nio/ch/Net.pollinValue()S", Any)]
#[async_method]
pub async fn pollin_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0001)))
}

#[intrinsic_method("sun/nio/ch/Net.pollnvalValue()S", Any)]
#[async_method]
pub async fn pollnval_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0020)))
}

#[intrinsic_method("sun/nio/ch/Net.polloutValue()S", Any)]
#[async_method]
pub async fn pollout_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0x0004)))
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
        socket
            .peer_addr()
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?
    };
    let addr_bytes: Vec<u8> = if let Some(v4) = addr.as_socket_ipv4() {
        v4.ip().octets().to_vec()
    } else if let Some(v6) = addr.as_socket_ipv6() {
        if let Some(mapped) = v6.ip().to_ipv4_mapped() {
            mapped.octets().to_vec()
        } else {
            v6.ip().octets().to_vec()
        }
    } else {
        vec![0, 0, 0, 0]
    };
    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = addr_bytes.iter().map(|&b| b as i8).collect();
    let byte_array_value =
        Value::new_object(vm.garbage_collector(), Reference::ByteArray(byte_array));
    let null_string = Value::Object(None);
    let result = thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[null_string, byte_array_value],
        )
        .await?;
    let Some(addr_value) = result else {
        return Err(InternalError("getByAddress returned null".to_string()));
    };
    Ok(Some(addr_value))
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
    let addr = socket
        .peer_addr()
        .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
    let port = addr
        .as_socket()
        .map_or(0, |a: std::net::SocketAddr| i32::from(a.port()));
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method(
    "sun/nio/ch/Net.sendOOB(Ljava/io/FileDescriptor;B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let byte = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let _ = (byte, fd_value);
    Ok(Some(Value::Int(1)))
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interf = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let _ = (interf, fd_value);
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.setInterface6(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_interface_6<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interf = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let _ = (interf, fd_value);
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
    Ok(Some(Value::from(false)))
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
        // Ignore ENOTCONN (57 on macOS, 107 on Linux, 10057 on Windows)
        #[cfg(target_os = "linux")]
        const ENOTCONN: i32 = 107;
        #[cfg(target_os = "windows")]
        const ENOTCONN: i32 = 10057;
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        const ENOTCONN: i32 = 57;
        if e.raw_os_error() == Some(ENOTCONN) {
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
    let socket =
        Socket::new(domain, sock_type, protocol).map_err(|e| InternalError(e.to_string()))?;
    if prefer_ipv6 {
        // Allow IPv4-mapped IPv6 addresses (dual-stack)
        if let Err(e) = socket.set_only_v6(false) {
            warn!("Failed to enable dual-stack (IPV6_V6ONLY=false): {e}");
        }
    }
    if reuse_addr {
        socket
            .set_reuse_address(true)
            .map_err(|e| InternalError(e.to_string()))?;
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
    Ok(Some(Value::from(true)))
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
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_can_join_6_with_ipv4_group_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = can_join_6_with_ipv4_group_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_can_use_ipv6_options_with_ipv4_local_address_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result =
            can_use_ipv6_options_with_ipv4_local_address_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
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
        assert_eq!(Some(Value::Int(-1)), result);
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
        assert_eq!(Some(Value::Int(1)), result);
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
        assert_eq!(Some(Value::Int(0x0004)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollerr_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollerr_value(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0x0008)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollhup_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollhup_value(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0x0010)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollin_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollin_value(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0x0001)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollnval_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollnval_value(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0x0020)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pollout_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pollout_value(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0x0004)), result);
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
        assert_eq!(Some(Value::from(false)), result);
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
        assert_eq!(Some(Value::from(true)), result);
        Ok(())
    }
}

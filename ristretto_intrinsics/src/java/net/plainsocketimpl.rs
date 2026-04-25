#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::Between;
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
#[cfg(not(target_os = "windows"))]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_os = "windows"))]
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
#[cfg(not(target_os = "windows"))]
use ristretto_types::VM;
#[cfg(not(target_os = "windows"))]
use ristretto_types::handles::SocketHandle;
use ristretto_types::handles::SocketType;
use ristretto_types::{JavaError, Parameters, Result};
#[cfg(not(target_os = "windows"))]
use socket2::{Domain, Protocol, SockAddr, Type};
use std::net::Ipv4Addr;
#[cfg(not(target_os = "windows"))]
use std::net::{Shutdown, SocketAddrV4};
use std::sync::Arc;
#[cfg(not(target_os = "windows"))]
use std::time::Duration;

/// Create a `socket2::Socket` from a `SocketType` for option operations.
/// The returned Socket is wrapped in `ManuallyDrop` so it won't close the handle.
#[expect(unsafe_code)]
fn socket_from_type(socket_type: &SocketType) -> std::mem::ManuallyDrop<socket2::Socket> {
    #[cfg(unix)]
    {
        use std::os::fd::FromRawFd;
        std::mem::ManuallyDrop::new(unsafe { socket2::Socket::from_raw_fd(socket_type.raw_fd()) })
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawSocket;
        std::mem::ManuallyDrop::new(unsafe {
            socket2::Socket::from_raw_socket(socket_type.raw_socket())
        })
    }
}

/// Java socket option IDs (from java.net.SocketOptions interface)
const JAVA_TCP_NODELAY: i32 = 0x0001;
const JAVA_IP_TOS: i32 = 0x0003;
const JAVA_SO_REUSEADDR: i32 = 0x04;
const JAVA_SO_KEEPALIVE: i32 = 0x0008;
const JAVA_SO_BINDADDR: i32 = 0x000F;
const JAVA_SO_LINGER: i32 = 0x0080;
const JAVA_SO_SNDBUF: i32 = 0x1001;
const JAVA_SO_RCVBUF: i32 = 0x1002;
const JAVA_SO_OOBINLINE: i32 = 0x1003;
const JAVA_SO_TIMEOUT: i32 = 0x1006;

fn get_fd_from_this(this: &Value) -> Result<i32> {
    let fd_value = {
        let this_ref = this.as_object_ref()?;
        this_ref.value("fd")?
    };
    let fd_ref = fd_value.as_object_ref()?;
    Ok(fd_ref.value("fd")?.as_i32()?)
}

fn set_fd_on_this(this: &Value, fd: i32) -> Result<()> {
    let fd_value = {
        let this_ref = this.as_object_ref()?;
        this_ref.value("fd")?
    };
    let mut fd_ref = fd_value.as_object_mut()?;
    fd_ref.set_value("fd", Value::Int(fd))?;
    Ok(())
}

fn get_inet_address_int(inet_addr: &Value) -> Result<i32> {
    let holder_value = {
        let object = inet_addr.as_object_ref()?;
        object.value("holder")?
    };
    let holder = holder_value.as_object_ref()?;
    Ok(holder.value("address")?.as_i32()?)
}

#[expect(clippy::cast_sign_loss)]
fn ipv4_from_int(addr: i32) -> Ipv4Addr {
    let bits = addr as u32;
    Ipv4Addr::new(
        ((bits >> 24) & 0xFF) as u8,
        ((bits >> 16) & 0xFF) as u8,
        ((bits >> 8) & 0xFF) as u8,
        (bits & 0xFF) as u8,
    )
}

fn get_int_from_object(value: &Value) -> Result<i32> {
    let obj = value.as_object_ref()?;
    Ok(obj.value("value")?.as_i32()?)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.initProto()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init_proto<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn socket_accept<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let socket_impl = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;

    // Check variant and get timeout, then release lock before blocking accept
    let (is_tcp_listener, timeout) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        (guard.socket_type.as_tcp_listener().is_some(), guard.timeout)
    };

    let (tokio_stream, addr) = if is_tcp_listener {
        // Clone the Arc to accept without removing from the handle map
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

        let result: std::result::Result<
            std::result::Result<(tokio::net::TcpStream, std::net::SocketAddr), _>,
            _,
        > = if let Some(duration) = timeout {
            tokio::time::timeout(duration, listener.accept()).await
        } else {
            Ok(listener.accept().await)
        };

        match result {
            Ok(Ok((stream, peer_addr))) => {
                let addr = SockAddr::from(peer_addr);
                (Some(stream), addr)
            }
            Ok(Err(e)) => {
                return Err(JavaError::SocketException(e.to_string()).into());
            }
            Err(_) => {
                return Err(
                    JavaError::SocketTimeoutException("Accept timed out".to_string()).into(),
                );
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
        if timeout.is_some() {
            cloned
                .set_nonblocking(true)
                .map_err(|e| InternalError(format!("accept: nonblocking: {e}")))?;
        } else {
            cloned
                .set_nonblocking(false)
                .map_err(|e| InternalError(format!("accept: blocking: {e}")))?;
        }
        drop(guard);

        let (raw_socket, addr) = if let Some(duration) = timeout {
            tokio::task::spawn_blocking(move || {
                let deadline = std::time::Instant::now() + duration;
                loop {
                    match cloned.accept() {
                        Ok(result) => {
                            result
                                .0
                                .set_nonblocking(false)
                                .map_err(|e| std::io::Error::other(e.to_string()))?;
                            return Ok(result);
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            if std::time::Instant::now() >= deadline {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::TimedOut,
                                    "Accept timed out",
                                ));
                            }
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        Err(e) => return Err(e),
                    }
                }
            })
            .await
            .map_err(|e| InternalError(format!("accept: spawn: {e}")))?
            .map_err(|e| -> ristretto_types::Error {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    JavaError::SocketTimeoutException("Accept timed out".to_string()).into()
                } else {
                    JavaError::SocketException(e.to_string()).into()
                }
            })?
        } else {
            tokio::task::spawn_blocking(move || cloned.accept())
                .await
                .map_err(|e| InternalError(format!("accept: spawn: {e}")))?
                .map_err(|e| -> ristretto_types::Error {
                    JavaError::SocketException(e.to_string()).into()
                })?
        };

        // Convert accepted raw socket to TcpStream
        raw_socket
            .set_nonblocking(true)
            .map_err(|e| InternalError(format!("accept: set_nonblocking: {e}")))?;
        let std_stream: std::net::TcpStream = raw_socket.into();
        let stream = tokio::net::TcpStream::from_std(std_stream)
            .map_err(|e| InternalError(format!("accept: from_std: {e}")))?;
        (Some(stream), addr)
    };

    let new_fd = vm.next_nio_fd();
    set_fd_on_this(&socket_impl, new_fd)?;

    if let Some(stream) = tokio_stream {
        vm.socket_handles()
            .insert(
                new_fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(stream))),
            )
            .await?;
    }

    let (addr_bytes, remote_port): (Vec<u8>, u16) = if let Some(v4) = addr.as_socket_ipv4() {
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

    let remote_port_i32 = i32::from(remote_port);
    let mut si = socket_impl.as_object_mut()?;
    si.set_value("address", inet_addr)?;
    si.set_value("port", Value::Int(remote_port_i32))?;
    drop(si);

    // Set localport from the server socket's local port
    {
        let local_port = {
            let this_ref = this.as_object_ref()?;
            this_ref.value("localport")?.as_i32()?
        };
        let mut si = socket_impl.as_object_mut()?;
        si.set_value("localport", Value::Int(local_port))?;
    }

    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _this = parameters.pop()?;
    Ok(Some(Value::Int(0)))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_bind<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let address = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&address)?;
    let ipv4 = ipv4_from_int(addr_int);
    let vm = thread.vm()?;
    let is_ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.is_ipv6);
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = if is_ipv6 {
        let v6 = if ipv4.is_unspecified() {
            std::net::Ipv6Addr::UNSPECIFIED
        } else {
            ipv4.to_ipv6_mapped()
        };
        SockAddr::from(std::net::SocketAddrV6::new(v6, port as u16, 0, 0))
    } else {
        SockAddr::from(SocketAddrV4::new(ipv4, port as u16))
    };

    let local_port = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket.bind(&addr).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                JavaError::BindException(e.to_string()).into()
            } else {
                InternalError(e.to_string())
            }
        })?;
        let local_addr = socket
            .local_addr()
            .map_err(|e| InternalError(e.to_string()))?;
        local_addr
            .as_socket_ipv4()
            .map(|a| i32::from(a.port()))
            .or_else(|| local_addr.as_socket_ipv6().map(|a| i32::from(a.port())))
            .unwrap_or(port)
    };

    let mut this_ref = this.as_object_mut()?;
    this_ref.set_value("localport", Value::Int(local_port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketClose0(Z)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_deferred = parameters.pop_bool()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    if fd >= 0 {
        let vm = thread.vm()?;
        vm.socket_handles().remove(&fd).await;
        set_fd_on_this(&this, -1)?;
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let port = parameters.pop_int()?;
    let address = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&address)?;
    let ipv4 = ipv4_from_int(addr_int);
    let vm = thread.vm()?;
    let is_ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.is_ipv6);
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let addr = if is_ipv6 {
        SockAddr::from(std::net::SocketAddrV6::new(
            ipv4.to_ipv6_mapped(),
            port as u16,
            0,
            0,
        ))
    } else {
        SockAddr::from(SocketAddrV4::new(ipv4, port as u16))
    };

    // Clone the raw socket for connecting (don't remove to avoid blocking accept)
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

    let connect_result = if timeout > 0 {
        #[expect(clippy::cast_sign_loss)]
        let dur = Duration::from_millis(timeout as u64);
        tokio::task::spawn_blocking(move || cloned_socket.connect_timeout(&addr, dur))
            .await
            .map_err(|e| InternalError(format!("connect: spawn: {e}")))?
    } else {
        tokio::task::spawn_blocking(move || cloned_socket.connect(&addr))
            .await
            .map_err(|e| InternalError(format!("connect: spawn: {e}")))?
    };
    connect_result.map_err(|e| {
        if e.kind() == std::io::ErrorKind::TimedOut {
            JavaError::SocketTimeoutException(e.to_string()).into()
        } else if e.kind() == std::io::ErrorKind::ConnectionRefused {
            JavaError::ConnectException(e.to_string()).into()
        } else {
            InternalError(e.to_string())
        }
    })?;

    // Now transition: remove Raw socket and convert to TcpStream
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

    let mut this_ref = this.as_object_mut()?;
    this_ref.set_value("address", address.clone())?;
    this_ref.set_value("port", Value::Int(port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketCreate(Z)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn socket_create_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_bool()?;
    let this = parameters.pop()?;
    let (sock_type, protocol) = if stream {
        (Type::STREAM, Some(Protocol::TCP))
    } else {
        (Type::DGRAM, Some(Protocol::UDP))
    };
    let socket = socket2::Socket::new(Domain::IPV4, sock_type, protocol)
        .map_err(|e| InternalError(e.to_string()))?;
    socket
        .set_reuse_address(true)
        .map_err(|e| InternalError(e.to_string()))?;
    let vm = thread.vm()?;
    let fd = vm.next_nio_fd();
    set_fd_on_this(&this, fd)?;
    vm.socket_handles()
        .insert(fd, SocketHandle::new(SocketType::Raw(socket)))
        .await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketCreate(ZZ)V", Equal(JAVA_17))]
#[async_method]
pub async fn socket_create_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let prefer_ipv6 = parameters.pop_bool()?;
    let stream = parameters.pop_bool()?;
    let this = parameters.pop()?;
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
    let socket = socket2::Socket::new(domain, sock_type, protocol)
        .map_err(|e| InternalError(e.to_string()))?;
    if prefer_ipv6 {
        let _ = socket.set_only_v6(false);
    }
    socket
        .set_reuse_address(true)
        .map_err(|e| InternalError(e.to_string()))?;
    let vm = thread.vm()?;
    let fd = vm.next_nio_fd();
    set_fd_on_this(&this, fd)?;
    let mut handle = SocketHandle::new(SocketType::Raw(socket));
    if prefer_ipv6 {
        handle.is_ipv6 = true;
    }
    vm.socket_handles().insert(fd, handle).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ia_container = parameters.pop()?;
    let opt = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;

    if opt == JAVA_SO_TIMEOUT {
        let timeout_ms = vm
            .socket_handles()
            .get(&fd)
            .await
            .map_or(0, |guard| guard.timeout_millis());
        return Ok(Some(Value::Int(timeout_ms)));
    }

    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);

    #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let value = match opt {
        JAVA_TCP_NODELAY => i32::from(
            socket
                .tcp_nodelay()
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        ),
        JAVA_SO_REUSEADDR => i32::from(
            socket
                .reuse_address()
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        ),
        JAVA_SO_KEEPALIVE => i32::from(
            socket
                .keepalive()
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        ),
        JAVA_SO_OOBINLINE => i32::from(
            socket
                .out_of_band_inline()
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        ),
        JAVA_SO_LINGER => socket
            .linger()
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?
            .map_or(-1, |d: Duration| d.as_secs() as i32),
        JAVA_SO_SNDBUF => socket
            .send_buffer_size()
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?
            as i32,
        JAVA_SO_RCVBUF => socket
            .recv_buffer_size()
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?
            as i32,
        JAVA_SO_BINDADDR | JAVA_IP_TOS => 0,
        _ => -1,
    };
    Ok(Some(Value::Int(value)))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketListen(I)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_listen<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let backlog = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;

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

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketSendUrgentData(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_send_urgent_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;

    #[expect(clippy::cast_sign_loss)]
    let byte = [(data & 0xFF) as u8];

    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;

    if let Some(stream) = guard.socket_type.as_tcp_stream() {
        let stream = stream.clone();
        drop(guard);
        loop {
            match stream.try_write(&byte) {
                Ok(_) => return Ok(None),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    stream
                        .writable()
                        .await
                        .map_err(|e| InternalError(format!("sendUrgentData: {e}")))?;
                }
                Err(e) => {
                    return Err(InternalError(format!("sendUrgentData: {e}")));
                }
            }
        }
    } else if let Some(socket) = guard.socket_type.as_raw() {
        let cloned = socket
            .try_clone()
            .map_err(|e| InternalError(format!("sendUrgentData: clone: {e}")))?;
        drop(guard);
        tokio::task::spawn_blocking(move || std::io::Write::write_all(&mut &cloned, &byte))
            .await
            .map_err(|e| InternalError(format!("sendUrgentData: spawn: {e}")))?
            .map_err(|e| InternalError(format!("sendUrgentData: {e}")))?;
    } else {
        return Err(InternalError(
            "expected TcpStream or Raw socket for sendUrgentData".to_string(),
        ));
    }

    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let on = parameters.pop_bool()?;
    let cmd = parameters.pop_int()?;
    let this = parameters.pop()?;

    if cmd == JAVA_SO_TIMEOUT {
        let timeout_ms = get_int_from_object(&value).unwrap_or(0);
        let fd = get_fd_from_this(&this)?;
        let vm = thread.vm()?;
        #[expect(clippy::cast_sign_loss)]
        if let Some(mut guard) = vm.socket_handles().get_mut(&fd).await {
            guard.timeout = if timeout_ms == 0 {
                None
            } else {
                Some(Duration::from_millis(timeout_ms as u64))
            };
        }
        return Ok(None);
    }

    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let socket = socket_from_type(&guard.socket_type);

    #[expect(clippy::cast_sign_loss)]
    match cmd {
        JAVA_TCP_NODELAY => socket
            .set_tcp_nodelay(on)
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        JAVA_SO_REUSEADDR => socket
            .set_reuse_address(on)
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        JAVA_SO_KEEPALIVE => socket
            .set_keepalive(on)
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        JAVA_SO_OOBINLINE => socket
            .set_out_of_band_inline(on)
            .map_err(|e: std::io::Error| InternalError(e.to_string()))?,
        JAVA_SO_LINGER => {
            if on {
                let secs = get_int_from_object(&value)?;
                socket
                    .set_linger(Some(Duration::from_secs(secs as u64)))
                    .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
            } else {
                socket
                    .set_linger(None)
                    .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
            }
        }
        JAVA_SO_SNDBUF => {
            let size = get_int_from_object(&value)?;
            socket
                .set_send_buffer_size(size as usize)
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
        }
        JAVA_SO_RCVBUF => {
            let size = get_int_from_object(&value)?;
            socket
                .set_recv_buffer_size(size as usize)
                .map_err(|e: std::io::Error| InternalError(e.to_string()))?;
        }
        _ => {}
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketShutdown(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_shutdown<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let how = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;
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
        #[cfg(target_os = "linux")]
        const ENOTCONN: i32 = 107;
        #[cfg(target_os = "windows")]
        const ENOTCONN: i32 = 10057;
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        const ENOTCONN: i32 = 57;
        if e.raw_os_error() == Some(ENOTCONN) {
            Ok(())
        } else {
            Err(JavaError::SocketException(e.to_string()))
        }
    })?;
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isaa = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.available0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn available0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.available0(I)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _excl_bind = parameters.pop_bool()?;
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.close0(I)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.close0(I)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.configureBlocking(IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocking = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.configureBlocking(IZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.getIntOption(II)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn get_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.getIntOption(II)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.initIDs()V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.initIDs()V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.listen0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn listen0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _backlog = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.listen0(II)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn local_address<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ia_container_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.localPort0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn local_port0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.localPort0(I)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.sendOOB(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.sendOOB(II)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setIntOption(III)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.setIntOption(III)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setSoTimeout0(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_so_timeout0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.setSoTimeout0(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.shutdown0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _howto = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.shutdown0(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.socket0(Z)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn socket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stream = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.socket0(Z)I".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForConnect(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.waitForConnect(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForNewConnection(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_new_connection<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.waitForNewConnection(II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn accept0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isaa = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.available0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn available0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.available0(I)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn bind0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _excl_bind = parameters.pop_bool()?;
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.close0(I)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn close0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.close0(I)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.configureBlocking(IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn configure_blocking_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocking = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.configureBlocking(IZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn connect0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.getIntOption(II)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn get_int_option_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.getIntOption(II)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.initIDs()V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn init_ids_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.initIDs()V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.listen0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn listen0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _backlog = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.listen0(II)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn local_address_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ia_container_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.localPort0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn local_port0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.localPort0(I)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.sendOOB(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn send_oob_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.sendOOB(II)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setIntOption(III)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_int_option_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.setIntOption(III)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setSoTimeout0(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_so_timeout0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.setSoTimeout0(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.shutdown0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn shutdown0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _howto = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.shutdown0(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.socket0(Z)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn socket0_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stream = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.socket0(Z)I".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForConnect(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_connect_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("java/net/PlainSocketImpl.waitForConnect(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForNewConnection(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_new_connection_windows_v11_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/PlainSocketImpl.waitForNewConnection(II)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_init_proto() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = init_proto(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_accept() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_accept(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_available() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_bind() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_bind(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_close_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_connect() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_connect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_create_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = socket_create_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_create_1() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_create_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_get_option(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_listen() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_listen(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_send_urgent_data() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_send_urgent_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_set_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[tokio::test]
    async fn test_socket_shutdown() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_shutdown(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_available0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.available0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_bind0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = configure_blocking(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.configureBlocking(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_int_option(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.getIntOption(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/PlainSocketImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_listen0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = listen0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.listen0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_address(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_port0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_port0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.localPort0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_oob(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.sendOOB(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_int_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.setIntOption(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_so_timeout0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_so_timeout0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.setSoTimeout0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.shutdown0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket0(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.socket0(Z)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            wait_for_connect(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.waitForConnect(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_new_connection() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            wait_for_new_connection(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/PlainSocketImpl.waitForNewConnection(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_available0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available0_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.available0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_bind0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind0_windows_v11_v17(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_configure_blocking_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = configure_blocking_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.configureBlocking(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_int_option_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_int_option_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.getIntOption(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids_windows_v11_v17(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/PlainSocketImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_listen0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            listen0_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/PlainSocketImpl.listen0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_address_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_address_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_port0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            local_port0_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.localPort0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_oob_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            send_oob_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/PlainSocketImpl.sendOOB(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_int_option_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_int_option_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.setIntOption(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_so_timeout0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_so_timeout0_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.setSoTimeout0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            shutdown0_windows_v11_v17(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/PlainSocketImpl.shutdown0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket0_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            socket0_windows_v11_v17(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java/net/PlainSocketImpl.socket0(Z)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_connect_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait_for_connect_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.waitForConnect(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_new_connection_windows_v11_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait_for_new_connection_windows_v11_v17(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/PlainSocketImpl.waitForNewConnection(II)V",
            result.unwrap_err().to_string()
        );
    }
}

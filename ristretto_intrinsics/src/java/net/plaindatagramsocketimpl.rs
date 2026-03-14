use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use socket2::{Domain, Protocol, SockAddr, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;

/// Java socket option IDs (from java.net.SocketOptions interface)
const JAVA_IP_TOS: i32 = 0x0003;
const JAVA_SO_REUSEADDR: i32 = 0x04;
const JAVA_SO_BINDADDR: i32 = 0x000F;
const JAVA_IP_MULTICAST_IF: i32 = 0x10;
const JAVA_IP_MULTICAST_LOOP: i32 = 0x12;
const JAVA_SO_LINGER: i32 = 0x0080;
const JAVA_SO_TIMEOUT: i32 = 0x1006;
const JAVA_SO_SNDBUF: i32 = 0x1001;
const JAVA_SO_RCVBUF: i32 = 0x1002;

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

/// Send a UDP datagram from a `DatagramPacket` (shared by `send` and `send0`).
async fn send_datagram<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;

    let (data, addr) = {
        let pkt_ref = packet.as_object_ref()?;
        let buf_val = pkt_ref.value("buf")?;
        let offset = pkt_ref.value("offset")?.as_i32()?;
        let length = pkt_ref.value("length")?.as_i32()?;
        let addr_val = pkt_ref.value("address")?;
        let port = pkt_ref.value("port")?.as_i32()?;

        let buf_guard = buf_val.as_byte_vec_ref()?;
        #[expect(clippy::cast_sign_loss)]
        let start = offset as usize;
        #[expect(clippy::cast_sign_loss)]
        let end = start + length as usize;
        #[expect(clippy::cast_sign_loss)]
        let data: Vec<u8> = buf_guard[start..end].iter().map(|&b| b as u8).collect();

        let addr_int = get_inet_address_int(&addr_val)?;
        let ipv4 = ipv4_from_int(addr_int);
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let target = SockAddr::from(SocketAddrV4::new(ipv4, port as u16));
        (data, target)
    };

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
            .map_err(|e| InternalError(format!("send: clone: {e}")))?
    };

    tokio::task::spawn_blocking(move || cloned_socket.send_to(&data, &addr))
        .await
        .map_err(|e| InternalError(format!("send: spawn: {e}")))?
        .map_err(|e| InternalError(format!("send: {e}")))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let addr = parameters.pop()?;
    let port = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&addr)?;
    let ipv4 = ipv4_from_int(addr_int);
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let sock_addr = SockAddr::from(SocketAddrV4::new(ipv4, port as u16));
    let vm = thread.vm()?;

    let local_port = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .bind(&sock_addr)
            .map_err(|e| InternalError(e.to_string()))?;
        let local_addr = socket
            .local_addr()
            .map_err(|e| InternalError(e.to_string()))?;
        local_addr
            .as_socket_ipv4()
            .map_or(port, |a| i32::from(a.port()))
    };

    let mut this_ref = this.as_object_mut()?;
    this_ref.set_value("localPort", Value::Int(local_port))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&addr)?;
    let ipv4 = ipv4_from_int(addr_int);
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let sock_addr = SockAddr::from(SocketAddrV4::new(ipv4, port as u16));
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
            .map_err(|e| InternalError(format!("connect: clone: {e}")))?
    };

    tokio::task::spawn_blocking(move || cloned_socket.connect(&sock_addr))
        .await
        .map_err(|e| InternalError(format!("connect: spawn: {e}")))?
        .map_err(|e| InternalError(format!("connect: {e}")))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _this = parameters.pop()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketClose()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_close<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    if fd >= 0 {
        let vm = thread.vm()?;
        vm.socket_handles().remove(&fd).await;
        set_fd_on_this(&this, -1)?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketCreate()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_create<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
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

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.disconnect0(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn disconnect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    let _this = parameters.pop()?;
    Ok(None)
}

#[intrinsic_method("java/net/PlainDatagramSocketImpl.getTTL()B", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn get_ttl<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let result = get_time_to_live(thread, parameters).await?;
    #[expect(clippy::cast_possible_truncation)]
    let byte_val = result.map_or(Value::Int(0), |v| {
        Value::Int(i32::from(v.as_i32().unwrap_or(0) as i8))
    });
    Ok(Some(byte_val))
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.getTimeToLive()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_time_to_live<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    let ttl = socket
        .multicast_ttl_v4()
        .map_err(|e| InternalError(e.to_string()))?;
    #[expect(clippy::cast_possible_wrap)]
    Ok(Some(Value::Int(ttl as i32)))
}

#[intrinsic_method("java/net/PlainDatagramSocketImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn join<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _network_interface = parameters.pop()?;
    let multicast_addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&multicast_addr)?;
    let multicast_ip = ipv4_from_int(addr_int);
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    socket
        .join_multicast_v4(&multicast_ip, &Ipv4Addr::UNSPECIFIED)
        .map_err(|e| InternalError(e.to_string()))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn leave<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _network_interface = parameters.pop()?;
    let multicast_addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let addr_int = get_inet_address_int(&multicast_addr)?;
    let multicast_ip = ipv4_from_int(addr_int);
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    socket
        .leave_multicast_v4(&multicast_ip, &Ipv4Addr::UNSPECIFIED)
        .map_err(|e| InternalError(e.to_string()))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _inet_address = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
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
            .map_err(|e| InternalError(format!("peek: clone: {e}")))?
    };

    let port = tokio::task::spawn_blocking(move || {
        let udp: std::net::UdpSocket = cloned_socket.into();
        let mut buf = [0u8; 1];
        udp.peek_from(&mut buf)
            .map(|(_, addr)| i32::from(addr.port()))
    })
    .await
    .map_err(|e| InternalError(format!("peek: spawn: {e}")))?
    .map_err(|e| InternalError(format!("peek: {e}")))?;
    Ok(Some(Value::Int(port)))
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek_data<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;

    let buf_len = {
        let pkt_ref = packet.as_object_ref()?;
        pkt_ref.value("bufLength")?.as_i32()?
    };

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
            .map_err(|e| InternalError(format!("peekData: clone: {e}")))?
    };

    #[expect(clippy::cast_sign_loss)]
    let capacity = buf_len as usize;
    let (n, data, from_port, from_ip) = tokio::task::spawn_blocking(move || {
        let udp: std::net::UdpSocket = cloned_socket.into();
        let mut buf = vec![0u8; capacity];
        let (n, addr) = udp.peek_from(&mut buf)?;
        buf.truncate(n);
        let port = i32::from(addr.port());
        let ip = match addr {
            std::net::SocketAddr::V4(v4) => v4.ip().octets(),
            std::net::SocketAddr::V6(_) => [0u8; 4],
        };
        Ok::<_, std::io::Error>((n, buf, port, ip))
    })
    .await
    .map_err(|e| InternalError(format!("peekData: spawn: {e}")))?
    .map_err(|e| InternalError(format!("peekData: {e}")))?;

    // Fill the DatagramPacket
    {
        let pkt_ref = packet.as_object_ref()?;
        let buf_val = pkt_ref.value("buf")?;
        let mut buf_guard = buf_val.as_byte_vec_mut()?;
        #[expect(clippy::cast_possible_wrap)]
        for (i, &b) in data.iter().enumerate() {
            if i < buf_guard.len() {
                buf_guard[i] = b as i8;
            }
        }
    }

    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = from_ip.iter().map(|&b| b as i8).collect();
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

    {
        let mut pkt_mut = packet.as_object_mut()?;
        #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        pkt_mut.set_value("length", Value::Int(n as i32))?;
        pkt_mut.set_value("address", inet_addr)?;
        pkt_mut.set_value("port", Value::Int(from_port))?;
    }

    Ok(Some(Value::Int(from_port)))
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn receive_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;

    let buf_len = {
        let pkt_ref = packet.as_object_ref()?;
        pkt_ref.value("bufLength")?.as_i32()?
    };

    let vm = thread.vm()?;
    let timeout = vm
        .socket_handles()
        .get(&fd)
        .await
        .and_then(|guard| guard.timeout);

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

    #[expect(clippy::cast_sign_loss)]
    let capacity = buf_len as usize;
    let (n, data, from_port, from_ip) = tokio::task::spawn_blocking(move || {
        let udp: std::net::UdpSocket = cloned_socket.into();
        // Apply SO_TIMEOUT as read timeout
        udp.set_read_timeout(timeout)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        let mut buf = vec![0u8; capacity];
        let (n, addr) = udp.recv_from(&mut buf)?;
        buf.truncate(n);
        let port = i32::from(addr.port());
        let ip = match addr {
            std::net::SocketAddr::V4(v4) => v4.ip().octets(),
            std::net::SocketAddr::V6(_) => [0u8; 4],
        };
        Ok::<_, std::io::Error>((n, buf, port, ip))
    })
    .await
    .map_err(|e| InternalError(format!("receive: spawn: {e}")))?
    .map_err(|e| {
        if e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock {
            ristretto_types::Error::JavaError(JavaError::SocketTimeoutException(
                "Receive timed out".to_string(),
            ))
        } else {
            InternalError(format!("receive: {e}"))
        }
    })?;

    // Fill the DatagramPacket buffer
    {
        let pkt_ref = packet.as_object_ref()?;
        let buf_val = pkt_ref.value("buf")?;
        let mut buf_guard = buf_val.as_byte_vec_mut()?;
        #[expect(clippy::cast_possible_wrap)]
        for (i, &b) in data.iter().enumerate() {
            if i < buf_guard.len() {
                buf_guard[i] = b as i8;
            }
        }
    }

    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = from_ip.iter().map(|&b| b as i8).collect();
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

    {
        let mut pkt_mut = packet.as_object_mut()?;
        #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        pkt_mut.set_value("length", Value::Int(n as i32))?;
        pkt_mut.set_value("address", inet_addr)?;
        pkt_mut.set_value("port", Value::Int(from_port))?;
    }

    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn send<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_datagram(thread, parameters).await
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn send_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_datagram(thread, parameters).await
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTTL(B)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_ttl<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ttl = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    #[expect(clippy::cast_sign_loss)]
    socket
        .set_multicast_ttl_v4(ttl as u32)
        .map_err(|e| InternalError(e.to_string()))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTimeToLive(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_time_to_live<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ttl = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_fd_from_this(&this)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    #[expect(clippy::cast_sign_loss)]
    socket
        .set_multicast_ttl_v4(ttl as u32)
        .map_err(|e| InternalError(e.to_string()))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
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
        let result = thread
            .object("java.lang.Integer", "(I)V", &[Value::Int(timeout_ms)])
            .await?;
        return Ok(Some(result));
    }

    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };

    #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let value = match opt {
        JAVA_SO_REUSEADDR => i32::from(
            socket
                .reuse_address()
                .map_err(|e| InternalError(e.to_string()))?,
        ),
        JAVA_SO_SNDBUF => socket
            .send_buffer_size()
            .map_err(|e| InternalError(e.to_string()))? as i32,
        JAVA_SO_RCVBUF => socket
            .recv_buffer_size()
            .map_err(|e| InternalError(e.to_string()))? as i32,
        JAVA_IP_TOS | JAVA_SO_BINDADDR | JAVA_IP_MULTICAST_IF => 0,
        JAVA_IP_MULTICAST_LOOP => i32::from(
            socket
                .multicast_loop_v4()
                .map_err(|e| InternalError(e.to_string()))?,
        ),
        JAVA_SO_LINGER => socket
            .linger()
            .map_err(|e| InternalError(e.to_string()))?
            .map_or(-1, |d| d.as_secs() as i32),
        _ => -1,
    };
    drop(guard);

    let result = thread
        .object("java.lang.Integer", "(I)V", &[Value::Int(value)])
        .await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_option_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
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
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };

    #[expect(clippy::cast_sign_loss)]
    match cmd {
        JAVA_SO_REUSEADDR => {
            let on = get_int_from_object(&value)? != 0;
            socket
                .set_reuse_address(on)
                .map_err(|e| InternalError(e.to_string()))?;
        }
        JAVA_SO_SNDBUF => {
            let size = get_int_from_object(&value)?;
            socket
                .set_send_buffer_size(size as usize)
                .map_err(|e| InternalError(e.to_string()))?;
        }
        JAVA_SO_RCVBUF => {
            let size = get_int_from_object(&value)?;
            socket
                .set_recv_buffer_size(size as usize)
                .map_err(|e| InternalError(e.to_string()))?;
        }
        JAVA_IP_MULTICAST_LOOP => {
            let on = get_int_from_object(&value)? != 0;
            socket
                .set_multicast_loop_v4(on)
                .map_err(|e| InternalError(e.to_string()))?;
        }
        _ => {}
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_datagram_socket_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = datagram_socket_close(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_datagram_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = datagram_socket_create(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disconnect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ttl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_time_to_live(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_join() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = join(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_leave() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = leave(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peek(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peek_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_ttl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_time_to_live(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_get_option(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_set_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

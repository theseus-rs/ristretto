use crate::bounds;
use crate::java::net::socket_ops;
use crate::net_helpers::{
    InetAddressValue, inet_address_from_socket, inet_address_value, java_inet_address,
    set_inet_address_value, socket_from_type, std_socket_address,
};
use ristretto_classloader::Value;
use ristretto_types::{JavaError, Result, Thread, VM};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

pub(crate) fn implementation_timeout_millis(implementation: &Value) -> Result<Option<i32>> {
    let timeout = implementation.as_object_ref()?.value("timeout")?.as_i32()?;
    Ok((timeout > 0).then_some(timeout))
}

pub(crate) fn implementation_timeout(implementation: &Value) -> Result<Option<Duration>> {
    implementation_timeout_millis(implementation).map(|timeout| {
        timeout.and_then(|timeout| u64::try_from(timeout).ok().map(Duration::from_millis))
    })
}

pub(crate) const IP_TOS: i32 = 0x0003;
pub(crate) const SO_REUSEADDR: i32 = 0x0004;
pub(crate) const SO_REUSEPORT: i32 = 0x000e;
pub(crate) const SO_BINDADDR: i32 = 0x000f;
pub(crate) const IP_MULTICAST_IF: i32 = 0x0010;
pub(crate) const IP_MULTICAST_LOOP: i32 = 0x0012;
pub(crate) const IP_MULTICAST_IF2: i32 = 0x001f;
pub(crate) const SO_BROADCAST: i32 = 0x0020;
pub(crate) const SO_SNDBUF: i32 = 0x1001;
pub(crate) const SO_RCVBUF: i32 = 0x1002;

fn error(operation: &str, error: std::io::Error) -> ristretto_types::Error {
    if matches!(
        error.kind(),
        std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionReset
    ) {
        return JavaError::PortUnreachableException(error.to_string()).into();
    }
    crate::net_helpers::socket_io_error(operation, error)
}

#[cfg(unix)]
fn reuse_port(socket: &socket2::Socket) -> std::io::Result<bool> {
    socket.reuse_port()
}

#[cfg(windows)]
fn reuse_port(_socket: &socket2::Socket) -> std::io::Result<bool> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "SO_REUSEPORT is not supported by Winsock",
    ))
}

#[cfg(unix)]
fn set_reuse_port(socket: &socket2::Socket, enabled: bool) -> std::io::Result<()> {
    socket.set_reuse_port(enabled)
}

#[cfg(windows)]
fn set_reuse_port(_socket: &socket2::Socket, _enabled: bool) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "SO_REUSEPORT is not supported by Winsock",
    ))
}

fn traffic_class(socket: &socket2::Socket, ipv6: bool) -> std::io::Result<u32> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if ipv6 {
        return socket.tclass_v6();
    }
    let _ = ipv6;
    socket.tos_v4()
}

fn set_traffic_class(socket: &socket2::Socket, ipv6: bool, value: u32) -> std::io::Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if ipv6 {
        return socket.set_tclass_v6(value);
    }
    let _ = ipv6;
    socket.set_tos_v4(value)
}

pub(crate) async fn create<V: VM + ?Sized>(vm: &V) -> Result<i32> {
    socket_ops::create_preferred(vm, false).await
}

pub(crate) async fn bind<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    address: InetAddressValue,
    port: i32,
) -> Result<i32> {
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let address = std_socket_address(address, port, ipv6)?;
    socket_ops::bind(vm, fd, address).await?;
    let port = i32::from(socket_ops::local_address(vm, fd).await?.port());
    socket_ops::ensure_udp(vm, fd).await?;
    Ok(port)
}

pub(crate) async fn connect<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    address: InetAddressValue,
    port: i32,
) -> Result<()> {
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let address = std_socket_address(address, port, ipv6)?;
    socket_ops::ensure_udp(vm, fd)
        .await?
        .0
        .connect(address)
        .await
        .map_err(|error| self::error("datagram connect", error))?;
    #[cfg(windows)]
    socket_ops::set_peer_address(vm, fd, Some(address)).await?;
    Ok(())
}

pub(crate) async fn send_bytes<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    data: &[u8],
    target: Option<(InetAddressValue, i32)>,
) -> Result<()> {
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let (socket, lifecycle) = socket_ops::ensure_udp(vm, fd).await?;
    let send = async {
        if socket
            .peer_addr()
            .is_ok_and(|peer| peer.port() != 0 && !peer.ip().is_unspecified())
        {
            socket
                .send(data)
                .await
                .map_err(|error| self::error("datagram send", error))?;
        } else if let Some((address, port)) = target {
            let address = std_socket_address(address, port, ipv6)?;
            socket
                .send_to(data, address)
                .await
                .map_err(|error| self::error("datagram send", error))?;
        } else {
            socket
                .send(data)
                .await
                .map_err(|error| self::error("datagram send", error))?;
        }
        Ok(())
    };
    tokio::select! {
        result = send => result,
        () = lifecycle.cancelled() => Err(JavaError::SocketException("Socket closed".to_string()).into()),
    }
}

pub(crate) async fn send_packet<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    packet: &Value,
) -> Result<()> {
    let (data, target) = {
        let packet = packet.as_object_ref()?;
        let buffer = packet.value("buf")?;
        let offset = usize::try_from(packet.value("offset")?.as_i32()?)?;
        let length = usize::try_from(packet.value("length")?.as_i32()?)?;
        let end = offset
            .checked_add(length)
            .ok_or_else(|| JavaError::SocketException("Datagram range overflow".to_string()))?;
        let data = {
            let buffer = buffer.as_byte_vec_ref()?;
            bounds::range(&buffer, offset..end, "DatagramPacket buffer")?
                .iter()
                .map(|byte| byte.to_ne_bytes()[0])
                .collect::<Vec<u8>>()
        };
        let address = packet.value("address")?;
        let port = packet.value("port")?.as_i32()?;
        (data, Some((inet_address_value(&address)?, port)))
    };
    send_bytes(thread.vm()?.as_ref(), fd, &data, target).await
}

pub(crate) async fn receive_packet<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    packet: &Value,
    timeout_millis: Option<i32>,
    peek: bool,
) -> Result<i32> {
    let (offset, capacity, buffer) = {
        let packet = packet.as_object_ref()?;
        let offset = usize::try_from(packet.value("offset")?.as_i32()?)?;
        let capacity = usize::try_from(packet.value("bufLength")?.as_i32()?)?;
        let buffer = packet.value("buf")?;
        let end = offset
            .checked_add(capacity)
            .ok_or_else(|| JavaError::SocketException("Datagram range overflow".to_string()))?;
        {
            let bytes = buffer.as_byte_vec_ref()?;
            bounds::range(&bytes, offset..end, "DatagramPacket buffer")?;
        }
        (offset, capacity, buffer)
    };
    let vm = thread.vm()?;
    let (socket, lifecycle) = socket_ops::ensure_udp(vm.as_ref(), fd).await?;
    let handle_timeout = vm
        .socket_handles()
        .get(&fd)
        .await
        .and_then(|handle| handle.timeout);
    let timeout = timeout_millis
        .filter(|timeout| *timeout > 0)
        .and_then(|timeout| u64::try_from(timeout).ok().map(Duration::from_millis))
        .or(handle_timeout);
    let mut data = vec![0; capacity];
    let receive = async {
        loop {
            let result = if peek {
                socket.peek_from(&mut data).await
            } else {
                socket.recv_from(&mut data).await
            };
            // Winsock reports a prior ICMP port-unreachable as WSAECONNRESET even for an
            // unconnected socket. OpenJDK discards that notification and keeps waiting; only a
            // connected DatagramSocket exposes it as PortUnreachableException.
            if let Err(error) = &result
                && matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionReset
                )
                && socket.peer_addr().is_err()
            {
                tokio::task::yield_now().await;
                continue;
            }
            break result;
        }
    };
    let (length, source) = if let Some(timeout) = timeout {
        tokio::select! {
            result = tokio::time::timeout(timeout, receive) => result.map_err(|_| ristretto_types::Error::JavaError(JavaError::SocketTimeoutException("Receive timed out".to_string())))?.map_err(|error| self::error("datagram receive", error))?,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    } else {
        tokio::select! {
            result = receive => result.map_err(|error| self::error("datagram receive", error))?,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    };
    {
        let mut buffer = buffer.as_byte_vec_mut()?;
        #[expect(clippy::cast_possible_wrap)]
        for (target, source) in bounds::range_mut(
            &mut buffer,
            offset..offset + length,
            "DatagramPacket buffer",
        )?
        .iter_mut()
        .zip(data)
        {
            *target = source as i8;
        }
    }
    let address = java_inet_address(thread, inet_address_from_socket(source), None).await?;
    let mut packet = packet.as_object_mut()?;
    packet.set_value(
        "length",
        Value::Int(i32::try_from(length).unwrap_or(i32::MAX)),
    )?;
    packet.set_value("address", address)?;
    packet.set_value("port", Value::Int(i32::from(source.port())))?;
    Ok(i32::from(source.port()))
}

pub(crate) async fn peek_address<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    address: &Value,
    timeout: Option<Duration>,
) -> Result<i32> {
    let (socket, lifecycle) = socket_ops::ensure_udp(vm, fd).await?;
    let mut data = [0; 1];
    let receive = socket.peek_from(&mut data);
    let (_, source) = if let Some(timeout) = timeout {
        tokio::select! {
            result = tokio::time::timeout(timeout, receive) => result.map_err(|_| ristretto_types::Error::JavaError(JavaError::SocketTimeoutException("Receive timed out".to_string())))?.map_err(|error| self::error("datagram peek", error))?,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    } else {
        tokio::select! {
            result = receive => result.map_err(|error| self::error("datagram peek", error))?,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    };
    set_inet_address_value(address, inet_address_from_socket(source))?;
    Ok(i32::from(source.port()))
}

pub(crate) async fn get_int_option<V: VM + ?Sized>(vm: &V, fd: i32, option: i32) -> Result<i32> {
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    let socket = socket_from_type(&handle.socket_type);
    #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let value = match option {
        IP_TOS => traffic_class(&socket, handle.is_ipv6)
            .map_err(|error| self::error("get IP_TOS", error))? as i32,
        SO_REUSEADDR => i32::from(
            socket
                .reuse_address()
                .map_err(|e| error("get SO_REUSEADDR", e))?,
        ),
        SO_REUSEPORT => i32::from(reuse_port(&socket).map_err(|e| error("get SO_REUSEPORT", e))?),
        SO_BROADCAST => i32::from(
            socket
                .broadcast()
                .map_err(|e| error("get SO_BROADCAST", e))?,
        ),
        SO_SNDBUF => socket
            .send_buffer_size()
            .map_err(|e| error("get SO_SNDBUF", e))? as i32,
        SO_RCVBUF => socket
            .recv_buffer_size()
            .map_err(|e| error("get SO_RCVBUF", e))? as i32,
        IP_MULTICAST_LOOP => i32::from(if handle.is_ipv6 {
            socket
                .multicast_loop_v6()
                .map_err(|e| error("get IP_MULTICAST_LOOP", e))?
        } else {
            socket
                .multicast_loop_v4()
                .map_err(|e| error("get IP_MULTICAST_LOOP", e))?
        }),
        IP_MULTICAST_IF => i32::from_ne_bytes(
            u32::from(
                socket
                    .multicast_if_v4()
                    .map_err(|e| error("get IP_MULTICAST_IF", e))?,
            )
            .to_ne_bytes(),
        ),
        IP_MULTICAST_IF2 => i32::try_from(
            socket
                .multicast_if_v6()
                .map_err(|e| error("get IP_MULTICAST_IF2", e))?,
        )
        .unwrap_or(i32::MAX),
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
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    let socket = socket_from_type(&handle.socket_type);
    #[expect(clippy::cast_sign_loss)]
    let result = match option {
        IP_TOS => set_traffic_class(&socket, handle.is_ipv6, value as u32),
        SO_REUSEADDR => socket.set_reuse_address(value != 0),
        SO_REUSEPORT => set_reuse_port(&socket, value != 0),
        SO_BROADCAST => socket.set_broadcast(value != 0),
        SO_SNDBUF => socket.set_send_buffer_size(value as usize),
        SO_RCVBUF => socket.set_recv_buffer_size(value as usize),
        IP_MULTICAST_LOOP if handle.is_ipv6 => socket.set_multicast_loop_v6(value != 0),
        IP_MULTICAST_LOOP => socket.set_multicast_loop_v4(value != 0),
        IP_MULTICAST_IF => {
            socket.set_multicast_if_v4(&Ipv4Addr::from(u32::from_ne_bytes(value.to_ne_bytes())))
        }
        IP_MULTICAST_IF2 => socket.set_multicast_if_v6(value as u32),
        _ => {
            return Err(
                JavaError::SocketException(format!("Invalid socket option: {option}")).into(),
            );
        }
    };
    result.map_err(|error| self::error("set datagram option", error))
}

pub(crate) async fn set_ttl<V: VM + ?Sized>(vm: &V, fd: i32, ttl: i32) -> Result<()> {
    if !(0..=255).contains(&ttl) {
        return Err(JavaError::IllegalArgumentException(format!("invalid TTL: {ttl}")).into());
    }
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    let socket = socket_from_type(&handle.socket_type);
    let ttl = u32::try_from(ttl).unwrap_or(0);
    let result = if handle.is_ipv6 {
        socket.set_multicast_hops_v6(ttl)
    } else {
        socket.set_multicast_ttl_v4(ttl)
    };
    result.map_err(|error| self::error("set multicast TTL", error))
}

pub(crate) async fn get_ttl<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<i32> {
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    let socket = socket_from_type(&handle.socket_type);
    let ttl = if handle.is_ipv6 {
        socket.multicast_hops_v6()
    } else {
        socket.multicast_ttl_v4()
    }
    .map_err(|error| self::error("get multicast TTL", error))?;
    Ok(i32::try_from(ttl).unwrap_or(i32::MAX))
}

fn interface_index(interface: &Value) -> Result<u32> {
    if interface.is_null() {
        return Ok(0);
    }
    Ok(u32::try_from(interface.as_object_ref()?.value("index")?.as_i32()?).unwrap_or(0))
}

fn interface_v4(interface: &Value) -> Result<Ipv4Addr> {
    if interface.is_null() {
        return Ok(Ipv4Addr::UNSPECIFIED);
    }
    let addresses = interface.as_object_ref()?.value("addrs")?;
    let (_, addresses) = addresses.as_class_vec_ref()?;
    for address in addresses.iter() {
        if let InetAddressValue::V4(address) = inet_address_value(address)? {
            return Ok(address);
        }
    }
    Ok(Ipv4Addr::UNSPECIFIED)
}

pub(crate) async fn multicast<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    group: InetAddressValue,
    interface: &Value,
    join: bool,
) -> Result<()> {
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    let socket = socket_from_type(&handle.socket_type);
    let result = match group {
        InetAddressValue::V4(group) => {
            let interface = interface_v4(interface)?;
            if join {
                socket.join_multicast_v4(&group, &interface)
            } else {
                socket.leave_multicast_v4(&group, &interface)
            }
        }
        InetAddressValue::V6(group, _) => {
            let interface = interface_index(interface)?;
            if join {
                socket.join_multicast_v6(&group, interface)
            } else {
                socket.leave_multicast_v6(&group, interface)
            }
        }
    };
    result.map_err(|error| self::error("multicast membership", error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::Error;

    fn byte_array<V: VM + ?Sized>(vm: &V, bytes: &[i8]) -> Value {
        Value::new_object(
            vm.garbage_collector(),
            Reference::ByteArray(bytes.to_vec().into_boxed_slice()),
        )
    }

    fn is_darwin_invalid_argument(error: &Error) -> bool {
        cfg!(target_os = "macos")
            && matches!(
                error,
                Error::JavaError(JavaError::SocketException(message))
                    if message.contains("Invalid argument")
            )
    }

    async fn set_option_if_supported<V: VM + ?Sized>(
        vm: &V,
        fd: i32,
        option: i32,
        value: i32,
    ) -> Result<bool> {
        match set_int_option(vm, fd, option, value).await {
            Ok(()) => Ok(true),
            Err(error) if is_darwin_invalid_argument(&error) => Ok(false),
            Err(error) => Err(error),
        }
    }

    #[test]
    fn option_helpers_and_error_translation() -> Result<()> {
        assert_eq!(
            None,
            implementation_timeout_millis(&Value::Object(None)).ok()
        );
        assert_eq!(0, interface_index(&Value::Object(None))?);
        assert_eq!(Ipv4Addr::UNSPECIFIED, interface_v4(&Value::Object(None))?);
        assert!(matches!(
            error(
                "datagram",
                std::io::Error::from(std::io::ErrorKind::ConnectionRefused)
            ),
            Error::JavaError(JavaError::PortUnreachableException(_))
        ));
        assert!(matches!(
            error(
                "datagram",
                std::io::Error::from(std::io::ErrorKind::ConnectionReset)
            ),
            Error::JavaError(JavaError::PortUnreachableException(_))
        ));
        assert!(matches!(
            error(
                "datagram",
                std::io::Error::from(std::io::ErrorKind::BrokenPipe)
            ),
            Error::JavaError(JavaError::SocketException(_))
        ));
        Ok(())
    }

    #[tokio::test]
    #[expect(clippy::too_many_lines)]
    async fn datagram_packets_options_and_lifecycle() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let loopback = InetAddressValue::V4(Ipv4Addr::LOCALHOST);
        let receiver_fd = socket_ops::create(vm.as_ref(), false, false).await?;
        let receiver_port = bind(vm.as_ref(), receiver_fd, loopback, 0).await?;
        assert!(receiver_port > 0);
        let sender_fd = socket_ops::create(vm.as_ref(), false, false).await?;
        #[cfg(unix)]
        {
            // Darwin requires SO_REUSEPORT to be configured before bind.
            if set_option_if_supported(vm.as_ref(), sender_fd, SO_REUSEPORT, 1).await? {
                assert_eq!(
                    1,
                    get_int_option(vm.as_ref(), sender_fd, SO_REUSEPORT).await?
                );
            }
        }
        let sender_port = bind(vm.as_ref(), sender_fd, loopback, 0).await?;
        assert!(sender_port > 0);

        for (option, value) in [
            (IP_TOS, 0x10),
            (SO_REUSEADDR, 1),
            (SO_BROADCAST, 1),
            (SO_SNDBUF, 8192),
            (SO_RCVBUF, 8192),
            (IP_MULTICAST_LOOP, 1),
            (IP_MULTICAST_IF, 0),
        ] {
            if !set_option_if_supported(vm.as_ref(), sender_fd, option, value).await? {
                let _ = get_int_option(vm.as_ref(), sender_fd, option).await?;
                continue;
            }
            let actual = get_int_option(vm.as_ref(), sender_fd, option).await?;
            if matches!(option, SO_SNDBUF | SO_RCVBUF) {
                assert!(actual >= value);
            } else {
                assert_eq!(value, actual);
            }
        }
        assert!(get_int_option(vm.as_ref(), sender_fd, -1).await.is_err());
        assert!(set_int_option(vm.as_ref(), sender_fd, -1, 0).await.is_err());
        assert!(set_ttl(vm.as_ref(), sender_fd, -1).await.is_err());
        assert!(set_ttl(vm.as_ref(), sender_fd, 256).await.is_err());
        set_ttl(vm.as_ref(), sender_fd, 17).await?;
        assert_eq!(17, get_ttl(vm.as_ref(), sender_fd).await?);

        let address = java_inet_address(&thread, loopback, None).await?;
        let source = byte_array(vm.as_ref(), &[99, 1, 2, 3, 88]);
        let send_packet_value = thread
            .object(
                "java.net.DatagramPacket",
                "[BIILjava/net/InetAddress;I",
                &[
                    source,
                    Value::Int(1),
                    Value::Int(3),
                    address,
                    Value::Int(receiver_port),
                ],
            )
            .await?;
        send_packet(&thread, sender_fd, &send_packet_value).await?;

        let target = byte_array(vm.as_ref(), &[-1; 8]);
        let receive_packet_value = thread
            .object(
                "java.net.DatagramPacket",
                "[BII",
                &[target.clone(), Value::Int(2), Value::Int(4)],
            )
            .await?;
        assert_eq!(
            sender_port,
            receive_packet(
                &thread,
                receiver_fd,
                &receive_packet_value,
                Some(2_000),
                false
            )
            .await?
        );
        assert_eq!(
            vec![-1, -1, 1, 2, 3, -1, -1, -1],
            target.as_byte_vec_ref()?.to_vec()
        );
        assert_eq!(
            3,
            receive_packet_value
                .as_object_ref()?
                .value("length")?
                .as_i32()?
        );

        send_bytes(
            vm.as_ref(),
            sender_fd,
            &[4],
            Some((loopback, receiver_port)),
        )
        .await?;
        assert_eq!(
            sender_port,
            receive_packet(&thread, receiver_fd, &receive_packet_value, None, true).await?
        );
        assert_eq!(
            sender_port,
            receive_packet(&thread, receiver_fd, &receive_packet_value, None, false).await?
        );

        send_bytes(
            vm.as_ref(),
            sender_fd,
            &[5],
            Some((loopback, receiver_port)),
        )
        .await?;
        let peek_target =
            java_inet_address(&thread, InetAddressValue::V4(Ipv4Addr::UNSPECIFIED), None).await?;
        assert_eq!(
            sender_port,
            peek_address(vm.as_ref(), receiver_fd, &peek_target, None).await?
        );
        assert_eq!(loopback, inet_address_value(&peek_target)?);
        receive_packet(
            &thread,
            receiver_fd,
            &receive_packet_value,
            Some(2_000),
            false,
        )
        .await?;
        assert!(
            peek_address(
                vm.as_ref(),
                receiver_fd,
                &peek_target,
                Some(Duration::from_millis(5))
            )
            .await
            .is_err()
        );

        connect(vm.as_ref(), sender_fd, loopback, receiver_port).await?;
        send_bytes(vm.as_ref(), sender_fd, &[6], None).await?;
        receive_packet(
            &thread,
            receiver_fd,
            &receive_packet_value,
            Some(2_000),
            false,
        )
        .await?;
        socket_ops::disconnect_udp(vm.as_ref(), sender_fd).await?;

        let unconnected_fd = socket_ops::create(vm.as_ref(), false, false).await?;
        bind(vm.as_ref(), unconnected_fd, loopback, 0).await?;
        assert!(
            send_bytes(vm.as_ref(), unconnected_fd, &[7], None)
                .await
                .is_err()
        );

        let group = InetAddressValue::V4(Ipv4Addr::new(224, 0, 0, 251));
        match multicast(vm.as_ref(), receiver_fd, group, &Value::Object(None), true).await {
            Ok(()) => {
                match multicast(vm.as_ref(), receiver_fd, group, &Value::Object(None), false).await
                {
                    Ok(()) => {}
                    Err(error) if is_darwin_invalid_argument(&error) => {}
                    Err(error) => return Err(error),
                }
            }
            Err(error) if is_darwin_invalid_argument(&error) => {}
            Err(error) => return Err(error),
        }

        socket_ops::close(vm.as_ref(), sender_fd).await;
        socket_ops::close(vm.as_ref(), receiver_fd).await;
        socket_ops::close(vm.as_ref(), unconnected_fd).await;
        assert!(get_ttl(vm.as_ref(), sender_fd).await.is_err());
        assert!(
            get_int_option(vm.as_ref(), sender_fd, SO_RCVBUF)
                .await
                .is_err()
        );
        assert!(
            set_int_option(vm.as_ref(), sender_fd, SO_RCVBUF, 1024)
                .await
                .is_err()
        );
        assert!(
            multicast(vm.as_ref(), sender_fd, group, &Value::Object(None), true)
                .await
                .is_err()
        );

        if let Ok(ipv6_fd) = socket_ops::create(vm.as_ref(), false, true).await {
            for (option, value) in [
                (IP_TOS, 0x20),
                (IP_MULTICAST_LOOP, 1),
                (IP_MULTICAST_IF2, 0),
            ] {
                if set_option_if_supported(vm.as_ref(), ipv6_fd, option, value).await? {
                    assert_eq!(value, get_int_option(vm.as_ref(), ipv6_fd, option).await?);
                }
            }
            set_ttl(vm.as_ref(), ipv6_fd, 11).await?;
            assert_eq!(11, get_ttl(vm.as_ref(), ipv6_fd).await?);
            socket_ops::close(vm.as_ref(), ipv6_fd).await;
        }
        Ok(())
    }
}

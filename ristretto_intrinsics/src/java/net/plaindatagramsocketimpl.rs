use crate::java::io::socketfiledescriptor::get_impl_fd;
#[cfg(target_family = "unix")]
use crate::java::io::socketfiledescriptor::set_impl_fd;
#[cfg(target_family = "unix")]
use crate::java::net::datagram_ops;
#[cfg(target_family = "unix")]
use crate::java::net::socket_ops;
#[cfg(target_family = "unix")]
use crate::net_helpers::{
    InetAddressValue, boxed_int_value, close_socket, inet_address_from_socket, inet_address_value,
    java_inet_address, std_socket_address,
};
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
#[cfg(not(target_os = "windows"))]
use ristretto_macros::async_method;
#[cfg(not(target_os = "windows"))]
use ristretto_macros::intrinsic_method;
#[cfg(not(target_os = "windows"))]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
#[cfg(target_family = "unix")]
use std::net::Ipv4Addr;
use std::sync::Arc;

/// Java socket option IDs (from java.net.SocketOptions interface)
const JAVA_IP_TOS: i32 = 0x0003;
const JAVA_SO_REUSEADDR: i32 = 0x04;
const JAVA_SO_REUSEPORT: i32 = 0x000e;
const JAVA_SO_BINDADDR: i32 = 0x000F;
const JAVA_IP_MULTICAST_IF: i32 = 0x10;
const JAVA_IP_MULTICAST_LOOP: i32 = 0x12;
const JAVA_IP_MULTICAST_IF2: i32 = 0x001f;
const JAVA_SO_BROADCAST: i32 = 0x0020;
const JAVA_SO_TIMEOUT: i32 = 0x1006;
const JAVA_SO_SNDBUF: i32 = 0x1001;
const JAVA_SO_RCVBUF: i32 = 0x1002;

#[cfg(target_family = "unix")]
fn multicast_interface_index(network_interface: &Value) -> Result<u32> {
    if network_interface.is_null() {
        return Ok(0);
    }
    let interface = network_interface.as_object_ref()?;
    Ok(u32::try_from(interface.value("index")?.as_i32()?).unwrap_or(0))
}

#[cfg(target_family = "unix")]
fn multicast_interface_v4(network_interface: &Value) -> Result<Ipv4Addr> {
    if network_interface.is_null() {
        return Ok(Ipv4Addr::UNSPECIFIED);
    }
    let addresses = {
        let interface = network_interface.as_object_ref()?;
        interface.value("addrs")?
    };
    let (_, addresses) = addresses.as_class_vec_ref()?;
    for address in addresses.iter() {
        if let InetAddressValue::V4(address) = inet_address_value(address)? {
            return Ok(address);
        }
    }
    Ok(Ipv4Addr::UNSPECIFIED)
}

#[cfg(target_family = "unix")]
async fn receive_datagram<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    packet: &Value,
    timeout_millis: Option<i32>,
    peek: bool,
) -> Result<i32> {
    datagram_ops::receive_packet(thread, fd, packet, timeout_millis, peek).await
}

/// Send a UDP datagram from a `DatagramPacket` (shared by `send` and `send0`).
#[cfg(target_family = "unix")]
async fn send_datagram<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    datagram_ops::send_packet(&thread, fd, &packet).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let addr = parameters.pop()?;
    let port = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let is_ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let sock_addr = std_socket_address(inet_address_value(&addr)?, port, is_ipv6)?;
    socket_ops::bind(vm.as_ref(), fd, sock_addr).await?;
    let local_port = i32::from(socket_ops::local_address(vm.as_ref(), fd).await?.port());
    socket_ops::ensure_udp(vm.as_ref(), fd).await?;

    let mut this_ref = this.as_object_mut()?;
    this_ref.set_value("localPort", Value::Int(local_port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    datagram_ops::connect(vm.as_ref(), fd, inet_address_value(&addr)?, port).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    Ok(Some(Value::Int(
        socket_ops::available(thread.vm()?.as_ref(), fd).await?,
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketClose()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_close<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    if fd >= 0 {
        let vm = thread.vm()?;
        close_socket(vm.as_ref(), fd).await;
        set_impl_fd(&this, -1)?;
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketCreate()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_create<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = socket_ops::create_preferred(vm.as_ref(), false).await?;
    set_impl_fd(&this, fd)?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.disconnect0(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn disconnect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    socket_ops::disconnect_udp(thread.vm()?.as_ref(), fd).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainDatagramSocketImpl.getTTL()B", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn get_ttl<T: Thread + 'static>(
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

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.getTimeToLive()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_time_to_live<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        datagram_ops::get_ttl(vm.as_ref(), fd).await?,
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainDatagramSocketImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn join<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let network_interface = parameters.pop()?;
    let multicast_addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    datagram_ops::multicast(
        thread.vm()?.as_ref(),
        fd,
        inet_address_value(&multicast_addr)?,
        &network_interface,
        true,
    )
    .await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn leave<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let network_interface = parameters.pop()?;
    let multicast_addr = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    datagram_ops::multicast(
        thread.vm()?.as_ref(),
        fd,
        inet_address_value(&multicast_addr)?,
        &network_interface,
        false,
    )
    .await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let inet_address = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let timeout = datagram_ops::implementation_timeout(&this)?;
    Ok(Some(Value::Int(
        datagram_ops::peek_address(vm.as_ref(), fd, &inet_address, timeout).await?,
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let timeout = datagram_ops::implementation_timeout_millis(&this)?;
    let port = receive_datagram(&thread, get_impl_fd(&this)?, &packet, timeout, true).await?;
    Ok(Some(Value::Int(port)))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn receive_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let timeout = datagram_ops::implementation_timeout_millis(&this)?;
    receive_datagram(&thread, get_impl_fd(&this)?, &packet, timeout, false).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
    Between(JAVA_8, JAVA_11)
)]
#[async_method]
pub async fn send<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_datagram(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn send_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_datagram(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTTL(B)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_ttl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ttl = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    // The byte overload is unsigned in the Java API.
    datagram_ops::set_ttl(vm.as_ref(), fd, ttl & 0xff).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTimeToLive(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_time_to_live<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ttl = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    datagram_ops::set_ttl(vm.as_ref(), fd, ttl).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let opt = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;

    if opt == JAVA_SO_TIMEOUT {
        let timeout_ms = vm
            .socket_handles()
            .get(&fd)
            .await
            .map_or(0, |guard| guard.timeout_millis());
        let result = thread
            .object("java.lang.Integer", "I", &[Value::Int(timeout_ms)])
            .await?;
        return Ok(Some(result));
    }

    if opt == JAVA_SO_BINDADDR {
        let address = socket_ops::local_address(vm.as_ref(), fd).await?;
        return Ok(Some(
            java_inet_address(&thread, inet_address_from_socket(address), None).await?,
        ));
    }

    if opt == JAVA_IP_MULTICAST_IF {
        let is_ipv6 = vm
            .socket_handles()
            .get(&fd)
            .await
            .is_some_and(|handle| handle.is_ipv6);
        let address = if is_ipv6 {
            let index =
                datagram_ops::get_int_option(vm.as_ref(), fd, JAVA_IP_MULTICAST_IF2).await?;
            super::networkinterface::interface_address_by_index(index)
                .unwrap_or(InetAddressValue::V4(Ipv4Addr::UNSPECIFIED))
        } else {
            let value = datagram_ops::get_int_option(vm.as_ref(), fd, opt).await?;
            InetAddressValue::V4(Ipv4Addr::from(u32::from_ne_bytes(value.to_ne_bytes())))
        };
        return Ok(Some(java_inet_address(&thread, address, None).await?));
    }

    if opt == JAVA_IP_MULTICAST_IF2 {
        let index = datagram_ops::get_int_option(vm.as_ref(), fd, opt).await?;
        return Ok(Some(
            super::networkinterface::interface_by_index(&thread, index).await?,
        ));
    }

    if matches!(
        opt,
        JAVA_SO_REUSEADDR | JAVA_SO_REUSEPORT | JAVA_SO_BROADCAST | JAVA_IP_MULTICAST_LOOP
    ) {
        let mut value = datagram_ops::get_int_option(vm.as_ref(), fd, opt).await? != 0;
        if opt == JAVA_IP_MULTICAST_LOOP {
            // The legacy SocketOptions contract reports whether loopback is disabled.
            value = !value;
        }
        return Ok(Some(
            thread
                .object("java.lang.Boolean", "Z", &[Value::Int(i32::from(value))])
                .await?,
        ));
    }

    let value = match opt {
        JAVA_IP_TOS | JAVA_SO_SNDBUF | JAVA_SO_RCVBUF => {
            datagram_ops::get_int_option(vm.as_ref(), fd, opt).await?
        }
        _ => {
            return Err(JavaError::SocketException(format!("Invalid socket option: {opt}")).into());
        }
    };

    let result = thread
        .object("java.lang.Integer", "I", &[Value::Int(value)])
        .await?;
    Ok(Some(result))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let cmd = parameters.pop_int()?;
    let this = parameters.pop()?;

    if cmd == JAVA_SO_TIMEOUT {
        let timeout_ms = boxed_int_value(&value)?;
        let fd = get_impl_fd(&this)?;
        let vm = thread.vm()?;
        socket_ops::set_timeout(vm.as_ref(), fd, timeout_ms).await?;
        return Ok(None);
    }

    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    match cmd {
        JAVA_SO_BINDADDR => {
            return Err(JavaError::SocketException("Cannot re-bind Socket".to_string()).into());
        }
        JAVA_IP_MULTICAST_IF => {
            let address = inet_address_value(&value)?;
            let is_ipv6 = vm
                .socket_handles()
                .get(&fd)
                .await
                .is_some_and(|handle| handle.is_ipv6);
            let option = match (is_ipv6, address) {
                (true, InetAddressValue::V4(address)) if address.is_unspecified() => {
                    (JAVA_IP_MULTICAST_IF2, 0)
                }
                (true, address) => {
                    let index = match address {
                        InetAddressValue::V6(_, scope) if scope != 0 => {
                            i32::try_from(scope).unwrap_or(i32::MAX)
                        }
                        _ => super::networkinterface::interface_index_by_address(address)
                            .ok_or_else(|| {
                                JavaError::SocketException(
                                    "Network interface not found for multicast address".to_string(),
                                )
                            })?,
                    };
                    (JAVA_IP_MULTICAST_IF2, index)
                }
                (false, InetAddressValue::V4(address)) => {
                    let encoded = i32::from_ne_bytes(u32::from(address).to_ne_bytes());
                    (JAVA_IP_MULTICAST_IF, encoded)
                }
                (false, InetAddressValue::V6(_, scope)) => (
                    JAVA_IP_MULTICAST_IF2,
                    i32::try_from(scope).unwrap_or(i32::MAX),
                ),
            };
            datagram_ops::set_int_option(vm.as_ref(), fd, option.0, option.1).await?;
        }
        JAVA_IP_MULTICAST_IF2 => {
            let is_ipv6 = vm
                .socket_handles()
                .get(&fd)
                .await
                .is_some_and(|handle| handle.is_ipv6);
            let (option, interface) = if is_ipv6 {
                (
                    JAVA_IP_MULTICAST_IF2,
                    i32::try_from(multicast_interface_index(&value)?).unwrap_or(i32::MAX),
                )
            } else {
                let address = multicast_interface_v4(&value)?;
                (
                    JAVA_IP_MULTICAST_IF,
                    i32::from_ne_bytes(u32::from(address).to_ne_bytes()),
                )
            };
            datagram_ops::set_int_option(vm.as_ref(), fd, option, interface).await?;
        }
        JAVA_IP_MULTICAST_LOOP => {
            let disabled = boxed_int_value(&value)? != 0;
            datagram_ops::set_int_option(vm.as_ref(), fd, cmd, i32::from(!disabled)).await?;
        }
        JAVA_IP_TOS | JAVA_SO_REUSEADDR | JAVA_SO_REUSEPORT | JAVA_SO_BROADCAST
        | JAVA_SO_SNDBUF | JAVA_SO_RCVBUF => {
            datagram_ops::set_int_option(vm.as_ref(), fd, cmd, boxed_int_value(&value)?).await?;
        }
        _ => {
            return Err(JavaError::SocketException(format!("Invalid socket option: {cmd}")).into());
        }
    }
    Ok(None)
}

#[cfg(all(test, target_family = "unix", not(target_family = "wasm")))]
mod tests {
    use super::*;
    use ristretto_classloader::{Object, Reference};
    use ristretto_types::Error;

    async fn boxed_integer<T: Thread + 'static>(thread: &Arc<T>, value: i32) -> Result<Value> {
        thread
            .object("java.lang.Integer", "I", &[Value::Int(value)])
            .await
    }

    fn is_darwin_invalid_argument(error: &Error) -> bool {
        cfg!(target_os = "macos")
            && matches!(
                error,
                Error::JavaError(JavaError::SocketException(message))
                    if message.contains("Invalid argument")
            )
    }

    #[tokio::test]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = bind_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = data_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_datagram_socket_close() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = datagram_socket_close(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_datagram_socket_create() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = datagram_socket_create(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = disconnect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ttl() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = get_ttl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_time_to_live() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = get_time_to_live(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_join() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = join(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_leave() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = leave(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = peek(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek_data() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = peek_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = receive_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = send(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = send_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_ttl() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = set_ttl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_time_to_live() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = set_time_to_live(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_get_option(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_set_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[expect(clippy::too_many_lines)]
    async fn implementation_lifecycle_and_option_wrappers() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let implementation_class = thread.class("java.net.PlainDatagramSocketImpl").await?;
        let implementation = Value::new_object(
            vm.garbage_collector(),
            Reference::Object(Object::new(implementation_class)?),
        );
        let file_descriptor = thread.object("java.io.FileDescriptor", "", &[]).await?;
        implementation
            .as_object_mut()?
            .set_value("fd", file_descriptor)?;
        implementation
            .as_object_mut()?
            .set_value("timeout", Value::Int(25))?;
        assert_eq!(
            Some(25),
            datagram_ops::implementation_timeout_millis(&implementation)?
        );
        assert_eq!(
            Some(std::time::Duration::from_millis(25)),
            datagram_ops::implementation_timeout(&implementation)?
        );
        assert_eq!(
            None,
            datagram_socket_create(
                thread.clone(),
                Parameters::new(vec![implementation.clone()])
            )
            .await?
        );
        let fd = get_impl_fd(&implementation)?;
        assert!(fd >= 0);

        let loopback =
            java_inet_address(&thread, InetAddressValue::V4(Ipv4Addr::LOCALHOST), None).await?;
        assert_eq!(
            None,
            bind_0(
                thread.clone(),
                Parameters::new(vec![
                    implementation.clone(),
                    Value::Int(0),
                    loopback.clone()
                ])
            )
            .await?
        );
        let local_port = implementation
            .as_object_ref()?
            .value("localPort")?
            .as_i32()?;
        assert!(local_port > 0);
        assert_eq!(
            Some(Value::Int(0)),
            data_available(
                thread.clone(),
                Parameters::new(vec![implementation.clone()])
            )
            .await?
        );

        for (option, value) in [
            (JAVA_SO_TIMEOUT, 25),
            (JAVA_SO_REUSEADDR, 1),
            (JAVA_SO_BROADCAST, 1),
            (JAVA_SO_SNDBUF, 8192),
            (JAVA_SO_RCVBUF, 8192),
            (JAVA_IP_TOS, 0x10),
            (JAVA_IP_MULTICAST_LOOP, 0),
        ] {
            assert_eq!(
                None,
                socket_set_option_0(
                    thread.clone(),
                    Parameters::new(vec![
                        implementation.clone(),
                        Value::Int(option),
                        boxed_integer(&thread, value).await?
                    ])
                )
                .await?
            );
            assert!(
                socket_get_option(
                    thread.clone(),
                    Parameters::new(vec![implementation.clone(), Value::Int(option)])
                )
                .await?
                .is_some()
            );
        }
        assert!(
            socket_get_option(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(JAVA_SO_BINDADDR)])
            )
            .await?
            .is_some()
        );
        assert!(
            socket_get_option(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(-1)])
            )
            .await
            .is_err()
        );
        assert!(
            socket_set_option_0(
                thread.clone(),
                Parameters::new(vec![
                    implementation.clone(),
                    Value::Int(JAVA_SO_BINDADDR),
                    boxed_integer(&thread, 0).await?
                ])
            )
            .await
            .is_err()
        );
        assert!(
            socket_set_option_0(
                thread.clone(),
                Parameters::new(vec![
                    implementation.clone(),
                    Value::Int(-1),
                    boxed_integer(&thread, 0).await?
                ])
            )
            .await
            .is_err()
        );

        assert_eq!(
            None,
            set_ttl(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(-1)])
            )
            .await?
        );
        assert_eq!(
            Some(Value::Int(-1)),
            get_ttl(
                thread.clone(),
                Parameters::new(vec![implementation.clone()])
            )
            .await?
        );
        assert_eq!(
            None,
            set_time_to_live(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(17)])
            )
            .await?
        );
        assert_eq!(
            Some(Value::Int(17)),
            get_time_to_live(
                thread.clone(),
                Parameters::new(vec![implementation.clone()])
            )
            .await?
        );

        let interface = Value::Object(None);
        assert_eq!(0, multicast_interface_index(&interface)?);
        assert_eq!(Ipv4Addr::UNSPECIFIED, multicast_interface_v4(&interface)?);

        let group = java_inet_address(
            &thread,
            InetAddressValue::V4(Ipv4Addr::new(224, 0, 0, 251)),
            None,
        )
        .await?;
        let join_result = join(
            thread.clone(),
            Parameters::new(vec![
                implementation.clone(),
                group.clone(),
                interface.clone(),
            ]),
        )
        .await;
        match join_result {
            Ok(result) => {
                assert_eq!(None, result);
                match leave(
                    thread.clone(),
                    Parameters::new(vec![implementation.clone(), group, interface.clone()]),
                )
                .await
                {
                    Ok(result) => assert_eq!(None, result),
                    Err(error) if is_darwin_invalid_argument(&error) => {}
                    Err(error) => return Err(error),
                }
            }
            Err(error) if is_darwin_invalid_argument(&error) => {}
            Err(error) => return Err(error),
        }

        for (option, value) in [
            (JAVA_IP_MULTICAST_IF2, interface),
            (JAVA_IP_MULTICAST_IF, loopback.clone()),
        ] {
            match socket_set_option_0(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(option), value]),
            )
            .await
            {
                Ok(result) => assert_eq!(None, result),
                Err(error) if is_darwin_invalid_argument(&error) => {}
                Err(error) => return Err(error),
            }
        }

        let peer = tokio::net::UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).await?;
        assert_eq!(
            None,
            connect_0(
                thread.clone(),
                Parameters::new(vec![
                    implementation.clone(),
                    loopback,
                    Value::Int(i32::from(peer.local_addr()?.port()))
                ])
            )
            .await?
        );
        assert_eq!(
            None,
            disconnect_0(
                thread.clone(),
                Parameters::new(vec![implementation.clone(), Value::Int(1)])
            )
            .await?
        );

        assert_eq!(
            None,
            datagram_socket_close(
                thread.clone(),
                Parameters::new(vec![implementation.clone()])
            )
            .await?
        );
        assert_eq!(-1, get_impl_fd(&implementation)?);
        assert_eq!(
            None,
            datagram_socket_close(thread, Parameters::new(vec![implementation])).await?
        );
        assert!(vm.socket_handles().get(&fd).await.is_none());
        Ok(())
    }
}

use crate::java::io::socketfiledescriptor::{get_impl_fd, set_impl_fd};
use crate::java::net::{datagram_ops, socket_ops};
use crate::net_helpers::{
    InetAddressValue, boxed_int_value, inet_address_from_socket, inet_address_value,
    java_inet_address,
};
use ristretto_classfile::VersionSpecification::{Between, Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread, VM};
use std::net::Ipv4Addr;
use std::sync::Arc;

const SO_TIMEOUT: i32 = 0x1006;

fn fd_from_parameters(parameters: &mut Parameters) -> Result<i32> {
    get_impl_fd(&parameters.pop()?)
}

fn multicast_interface_index(interface: &Value) -> Result<i32> {
    if interface.is_null() {
        return Ok(0);
    }
    Ok(interface.as_object_ref()?.value("index")?.as_i32()?)
}

fn multicast_interface_v4(interface: &Value) -> Result<Ipv4Addr> {
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

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;Z)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let exclusive = parameters.pop_bool()?;
    let address = inet_address_value(&parameters.pop()?)?;
    let port = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    socket_ops::set_exclusive_bind(vm.as_ref(), fd, exclusive).await?;
    let local_port = datagram_ops::bind(vm.as_ref(), fd, address, port).await?;
    this.as_object_mut()?
        .set_value("localPort", Value::Int(local_port))?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let address = inet_address_value(&parameters.pop()?)?;
    let fd = fd_from_parameters(&mut parameters)?;
    datagram_ops::connect(thread.vm()?.as_ref(), fd, address, port).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = fd_from_parameters(&mut parameters)?;
    Ok(Some(Value::Int(
        socket_ops::available(thread.vm()?.as_ref(), fd).await?,
    )))
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketClose()V",
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
        socket_ops::close(thread.vm()?.as_ref(), fd).await;
        set_impl_fd(&this, -1)?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketCreate()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_create<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let this = parameters.pop()?;
    let fd = datagram_ops::create(thread.vm()?.as_ref()).await?;
    set_impl_fd(&this, fd)?;
    // A null `fd1` tells the Windows JDK that the native implementation uses
    // one dual-stack descriptor. Leaving an unopened FileDescriptor object in
    // this field makes Java report its unset `anyLocalBoundAddr` instead.
    if this.as_object_ref()?.value("fd1").is_ok() {
        this.as_object_mut()?
            .set_value("fd1", Value::Object(None))?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.disconnect0(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn disconnect0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    socket_ops::disconnect_udp(thread.vm()?.as_ref(), fd_from_parameters(&mut parameters)?).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.getTTL()B",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_ttl<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_time_to_live(thread, parameters).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.getTimeToLive()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_time_to_live<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(
        datagram_ops::get_ttl(thread.vm()?.as_ref(), fd_from_parameters(&mut parameters)?).await?,
    )))
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.init()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

async fn membership<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
    join: bool,
) -> Result<Option<Value>> {
    let interface = parameters.pop()?;
    let group = inet_address_value(&parameters.pop()?)?;
    let fd = fd_from_parameters(&mut parameters)?;
    datagram_ops::multicast(thread.vm()?.as_ref(), fd, group, &interface, join).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn join<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    membership(thread, parameters, true).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn leave<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    membership(thread, parameters, false).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let timeout = datagram_ops::implementation_timeout(&this)?;
    Ok(Some(Value::Int(
        datagram_ops::peek_address(vm.as_ref(), fd, &address, timeout).await?,
    )))
}

async fn receive<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
    peek: bool,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let timeout = datagram_ops::implementation_timeout_millis(&this)?;
    let port = datagram_ops::receive_packet(&thread, fd, &packet, timeout, peek).await?;
    Ok(peek.then_some(Value::Int(port)))
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek_data<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    receive(thread, parameters, true).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn receive0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    receive(thread, parameters, false).await
}

async fn send_packet<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let packet = parameters.pop()?;
    let fd = fd_from_parameters(&mut parameters)?;
    datagram_ops::send_packet(&thread, fd, &packet).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn send<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_packet(thread, parameters).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn send0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    send_packet(thread, parameters).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.setTTL(B)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_ttl<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_time_to_live(thread, parameters).await
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.setTimeToLive(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_time_to_live<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ttl = parameters.pop_int()?;
    let fd = fd_from_parameters(&mut parameters)?;
    datagram_ops::set_ttl(thread.vm()?.as_ref(), fd, ttl).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let option = parameters.pop_int()?;
    let fd = fd_from_parameters(&mut parameters)?;
    let vm = thread.vm()?;
    if option == SO_TIMEOUT {
        let timeout = vm
            .socket_handles()
            .get(&fd)
            .await
            .map_or(0, |handle| handle.timeout_millis());
        return Ok(Some(
            thread
                .object("java.lang.Integer", "I", &[Value::Int(timeout)])
                .await?,
        ));
    }
    if option == datagram_ops::SO_BINDADDR {
        let address = socket_ops::local_address(vm.as_ref(), fd).await?;
        return Ok(Some(
            java_inet_address(&thread, inet_address_from_socket(address), None).await?,
        ));
    }
    if option == datagram_ops::IP_MULTICAST_IF {
        let is_ipv6 = vm
            .socket_handles()
            .get(&fd)
            .await
            .is_some_and(|handle| handle.is_ipv6);
        let address = if is_ipv6 {
            let index =
                datagram_ops::get_int_option(vm.as_ref(), fd, datagram_ops::IP_MULTICAST_IF2)
                    .await?;
            super::networkinterface::interface_address_by_index(index)
                .unwrap_or(InetAddressValue::V4(Ipv4Addr::UNSPECIFIED))
        } else {
            let value = datagram_ops::get_int_option(vm.as_ref(), fd, option).await?;
            InetAddressValue::V4(Ipv4Addr::from(u32::from_ne_bytes(value.to_ne_bytes())))
        };
        return Ok(Some(java_inet_address(&thread, address, None).await?));
    }
    if option == datagram_ops::IP_MULTICAST_IF2 {
        let index = datagram_ops::get_int_option(vm.as_ref(), fd, option).await?;
        return Ok(Some(
            super::networkinterface::interface_by_index(&thread, index).await?,
        ));
    }
    let value = datagram_ops::get_int_option(vm.as_ref(), fd, option).await?;
    if matches!(
        option,
        datagram_ops::SO_REUSEADDR
            | datagram_ops::SO_REUSEPORT
            | datagram_ops::SO_BROADCAST
            | datagram_ops::IP_MULTICAST_LOOP
    ) {
        let enabled = if option == datagram_ops::IP_MULTICAST_LOOP {
            value == 0
        } else {
            value != 0
        };
        return Ok(Some(
            thread
                .object("java.lang.Boolean", "Z", &[Value::Int(i32::from(enabled))])
                .await?,
        ));
    }
    Ok(Some(
        thread
            .object("java.lang.Integer", "I", &[Value::Int(value)])
            .await?,
    ))
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_address<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    let address =
        socket_ops::local_address(thread.vm()?.as_ref(), fd_from_parameters(&mut parameters)?)
            .await?;
    Ok(Some(
        java_inet_address(&thread, inet_address_from_socket(address), None).await?,
    ))
}

#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketNativeSetOption(ILjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_native_set_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let option = parameters.pop_int()?;
    let fd = fd_from_parameters(&mut parameters)?;
    if option == SO_TIMEOUT {
        socket_ops::set_timeout(thread.vm()?.as_ref(), fd, boxed_int_value(&value)?).await?;
    } else {
        let vm = thread.vm()?;
        match option {
            datagram_ops::SO_BINDADDR => {
                return Err(ristretto_types::JavaError::SocketException(
                    "Cannot re-bind Socket".to_string(),
                )
                .into());
            }
            datagram_ops::IP_MULTICAST_IF => {
                let address = inet_address_value(&value)?;
                let is_ipv6 = vm
                    .socket_handles()
                    .get(&fd)
                    .await
                    .is_some_and(|handle| handle.is_ipv6);
                let (option, interface) = match (is_ipv6, address) {
                    (true, InetAddressValue::V4(address)) if address.is_unspecified() => {
                        (datagram_ops::IP_MULTICAST_IF2, 0)
                    }
                    (true, address) => {
                        let index = match address {
                            InetAddressValue::V6(_, scope) if scope != 0 => {
                                i32::try_from(scope).unwrap_or(i32::MAX)
                            }
                            _ => super::networkinterface::interface_index_by_address(address)
                                .ok_or_else(|| {
                                    ristretto_types::JavaError::SocketException(
                                        "Network interface not found for multicast address"
                                            .to_string(),
                                    )
                                })?,
                        };
                        (datagram_ops::IP_MULTICAST_IF2, index)
                    }
                    (false, InetAddressValue::V4(address)) => (
                        datagram_ops::IP_MULTICAST_IF,
                        i32::from_ne_bytes(u32::from(address).to_ne_bytes()),
                    ),
                    (false, InetAddressValue::V6(_, scope)) => (
                        datagram_ops::IP_MULTICAST_IF2,
                        i32::try_from(scope).unwrap_or(i32::MAX),
                    ),
                };
                datagram_ops::set_int_option(vm.as_ref(), fd, option, interface).await?;
            }
            datagram_ops::IP_MULTICAST_IF2 => {
                let is_ipv6 = vm
                    .socket_handles()
                    .get(&fd)
                    .await
                    .is_some_and(|handle| handle.is_ipv6);
                let (option, interface) = if is_ipv6 {
                    (
                        datagram_ops::IP_MULTICAST_IF2,
                        multicast_interface_index(&value)?,
                    )
                } else {
                    let address = multicast_interface_v4(&value)?;
                    (
                        datagram_ops::IP_MULTICAST_IF,
                        i32::from_ne_bytes(u32::from(address).to_ne_bytes()),
                    )
                };
                datagram_ops::set_int_option(vm.as_ref(), fd, option, interface).await?;
            }
            datagram_ops::IP_MULTICAST_LOOP => {
                let disabled = boxed_int_value(&value)? != 0;
                datagram_ops::set_int_option(vm.as_ref(), fd, option, i32::from(!disabled)).await?;
            }
            _ => {
                datagram_ops::set_int_option(vm.as_ref(), fd, option, boxed_int_value(&value)?)
                    .await?;
            }
        }
    }
    Ok(None)
}

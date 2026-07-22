use crate::bounds;
use crate::java::io::socketfiledescriptor::get_impl_fd;
use crate::java::net::{datagram_ops, socket_ops};
use crate::net_helpers::{inet_address_from_socket, inet_address_value, java_inet_address};
use ristretto_classfile::VersionSpecification::{Between, Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{JavaError, Parameters, Result, Thread};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = get_impl_fd(&parameters.pop()?)?;
    Ok(Some(Value::Int(
        socket_ops::available(thread.vm()?.as_ref(), fd).await?,
    )))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.initIDs()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketBind(ILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_bind<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let exclusive = parameters.pop_bool()?;
    let port = parameters.pop_int()?;
    let address = inet_address_value(&parameters.pop()?)?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    socket_ops::set_exclusive_bind(vm.as_ref(), fd, exclusive).await?;
    datagram_ops::bind(vm.as_ref(), fd, address, port).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketClose(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_close<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    socket_ops::close(thread.vm()?.as_ref(), parameters.pop_int()?).await;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketConnect(ILjava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let address = inet_address_value(&parameters.pop()?)?;
    let fd = parameters.pop_int()?;
    datagram_ops::connect(thread.vm()?.as_ref(), fd, address, port).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketCreate()I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn socket_create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(
        datagram_ops::create(thread.vm()?.as_ref()).await?,
    )))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketCreate(Z)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_create_windows_v8<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _exclusive = parameters.pop_bool()?;
    socket_create(thread, parameters).await
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketDisconnect(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_disconnect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    socket_ops::disconnect_udp(thread.vm()?.as_ref(), parameters.pop_int()?).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketGetIntOption(II)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let option = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        datagram_ops::get_int_option(thread.vm()?.as_ref(), fd, option).await?,
    )))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_address<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = socket_ops::local_address(thread.vm()?.as_ref(), parameters.pop_int()?).await?;
    Ok(Some(
        java_inet_address(&thread, inet_address_from_socket(address), None).await?,
    ))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketLocalPort(I)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = socket_ops::local_address(thread.vm()?.as_ref(), parameters.pop_int()?).await?;
    Ok(Some(Value::Int(i32::from(address.port()))))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketReceiveOrPeekData(ILjava/net/DatagramPacket;IZZ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_receive_or_peek_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let peek = parameters.pop_bool()?;
    let _connected = parameters.pop_bool()?;
    let timeout = parameters.pop_int()?;
    let packet = parameters.pop()?;
    let fd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        datagram_ops::receive_packet(&thread, fd, &packet, Some(timeout), peek).await?,
    )))
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketSend(I[BIILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_send<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let connected = parameters.pop_bool()?;
    let port = parameters.pop_int()?;
    let address = parameters.pop()?;
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let data = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let data = {
        let data = data.as_byte_vec_ref()?;
        let end = offset
            .checked_add(length)
            .ok_or_else(|| JavaError::SocketException("Datagram range overflow".to_string()))?;
        bounds::range(&data, offset..end, "datagram send buffer")?
            .iter()
            .map(|byte| byte.to_ne_bytes()[0])
            .collect::<Vec<_>>()
    };
    let target = if connected {
        None
    } else {
        Some((inet_address_value(&address)?, port))
    };
    datagram_ops::send_bytes(thread.vm()?.as_ref(), fd, &data, target).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketSetIntOption(III)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let option = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    datagram_ops::set_int_option(thread.vm()?.as_ref(), fd, option, value).await?;
    Ok(None)
}

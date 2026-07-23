#[cfg(target_family = "unix")]
use crate::java::io::socketfiledescriptor::{get_impl_fd, set_impl_fd};
use crate::java::net::socket_ops;
#[cfg(target_family = "unix")]
use crate::net_helpers::{
    boxed_int_value, close_socket, inet_address_from_socket, inet_address_value, java_inet_address,
    std_socket_address,
};
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::Between;
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
#[cfg(target_family = "unix")]
use ristretto_types::VM;
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

const SO_BINDADDR: i32 = 0x000f;
const SO_TIMEOUT: i32 = 0x1006;

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
pub async fn socket_accept<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let accepted = parameters.pop()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let (stream, peer) = socket_ops::accept(vm.as_ref(), fd, None).await?;
    let accepted_fd = socket_ops::insert_accepted(vm.as_ref(), stream, peer.is_ipv6()).await?;
    set_impl_fd(&accepted, accepted_fd)?;
    let address = java_inet_address(&thread, inet_address_from_socket(peer), None).await?;
    let local_port = i32::from(socket_ops::local_address(vm.as_ref(), fd).await?.port());
    let mut object = accepted.as_object_mut()?;
    object.set_value("address", address)?;
    object.set_value("port", Value::Int(i32::from(peer.port())))?;
    object.set_value("localport", Value::Int(local_port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.socketAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_available<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = get_impl_fd(&parameters.pop()?)?;
    Ok(Some(Value::Int(
        socket_ops::available(thread.vm()?.as_ref(), fd).await?,
    )))
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
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let native_address = std_socket_address(inet_address_value(&address)?, port, ipv6)?;
    socket_ops::bind(vm.as_ref(), fd, native_address).await?;
    let local_port = i32::from(socket_ops::local_address(vm.as_ref(), fd).await?.port());
    let mut object = this.as_object_mut()?;
    object.set_value("address", address)?;
    object.set_value("localport", Value::Int(local_port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketClose0(Z)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _deferred = parameters.pop_bool()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    if fd >= 0 {
        close_socket(thread.vm()?.as_ref(), fd).await;
        set_impl_fd(&this, -1)?;
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
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let target = std_socket_address(inet_address_value(&address)?, port, ipv6)?;
    if socket_ops::connect_start(vm.as_ref(), fd, &target).await? == socket_ops::IO_UNAVAILABLE {
        socket_ops::wait_for_connect(vm.as_ref(), fd, timeout).await?;
    }
    let local_port = i32::from(socket_ops::local_address(vm.as_ref(), fd).await?.port());
    let mut object = this.as_object_mut()?;
    object.set_value("address", address)?;
    object.set_value("port", Value::Int(port))?;
    object.set_value("localport", Value::Int(local_port))?;
    Ok(None)
}

#[cfg(target_family = "unix")]
async fn create_instance<T: Thread + 'static>(
    thread: &Arc<T>,
    this: &Value,
    stream: bool,
    prefer_ipv6: bool,
) -> Result<()> {
    let vm = thread.vm()?;
    let fd = if prefer_ipv6 {
        match socket_ops::create(vm.as_ref(), stream, true).await {
            Ok(fd) => fd,
            Err(_) => socket_ops::create(vm.as_ref(), stream, false).await?,
        }
    } else {
        socket_ops::create(vm.as_ref(), stream, false).await?
    };
    set_impl_fd(this, fd)
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
    let prefer_ipv6 = !socket_ops::prefer_ipv4_stack(thread.vm()?.as_ref());
    create_instance(&thread, &this, stream, prefer_ipv6).await?;
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
    create_instance(&thread, &this, stream, prefer_ipv6).await?;
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
    let container = parameters.pop()?;
    let option = parameters.pop_int()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    let vm = thread.vm()?;
    if option == SO_TIMEOUT {
        let value = vm
            .socket_handles()
            .get(&fd)
            .await
            .map_or(0, |handle| handle.timeout_millis());
        return Ok(Some(Value::Int(value)));
    }
    if option == SO_BINDADDR {
        let address = socket_ops::local_address(vm.as_ref(), fd).await?;
        let address = java_inet_address(&thread, inet_address_from_socket(address), None).await?;
        container.as_object_mut()?.set_value("addr", address)?;
        return Ok(Some(Value::Int(0)));
    }
    Ok(Some(Value::Int(
        socket_ops::get_int_option(vm.as_ref(), fd, option).await?,
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("java/net/PlainSocketImpl.socketListen(I)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_listen<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let backlog = parameters.pop_int()?;
    let fd = get_impl_fd(&parameters.pop()?)?;
    socket_ops::listen(thread.vm()?.as_ref(), fd, backlog).await?;
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
    let fd = get_impl_fd(&parameters.pop()?)?;
    socket_ops::send_oob(thread.vm()?.as_ref(), fd, data).await?;
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
    let enabled = parameters.pop_bool()?;
    let option = parameters.pop_int()?;
    let fd = get_impl_fd(&parameters.pop()?)?;
    let vm = thread.vm()?;
    let value = boxed_int_value(&value)?;
    if option == SO_TIMEOUT {
        socket_ops::set_timeout(vm.as_ref(), fd, value).await?;
    } else {
        let value = if option == socket_ops::SO_LINGER && !enabled {
            -1
        } else if matches!(
            option,
            socket_ops::TCP_NODELAY
                | socket_ops::SO_REUSEADDR
                | socket_ops::SO_REUSEPORT
                | socket_ops::SO_KEEPALIVE
                | socket_ops::SO_OOBINLINE
        ) {
            i32::from(enabled)
        } else {
            value
        };
        socket_ops::set_int_option(vm.as_ref(), fd, option, value).await?;
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
    let fd = get_impl_fd(&parameters.pop()?)?;
    socket_ops::shutdown(thread.vm()?.as_ref(), fd, how).await?;
    Ok(None)
}

// Windows 11-17 uses the same static native contract as Java 8's dual-stack
// implementation. Keep these as signature adapters over the shared core.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::accept0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.available0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn available0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::available0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::bind0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.close0(I)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::close0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.configureBlocking(IZ)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::configure_blocking(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::connect0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.getIntOption(II)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn get_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::get_int_option(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.listen0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn listen0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::listen0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn local_address<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::local_address(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.localPort0(I)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn local_port0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::local_port0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.sendOOB(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::send_oob(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setIntOption(III)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::set_int_option(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.shutdown0(II)V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::shutdown0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForConnect(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_connect<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::wait_for_connect(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.waitForNewConnection(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn wait_for_new_connection<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::dualstackplainsocketimpl::wait_for_new_connection(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.initIDs()V", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/net/PlainSocketImpl.setSoTimeout0(II)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_so_timeout0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::set_timeout(thread.vm()?.as_ref(), fd, timeout).await?;
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method("java/net/PlainSocketImpl.socket0(Z)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn socket0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_bool()?;
    Ok(Some(Value::Int(
        socket_ops::create_preferred(thread.vm()?.as_ref(), stream).await?,
    )))
}

#[cfg(all(test, target_family = "unix"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_is_a_no_op() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        assert_eq!(None, init_proto(thread, Parameters::default()).await?);
        Ok(())
    }
}

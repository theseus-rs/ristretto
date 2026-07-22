use crate::java::io::socketfiledescriptor::{get_impl_fd, set_impl_fd};
use crate::java::net::socket_ops;
use crate::net_helpers::{
    boxed_int_value, inet_address_from_socket, inet_address_value, java_inet_address,
    std_socket_address,
};
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;

const SO_BINDADDR: i32 = 0x000f;
const SO_TIMEOUT: i32 = 0x1006;

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.initProto()V", Equal(JAVA_8))]
#[async_method]
pub async fn init_proto<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V",
    Equal(JAVA_8)
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

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketAvailable()I", Equal(JAVA_8))]
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

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketBind(Ljava/net/InetAddress;IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_bind<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let exclusive = parameters.pop_bool()?;
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
    let native = std_socket_address(inet_address_value(&address)?, port, ipv6)?;
    socket_ops::set_exclusive_bind(vm.as_ref(), fd, exclusive).await?;
    socket_ops::bind(vm.as_ref(), fd, native).await?;
    let port = i32::from(socket_ops::local_address(vm.as_ref(), fd).await?.port());
    let mut object = this.as_object_mut()?;
    object.set_value("address", address)?;
    object.set_value("localport", Value::Int(port))?;
    Ok(None)
}

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketClose0(Z)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _deferred = parameters.pop_bool()?;
    let this = parameters.pop()?;
    let fd = get_impl_fd(&this)?;
    if fd >= 0 {
        socket_ops::close(thread.vm()?.as_ref(), fd).await;
        set_impl_fd(&this, -1)?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V",
    Equal(JAVA_8)
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

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketCreate(Z)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_create<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_bool()?;
    let this = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = socket_ops::create_preferred(vm.as_ref(), stream).await?;
    set_impl_fd(&this, fd)?;
    // The Windows JDK checks whether `fd1` itself is null to distinguish a
    // single dual-stack descriptor from its historical two-socket setup.
    if this.as_object_ref()?.value("fd1").is_ok() {
        this.as_object_mut()?
            .set_value("fd1", Value::Object(None))?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketGetOption(ILjava/lang/Object;)I",
    Equal(JAVA_8)
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
        return Ok(Some(Value::Int(
            vm.socket_handles()
                .get(&fd)
                .await
                .map_or(0, |handle| handle.timeout_millis()),
        )));
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

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketListen(I)V", Equal(JAVA_8))]
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

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketNativeSetOption(IZLjava/lang/Object;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_native_set_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let enabled = parameters.pop_bool()?;
    let option = parameters.pop_int()?;
    let fd = get_impl_fd(&parameters.pop()?)?;
    let value = boxed_int_value(&value)?;
    if option == SO_TIMEOUT {
        socket_ops::set_timeout(thread.vm()?.as_ref(), fd, value).await?;
    } else {
        let value = if option == socket_ops::SO_LINGER && !enabled {
            -1
        } else if matches!(
            option,
            socket_ops::TCP_NODELAY
                | socket_ops::SO_REUSEADDR
                | socket_ops::SO_KEEPALIVE
                | socket_ops::SO_OOBINLINE
        ) {
            i32::from(enabled)
        } else {
            value
        };
        socket_ops::set_int_option(thread.vm()?.as_ref(), fd, option, value).await?;
    }
    Ok(None)
}

#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketSendUrgentData(I)V",
    Equal(JAVA_8)
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

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketShutdown(I)V", Equal(JAVA_8))]
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

use crate::java::net::socket_ops;
use crate::net_helpers::{
    inet_address_from_socket, inet_address_value, java_inet_address, java_inet_socket_address,
    std_socket_address,
};
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.accept0(I[Ljava/net/InetSocketAddress;)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let addresses = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let accepted = socket_ops::accept(vm.as_ref(), fd, None).await;
    // waitForNewConnection stores a one-shot timeout for this accept because Tokio does not
    // expose the legacy native's non-consuming listener wait. Do not leak it into later accepts.
    let _ = socket_ops::set_timeout(vm.as_ref(), fd, 0).await;
    let (stream, address) = accepted?;
    let accepted_fd = socket_ops::insert_accepted(vm.as_ref(), stream, address.is_ipv6()).await?;
    let java_address = java_inet_socket_address(&thread, address).await?;
    let (_, mut addresses) = addresses.as_class_vec_mut()?;
    if let Some(target) = addresses.first_mut() {
        *target = java_address;
    }
    Ok(Some(Value::Int(accepted_fd)))
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.available0(I)I", Equal(JAVA_8))]
#[async_method]
pub async fn available0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        socket_ops::available(thread.vm()?.as_ref(), fd).await?,
    )))
}

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.bind0(ILjava/net/InetAddress;IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let exclusive_bind = parameters.pop_bool()?;
    let port = parameters.pop_int()?;
    let address = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let address = std_socket_address(inet_address_value(&address)?, port, ipv6)?;
    socket_ops::set_exclusive_bind(vm.as_ref(), fd, exclusive_bind).await?;
    socket_ops::bind(vm.as_ref(), fd, address).await?;
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.close0(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    socket_ops::close(thread.vm()?.as_ref(), parameters.pop_int()?).await;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.configureBlocking(IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let blocking = parameters.pop_bool()?;
    let fd = parameters.pop_int()?;
    socket_ops::configure_blocking(thread.vm()?.as_ref(), fd, blocking).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.connect0(ILjava/net/InetAddress;I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let address = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let ipv6 = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|handle| handle.is_ipv6);
    let address = std_socket_address(inet_address_value(&address)?, port, ipv6)?;
    Ok(Some(Value::Int(
        socket_ops::connect_start(vm.as_ref(), fd, &address).await?,
    )))
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.getIntOption(II)I", Equal(JAVA_8))]
#[async_method]
pub async fn get_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let option = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        socket_ops::get_int_option(thread.vm()?.as_ref(), fd, option).await?,
    )))
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.initIDs()V", Equal(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.listen0(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn listen0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let backlog = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::listen(thread.vm()?.as_ref(), fd, backlog).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.localAddress(ILjava/net/InetAddressContainer;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn local_address<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let container = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let address = socket_ops::local_address(thread.vm()?.as_ref(), fd).await?;
    let address = java_inet_address(&thread, inet_address_from_socket(address), None).await?;
    container.as_object_mut()?.set_value("addr", address)?;
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.localPort0(I)I", Equal(JAVA_8))]
#[async_method]
pub async fn local_port0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = socket_ops::local_address(thread.vm()?.as_ref(), parameters.pop_int()?).await?;
    Ok(Some(Value::Int(i32::from(address.port()))))
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.sendOOB(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn send_oob<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::send_oob(thread.vm()?.as_ref(), fd, data).await?;
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.setIntOption(III)V", Equal(JAVA_8))]
#[async_method]
pub async fn set_int_option<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let option = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::set_int_option(thread.vm()?.as_ref(), fd, option, value).await?;
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.shutdown0(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let how = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::shutdown(thread.vm()?.as_ref(), fd, how).await?;
    Ok(None)
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.socket0(ZZ)I", Equal(JAVA_8))]
#[async_method]
pub async fn socket0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let v6_only = parameters.pop_bool()?;
    let stream = parameters.pop_bool()?;
    let vm = thread.vm()?;
    let fd = socket_ops::create(vm.as_ref(), stream, true).await?;
    if let Err(error) = socket_ops::set_only_v6(vm.as_ref(), fd, v6_only).await {
        socket_ops::close(vm.as_ref(), fd).await;
        return Err(error);
    }
    Ok(Some(Value::Int(fd)))
}

#[intrinsic_method("java/net/DualStackPlainSocketImpl.waitForConnect(II)V", Equal(JAVA_8))]
#[async_method]
pub async fn wait_for_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    socket_ops::wait_for_connect(thread.vm()?.as_ref(), fd, timeout).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/net/DualStackPlainSocketImpl.waitForNewConnection(II)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn wait_for_new_connection<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    // Tokio listeners stay non-blocking internally. Preserve the native
    // two-step contract by applying this timeout to the immediately following
    // accept, which performs the actual readiness wait.
    socket_ops::set_timeout(thread.vm()?.as_ref(), fd, timeout).await?;
    Ok(None)
}

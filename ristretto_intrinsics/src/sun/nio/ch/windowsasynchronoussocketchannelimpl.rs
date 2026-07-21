use crate::net_helpers::{inet_address_ipv4, ipv6_from_java_bytes, socket_from_type};
use crate::sun::nio::ch::iocp::{
    CompletionTarget, begin_operation, io_error_code, is_associated, mark_closed,
    operation_is_open, post_operation,
};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::native_memory::NativeMemory;
use ristretto_types::{Parameters, Result, VM};
use socket2::SockAddr;
use std::mem::size_of;
use std::net::{Shutdown, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;
use std::time::Duration;

const IO_UNAVAILABLE: i32 = -2;
const ERROR_OPERATION_ABORTED: i32 = 995;
const MAX_WSABUF: i32 = 16;

fn socket_exception(operation: &str, error: impl std::fmt::Display) -> ristretto_types::Error {
    JavaError::IoException(format!("{operation} failed: {error}")).into()
}

fn parse_wsa_buffers(memory: &NativeMemory, address: i64, count: i32) -> Result<Vec<(i64, usize)>> {
    if !(0..=MAX_WSABUF).contains(&count) {
        return Err(socket_exception("WSABUF", "invalid buffer count"));
    }
    let pointer_size = size_of::<usize>();
    let entry_size = pointer_size
        .checked_mul(2)
        .ok_or_else(|| InternalError("WSABUF entry size overflow".to_string()))?;
    let mut buffers = Vec::with_capacity(usize::try_from(count)?);
    for index in 0..count {
        let entry = address
            .checked_add(i64::from(index) * i64::try_from(entry_size)?)
            .ok_or_else(|| InternalError("WSABUF address overflow".to_string()))?;
        let length = memory
            .read_i32(entry)
            .ok_or_else(|| socket_exception("WSABUF", "invalid length address"))?;
        let length = usize::try_from(length)
            .map_err(|_| socket_exception("WSABUF", "negative buffer length"))?;
        let buffer_address = if pointer_size == 8 {
            memory.read_i64(entry + 8)
        } else {
            memory.read_i32(entry + 4).map(i64::from)
        }
        .ok_or_else(|| socket_exception("WSABUF", "invalid buffer address"))?;
        if length > 0 && memory.read_with(buffer_address, length, |_| ()).is_none() {
            return Err(socket_exception("WSABUF", "invalid native buffer"));
        }
        buffers.push((buffer_address, length));
    }
    Ok(buffers)
}

fn copy_to_wsa_buffers(memory: &NativeMemory, buffers: &[(i64, usize)], data: &[u8]) {
    let mut offset = 0usize;
    for (address, length) in buffers {
        if offset >= data.len() {
            break;
        }
        let count = (*length).min(data.len().saturating_sub(offset));
        if count == 0 {
            continue;
        }
        if let Some(bytes) = data.get(offset..offset + count) {
            memory.write_bytes(*address, bytes);
        }
        offset += count;
    }
}

fn collect_wsa_buffers(memory: &NativeMemory, buffers: &[(i64, usize)]) -> Result<Vec<u8>> {
    let capacity = buffers.iter().try_fold(0usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| socket_exception("WSABUF", "total length overflow"))
    })?;
    let mut data = Vec::new();
    data.try_reserve_exact(capacity)
        .map_err(|error| socket_exception("WSABUF", error))?;
    for (address, length) in buffers {
        if *length == 0 {
            continue;
        }
        let bytes = memory
            .try_read_bytes(*address, *length)
            .ok_or_else(|| socket_exception("WSABUF", "invalid native buffer"))?;
        data.extend_from_slice(&bytes);
    }
    Ok(data)
}

fn connect_address(inet_address: &Value, use_ipv6: bool, port: u16) -> Result<SockAddr> {
    let is_ipv6_address = inet_address
        .as_object_ref()?
        .class()
        .name()
        .ends_with("Inet6Address");
    if is_ipv6_address {
        let holder = inet_address.as_object_ref()?.value("holder6")?;
        let holder = holder.as_object_ref()?;
        let bytes = holder.value("ipaddress")?;
        let bytes = bytes.as_byte_vec_ref()?;
        let address = ipv6_from_java_bytes(bytes.as_ref())
            .ok_or_else(|| socket_exception("ConnectEx", "invalid IPv6 address"))?;
        let scope_id = u32::try_from(holder.value("scope_id")?.as_i32()?).unwrap_or(0);
        if use_ipv6 {
            return Ok(SockAddr::from(SocketAddrV6::new(
                address, port, 0, scope_id,
            )));
        }
        let ipv4 = address.to_ipv4_mapped().ok_or_else(|| {
            socket_exception("ConnectEx", "IPv6 address used with an IPv4 socket")
        })?;
        return Ok(SockAddr::from(SocketAddrV4::new(ipv4, port)));
    }

    let ipv4 = inet_address_ipv4(inet_address)?;
    if use_ipv6 {
        Ok(SockAddr::from(SocketAddrV6::new(
            ipv4.to_ipv6_mapped(),
            port,
            0,
            0,
        )))
    } else {
        Ok(SockAddr::from(SocketAddrV4::new(ipv4, port)))
    }
}

async fn validate_socket<V: VM + ?Sized>(vm: &V, socket: i64, operation: &str) -> Result<i32> {
    let fd = i32::try_from(socket).map_err(|_| socket_exception(operation, "invalid socket"))?;
    if vm.socket_handles().get(&fd).await.is_none() {
        return Err(socket_exception(operation, "invalid socket"));
    }
    if !is_associated(vm, socket)? {
        return Err(socket_exception(
            operation,
            "socket is not associated with an I/O completion port",
        ));
    }
    Ok(fd)
}

fn post_if_open<V: VM + ?Sized>(
    vm: &V,
    target: CompletionTarget,
    error: i32,
    bytes: usize,
    overlapped: i64,
) {
    let error = if operation_is_open(vm, target).unwrap_or(false) {
        error
    } else {
        ERROR_OPERATION_ABORTED
    };
    let _ = post_operation(vm, target, error, bytes, overlapped);
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.closesocket0(J)V",
    Any
)]
#[async_method]
pub async fn closesocket0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let socket = parameters.pop_long()?;
    let fd =
        i32::try_from(socket).map_err(|_| socket_exception("closesocket", "invalid socket"))?;
    let vm = thread.vm()?;
    if vm.socket_handles().get(&fd).await.is_none() {
        return Err(socket_exception("closesocket", "invalid socket"));
    }
    mark_closed(vm.as_ref(), socket);
    vm.socket_handles().remove(&fd).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.connect0(JZLjava/net/InetAddress;IJ)I",
    Any
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn connect0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let port = parameters.pop_int()?;
    let inet_address = parameters
        .pop_reference()?
        .ok_or_else(|| socket_exception("ConnectEx", "null address"))?;
    let prefer_ipv6 = parameters.pop_bool()?;
    let socket = parameters.pop_long()?;
    let port = u16::try_from(port).map_err(|_| socket_exception("ConnectEx", "invalid port"))?;
    let inet_address = Value::Object(Some(inet_address));
    let vm = thread.vm()?;
    let fd = validate_socket(vm.as_ref(), socket, "ConnectEx").await?;

    let (raw_socket, use_ipv6) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| socket_exception("ConnectEx", "invalid socket"))?;
        let raw = handle
            .socket_type
            .as_raw()
            .ok_or_else(|| socket_exception("ConnectEx", "socket is already connected"))?;
        (
            raw.try_clone()
                .map_err(|error| socket_exception("ConnectEx", error))?,
            handle.is_ipv6 || prefer_ipv6,
        )
    };
    let address = connect_address(&inet_address, use_ipv6, port)?;
    raw_socket
        .set_nonblocking(true)
        .map_err(|error| socket_exception("ConnectEx", error))?;
    let pending = match raw_socket.connect(&address) {
        Ok(()) => false,
        Err(error)
            if error.kind() == std::io::ErrorKind::WouldBlock
                || matches!(error.raw_os_error(), Some(997 | 10035 | 10036 | 10037)) =>
        {
            true
        }
        Err(error) => return Err(socket_exception("ConnectEx", error)),
    };
    let probe: std::net::TcpStream = raw_socket.into();
    let probe = tokio::net::TcpStream::from_std(probe)
        .map_err(|error| socket_exception("ConnectEx", error))?;
    let target = begin_operation(vm.as_ref(), socket)?;

    tokio::spawn(async move {
        let result = if pending {
            loop {
                tokio::select! {
                    ready = probe.writable() => {
                        match ready {
                            Ok(()) => break probe.take_error(),
                            Err(error) => break Err(error),
                        }
                    }
                    () = tokio::time::sleep(Duration::from_millis(10)) => {
                        if vm.socket_handles().get(&fd).await.is_none() {
                            let _ = post_operation(
                                vm.as_ref(),
                                target,
                                ERROR_OPERATION_ABORTED,
                                0,
                                overlapped,
                            );
                            return;
                        }
                    }
                }
            }
        } else {
            Ok(None)
        };
        match result {
            Ok(None) => {
                let Some(handle) = vm.socket_handles().remove(&fd).await else {
                    let _ =
                        post_operation(vm.as_ref(), target, ERROR_OPERATION_ABORTED, 0, overlapped);
                    return;
                };
                let SocketHandle {
                    socket_type,
                    timeout,
                    is_ipv6,
                    ..
                } = handle;
                if !matches!(socket_type, SocketType::Raw(_)) {
                    let _ = post_operation(vm.as_ref(), target, 10022, 0, overlapped);
                    return;
                }
                let replacement = SocketHandle {
                    socket_type: SocketType::TcpStream(Arc::new(probe)),
                    timeout,
                    is_ipv6,
                    is_unix: false,
                    non_blocking: true,
                };
                if vm.socket_handles().insert(fd, replacement).await.is_err() {
                    let _ =
                        post_operation(vm.as_ref(), target, ERROR_OPERATION_ABORTED, 0, overlapped);
                    return;
                }
                let _ = post_operation(vm.as_ref(), target, 0, 0, overlapped);
            }
            Ok(Some(error)) | Err(error) => {
                post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.read0(JIJJ)I", Any)]
#[async_method]
pub async fn read0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let socket = parameters.pop_long()?;
    let vm = thread.vm()?;
    let fd = validate_socket(vm.as_ref(), socket, "WSARecv").await?;
    let buffers = parse_wsa_buffers(vm.native_memory(), address, count)?;
    let total_length = buffers.iter().try_fold(0usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| socket_exception("WSARecv", "total buffer length overflow"))
    })?;
    let stream = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| socket_exception("WSARecv", "invalid socket"))?;
        handle
            .socket_type
            .as_tcp_stream()
            .ok_or_else(|| socket_exception("WSARecv", "socket is not connected"))?
            .clone()
    };
    let mut data = Vec::new();
    data.try_reserve_exact(total_length)
        .map_err(|error| socket_exception("WSARecv", error))?;
    data.resize(total_length, 0);
    let target = begin_operation(vm.as_ref(), socket)?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                ready = stream.readable() => {
                    if let Err(error) = ready {
                        post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
                        return;
                    }
                    match stream.try_read(&mut data) {
                        Ok(bytes_read) => {
                            if let Some(bytes) = data.get(..bytes_read) {
                                copy_to_wsa_buffers(vm.native_memory(), &buffers, bytes);
                            }
                            post_if_open(vm.as_ref(), target, 0, bytes_read, overlapped);
                            return;
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                        Err(error) => {
                            post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
                            return;
                        }
                    }
                }
                () = tokio::time::sleep(Duration::from_millis(10)) => {
                    if vm.socket_handles().get(&fd).await.is_none() {
                        let _ = post_operation(vm.as_ref(), target, ERROR_OPERATION_ABORTED, 0, overlapped);
                        return;
                    }
                }
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.shutdown0(JI)V", Any)]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let how = parameters.pop_int()?;
    let socket = parameters.pop_long()?;
    let fd = i32::try_from(socket).map_err(|_| socket_exception("shutdown", "invalid socket"))?;
    let vm = thread.vm()?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| socket_exception("shutdown", "invalid socket"))?;
    let how = match how {
        0 => Shutdown::Read,
        1 => Shutdown::Write,
        2 => Shutdown::Both,
        _ => return Err(socket_exception("shutdown", "invalid shutdown mode")),
    };
    socket_from_type(&handle.socket_type)
        .shutdown(how)
        .map_err(|error| socket_exception("shutdown", error))?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.updateConnectContext(J)V",
    Any
)]
#[async_method]
pub async fn update_connect_context<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let socket = parameters.pop_long()?;
    let fd = i32::try_from(socket)
        .map_err(|_| socket_exception("SO_UPDATE_CONNECT_CONTEXT", "invalid socket"))?;
    let vm = thread.vm()?;
    let handle = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| socket_exception("SO_UPDATE_CONNECT_CONTEXT", "invalid socket"))?;
    if handle.socket_type.as_tcp_stream().is_none() {
        return Err(socket_exception(
            "SO_UPDATE_CONNECT_CONTEXT",
            "socket is not connected",
        ));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.write0(JIJJ)I", Any)]
#[async_method]
pub async fn write0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let overlapped = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let socket = parameters.pop_long()?;
    let vm = thread.vm()?;
    let fd = validate_socket(vm.as_ref(), socket, "WSASend").await?;
    let buffers = parse_wsa_buffers(vm.native_memory(), address, count)?;
    let data = collect_wsa_buffers(vm.native_memory(), &buffers)?;
    let stream = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| socket_exception("WSASend", "invalid socket"))?;
        handle
            .socket_type
            .as_tcp_stream()
            .ok_or_else(|| socket_exception("WSASend", "socket is not connected"))?
            .clone()
    };
    let target = begin_operation(vm.as_ref(), socket)?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                ready = stream.writable() => {
                    if let Err(error) = ready {
                        post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
                        return;
                    }
                    match stream.try_write(&data) {
                        Ok(bytes_written) => {
                            post_if_open(vm.as_ref(), target, 0, bytes_written, overlapped);
                            return;
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => {
                            post_if_open(vm.as_ref(), target, 10058, 0, overlapped);
                            return;
                        }
                        Err(error) => {
                            post_if_open(vm.as_ref(), target, io_error_code(&error), 0, overlapped);
                            return;
                        }
                    }
                }
                () = tokio::time::sleep(Duration::from_millis(10)) => {
                    if vm.socket_handles().get(&fd).await.is_none() {
                        let _ = post_operation(vm.as_ref(), target, ERROR_OPERATION_ABORTED, 0, overlapped);
                        return;
                    }
                }
            }
        }
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_closesocket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = closesocket0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.unwrap_err().to_string().contains("null address"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(None, result.expect("initIDs"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown0(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0_write_side() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let connect = tokio::spawn(tokio::net::TcpStream::connect(address));
        let (mut peer, _) = listener.accept().await?;
        let client = connect.await.map_err(std::io::Error::other)??;
        let fd = 7_701;
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(client))),
            )
            .await?;

        assert_eq!(
            None,
            shutdown0(
                thread,
                Parameters::new(vec![Value::Long(i64::from(fd)), Value::Int(1)]),
            )
            .await?
        );
        let mut byte = [0u8; 1];
        let read = tokio::time::timeout(Duration::from_secs(1), peer.read(&mut byte))
            .await
            .map_err(std::io::Error::other)??;
        assert_eq!(0, read);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_connect_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_connect_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }
}

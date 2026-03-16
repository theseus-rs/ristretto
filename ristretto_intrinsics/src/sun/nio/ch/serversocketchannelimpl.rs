use crate::java::io::socketfiledescriptor::{get_fd, set_fd};
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/ServerSocketChannelImpl.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn accept_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let isas = parameters.pop()?;
    let new_fd_value = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let vm = thread.vm()?;

    // Check variant, then release lock before blocking accept
    let is_tcp_listener = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        guard.socket_type.as_tcp_listener().is_some()
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

        let result: std::result::Result<(tokio::net::TcpStream, std::net::SocketAddr), _> =
            listener.accept().await;

        match result {
            Ok((stream, peer_addr)) => {
                let addr = socket2::SockAddr::from(peer_addr);
                (Some(stream), addr)
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return Ok(Some(Value::Int(-2)));
            }
            Err(e) => return Err(InternalError(format!("accept: {e}"))),
        }
    } else {
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
        drop(guard);

        let accept_result = tokio::task::spawn_blocking(move || cloned.accept())
            .await
            .map_err(|e| InternalError(format!("accept: spawn: {e}")))?;

        match accept_result {
            Ok((new_socket, addr)) => {
                new_socket
                    .set_nonblocking(true)
                    .map_err(|e| InternalError(format!("accept: set_nonblocking: {e}")))?;
                let std_stream: std::net::TcpStream = new_socket.into();
                let stream = tokio::net::TcpStream::from_std(std_stream)
                    .map_err(|e| InternalError(format!("accept: from_std: {e}")))?;
                (Some(stream), addr)
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return Ok(Some(Value::Int(-2)));
            }
            Err(e) => return Err(InternalError(format!("accept: {e}"))),
        }
    };

    let new_fd = vm.next_nio_fd();
    set_fd(&new_fd_value, new_fd)?;

    if let Some(stream) = tokio_stream {
        vm.socket_handles()
            .insert(
                new_fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(stream))),
            )
            .await?;
    }

    // Fill in isas[0] with the remote address
    let (addr_bytes, port): (Vec<u8>, u16) = if let Some(v4) = addr.as_socket_ipv4() {
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
    let port_value = Value::Int(i32::from(port));
    let isa = thread
        .object(
            "java.net.InetSocketAddress",
            "Ljava/net/InetAddress;I",
            &[inet_addr, port_value],
        )
        .await?;
    let mut guard = isas.as_reference_mut()?;
    if let Reference::Array(object_array) = &mut *guard
        && !object_array.elements.is_empty()
    {
        object_array.elements[0] = isa;
    }

    Ok(Some(Value::Int(new_fd)))
}

#[intrinsic_method(
    "sun/nio/ch/ServerSocketChannelImpl.initIDs()V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

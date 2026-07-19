use super::sctpnet::{build_socket_address, duplicate_socket, socket_info};
use crate::java::io::socketfiledescriptor::set_fd;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::Error::InternalError;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{JavaError, Parameters, Result, Thread, VM};
use socket2::{SockAddr, Socket};
use std::sync::Arc;

#[expect(
    clippy::needless_pass_by_value,
    reason = "ownership keeps the duplicated listener alive for the entire blocking operation"
)]
fn accept_socket(
    listener: Socket,
    non_blocking: bool,
) -> std::io::Result<Option<(Socket, SockAddr)>> {
    loop {
        match listener.accept() {
            Ok(connection) => return Ok(Some(connection)),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if non_blocking {
                    return Ok(None);
                }
                let mut descriptor = libc::pollfd {
                    fd: std::os::fd::AsRawFd::as_raw_fd(&listener),
                    events: libc::POLLIN,
                    revents: 0,
                };
                #[expect(unsafe_code)]
                // SAFETY: descriptor is writable and the array length is exactly one.
                let result = unsafe { libc::poll(&raw mut descriptor, 1, -1) };
                if result < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Err(error) if error.raw_os_error() == Some(libc::ECONNABORTED) => {}
            Err(error) if error.raw_os_error() == Some(libc::EINTR) => {
                return Err(error);
            }
            Err(error) => return Err(error),
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpServerChannelImpl.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let addresses = parameters.pop()?;
    let new_fd_value = parameters.pop()?;
    let server_fd_value = parameters.pop()?;
    let server_fd = crate::java::io::socketfiledescriptor::get_fd(&server_fd_value)?;
    let vm = thread.vm()?;
    let info = socket_info(&vm, server_fd).await?;
    let listener = duplicate_socket(info.raw_fd, "duplicate SCTP listener for accept")?;
    let accepted = tokio::task::spawn_blocking(move || accept_socket(listener, info.non_blocking))
        .await
        .map_err(|error| InternalError(format!("SCTP accept task failed: {error}")))?;
    let accepted = match accepted {
        Ok(accepted) => accepted,
        Err(error) if error.raw_os_error() == Some(libc::EINTR) => {
            return Ok(Some(Value::Int(-3)));
        }
        Err(error) => {
            return Err(JavaError::IoException(format!("SCTP accept failed: {error}")).into());
        }
    };
    let Some((socket, address)) = accepted else {
        return Ok(Some(Value::Int(-2)));
    };

    let (bytes, port, scope_id) = if let Some(address) = address.as_socket_ipv4() {
        (address.ip().octets().to_vec(), address.port(), 0)
    } else if let Some(address) = address.as_socket_ipv6() {
        if let Some(ipv4) = address.ip().to_ipv4_mapped() {
            (ipv4.octets().to_vec(), address.port(), 0)
        } else {
            (
                address.ip().octets().to_vec(),
                address.port(),
                address.scope_id(),
            )
        }
    } else {
        return Err(InternalError(
            "SCTP accept returned a non-IP peer address".to_string(),
        ));
    };

    let new_fd = vm.next_nio_fd();
    set_fd(&new_fd_value, new_fd)?;
    let mut handle = SocketHandle::new(SocketType::Raw(socket));
    handle.is_ipv6 = info.is_ipv6;
    vm.socket_handles().insert(new_fd, handle).await?;

    let address = build_socket_address(&thread, bytes, port, scope_id).await?;
    let mut guard = addresses.as_reference_mut()?;
    let Reference::Array(array) = &mut *guard else {
        return Err(InternalError(
            "accept0 expected InetSocketAddress[]".to_string(),
        ));
    };
    let Some(first) = array.elements.first_mut() else {
        return Err(InternalError(
            "accept0 requires a one-element address array".to_string(),
        ));
    };
    *first = address;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpServerChannelImpl.initIDs()V",
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
    async fn test_accept_rejects_missing_parameters() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        assert!(accept0(thread, Parameters::default()).await.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        assert_eq!(None, init_ids(thread, Parameters::default()).await?);
        Ok(())
    }
}

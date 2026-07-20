use crate::sun::nio::ch::iocp::{
    begin_operation, io_error_code, is_associated, mark_closed, post_operation,
};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::time::Duration;

const IO_UNAVAILABLE: i32 = -2;
const ERROR_OPERATION_ABORTED: i32 = 995;

fn socket_exception(operation: &str, error: impl std::fmt::Display) -> ristretto_types::Error {
    JavaError::IoException(format!("{operation} failed: {error}")).into()
}

async fn socket_fd<V: VM + ?Sized>(vm: &V, socket: i64, operation: &str) -> Result<i32> {
    let fd = i32::try_from(socket).map_err(|_| socket_exception(operation, "invalid socket"))?;
    if vm.socket_handles().get(&fd).await.is_none() {
        return Err(socket_exception(operation, "invalid socket"));
    }
    Ok(fd)
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.accept0(JJJJ)I",
    Any
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn accept0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer = parameters.pop_long()?;
    let overlapped = parameters.pop_long()?;
    let accept_socket = parameters.pop_long()?;
    let listen_socket = parameters.pop_long()?;
    let vm = thread.vm()?;
    let listen_fd = socket_fd(vm.as_ref(), listen_socket, "AcceptEx").await?;
    let accept_fd = socket_fd(vm.as_ref(), accept_socket, "AcceptEx").await?;
    if !is_associated(vm.as_ref(), listen_socket)? {
        return Err(socket_exception(
            "AcceptEx",
            "listening socket is not associated with an I/O completion port",
        ));
    }
    if !is_associated(vm.as_ref(), accept_socket)? {
        return Err(socket_exception(
            "AcceptEx",
            "accept socket is not associated with an I/O completion port",
        ));
    }
    let listener = {
        let handle = vm
            .socket_handles()
            .get(&listen_fd)
            .await
            .ok_or_else(|| socket_exception("AcceptEx", "invalid listening socket"))?;
        handle
            .socket_type
            .as_tcp_listener()
            .ok_or_else(|| socket_exception("AcceptEx", "socket is not listening"))?
            .clone()
    };
    {
        let child = vm
            .socket_handles()
            .get(&accept_fd)
            .await
            .ok_or_else(|| socket_exception("AcceptEx", "invalid accept socket"))?;
        if child.socket_type.as_raw().is_none() {
            return Err(socket_exception(
                "AcceptEx",
                "accept socket is already connected",
            ));
        }
    }
    let target = begin_operation(vm.as_ref(), listen_socket)?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                accepted = listener.accept() => {
                    match accepted {
                        Ok((stream, _remote_address)) => {
                            let Some(child) = vm.socket_handles().remove(&accept_fd).await else {
                                let _ = post_operation(
                                    vm.as_ref(),
                                    target,
                                    ERROR_OPERATION_ABORTED,
                                    0,
                                    overlapped,
                                );
                                return;
                            };
                            let SocketHandle { timeout, is_ipv6, .. } = child;
                            let connected = SocketHandle {
                                socket_type: SocketType::TcpStream(Arc::new(stream)),
                                timeout,
                                is_ipv6,
                                non_blocking: true,
                            };
                            if vm.socket_handles().insert(accept_fd, connected).await.is_err() {
                                let _ = post_operation(
                                    vm.as_ref(),
                                    target,
                                    ERROR_OPERATION_ABORTED,
                                    0,
                                    overlapped,
                                );
                                return;
                            }
                            let _ = post_operation(vm.as_ref(), target, 0, 0, overlapped);
                        }
                        Err(error) => {
                            let _ = post_operation(
                                vm.as_ref(),
                                target,
                                io_error_code(&error),
                                0,
                                overlapped,
                            );
                        }
                    }
                    return;
                }
                () = tokio::time::sleep(Duration::from_millis(10)) => {
                    if vm.socket_handles().get(&listen_fd).await.is_none()
                        || vm.socket_handles().get(&accept_fd).await.is_none()
                    {
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
    });
    Ok(Some(Value::Int(IO_UNAVAILABLE)))
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.closesocket0(J)V",
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
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.initIDs()V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.updateAcceptContext(JJ)V",
    Any
)]
#[async_method]
pub async fn update_accept_context<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let accept_socket = parameters.pop_long()?;
    let listen_socket = parameters.pop_long()?;
    let vm = thread.vm()?;
    let listen_fd = socket_fd(vm.as_ref(), listen_socket, "SO_UPDATE_ACCEPT_CONTEXT").await?;
    let accept_fd = socket_fd(vm.as_ref(), accept_socket, "SO_UPDATE_ACCEPT_CONTEXT").await?;
    let listener = vm
        .socket_handles()
        .get(&listen_fd)
        .await
        .ok_or_else(|| socket_exception("SO_UPDATE_ACCEPT_CONTEXT", "invalid listener"))?;
    if listener.socket_type.as_tcp_listener().is_none() {
        return Err(socket_exception(
            "SO_UPDATE_ACCEPT_CONTEXT",
            "socket is not listening",
        ));
    }
    drop(listener);
    let accepted =
        vm.socket_handles().get(&accept_fd).await.ok_or_else(|| {
            socket_exception("SO_UPDATE_ACCEPT_CONTEXT", "invalid accepted socket")
        })?;
    if accepted.socket_type.as_tcp_stream().is_none() {
        return Err(socket_exception(
            "SO_UPDATE_ACCEPT_CONTEXT",
            "socket is not connected",
        ));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_closesocket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = closesocket0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.unwrap_err().to_string().contains("invalid socket"));
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
    async fn test_update_accept_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_accept_context(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.unwrap_err().to_string().contains("invalid"));
    }
}

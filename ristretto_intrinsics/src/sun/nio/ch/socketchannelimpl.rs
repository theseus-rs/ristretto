use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

fn get_fd(fd_value: &Value) -> Result<i32> {
    let guard = fd_value.as_reference()?;
    let Reference::Object(object) = &*guard else {
        return Err(InternalError("not a FileDescriptor object".to_string()));
    };
    Ok(object.value("fd")?.as_i32()?)
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn check_connect_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ready = parameters.pop_bool()?;
    let _block = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    // Check if connected by trying to get peer address
    match socket.peer_addr() {
        Ok(_) => Ok(Some(Value::Int(1))),  // connected
        Err(_) => Ok(Some(Value::Int(0))), // not yet
    }
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;Z)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_connect_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _block = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    match socket.peer_addr() {
        Ok(_) => Ok(Some(Value::Int(1))),
        Err(_) => Ok(Some(Value::Int(0))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.sendOutOfBandData(Ljava/io/FileDescriptor;B)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_out_of_band_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let cloned = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let Some(socket) = guard.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("oob: clone: {e}")))?
    };
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let byte = data as u8;
    let n = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned, &[byte]))
        .await
        .map_err(|e| InternalError(format!("oob: spawn: {e}")))?
        .map_err(|e| InternalError(format!("sendOOB: {e}")))?;
    #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    Ok(Some(Value::Int(n as i32)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_connect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_connect_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_out_of_band_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_out_of_band_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

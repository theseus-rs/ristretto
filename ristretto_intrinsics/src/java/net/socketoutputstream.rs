use crate::bounds;
use crate::net_helpers::socket_io_error;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{JavaError, Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method("java/net/SocketOutputStream.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_write_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let buf = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let _this = parameters.pop()?;

    let fd = {
        let guard = fd_value.as_object_ref()?;
        guard.value("fd")?.as_i32()?
    };

    let data = {
        let arr_guard = buf.as_reference()?;
        if let Reference::ByteArray(byte_array) = &*arr_guard {
            let off = usize::try_from(off).map_err(|e| InternalError(e.to_string()))?;
            let len = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;
            let end = off
                .checked_add(len)
                .ok_or_else(|| InternalError("SocketOutputStream range overflow".to_string()))?;
            #[expect(clippy::cast_sign_loss)]
            let data: Vec<u8> = bounds::range(byte_array, off..end, "SocketOutputStream.write")?
                .iter()
                .map(|&b| b as u8)
                .collect::<Vec<u8>>();
            data
        } else {
            return Err(InternalError("not a byte array".to_string()));
        }
    };

    let vm = thread.vm()?;

    let (stream, lifecycle) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
        (
            handle
                .socket_type
                .as_tcp_stream()
                .ok_or_else(|| JavaError::SocketException("Socket is not connected".to_string()))?
                .clone(),
            handle.lifecycle.clone(),
        )
    };
    let mut written = 0;
    while written < data.len() {
        let source = bounds::range_from(&data, written.., "SocketOutputStream.write")?;
        match stream.try_write(source) {
            Ok(0) => {
                return Err(
                    JavaError::SocketException("Socket write returned zero".to_string()).into(),
                );
            }
            Ok(count) => written += count,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                tokio::select! {
                    result = stream.writable() => result.map_err(|error| socket_io_error("write", error))?,
                    () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
                }
            }
            Err(error) => return Err(socket_io_error("write", error)),
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_socket_write_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_write_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

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

#[intrinsic_method("java/net/SocketInputStream.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_read_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let buf = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let _this = parameters.pop()?;

    let fd = {
        let guard = fd_value.as_object_ref()?;
        guard.value("fd")?.as_i32()?
    };

    let vm = thread.vm()?;
    let len_usize = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;
    let offset = usize::try_from(off).map_err(|e| InternalError(e.to_string()))?;
    if len_usize == 0 {
        return Ok(Some(Value::Int(0)));
    }
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
    let mut read_buf = vec![0u8; len_usize];
    let read_future = async {
        loop {
            match stream.try_read(&mut read_buf) {
                Ok(0) => return Ok((-1i32, Vec::new())),
                Ok(n) => {
                    read_buf.truncate(n);
                    return Ok((i32::try_from(n).unwrap_or(i32::MAX), read_buf));
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => stream
                    .readable()
                    .await
                    .map_err(|error| socket_io_error("read", error))?,
                Err(error) => return Err(socket_io_error("read", error)),
            }
        }
    };
    let result = if timeout > 0 {
        let duration = std::time::Duration::from_millis(u64::try_from(timeout)?);
        tokio::select! {
            result = tokio::time::timeout(duration, read_future) => result.map_err(|_| ristretto_types::Error::JavaError(JavaError::SocketTimeoutException("Read timed out".to_string())))?,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    } else {
        tokio::select! {
            result = read_future => result,
            () = lifecycle.cancelled() => return Err(JavaError::SocketException("Socket closed".to_string()).into()),
        }
    }?;
    let (n, data) = result;
    if n > 0 {
        let mut arr_guard = buf.as_reference_mut()?;
        let Reference::ByteArray(byte_array) = &mut *arr_guard else {
            return Err(InternalError("not a byte array".to_string()));
        };
        for (index, byte) in data.into_iter().enumerate() {
            *bounds::index_mut(
                byte_array,
                offset
                    .checked_add(index)
                    .ok_or_else(|| InternalError("SocketInputStream range overflow".into()))?,
                "SocketInputStream.read",
            )? = i8::from_ne_bytes(byte.to_ne_bytes());
        }
    }
    Ok(Some(Value::Int(n)))
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
    async fn test_socket_read_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = socket_read_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

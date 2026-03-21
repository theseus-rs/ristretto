use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
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
            #[expect(clippy::cast_sign_loss)]
            let off = off as usize;
            #[expect(clippy::cast_sign_loss)]
            let len = len as usize;
            #[expect(clippy::cast_sign_loss)]
            let data: Vec<u8> = byte_array[off..off + len]
                .iter()
                .map(|&b| b as u8)
                .collect::<Vec<u8>>();
            data
        } else {
            return Err(InternalError("not a byte array".to_string()));
        }
    };

    let vm = thread.vm()?;

    // Check variant first
    let is_tcp_stream = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        handle.socket_type.as_tcp_stream().is_some()
    };

    if is_tcp_stream {
        // Async write to TcpStream, using writable() for backpressure
        let mut written = 0;
        while written < data.len() {
            let handle = vm
                .socket_handles()
                .get(&fd)
                .await
                .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
            let Some(stream) = handle.socket_type.as_tcp_stream() else {
                return Err(InternalError("expected TcpStream".to_string()));
            };
            let stream = stream.clone();
            drop(handle);
            match stream.try_write(&data[written..]) {
                Ok(n) => written += n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    stream
                        .writable()
                        .await
                        .map_err(|e| InternalError(format!("socketWrite0: {e}")))?;
                }
                Err(e) => return Err(InternalError(format!("socketWrite0: {e}"))),
            }
        }
    } else {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for write".to_string(),
            ));
        };
        let cloned = socket
            .try_clone()
            .map_err(|e| InternalError(format!("write: clone: {e}")))?;
        drop(handle);
        tokio::task::spawn_blocking(move || std::io::Write::write_all(&mut &cloned, &data))
            .await
            .map_err(|e| InternalError(format!("write: spawn: {e}")))?
            .map_err(|e| InternalError(format!("socketWrite0: {e}")))?;
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

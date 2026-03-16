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
#[expect(clippy::too_many_lines)]
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

    // Check variant first, then handle accordingly
    let is_tcp_stream = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        handle.socket_type.as_tcp_stream().is_some()
    };

    if is_tcp_stream {
        // Async read from TcpStream with optional timeout, dropping guard between retries
        let mut read_buf = vec![0u8; len_usize];
        let read_future = async {
            loop {
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
                match stream.try_read(&mut read_buf) {
                    Ok(0) => return Ok((-1i32, Vec::new())),
                    Ok(n) => {
                        read_buf.truncate(n);
                        return Ok((i32::try_from(n).unwrap_or(i32::MAX), read_buf));
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        stream
                            .readable()
                            .await
                            .map_err(|e| InternalError(format!("socketRead0: {e}")))?;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                        return Ok((-1, Vec::new()));
                    }
                    Err(e) => return Err(InternalError(format!("socketRead0: {e}"))),
                }
            }
        };

        let result = if timeout > 0 {
            #[expect(clippy::cast_sign_loss)]
            let dur = std::time::Duration::from_millis(timeout as u64);
            tokio::time::timeout(dur, read_future).await.map_err(|_| {
                ristretto_types::Error::from(JavaError::SocketTimeoutException(
                    "Read timed out".to_string(),
                ))
            })?
        } else {
            read_future.await
        }?;

        let (n, data) = result;
        if n > 0 {
            let mut arr_guard = buf.as_reference_mut()?;
            if let Reference::ByteArray(byte_array) = &mut *arr_guard {
                #[expect(clippy::cast_sign_loss)]
                let off = off as usize;
                for (i, &byte_val) in data.iter().enumerate() {
                    #[expect(clippy::cast_possible_wrap)]
                    {
                        byte_array[off + i] = byte_val as i8;
                    }
                }
            }
        }
        Ok(Some(Value::Int(n)))
    } else {
        // Raw socket fallback
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for read".to_string(),
            ));
        };
        // Fallback: spawn_blocking for raw sockets
        if timeout > 0 {
            #[expect(clippy::cast_sign_loss)]
            let dur = std::time::Duration::from_millis(timeout as u64);
            let _ = socket.set_read_timeout(Some(dur));
        }
        let cloned = socket
            .try_clone()
            .map_err(|e| InternalError(format!("read: clone: {e}")))?;
        drop(handle);

        let result = tokio::task::spawn_blocking(move || {
            let mut read_buf = vec![0u8; len_usize];
            match std::io::Read::read(&mut &cloned, &mut read_buf) {
                Ok(0) => Ok((-1i32, Vec::new())),
                Ok(n) => {
                    read_buf.truncate(n);
                    Ok((i32::try_from(n).unwrap_or(i32::MAX), read_buf))
                }
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    Err(JavaError::SocketTimeoutException("Read timed out".to_string()).into())
                }
                Err(e) => Err(InternalError(format!("socketRead0: {e}"))),
            }
        })
        .await
        .map_err(|e| InternalError(format!("read: spawn: {e}")))?;

        match result {
            Ok((n, data)) => {
                if n > 0 {
                    let mut arr_guard = buf.as_reference_mut()?;
                    if let Reference::ByteArray(byte_array) = &mut *arr_guard {
                        #[expect(clippy::cast_sign_loss)]
                        let off = off as usize;
                        for (i, &byte_val) in data.iter().enumerate() {
                            #[expect(clippy::cast_possible_wrap)]
                            {
                                byte_array[off + i] = byte_val as i8;
                            }
                        }
                    }
                }
                // Reset read timeout
                if timeout > 0
                    && let Some(h) = vm.socket_handles().get(&fd).await
                    && let Some(s) = h.socket_type.as_raw()
                {
                    let _ = s.set_read_timeout(None);
                }
                Ok(Some(Value::Int(n)))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_socket_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_read_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use windows_sys::Win32::Networking::WinSock::{
    FIONREAD, MSG_OOB, SOCKET_ERROR, ioctlsocket, recv, send,
};

async fn raw_socket<V: VM + ?Sized>(vm: &V, fd: i32) -> Option<usize> {
    let handle = vm.socket_handles().get(&fd).await?;
    usize::try_from(handle.socket_type.raw_socket()).ok()
}

#[intrinsic_method(
    "sun/nio/ch/WindowsSelectorImpl.discardUrgentData(I)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
#[expect(unsafe_code)]
pub async fn discard_urgent_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let Some(socket) = raw_socket(thread.vm()?.as_ref(), fd).await else {
        return Ok(Some(Value::from(false)));
    };
    let mut discarded = false;
    let mut data = [0u8; 8];
    loop {
        let read = unsafe {
            recv(
                socket,
                data.as_mut_ptr(),
                i32::try_from(data.len()).unwrap_or(8),
                MSG_OOB,
            )
        };
        if read <= 0 {
            break;
        }
        discarded = true;
    }
    Ok(Some(Value::from(discarded)))
}

#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl.resetWakeupSocket0(I)V", Any)]
#[async_method]
#[expect(unsafe_code)]
pub async fn reset_wakeup_socket0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let Some(socket) = raw_socket(thread.vm()?.as_ref(), fd).await else {
        return Ok(None);
    };
    let mut available = 0u32;
    let result = unsafe { ioctlsocket(socket, FIONREAD, &raw mut available) };
    if result == SOCKET_ERROR || available == 0 {
        return Ok(None);
    }
    let mut remaining = usize::try_from(available).unwrap_or(usize::MAX);
    let mut buffer = vec![0u8; remaining.min(64 * 1024)];
    while remaining > 0 {
        let length = remaining.min(buffer.len()).min(i32::MAX as usize);
        let read = unsafe {
            recv(
                socket,
                buffer.as_mut_ptr(),
                i32::try_from(length).unwrap_or(i32::MAX),
                0,
            )
        };
        if read <= 0 {
            break;
        }
        remaining = remaining.saturating_sub(usize::try_from(read).unwrap_or(remaining));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl.setWakeupSocket0(I)V", Any)]
#[async_method]
#[expect(unsafe_code)]
pub async fn set_wakeup_socket0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    if let Some(socket) = raw_socket(thread.vm()?.as_ref(), fd).await {
        let byte = [1u8];
        // OpenJDK deliberately ignores send failures here: wakeup is best-effort and a concurrent
        // selector close can invalidate the socket at any point.
        let _ = unsafe { send(socket, byte.as_ptr(), 1, 0) };
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::handles::{SocketHandle, SocketType};

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_discard_urgent_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = discard_urgent_data(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(Some(Value::from(false)), result.expect("discardUrgentData"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reset_wakeup_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_wakeup_socket0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(None, result.expect("resetWakeupSocket0"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_wakeup_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_wakeup_socket0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(None, result.expect("setWakeupSocket0"));
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_and_reset_wakeup_socket() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let connect = tokio::spawn(tokio::net::TcpStream::connect(address));
        let (source, _) = listener.accept().await?;
        let sink = connect.await.map_err(std::io::Error::other)??;
        let source = Arc::new(source);
        let source_fd = 8_801;
        let sink_fd = 8_802;
        vm.socket_handles()
            .insert(
                source_fd,
                SocketHandle::new(SocketType::TcpStream(source.clone())),
            )
            .await?;
        vm.socket_handles()
            .insert(
                sink_fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(sink))),
            )
            .await?;

        set_wakeup_socket0(thread.clone(), Parameters::new(vec![Value::Int(sink_fd)])).await?;
        tokio::time::timeout(std::time::Duration::from_secs(1), source.readable())
            .await
            .map_err(std::io::Error::other)??;
        reset_wakeup_socket0(thread, Parameters::new(vec![Value::Int(source_fd)])).await?;
        let mut byte = [0u8; 1];
        let error = source
            .try_read(&mut byte)
            .expect_err("wakeup byte was drained");
        assert_eq!(std::io::ErrorKind::WouldBlock, error.kind());
        Ok(())
    }
}

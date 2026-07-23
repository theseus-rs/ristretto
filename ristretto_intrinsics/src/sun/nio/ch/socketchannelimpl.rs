use crate::java::io::socketfiledescriptor::get_fd;
use crate::net_helpers::socket_from_type;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

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
    let block = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        super::net::check_connect(&*vm, fd, block).await?,
    )))
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;Z)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn check_connect_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let block = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        super::net::check_connect(&*vm, fd, block).await?,
    )))
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
    let (cloned, non_blocking) = {
        let guard = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
        let socket = socket_from_type(&guard.socket_type);
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("oob: clone: {e}")))?,
            guard.non_blocking,
        )
    };
    let byte = data.to_ne_bytes()[0];
    let result = tokio::task::spawn_blocking(move || {
        loop {
            match cloned.send_out_of_band(&[byte]) {
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock && !non_blocking => {
                    #[cfg(unix)]
                    {
                        use std::os::fd::AsRawFd;
                        let mut descriptor = libc::pollfd {
                            fd: cloned.as_raw_fd(),
                            events: libc::POLLOUT,
                            revents: 0,
                        };
                        #[expect(unsafe_code)]
                        // SAFETY: descriptor points to one live pollfd record.
                        let ready = unsafe { libc::poll(&raw mut descriptor, 1, -1) };
                        if ready == -1 {
                            break Err(std::io::Error::last_os_error());
                        }
                    }
                    #[cfg(windows)]
                    {
                        use std::os::windows::io::AsRawSocket;
                        let mut descriptor = windows_sys::Win32::Networking::WinSock::WSAPOLLFD {
                            fd: usize::try_from(cloned.as_raw_socket()).unwrap_or(usize::MAX),
                            events: windows_sys::Win32::Networking::WinSock::POLLOUT,
                            revents: 0,
                        };
                        #[expect(unsafe_code)]
                        // SAFETY: descriptor points to one live WSAPOLLFD record.
                        let ready = unsafe {
                            windows_sys::Win32::Networking::WinSock::WSAPoll(
                                &raw mut descriptor,
                                1,
                                -1,
                            )
                        };
                        if ready == -1 {
                            #[expect(unsafe_code)]
                            // SAFETY: WSAGetLastError has no pointer arguments.
                            let code = unsafe {
                                windows_sys::Win32::Networking::WinSock::WSAGetLastError()
                            };
                            break Err(std::io::Error::from_raw_os_error(code));
                        }
                    }
                }
                result => break result,
            }
        }
    })
    .await
    .map_err(|e| InternalError(format!("oob: spawn: {e}")))?;
    match result {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            Ok(Some(Value::Int(super::IOS_UNAVAILABLE)))
        }
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            Ok(Some(Value::Int(super::IOS_INTERRUPTED)))
        }
        Err(error) => Err(ristretto_types::JavaError::IoException(error.to_string()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_connect_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = check_connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_connect_1() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = check_connect_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_out_of_band_data() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = send_out_of_band_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

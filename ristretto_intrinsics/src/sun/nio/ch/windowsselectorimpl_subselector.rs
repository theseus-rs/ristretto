use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::os::windows::io::AsRawSocket;
use std::sync::Arc;
use windows_sys::Win32::Networking::WinSock::{
    POLLERR as WSA_POLLERR, POLLHUP as WSA_POLLHUP, POLLIN as WSA_POLLIN, POLLNVAL as WSA_POLLNVAL,
    POLLOUT as WSA_POLLOUT, POLLPRI as WSA_POLLPRI, SOCKET_ERROR, WSAGetLastError, WSAPOLLFD,
    WSAPoll,
};

const JAVA_POLLIN: i16 = 0x0001;
const JAVA_POLLOUT: i16 = 0x0004;
const MAX_SELECTABLE_FDS: i32 = 1024;
const POLLFD_SIZE: i64 = 8;
const POLLFD_EVENTS_OFFSET: i64 = 4;

fn io_exception(operation: &str, error: impl std::fmt::Display) -> ristretto_types::Error {
    JavaError::IoException(format!("{operation} failed: {error}")).into()
}

fn set_fd_array(array: &Value, descriptors: &[i32]) -> Result<()> {
    let mut values = array.as_int_vec_mut()?;
    let required = descriptors
        .len()
        .checked_add(1)
        .ok_or_else(|| InternalError("selector descriptor count overflow".to_string()))?;
    if values.len() < required {
        return Err(InternalError(format!(
            "selector result array too small: need {required}, have {}",
            values.len()
        )));
    }
    let (count, values) = values
        .split_first_mut()
        .ok_or_else(|| InternalError("selector result array is empty".to_string()))?;
    *count = i32::try_from(descriptors.len())?;
    let destination = values.get_mut(..descriptors.len()).ok_or_else(|| {
        InternalError("selector result array changed while filling results".to_string())
    })?;
    destination.copy_from_slice(descriptors);
    Ok(())
}

#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl$SubSelector.poll0(JI[I[I[IJJ)I", Any)]
#[async_method]
#[expect(unsafe_code)]
#[expect(clippy::too_many_lines)]
pub async fn poll0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fds_buffer = parameters.pop_long()?;
    let timeout = parameters.pop_long()?;
    let except_fds = Value::Object(parameters.pop_reference()?);
    let write_fds = Value::Object(parameters.pop_reference()?);
    let read_fds = Value::Object(parameters.pop_reference()?);
    let num_fds = parameters.pop_int()?;
    let poll_address = parameters.pop_long()?;
    if !(0..=MAX_SELECTABLE_FDS).contains(&num_fds) {
        return Err(io_exception("select", "invalid descriptor count"));
    }
    if num_fds == 0 {
        set_fd_array(&read_fds, &[])?;
        set_fd_array(&write_fds, &[])?;
        set_fd_array(&except_fds, &[])?;
        return Ok(Some(Value::Int(0)));
    }

    let vm = thread.vm()?;
    let mut sockets = Vec::with_capacity(usize::try_from(num_fds)?);
    for index in 0..num_fds {
        let entry_address = poll_address
            .checked_add(i64::from(index) * POLLFD_SIZE)
            .ok_or_else(|| InternalError("selector poll address overflow".to_string()))?;
        let fd = vm
            .native_memory()
            .read_i32(entry_address)
            .ok_or_else(|| io_exception("select", "invalid poll array"))?;
        let events = vm
            .native_memory()
            .read_i16(entry_address + POLLFD_EVENTS_OFFSET)
            .ok_or_else(|| io_exception("select", "invalid poll array"))?;
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| io_exception("select", format!("invalid socket {fd}")))?;
        let socket = crate::net_helpers::socket_from_type(&handle.socket_type)
            .try_clone()
            .map_err(|error| io_exception("select", error))?;
        sockets.push((fd, events, socket));
    }

    let poll_result = tokio::task::spawn_blocking(move || {
        let mut poll_descriptors: Vec<WSAPOLLFD> = sockets
            .iter()
            .map(|(_, events, socket)| WSAPOLLFD {
                fd: usize::try_from(socket.as_raw_socket()).unwrap_or(usize::MAX),
                events: (if events & JAVA_POLLIN != 0 {
                    WSA_POLLIN
                } else {
                    0
                }) | (if events & JAVA_POLLOUT != 0 {
                    WSA_POLLOUT
                } else {
                    0
                }),
                revents: 0,
            })
            .collect();
        let mut remaining_timeout = timeout;
        loop {
            let poll_timeout = if remaining_timeout < 0 {
                -1
            } else {
                i32::try_from(remaining_timeout).unwrap_or(i32::MAX)
            };
            let result = unsafe {
                WSAPoll(
                    poll_descriptors.as_mut_ptr(),
                    u32::try_from(poll_descriptors.len()).unwrap_or(u32::MAX),
                    poll_timeout,
                )
            };
            if result == SOCKET_ERROR {
                return Err(unsafe { WSAGetLastError() });
            }
            if result != 0 || remaining_timeout <= i64::from(i32::MAX) {
                break;
            }
            remaining_timeout -= i64::from(i32::MAX);
            for descriptor in &mut poll_descriptors {
                descriptor.revents = 0;
            }
        }
        Ok(sockets
            .into_iter()
            .zip(poll_descriptors)
            .map(|((fd, events, _socket), descriptor)| (fd, events, descriptor.revents))
            .collect::<Vec<_>>())
    })
    .await
    .map_err(|error| InternalError(format!("select worker failed: {error}")))?
    .map_err(|error| io_exception("select", std::io::Error::from_raw_os_error(error)))?;

    let mut readable = Vec::new();
    let mut writable = Vec::new();
    let mut exceptional = Vec::new();
    for (fd, events, returned) in poll_result {
        if returned & WSA_POLLNVAL != 0 {
            return Err(io_exception("select", format!("invalid socket {fd}")));
        }
        if events & JAVA_POLLIN != 0 && returned & (WSA_POLLIN | WSA_POLLHUP) != 0 {
            readable.push(fd);
        }
        if events & JAVA_POLLOUT != 0 && returned & WSA_POLLOUT != 0 {
            writable.push(fd);
        }
        if returned & (WSA_POLLERR | WSA_POLLPRI) != 0 {
            exceptional.push(fd);
        }
    }
    set_fd_array(&read_fds, &readable)?;
    set_fd_array(&write_fds, &writable)?;
    set_fd_array(&except_fds, &exceptional)?;
    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::handles::{SocketHandle, SocketType};
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_poll0() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let new_array = || {
            Value::new_object(
                vm.garbage_collector(),
                Reference::IntArray(vec![99; 1].into_boxed_slice()),
            )
        };
        let read = new_array();
        let write = new_array();
        let except = new_array();
        let result = poll0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                read.clone(),
                write.clone(),
                except.clone(),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await
        .expect("poll0");
        assert_eq!(Some(Value::Int(0)), result);
        assert_eq!(&[0], read.as_int_vec_ref().expect("read").as_ref());
        assert_eq!(&[0], write.as_int_vec_ref().expect("write").as_ref());
        assert_eq!(&[0], except.as_int_vec_ref().expect("except").as_ref());
    }

    #[tokio::test]
    async fn test_poll0_reports_readable_socket() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let connect = tokio::spawn(tokio::net::TcpStream::connect(address));
        let (accepted, _) = listener.accept().await?;
        let mut client = connect.await.map_err(std::io::Error::other)??;
        let fd = 4_242;
        vm.socket_handles()
            .insert(
                fd,
                SocketHandle::new(SocketType::TcpStream(Arc::new(accepted))),
            )
            .await?;
        client.write_all(b"ready").await?;

        let poll_address = vm.native_memory().allocate(8);
        vm.native_memory().write_i32(poll_address, fd);
        vm.native_memory()
            .write_i16(poll_address + POLLFD_EVENTS_OFFSET, JAVA_POLLIN);
        let new_array = || {
            Value::new_object(
                vm.garbage_collector(),
                Reference::IntArray(vec![0; 2].into_boxed_slice()),
            )
        };
        let read = new_array();
        let write = new_array();
        let except = new_array();
        let result = poll0(
            thread,
            Parameters::new(vec![
                Value::Long(poll_address),
                Value::Int(1),
                read.clone(),
                write.clone(),
                except.clone(),
                Value::Long(1_000),
                Value::Long(0),
            ]),
        )
        .await?;
        vm.native_memory().free(poll_address);

        assert_eq!(Some(Value::Int(0)), result);
        assert_eq!(&[1, fd], read.as_int_vec_ref()?.as_ref());
        assert_eq!(&[0, 0], write.as_int_vec_ref()?.as_ref());
        assert_eq!(&[0, 0], except.as_int_vec_ref()?.as_ref());
        Ok(())
    }
}

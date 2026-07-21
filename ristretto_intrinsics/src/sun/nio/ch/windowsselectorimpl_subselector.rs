use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::time::{Duration, Instant};
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Networking::WinSock::{
    FD_ACCEPT, FD_CLOSE, FD_CONNECT, FD_OOB, FD_READ, FD_WRITE, FIONBIO, POLLIN as WSA_POLLIN,
    POLLOUT as WSA_POLLOUT, SOCKET, SOCKET_ERROR, WSA_INVALID_EVENT, WSA_MAXIMUM_WAIT_EVENTS,
    WSA_WAIT_FAILED, WSA_WAIT_TIMEOUT, WSACloseEvent, WSACreateEvent, WSAEVENT,
    WSAEnumNetworkEvents, WSAEventSelect, WSAGetLastError, WSANETWORKEVENTS,
    WSAWaitForMultipleEvents, ioctlsocket,
};

const JAVA_POLLIN: i16 = WSA_POLLIN;
const JAVA_POLLOUT: i16 = WSA_POLLOUT;
const MAX_SELECTABLE_FDS: i32 = 1024;
const POLLFD_SIZE: i64 = 8;
const POLLFD_EVENTS_OFFSET: i64 = 4;
const SELECT_RETRY_DELAY: Duration = Duration::from_millis(1);

struct RegisteredSocketEvent {
    fd: i32,
    interests: i16,
    raw_socket: SOCKET,
    event: WSAEVENT,
    non_blocking: bool,
}

#[expect(unsafe_code)]
impl Drop for RegisteredSocketEvent {
    fn drop(&mut self) {
        let mut non_blocking = u32::from(self.non_blocking);
        // SAFETY: The socket and event remain valid until this cleanup runs.
        // Removing the event association must precede restoring blocking mode.
        unsafe {
            WSAEventSelect(self.raw_socket, WSA_INVALID_EVENT, 0);
            ioctlsocket(self.raw_socket, FIONBIO, &raw mut non_blocking);
            WSACloseEvent(self.event);
        }
    }
}

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
        let raw_socket = handle.socket_type.raw_socket();
        sockets.push((fd, events, raw_socket, handle.is_unix, handle.non_blocking));
    }

    let selected = tokio::task::spawn_blocking(move || {
        let mut registered = Vec::with_capacity(sockets.len());
        for (fd, interests, raw_socket, is_unix, non_blocking) in sockets {
            let raw_socket = usize::try_from(raw_socket).unwrap_or(usize::MAX);
            // SAFETY: WSACreateEvent has no pointer arguments.
            let event = unsafe { WSACreateEvent() };
            if event == WSA_INVALID_EVENT {
                // SAFETY: WSAGetLastError has no pointer arguments.
                let error = unsafe { WSAGetLastError() };
                tracing::error!(fd, error, "Windows selector event creation failed");
                return Err(error);
            }

            let mut network_events = 0_u32;
            if interests & JAVA_POLLIN != 0 {
                network_events |= FD_READ | FD_ACCEPT | FD_CLOSE;
            }
            if interests & JAVA_POLLOUT != 0 {
                network_events |= FD_WRITE | FD_CONNECT | FD_CLOSE;
            }
            // Microsoft's AF_UNIX provider rejects FD_OOB with WSAEINVAL.
            if !is_unix {
                network_events |= FD_OOB;
            }
            // SAFETY: raw_socket is owned by the VM and event is valid for this worker.
            let result = unsafe { WSAEventSelect(raw_socket, event, network_events.cast_signed()) };
            if result == SOCKET_ERROR {
                // SAFETY: WSAGetLastError has no pointer arguments, and event was
                // created successfully but was not registered.
                let error = unsafe { WSAGetLastError() };
                tracing::error!(
                    fd,
                    error,
                    network_events,
                    "Windows selector event registration failed"
                );
                unsafe { WSACloseEvent(event) };
                return Err(error);
            }
            registered.push(RegisteredSocketEvent {
                fd,
                interests,
                raw_socket,
                event,
                non_blocking,
            });
        }
        let event_handles: Vec<HANDLE> = registered
            .iter()
            .map(|entry| entry.event as HANDLE)
            .collect();

        let started = Instant::now();
        let deadline = if timeout < 0 {
            None
        } else {
            let duration = Duration::from_millis(u64::try_from(timeout).unwrap_or(u64::MAX));
            started.checked_add(duration)
        };
        loop {
            let mut signaled = false;
            for handles in event_handles.chunks(WSA_MAXIMUM_WAIT_EVENTS as usize) {
                // SAFETY: handles contains valid Winsock event handles.
                let result = unsafe {
                    WSAWaitForMultipleEvents(
                        u32::try_from(handles.len()).unwrap_or(WSA_MAXIMUM_WAIT_EVENTS),
                        handles.as_ptr(),
                        0,
                        0,
                        0,
                    )
                };
                if result == WSA_WAIT_FAILED {
                    // SAFETY: WSAGetLastError has no pointer arguments.
                    let error = unsafe { WSAGetLastError() };
                    tracing::error!(error, "Windows selector event wait failed");
                    return Err(error);
                }
                if result != WSA_WAIT_TIMEOUT {
                    signaled = true;
                }
            }

            if signaled {
                let mut selected = Vec::with_capacity(registered.len());
                for entry in &registered {
                    let mut network_events = WSANETWORKEVENTS::default();
                    // SAFETY: The socket, event, and output pointer are valid.
                    let result = unsafe {
                        WSAEnumNetworkEvents(
                            entry.raw_socket,
                            entry.event,
                            std::ptr::from_mut(&mut network_events),
                        )
                    };
                    if result == SOCKET_ERROR {
                        // SAFETY: WSAGetLastError has no pointer arguments.
                        let error = unsafe { WSAGetLastError() };
                        tracing::error!(
                            fd = entry.fd,
                            error,
                            "Windows selector event enumeration failed"
                        );
                        return Err(error);
                    }

                    let occurred = network_events.lNetworkEvents.cast_unsigned();
                    let has_error = network_events.iErrorCode.iter().any(|error| *error != 0);
                    let readable = entry.interests & JAVA_POLLIN != 0
                        && occurred & (FD_READ | FD_ACCEPT | FD_CLOSE) != 0;
                    let writable = entry.interests & JAVA_POLLOUT != 0
                        && occurred & (FD_WRITE | FD_CONNECT) != 0
                        && !has_error;
                    let exceptional = has_error
                        || occurred & FD_OOB != 0
                        || (occurred & FD_CLOSE != 0 && !readable);
                    selected.push((entry.fd, readable, writable, exceptional));
                }
                return Ok(selected);
            }
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                return Ok(registered
                    .iter()
                    .map(|entry| (entry.fd, false, false, false))
                    .collect());
            }

            let delay = deadline.map_or(SELECT_RETRY_DELAY, |deadline| {
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(SELECT_RETRY_DELAY)
            });
            if delay.is_zero() {
                return Ok(registered
                    .iter()
                    .map(|entry| (entry.fd, false, false, false))
                    .collect());
            }
            std::thread::sleep(delay);
        }
    })
    .await
    .map_err(|error| InternalError(format!("select worker failed: {error}")))?
    .map_err(|error| io_exception("select", std::io::Error::from_raw_os_error(error)))?;

    let mut readable = Vec::new();
    let mut writable = Vec::new();
    let mut exceptional = Vec::new();
    for (fd, is_readable, is_writable, is_exceptional) in selected {
        if is_readable {
            readable.push(fd);
        }
        if is_writable {
            writable.push(fd);
        }
        if is_exceptional {
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

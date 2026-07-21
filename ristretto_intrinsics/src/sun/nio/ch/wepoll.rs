use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Parameters, Result, Thread, VM};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use windows_sys::Win32::Networking::WinSock::{
    FD_SET, FIONREAD, SO_ACCEPTCONN, SOCKET, SOCKET_ERROR, SOL_SOCKET, TIMEVAL, WSAGetLastError,
    getsockopt, ioctlsocket, select,
};

const EVENT_SIZE: i32 = 16;
const EVENTS_OFFSET: i32 = 0;
const DATA_OFFSET: i32 = 8;
const CTL_ADD: i32 = 1;
const CTL_MOD: i32 = 2;
const CTL_DEL: i32 = 3;
const EPOLL_IN: i32 = 1;
const EPOLL_PRI: i32 = 2;
const EPOLL_OUT: i32 = 4;
const EPOLL_ONESHOT: i32 = i32::MIN;
const POLL_RETRY_DELAY: Duration = Duration::from_millis(1);

struct PollSocket {
    descriptor: i64,
    events: i32,
    raw_socket: SOCKET,
    is_unix: bool,
    is_listener: bool,
}

#[derive(Debug, Default)]
struct Poller {
    registrations: HashMap<i64, i32>,
    wakeup_descriptor: Option<i64>,
    notified: bool,
}

#[derive(Debug, Default)]
struct Pollers {
    next: AtomicI64,
    pollers: Mutex<HashMap<i64, Poller>>,
}

impl Pollers {
    fn notify_all(&self) -> std::result::Result<(), String> {
        for poller in self
            .pollers
            .lock()
            .map_err(|error| error.to_string())?
            .values_mut()
        {
            poller.notified = true;
        }
        Ok(())
    }

    fn take_notification(&self, handle: i64) -> std::io::Result<Option<i64>> {
        let mut pollers = self
            .pollers
            .lock()
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        let poller = pollers.get_mut(&handle);
        Ok(poller.and_then(|poller| {
            if poller.notified {
                poller.notified = false;
                poller.wakeup_descriptor
            } else {
                None
            }
        }))
    }
}

fn pollers<V: VM + ?Sized>(vm: &V) -> Result<Arc<Pollers>> {
    vm.resource_manager().get_or_init(|| Pollers {
        next: AtomicI64::new(1),
        pollers: Mutex::new(HashMap::new()),
    })
}

pub(super) fn notify<V: VM + ?Sized>(vm: &V) -> Result<()> {
    pollers(vm)?.notify_all().map_err(InternalError)
}

#[expect(unsafe_code)]
fn socket_is_listener(socket: SOCKET) -> std::io::Result<bool> {
    let mut value = 0_i32;
    let mut length = i32::try_from(size_of::<i32>()).unwrap_or(i32::MAX);
    let result = unsafe {
        getsockopt(
            socket,
            SOL_SOCKET,
            SO_ACCEPTCONN,
            std::ptr::from_mut(&mut value).cast(),
            &raw mut length,
        )
    };
    if result == SOCKET_ERROR {
        return Err(std::io::Error::from_raw_os_error(unsafe {
            WSAGetLastError()
        }));
    }
    Ok(value != 0)
}

#[expect(unsafe_code)]
#[expect(clippy::too_many_lines)]
fn poll_sockets(
    pollers: &Pollers,
    poller_handle: i64,
    sockets: &[PollSocket],
    timeout: i32,
) -> std::io::Result<Vec<(i64, i32)>> {
    let deadline = (timeout >= 0).then(|| {
        Instant::now()
            .checked_add(Duration::from_millis(timeout.cast_unsigned().into()))
            .unwrap_or_else(Instant::now)
    });

    loop {
        if let Some(descriptor) = pollers.take_notification(poller_handle)? {
            return Ok(vec![(descriptor, EPOLL_IN)]);
        }

        let mut ready = Vec::new();
        for socket in sockets {
            if socket.is_unix && !socket.is_listener {
                let mut returned = 0;
                if socket.events & EPOLL_IN != 0 {
                    let mut available = 0_u32;
                    let result =
                        unsafe { ioctlsocket(socket.raw_socket, FIONREAD, &raw mut available) };
                    if result == SOCKET_ERROR {
                        return Err(std::io::Error::from_raw_os_error(unsafe {
                            WSAGetLastError()
                        }));
                    }
                    if available != 0 {
                        returned |= EPOLL_IN;
                    }
                }
                if socket.events & EPOLL_OUT != 0 {
                    returned |= EPOLL_OUT;
                }
                if returned != 0 {
                    ready.push((socket.descriptor, returned));
                }
                // Microsoft's AF_UNIX provider can block indefinitely in
                // select even with a zero timeout on connected sockets. Retry
                // the non-blocking byte-count probe until the poll deadline.
                continue;
            }

            let new_fd_set = || {
                let mut descriptors = FD_SET {
                    fd_count: 1,
                    ..FD_SET::default()
                };
                if let Some(descriptor) = descriptors.fd_array.first_mut() {
                    *descriptor = socket.raw_socket;
                }
                descriptors
            };
            let mut read_set = new_fd_set();
            let mut write_set = new_fd_set();
            let mut except_set = new_fd_set();
            let read_ptr = if socket.events & EPOLL_IN != 0 {
                std::ptr::from_mut(&mut read_set)
            } else {
                std::ptr::null_mut()
            };
            let write_ptr = if socket.events & EPOLL_OUT != 0 {
                std::ptr::from_mut(&mut write_set)
            } else {
                std::ptr::null_mut()
            };
            // The Microsoft AF_UNIX provider does not support exceptional
            // conditions. Passing only supported descriptor sets also keeps
            // each select call within a single service provider.
            let except_ptr = if !socket.is_unix && socket.events & EPOLL_PRI != 0 {
                std::ptr::from_mut(&mut except_set)
            } else {
                std::ptr::null_mut()
            };
            let timeout_value = TIMEVAL::default();
            // WinSock select requires sockets in one call to use the same
            // service provider. Polling one socket at a time supports both the
            // INET and AF_UNIX providers and preserves level-triggered
            // readiness when data arrived before the wait began.
            let result = unsafe {
                select(
                    0,
                    read_ptr,
                    write_ptr,
                    except_ptr,
                    std::ptr::from_ref(&timeout_value),
                )
            };
            if result == SOCKET_ERROR {
                return Err(std::io::Error::from_raw_os_error(unsafe {
                    WSAGetLastError()
                }));
            }
            let mut returned = 0;
            if !read_ptr.is_null() && read_set.fd_count != 0 {
                returned |= EPOLL_IN;
            }
            if !write_ptr.is_null() && write_set.fd_count != 0 {
                returned |= EPOLL_OUT;
            }
            if !except_ptr.is_null() && except_set.fd_count != 0 {
                returned |= EPOLL_PRI;
            }
            if returned != 0 {
                ready.push((socket.descriptor, returned));
            }
        }

        if !ready.is_empty() {
            return Ok(ready);
        }
        if timeout == 0 || deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(Vec::new());
        }
        let delay = deadline.map_or(POLL_RETRY_DELAY, |deadline| {
            deadline
                .saturating_duration_since(Instant::now())
                .min(POLL_RETRY_DELAY)
        });
        if delay.is_zero() {
            return Ok(Vec::new());
        }
        std::thread::sleep(delay);
    }
}

#[intrinsic_method("sun/nio/ch/WEPoll.close(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn close<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    pollers(&*vm)?
        .pollers
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .remove(&handle);
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/WEPoll.create()J", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let pollers = pollers(&*vm)?;
    let handle = pollers.next.fetch_add(1, Ordering::Relaxed);
    pollers
        .pollers
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .insert(handle, Poller::default());
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("sun/nio/ch/WEPoll.ctl(JIJI)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn ctl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let events = parameters.pop_int()?;
    let socket = parameters.pop_long()?;
    let opcode = parameters.pop_int()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let pollers = pollers(&*vm)?;
    let mut pollers = pollers
        .pollers
        .lock()
        .map_err(|error| InternalError(error.to_string()))?;
    let Some(poller) = pollers.get_mut(&handle) else {
        return Ok(Some(Value::Int(6)));
    };
    if opcode == CTL_ADD && poller.registrations.is_empty() && poller.wakeup_descriptor.is_none() {
        poller.wakeup_descriptor = Some(socket);
    }
    let registrations = &mut poller.registrations;
    let error = match opcode {
        CTL_ADD if registrations.contains_key(&socket) => 17,
        CTL_MOD if !registrations.contains_key(&socket) => 2,
        CTL_ADD | CTL_MOD => {
            registrations.insert(socket, events);
            0
        }
        CTL_DEL if registrations.remove(&socket).is_none() => 2,
        CTL_DEL => 0,
        _ => 22,
    };
    Ok(Some(Value::Int(error)))
}

#[intrinsic_method("sun/nio/ch/WEPoll.dataOffset()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn data_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(DATA_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/WEPoll.eventSize()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(EVENT_SIZE)))
}

#[intrinsic_method("sun/nio/ch/WEPoll.eventsOffset()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn events_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(EVENTS_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/WEPoll.wait(JJII)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn wait<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let maximum = usize::try_from(parameters.pop_int()?)
        .map_err(|_| InternalError("negative WEPoll event count".to_string()))?;
    let address = parameters.pop_long()?;
    let poller_handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let pollers = pollers(&*vm)?;
    let registered = pollers
        .pollers
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .get(&poller_handle)
        .map(|poller| poller.registrations.clone())
        .ok_or_else(|| JavaError::IoException("invalid WEPoll handle".to_string()))?;
    let mut sockets = Vec::new();
    for (&socket, &events) in &registered {
        if events == 0 {
            continue;
        }
        let key = i32::try_from(socket)
            .map_err(|_| InternalError("socket descriptor is outside i32 range".to_string()))?;
        let handle = vm
            .socket_handles()
            .get(&key)
            .await
            .ok_or_else(|| InternalError(format!("socket not found for fd {key}")))?;
        sockets.push(PollSocket {
            descriptor: socket,
            events,
            raw_socket: usize::try_from(handle.socket_type.raw_socket()).unwrap_or(usize::MAX),
            is_unix: handle.is_unix,
            is_listener: handle.is_unix
                && socket_is_listener(
                    usize::try_from(handle.socket_type.raw_socket()).unwrap_or(usize::MAX),
                )
                .map_err(|error| {
                    JavaError::IoException(format!("WEPoll socket query failed: {error}"))
                })?,
        });
    }
    let pollers_for_wait = pollers.clone();
    let polled = tokio::task::spawn_blocking(move || {
        poll_sockets(&pollers_for_wait, poller_handle, &sockets, timeout)
    })
    .await
    .map_err(|error| InternalError(format!("WEPoll wait task failed: {error}")))?
    .map_err(|error| JavaError::IoException(format!("WEPoll wait failed: {error}")))?;
    let mut ready = Vec::new();
    for (socket, returned) in polled {
        if ready.len() < maximum {
            ready.push((socket, returned));
            let events = registered.get(&socket).copied().unwrap_or_default();
            if events & EPOLL_ONESHOT != 0
                && let Some(registrations) = pollers
                    .pollers
                    .lock()
                    .map_err(|error| InternalError(error.to_string()))?
                    .get_mut(&poller_handle)
            {
                registrations.registrations.insert(socket, 0);
            }
        }
    }
    for (index, (socket, events)) in ready.iter().enumerate() {
        let base = address
            .checked_add(i64::try_from(index).unwrap_or(i64::MAX) * i64::from(EVENT_SIZE))
            .ok_or_else(|| InternalError("WEPoll result address overflow".to_string()))?;
        let mut bytes = [0_u8; EVENT_SIZE as usize];
        bytes[0..4].copy_from_slice(&events.to_ne_bytes());
        bytes[8..16].copy_from_slice(&socket.to_ne_bytes());
        if !vm.native_memory().try_write_bytes(base, &bytes) {
            return Err(InternalError("invalid WEPoll result address".to_string()));
        }
    }
    Ok(Some(Value::Int(i32::try_from(ready.len())?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_returns_first_registration_once() -> std::io::Result<()> {
        let pollers = Pollers::default();
        pollers
            .pollers
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(
                1,
                Poller {
                    registrations: HashMap::from([(41, EPOLL_IN), (42, EPOLL_OUT)]),
                    wakeup_descriptor: Some(41),
                    notified: false,
                },
            );

        pollers.notify_all().map_err(std::io::Error::other)?;
        assert_eq!(Some(41), pollers.take_notification(1)?);
        assert_eq!(None, pollers.take_notification(1)?);
        Ok(())
    }
}

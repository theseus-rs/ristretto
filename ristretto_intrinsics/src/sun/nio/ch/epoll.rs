use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::time::{Duration, Instant};

const EVENT_SIZE: i32 = 12;
const EVENTS_OFFSET: i32 = 0;
const DATA_OFFSET: i32 = 4;

#[expect(unsafe_code)]
pub(crate) fn create_epoll<V: VM + ?Sized>(vm: &V) -> Result<i32> {
    // SAFETY: epoll_create1 has no pointer arguments.
    let fd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
    if fd == -1 {
        return Err(super::posix::last_io_exception("epoll_create1"));
    }
    super::posix::register_descriptor(vm, fd)?;
    Ok(fd)
}

#[expect(unsafe_code)]
pub(crate) async fn control<V: VM + ?Sized>(
    vm: &V,
    epfd: i32,
    opcode: i32,
    fd: i32,
    events: i32,
) -> i32 {
    let raw_fd = super::posix::raw_descriptor(vm, fd).await;
    let mut event = libc::epoll_event {
        events: events.cast_unsigned(),
        u64: fd.cast_unsigned().into(),
    };
    // SAFETY: event is a valid epoll_event. Linux ignores the pointer for DEL,
    // but passing a valid pointer is accepted on every supported kernel.
    let result = unsafe { libc::epoll_ctl(epfd, opcode, raw_fd, &raw mut event) };
    if result == 0 {
        0
    } else {
        std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(libc::EIO)
    }
}

#[expect(unsafe_code)]
pub(crate) async fn wait_for_events<T: Thread + 'static>(
    thread: &Arc<T>,
    epfd: i32,
    address: i64,
    num_fds: i32,
    timeout: i32,
) -> Result<i32> {
    let count = usize::try_from(num_fds)
        .map_err(|_| InternalError("negative epoll event count".to_string()))?;
    if count == 0 {
        return Ok(0);
    }
    let deadline = (timeout >= 0).then(|| {
        Instant::now()
            .checked_add(Duration::from_millis(timeout.cast_unsigned().into()))
            .unwrap_or_else(Instant::now)
    });
    let mut first_wait = true;
    let (result, events) = loop {
        if super::nativethread::take_signal(&**thread)? {
            return Ok(super::posix::IOS_INTERRUPTED);
        }

        let wait_timeout = if let Some(deadline) = deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() && !first_wait {
                return Ok(0);
            }
            i32::try_from(remaining.as_millis())
                .unwrap_or(i32::MAX)
                .max(i32::from(!remaining.is_zero()))
                .min(100)
        } else {
            100
        };
        first_wait = false;
        let (result, events, errno) = tokio::task::spawn_blocking(move || {
            let mut events = vec![libc::epoll_event { events: 0, u64: 0 }; count];
            // SAFETY: events is writable for count epoll_event records.
            let result =
                unsafe { libc::epoll_wait(epfd, events.as_mut_ptr(), num_fds, wait_timeout) };
            let errno = (result == -1)
                .then(|| std::io::Error::last_os_error().raw_os_error())
                .flatten();
            (result, events, errno)
        })
        .await
        .map_err(|error| InternalError(format!("epoll_wait task failed: {error}")))?;
        if result > 0 {
            break (result, events);
        }
        if result == -1 {
            if errno == Some(libc::EINTR) {
                return Ok(super::posix::IOS_INTERRUPTED);
            }
            let error = errno.map_or_else(
                || std::io::Error::from_raw_os_error(libc::EIO),
                std::io::Error::from_raw_os_error,
            );
            return Err(super::posix::io_exception("epoll_wait", &error));
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(0);
        }
    };

    let vm = thread.vm()?;
    for (index, event) in events
        .into_iter()
        .take(usize::try_from(result).unwrap_or_default())
        .enumerate()
    {
        let event_address = address
            .checked_add(i64::try_from(index).unwrap_or(i64::MAX) * i64::from(EVENT_SIZE))
            .ok_or_else(|| InternalError("epoll result address overflow".to_string()))?;
        let mut bytes = [0_u8; EVENT_SIZE as usize];
        bytes[..4].copy_from_slice(&event.events.to_ne_bytes());
        bytes[4..12].copy_from_slice(&event.u64.to_ne_bytes());
        if !vm.native_memory().try_write_bytes(event_address, &bytes) {
            return Err(InternalError("invalid epoll result address".to_string()));
        }
    }
    Ok(result)
}

#[intrinsic_method("sun/nio/ch/EPoll.create()I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(create_epoll(&*vm)?)))
}

#[intrinsic_method("sun/nio/ch/EPoll.ctl(IIII)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn ctl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let events = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let opcode = parameters.pop_int()?;
    let epfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        control(&*vm, epfd, opcode, fd, events).await,
    )))
}

#[intrinsic_method("sun/nio/ch/EPoll.dataOffset()I", Any)]
#[async_method]
pub async fn data_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(DATA_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/EPoll.epollCreate()I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(create_epoll(&*vm)?)))
}

#[intrinsic_method("sun/nio/ch/EPoll.epollCtl(IIII)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_ctl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let events = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let opcode = parameters.pop_int()?;
    let epfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        control(&*vm, epfd, opcode, fd, events).await,
    )))
}

#[intrinsic_method("sun/nio/ch/EPoll.epollWait(IJI)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_wait<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let num_fds = parameters.pop_int()?;
    let poll_address = parameters.pop_long()?;
    let epfd = parameters.pop_int()?;
    let result = wait_for_events(&thread, epfd, poll_address, num_fds, -1).await?;
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method("sun/nio/ch/EPoll.eventSize()I", Any)]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(EVENT_SIZE)))
}

#[intrinsic_method("sun/nio/ch/EPoll.eventsOffset()I", Any)]
#[async_method]
pub async fn events_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(EVENTS_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/EPoll.wait(IJII)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn wait<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let num_fds = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let epfd = parameters.pop_int()?;
    let result = wait_for_events(&thread, epfd, address, num_fds, timeout).await?;
    Ok(Some(Value::Int(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await.expect("create");
        assert!(matches!(result, Some(Value::Int(fd)) if fd >= 0));
    }

    #[tokio::test]
    async fn test_ctl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ctl(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert!(matches!(result.expect("ctl"), Some(Value::Int(errno)) if errno != 0));
    }

    #[tokio::test]
    async fn test_data_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_offset(thread, Parameters::default())
            .await
            .expect("offset");
        assert_eq!(Some(Value::Int(4)), result);
    }

    #[tokio::test]
    async fn test_epoll_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_create(thread, Parameters::default())
            .await
            .expect("create");
        assert!(matches!(result, Some(Value::Int(fd)) if fd >= 0));
    }

    #[tokio::test]
    async fn test_epoll_ctl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_ctl(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert!(matches!(result.expect("ctl"), Some(Value::Int(errno)) if errno != 0));
    }

    #[tokio::test]
    async fn test_epoll_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_wait(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(Some(Value::Int(0)), result.expect("wait"));
    }

    #[tokio::test]
    async fn test_event_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_size(thread, Parameters::default())
            .await
            .expect("size");
        assert_eq!(Some(Value::Int(12)), result);
    }

    #[tokio::test]
    async fn test_events_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = events_offset(thread, Parameters::default())
            .await
            .expect("offset");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(Some(Value::Int(0)), result.expect("wait"));
    }
}

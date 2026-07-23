use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const KEVENT_SIZE: i32 = 32;
const IDENT_OFFSET: i32 = 0;
const FILTER_OFFSET: i32 = 8;
const FLAGS_OFFSET: i32 = 10;

#[derive(Debug, Default)]
struct Registrations(Mutex<HashMap<(i32, usize, i16), i32>>);

#[expect(unsafe_code)]
pub(crate) fn create_queue<V: VM + ?Sized>(vm: &V) -> Result<i32> {
    // SAFETY: kqueue has no pointer arguments.
    let fd = unsafe { libc::kqueue() };
    if fd == -1 {
        return Err(super::posix::last_io_exception("kqueue"));
    }
    super::posix::register_descriptor(vm, fd)?;
    Ok(fd)
}

#[expect(unsafe_code)]
pub(crate) async fn change<V: VM + ?Sized>(
    vm: &V,
    kqfd: i32,
    fd: i32,
    filter: i32,
    flags: i32,
) -> Result<i32> {
    let raw_fd = super::posix::raw_descriptor(vm, fd).await;
    let event = libc::kevent {
        ident: usize::try_from(raw_fd).unwrap_or_default(),
        filter: i16::try_from(filter).unwrap_or_default(),
        flags: u16::try_from(flags).unwrap_or_default(),
        fflags: 0,
        data: 0,
        udata: std::ptr::null_mut(),
    };
    // SAFETY: event is one readable kevent change record.
    let result = unsafe {
        libc::kevent(
            kqfd,
            std::ptr::from_ref(&event),
            1,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
        )
    };
    if result == -1 {
        return Ok(std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(libc::EIO));
    }
    let registrations = vm.resource_manager().get_or_init(Registrations::default)?;
    let mut registrations = registrations
        .0
        .lock()
        .map_err(|error| InternalError(error.to_string()))?;
    let key = (
        kqfd,
        usize::try_from(raw_fd).unwrap_or_default(),
        event.filter,
    );
    if event.flags & libc::EV_DELETE != 0 {
        registrations.remove(&key);
    } else {
        registrations.insert(key, fd);
    }
    Ok(0)
}

#[expect(unsafe_code, clippy::too_many_lines)]
pub(crate) async fn wait<T: Thread + 'static>(
    thread: &Arc<T>,
    kqfd: i32,
    address: i64,
    event_count: i32,
    timeout: i64,
) -> Result<i32> {
    let count = usize::try_from(event_count)
        .map_err(|_| InternalError("negative kqueue event count".to_string()))?;
    if count == 0 {
        return Ok(0);
    }
    let deadline = (timeout >= 0).then(|| {
        Instant::now()
            .checked_add(Duration::from_millis(timeout.cast_unsigned()))
            .unwrap_or_else(Instant::now)
    });
    let mut first_wait = true;
    let (result, events) = loop {
        if super::nativethread::take_signal(&**thread)? {
            return Ok(super::posix::IOS_INTERRUPTED);
        }

        let wait_millis = if let Some(deadline) = deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() && !first_wait {
                return Ok(0);
            }
            i64::try_from(remaining.as_millis())
                .unwrap_or(i64::MAX)
                .max(i64::from(!remaining.is_zero()))
                .min(100)
        } else {
            100
        };
        first_wait = false;
        let (result, events, errno) = tokio::task::spawn_blocking(move || {
            let mut events = vec![
                libc::kevent {
                    ident: 0,
                    filter: 0,
                    flags: 0,
                    fflags: 0,
                    data: 0,
                    udata: std::ptr::null_mut(),
                };
                count
            ];
            let timeout_value = libc::timespec {
                tv_sec: wait_millis.div_euclid(1000),
                tv_nsec: wait_millis.rem_euclid(1000) * 1_000_000,
            };
            // SAFETY: events is writable for event_count records and timeout_value is readable.
            let result = unsafe {
                libc::kevent(
                    kqfd,
                    std::ptr::null(),
                    0,
                    events.as_mut_ptr(),
                    event_count,
                    std::ptr::from_ref(&timeout_value),
                )
            };
            let errno = (result == -1)
                .then(|| std::io::Error::last_os_error().raw_os_error())
                .flatten();
            let events = events
                .into_iter()
                .map(|event| {
                    (
                        event.ident,
                        event.filter,
                        event.flags,
                        event.fflags,
                        event.data,
                        event.udata.addr(),
                    )
                })
                .collect::<Vec<_>>();
            (result, events, errno)
        })
        .await
        .map_err(|error| InternalError(format!("kevent task failed: {error}")))?;
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
            return Err(super::posix::io_exception("kevent", &error));
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(0);
        }
    };
    let vm = thread.vm()?;
    let registrations = vm.resource_manager().get_or_init(Registrations::default)?;
    let registrations = registrations
        .0
        .lock()
        .map_err(|error| InternalError(error.to_string()))?;
    for (index, (ident, filter, flags, fflags, data, udata)) in events
        .into_iter()
        .take(usize::try_from(result).unwrap_or_default())
        .enumerate()
    {
        let base = address
            .checked_add(i64::try_from(index).unwrap_or(i64::MAX) * i64::from(KEVENT_SIZE))
            .ok_or_else(|| InternalError("kqueue result address overflow".to_string()))?;
        let ident = registrations
            .get(&(kqfd, ident, filter))
            .copied()
            .map_or(ident, |fd| usize::try_from(fd).unwrap_or_default());
        let mut bytes = [0_u8; KEVENT_SIZE as usize];
        bytes[0..8].copy_from_slice(&u64::try_from(ident).unwrap_or_default().to_ne_bytes());
        bytes[8..10].copy_from_slice(&filter.to_ne_bytes());
        bytes[10..12].copy_from_slice(&flags.to_ne_bytes());
        bytes[12..16].copy_from_slice(&fflags.to_ne_bytes());
        bytes[16..24].copy_from_slice(&i64::try_from(data).unwrap_or_default().to_ne_bytes());
        bytes[24..32].copy_from_slice(&u64::try_from(udata).unwrap_or_default().to_ne_bytes());
        if !vm.native_memory().try_write_bytes(base, &bytes) {
            return Err(InternalError("invalid kqueue result address".to_string()));
        }
    }
    Ok(result)
}

#[intrinsic_method("sun/nio/ch/KQueue.create()I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(create_queue(&*vm)?)))
}

#[intrinsic_method("sun/nio/ch/KQueue.filterOffset()I", Any)]
#[async_method]
pub async fn filter_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(FILTER_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/KQueue.flagsOffset()I", Any)]
#[async_method]
pub async fn flags_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(FLAGS_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/KQueue.identOffset()I", Any)]
#[async_method]
pub async fn ident_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(IDENT_OFFSET)))
}

#[intrinsic_method("sun/nio/ch/KQueue.keventPoll(IJI)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_poll<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let event_count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let kqfd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        wait(&thread, kqfd, address, event_count, -1).await?,
    )))
}

#[intrinsic_method("sun/nio/ch/KQueue.keventRegister(IIII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_register<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let filter = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let kqfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        change(&*vm, kqfd, fd, filter, flags).await?,
    )))
}

#[intrinsic_method("sun/nio/ch/KQueue.keventSize()I", Any)]
#[async_method]
pub async fn kevent_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(KEVENT_SIZE)))
}

#[intrinsic_method("sun/nio/ch/KQueue.kqueue()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kqueue<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(create_queue(&*vm)?)))
}

#[intrinsic_method("sun/nio/ch/KQueue.poll(IJIJ)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_long()?;
    let event_count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let kqfd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        wait(&thread, kqfd, address, event_count, timeout).await?,
    )))
}

#[intrinsic_method("sun/nio/ch/KQueue.register(IIII)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn register_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let filter = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let kqfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    Ok(Some(Value::Int(
        change(&*vm, kqfd, fd, filter, flags).await?,
    )))
}

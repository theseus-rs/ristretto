//! Shared POSIX descriptor operations used by the NIO selector intrinsics.

use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Result, Thread, VM};
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub(crate) use super::IOS_INTERRUPTED;
pub(crate) const POLLFD_SIZE: i64 = 8;

#[derive(Debug, Default)]
struct NativeDescriptors {
    descriptors: Mutex<HashSet<i32>>,
}

impl NativeDescriptors {
    fn insert(&self, fd: i32) -> Result<()> {
        self.descriptors
            .lock()
            .map_err(|error| InternalError(error.to_string()))?
            .insert(fd);
        Ok(())
    }

    fn remove(&self, fd: i32) -> Result<bool> {
        Ok(self
            .descriptors
            .lock()
            .map_err(|error| InternalError(error.to_string()))?
            .remove(&fd))
    }
}

impl Drop for NativeDescriptors {
    #[expect(unsafe_code)]
    fn drop(&mut self) {
        if let Ok(descriptors) = self.descriptors.get_mut() {
            for fd in descriptors.drain() {
                // SAFETY: every descriptor in this set was returned by a successful
                // descriptor-creating syscall and ownership was transferred here.
                unsafe { libc::close(fd) };
            }
        }
    }
}

pub(crate) fn io_exception(operation: &str, error: &std::io::Error) -> ristretto_types::Error {
    JavaError::IoException(format!("{operation}: {error}")).into()
}

pub(crate) fn last_io_exception(operation: &str) -> ristretto_types::Error {
    io_exception(operation, &std::io::Error::last_os_error())
}

pub(crate) fn register_descriptor<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<()> {
    let result = vm
        .resource_manager()
        .get_or_init(NativeDescriptors::default)
        .and_then(|descriptors| descriptors.insert(fd));
    if result.is_err() {
        #[expect(unsafe_code)]
        // SAFETY: registration transfers ownership of this newly created descriptor;
        // a failed transfer leaves ownership with this function.
        unsafe {
            libc::close(fd);
        }
    }
    result
}

#[expect(unsafe_code)]
pub(crate) fn close_descriptor<V: VM + ?Sized>(vm: &V, fd: i32) -> Result<bool> {
    let descriptors = vm
        .resource_manager()
        .get_or_init(NativeDescriptors::default)?;
    if !descriptors.remove(fd)? {
        return Ok(false);
    }
    // SAFETY: successful removal proves that this registry owns the descriptor.
    if unsafe { libc::close(fd) } == -1 {
        return Err(last_io_exception("close"));
    }
    Ok(true)
}

pub(crate) async fn raw_descriptor<V: VM + ?Sized>(vm: &V, fd: i32) -> i32 {
    if let Some(handle) = vm.socket_handles().get(&fd).await {
        handle.socket_type.raw_fd()
    } else {
        fd
    }
}

/// Polls an array of native `pollfd` records stored in VM native memory.
#[expect(unsafe_code)]
#[expect(clippy::too_many_lines)]
pub(crate) async fn poll<T: Thread + 'static>(
    thread: &Arc<T>,
    poll_address: i64,
    num_fds: i32,
    timeout: i64,
) -> Result<i32> {
    let vm = thread.vm()?;
    let count = usize::try_from(num_fds)
        .map_err(|_| InternalError("negative poll descriptor count".to_string()))?;
    let timeout = if timeout < -1 {
        -1
    } else {
        i32::try_from(timeout).unwrap_or(i32::MAX)
    };

    let mut poll_fds = Vec::new();
    poll_fds
        .try_reserve_exact(count)
        .map_err(|error| InternalError(error.to_string()))?;
    for index in 0..count {
        let offset = i64::try_from(index)
            .map_err(|error| InternalError(error.to_string()))?
            .checked_mul(POLLFD_SIZE)
            .and_then(|offset| poll_address.checked_add(offset))
            .ok_or_else(|| InternalError("poll address overflow".to_string()))?;
        let fd = vm
            .native_memory()
            .read_i32(offset)
            .ok_or_else(|| InternalError("invalid pollfd address".to_string()))?;
        let events = vm
            .native_memory()
            .read_i16(offset + 4)
            .ok_or_else(|| InternalError("invalid pollfd events address".to_string()))?;
        poll_fds.push(libc::pollfd {
            fd: raw_descriptor(&*vm, fd).await,
            events,
            revents: 0,
        });
    }

    let deadline = (timeout >= 0).then(|| {
        Instant::now()
            .checked_add(Duration::from_millis(timeout.cast_unsigned().into()))
            .unwrap_or_else(Instant::now)
    });
    let mut first_wait = true;
    let (result, poll_fds) = loop {
        if super::nativethread::take_signal(&**thread)? {
            return Ok(IOS_INTERRUPTED);
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
        let mut wait_fds = poll_fds.clone();
        let (result, wait_fds, errno) = tokio::task::spawn_blocking(move || {
            // SAFETY: wait_fds is a live, writable array of pollfd records.
            let result = unsafe {
                libc::poll(
                    wait_fds.as_mut_ptr(),
                    libc::nfds_t::try_from(wait_fds.len()).unwrap_or(libc::nfds_t::MAX),
                    wait_timeout,
                )
            };
            let errno = (result == -1)
                .then(|| std::io::Error::last_os_error().raw_os_error())
                .flatten();
            (result, wait_fds, errno)
        })
        .await
        .map_err(|error| InternalError(format!("poll task failed: {error}")))?;
        if result > 0 {
            break (result, wait_fds);
        }
        if result == -1 {
            if errno == Some(libc::EINTR) {
                return Ok(IOS_INTERRUPTED);
            }
            let error = errno.map_or_else(
                || std::io::Error::from_raw_os_error(libc::EIO),
                std::io::Error::from_raw_os_error,
            );
            return Err(io_exception("poll", &error));
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(0);
        }
    };
    for (index, poll_fd) in poll_fds.iter().enumerate() {
        let offset = i64::try_from(index)
            .map_err(|error| InternalError(error.to_string()))?
            .checked_mul(POLLFD_SIZE)
            .and_then(|offset| poll_address.checked_add(offset + 6))
            .ok_or_else(|| InternalError("poll address overflow".to_string()))?;
        if !vm
            .native_memory()
            .try_write_bytes(offset, &poll_fd.revents.to_ne_bytes())
        {
            return Err(InternalError("invalid pollfd result address".to_string()));
        }
    }
    Ok(result)
}

#[expect(unsafe_code)]
pub(crate) fn write_descriptor(fd: i32, bytes: &[u8]) -> Result<i32> {
    // SAFETY: bytes points to a readable buffer for bytes.len(); fd is supplied by NIO.
    let result = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
    if result == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EINTR) {
            return Ok(IOS_INTERRUPTED);
        }
        if error.kind() == std::io::ErrorKind::WouldBlock {
            return Ok(super::IOS_UNAVAILABLE);
        }
        return Err(io_exception("write", &error));
    }
    i32::try_from(result).map_err(|error| InternalError(error.to_string()))
}

#[expect(unsafe_code)]
pub(crate) fn read_descriptor(fd: i32, bytes: &mut [u8]) -> Result<i32> {
    // SAFETY: bytes points to a writable buffer for bytes.len(); fd is supplied by NIO.
    let result = unsafe { libc::read(fd, bytes.as_mut_ptr().cast(), bytes.len()) };
    if result == -1 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EINTR) {
            return Ok(IOS_INTERRUPTED);
        }
        if error.kind() == std::io::ErrorKind::WouldBlock {
            return Ok(-2);
        }
        return Err(io_exception("read", &error));
    }
    i32::try_from(result).map_err(|error| InternalError(error.to_string()))
}

/// Returns whether a descriptor can be read without blocking.
#[expect(unsafe_code)]
pub(crate) fn descriptor_readable(fd: i32) -> Result<bool> {
    let mut poll_fd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        // SAFETY: poll_fd is a live, writable pollfd record and the count is one.
        let result = unsafe { libc::poll(&raw mut poll_fd, 1, 0) };
        if result >= 0 {
            return Ok(result > 0 && poll_fd.revents != 0);
        }
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EINTR) {
            return Err(io_exception("poll", &error));
        }
    }
}

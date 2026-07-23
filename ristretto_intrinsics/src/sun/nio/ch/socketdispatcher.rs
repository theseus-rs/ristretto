use crate::bounds;
use crate::java::io::socketfiledescriptor::get_fd;
use crate::sun::nio::fs::managed_files;
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::native_memory::NativeMemory;
use ristretto_types::{Parameters, Result, VM};
use std::mem::size_of;
use std::sync::Arc;
use std::time::Duration;

const SIGNAL_CHECK_INTERVAL: Duration = Duration::from_millis(100);

#[cfg(windows)]
#[expect(unsafe_code)]
fn wait_windows_socket(
    raw_socket: std::os::windows::io::RawSocket,
    write: bool,
) -> std::io::Result<bool> {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Networking::WinSock::{
        FD_ACCEPT, FD_CLOSE, FD_READ, FD_WRITE, FIONBIO, SOCKET_ERROR, WSA_INVALID_EVENT,
        WSA_WAIT_FAILED, WSA_WAIT_TIMEOUT, WSACloseEvent, WSACreateEvent, WSAEventSelect,
        WSAGetLastError, WSAWaitForMultipleEvents, ioctlsocket,
    };

    let raw_socket = usize::try_from(raw_socket).unwrap_or(usize::MAX);
    // SAFETY: WSACreateEvent has no pointer arguments.
    let event = unsafe { WSACreateEvent() };
    if event == WSA_INVALID_EVENT {
        // SAFETY: WSAGetLastError has no pointer arguments.
        return Err(std::io::Error::from_raw_os_error(unsafe {
            WSAGetLastError()
        }));
    }

    let network_events = if write {
        FD_WRITE | FD_CLOSE
    } else {
        FD_READ | FD_ACCEPT | FD_CLOSE
    };
    // SAFETY: raw_socket is live for this worker call and event is valid.
    let registration = unsafe { WSAEventSelect(raw_socket, event, network_events.cast_signed()) };
    if registration == SOCKET_ERROR {
        // SAFETY: WSAGetLastError has no pointer arguments, and event is valid but
        // was not successfully registered.
        let code = unsafe { WSAGetLastError() };
        unsafe { WSACloseEvent(event) };
        return Err(std::io::Error::from_raw_os_error(code));
    }

    let event_handle = event as HANDLE;
    // SAFETY: event points to one live Winsock event for this call.
    let wait_result = unsafe {
        WSAWaitForMultipleEvents(
            1,
            std::ptr::from_ref(&event_handle),
            0,
            u32::try_from(SIGNAL_CHECK_INTERVAL.as_millis()).unwrap_or(100),
            0,
        )
    };
    let wait_error = if wait_result == WSA_WAIT_FAILED {
        // SAFETY: WSAGetLastError has no pointer arguments.
        Some(unsafe { WSAGetLastError() })
    } else {
        None
    };

    // WSAEventSelect switches the socket to non-blocking mode. Remove the
    // registration and restore blocking mode before the caller performs I/O.
    let mut non_blocking = 0_u32;
    // SAFETY: raw_socket and event are valid for cleanup; non_blocking is a
    // writable u32 consumed synchronously by ioctlsocket.
    unsafe {
        WSAEventSelect(raw_socket, WSA_INVALID_EVENT, 0);
        ioctlsocket(raw_socket, FIONBIO, &raw mut non_blocking);
        WSACloseEvent(event);
    }

    if let Some(code) = wait_error {
        Err(std::io::Error::from_raw_os_error(code))
    } else {
        Ok(wait_result != WSA_WAIT_TIMEOUT)
    }
}

async fn socket_wait_interrupted<T: Thread + 'static>(thread: &Arc<T>, fd: i32) -> Result<bool> {
    let vm = thread.vm()?;
    if vm.socket_handles().get(&fd).await.is_none() {
        return Ok(true);
    }
    #[cfg(target_family = "unix")]
    if super::nativethread::take_signal(&**thread)? {
        return Ok(true);
    }
    Ok(false)
}

/// Waits for a raw socket without permanently occupying a blocking worker thread.
/// The bounded poll interval is also what lets `NativeThread.signal` and closing the
/// Java descriptor interrupt an otherwise blocking channel operation.
pub(super) async fn wait_raw_socket<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    write: bool,
) -> Result<i64> {
    loop {
        if socket_wait_interrupted(thread, fd).await? {
            return Ok(i64::from(super::IOS_INTERRUPTED));
        }
        let raw_socket = {
            let vm = thread.vm()?;
            let Some(handle) = vm.socket_handles().get(&fd).await else {
                return Ok(i64::from(super::IOS_INTERRUPTED));
            };
            #[cfg(unix)]
            let raw_socket = handle.socket_type.raw_fd();
            #[cfg(windows)]
            let raw_socket = handle.socket_type.raw_socket();
            raw_socket
        };
        let ready = tokio::task::spawn_blocking(move || {
            #[cfg(unix)]
            {
                let mut descriptor = libc::pollfd {
                    fd: raw_socket,
                    events: if write { libc::POLLOUT } else { libc::POLLIN },
                    revents: 0,
                };
                #[expect(unsafe_code)]
                // SAFETY: descriptor points to one initialized pollfd value.
                let result = unsafe {
                    libc::poll(
                        &raw mut descriptor,
                        1,
                        i32::try_from(SIGNAL_CHECK_INTERVAL.as_millis()).unwrap_or(100),
                    )
                };
                if result == -1 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(result > 0)
                }
            }
            #[cfg(windows)]
            {
                wait_windows_socket(raw_socket, write)
            }
        })
        .await
        .map_err(|error| InternalError(format!("socket poll task failed: {error}")))?;
        match ready {
            Ok(true) => return Ok(1),
            Ok(false) => {}
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                return Ok(i64::from(super::IOS_INTERRUPTED));
            }
            Err(error) => {
                return Err(ristretto_types::JavaError::IoException(format!(
                    "socket readiness failed: {error}"
                ))
                .into());
            }
        }
    }
}

/// Parses an array of `IOVec` entries from native memory.
/// Each entry matches the native `iovec` layout: a pointer-sized base address
/// followed by a pointer-sized length (e.g. 16 bytes on 64-bit, 8 bytes on 32-bit).
pub(super) fn parse_iovecs(
    memory: &NativeMemory,
    address: i64,
    count: i32,
) -> Result<Vec<(i64, usize)>> {
    let count =
        usize::try_from(count).map_err(|_| InternalError("negative iovec count".to_string()))?;
    let ptr_size = size_of::<usize>();
    let entry_size = ptr_size * 2;
    let entry_size_i64 = i64::try_from(entry_size).map_err(|e| InternalError(e.to_string()))?;
    let ptr_size_i64 = i64::try_from(ptr_size).map_err(|e| InternalError(e.to_string()))?;
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|error| InternalError(format!("iovec allocation failed: {error}")))?;
    for i in 0..count {
        let offset = i64::try_from(i)
            .ok()
            .and_then(|index| index.checked_mul(entry_size_i64))
            .ok_or_else(|| InternalError("iovec address overflow".to_string()))?;
        let entry_addr = address
            .checked_add(offset)
            .ok_or_else(|| InternalError("iovec address overflow".to_string()))?;
        let length_addr = entry_addr
            .checked_add(ptr_size_i64)
            .ok_or_else(|| InternalError("iovec address overflow".to_string()))?;
        let base_bytes = memory
            .try_read_bytes(entry_addr, ptr_size)
            .ok_or_else(|| InternalError("invalid iovec base address".to_string()))?;
        let len_bytes = memory
            .try_read_bytes(length_addr, ptr_size)
            .ok_or_else(|| InternalError("invalid iovec length address".to_string()))?;
        let base = match ptr_size {
            8 => i64::from_ne_bytes(
                base_bytes
                    .try_into()
                    .map_err(|_| InternalError("iov base".into()))?,
            ),
            4 => i64::from(u32::from_ne_bytes(
                base_bytes
                    .try_into()
                    .map_err(|_| InternalError("iov base".into()))?,
            )),
            _ => {
                return Err(InternalError(format!(
                    "unsupported pointer size: {ptr_size}"
                )));
            }
        };
        let len_usize = match ptr_size {
            8 => {
                let len = i64::from_ne_bytes(
                    len_bytes
                        .try_into()
                        .map_err(|_| InternalError("iov len".into()))?,
                );
                usize::try_from(len).map_err(|e| InternalError(e.to_string()))?
            }
            4 => u32::from_ne_bytes(
                len_bytes
                    .try_into()
                    .map_err(|_| InternalError("iov len".into()))?,
            ) as usize,
            _ => {
                return Err(InternalError(format!(
                    "unsupported pointer size: {ptr_size}"
                )));
            }
        };
        entries.push((base, len_usize));
    }
    Ok(entries)
}

pub(super) fn native_bytes(
    memory: &NativeMemory,
    address: i64,
    count: usize,
    operation: &str,
) -> Result<Vec<u8>> {
    if count == 0 {
        return Ok(Vec::new());
    }
    memory.try_read_bytes(address, count).ok_or_else(|| {
        InternalError(format!(
            "{operation}: invalid native memory range at {address} with length {count}"
        ))
    })
}

pub(super) fn write_native_bytes(
    memory: &NativeMemory,
    address: i64,
    data: &[u8],
    operation: &str,
) -> Result<()> {
    if data.is_empty() {
        return Ok(());
    }
    if memory.try_write_bytes(address, data) {
        Ok(())
    } else {
        Err(InternalError(format!(
            "{operation}: invalid native memory range at {address} with length {}",
            data.len()
        )))
    }
}

fn zeroed_buffer(count: usize, operation: &str) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count).map_err(|error| {
        InternalError(format!("{operation}: buffer allocation failed: {error}"))
    })?;
    buffer.resize(count, 0);
    Ok(buffer)
}

fn socket_io_status(operation: &str, error: &std::io::Error) -> Result<i64> {
    match error.kind() {
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
            Ok(i64::from(super::IOS_UNAVAILABLE))
        }
        std::io::ErrorKind::Interrupted => Ok(i64::from(super::IOS_INTERRUPTED)),
        std::io::ErrorKind::ConnectionReset => Err(
            ristretto_types::JavaError::ConnectionResetException(format!("{operation}: {error}"))
                .into(),
        ),
        _ => Err(ristretto_types::JavaError::IoException(format!("{operation}: {error}")).into()),
    }
}

async fn read_file_descriptor<V: VM>(
    vm: &V,
    fd: i32,
    address: i64,
    count: usize,
) -> Result<Option<Value>> {
    let mut buf = zeroed_buffer(count, "SocketDispatcher.read0")?;
    match managed_files::read(vm.file_handles(), i64::from(fd), &mut buf).await {
        Ok(0) if count > 0 => Ok(Some(Value::Int(-1))),
        Ok(n) => {
            if n > 0 {
                let bytes = bounds::range_to(&buf, ..n, "SocketDispatcher.read0")?;
                write_native_bytes(vm.native_memory(), address, bytes, "SocketDispatcher.read0")?;
            }
            Ok(Some(Value::Int(i32::try_from(n)?)))
        }
        Err(error) => Ok(Some(Value::Int(i32::try_from(socket_io_status(
            "SocketDispatcher.read0",
            &error,
        )?)?))),
    }
}

async fn readv_file_descriptor<V: VM>(
    vm: &V,
    fd: i32,
    iov_entries: &[(i64, usize)],
) -> Result<Option<Value>> {
    let mut chunks = Vec::new();
    chunks
        .try_reserve_exact(iov_entries.len())
        .map_err(|error| InternalError(format!("readv buffer allocation failed: {error}")))?;
    for (_, length) in iov_entries {
        chunks.push(zeroed_buffer(*length, "SocketDispatcher.readv0")?);
    }
    if chunks.is_empty() {
        return Ok(Some(Value::Long(0)));
    }

    let (n, returned_chunks) =
        match managed_files::readv(vm.file_handles(), i64::from(fd), chunks).await {
            Ok(result) => result,
            Err(error) => {
                return Ok(Some(Value::Long(socket_io_status(
                    "SocketDispatcher.readv0",
                    &error,
                )?)));
            }
        };
    if n == 0 {
        return Ok(Some(Value::Long(-1)));
    }

    let mut remaining = n;
    let mut total = 0i64;
    for ((address, _length), chunk) in iov_entries.iter().zip(returned_chunks) {
        if remaining == 0 {
            break;
        }
        let chunk_len = remaining.min(chunk.len());
        if chunk_len > 0 {
            let bytes = bounds::range_to(&chunk, ..chunk_len, "SocketDispatcher.readv0")?;
            write_native_bytes(
                vm.native_memory(),
                *address,
                bytes,
                "SocketDispatcher.readv0",
            )?;
            total += i64::try_from(chunk_len)?;
            remaining -= chunk_len;
        }
    }
    Ok(Some(Value::Long(total)))
}

async fn write_file_descriptor<V: VM>(vm: &V, fd: i32, data: &[u8]) -> Result<Option<Value>> {
    let n = match managed_files::write(vm.file_handles(), i64::from(fd), data).await {
        Ok(n) => n,
        Err(error) => {
            return Ok(Some(Value::Int(i32::try_from(socket_io_status(
                "SocketDispatcher.write0",
                &error,
            )?)?)));
        }
    };
    Ok(Some(Value::Int(i32::try_from(n)?)))
}

async fn writev_file_descriptor<V: VM>(vm: &V, fd: i32, data: &[u8]) -> Result<Option<Value>> {
    if data.is_empty() {
        return Ok(Some(Value::Long(0)));
    }
    let n = match managed_files::write(vm.file_handles(), i64::from(fd), data).await {
        Ok(n) => n,
        Err(error) => {
            return Ok(Some(Value::Long(socket_io_status(
                "SocketDispatcher.writev0",
                &error,
            )?)));
        }
    };
    Ok(Some(Value::Long(i64::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[expect(clippy::too_many_lines)]
#[async_method]
pub async fn read_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(count).map_err(|e| InternalError(e.to_string()))?;
    if count == 0 {
        return Ok(Some(Value::Int(0)));
    }

    let vm = thread.vm()?;
    let is_nonblocking = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.non_blocking);

    // For TcpStream: loop with try_read, dropping guard between retries to avoid deadlock
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return read_file_descriptor(&*vm, fd, address, count).await;
        };

        if let Some(stream) = handle.socket_type.as_tcp_stream() {
            let mut buf = zeroed_buffer(count, "SocketDispatcher.read0")?;
            match stream.try_read(&mut buf) {
                Ok(0) => return Ok(Some(Value::Int(-1))),
                Ok(n) => {
                    let bytes = bounds::range_to(&buf, ..n, "SocketDispatcher.read0")?;
                    write_native_bytes(
                        vm.native_memory(),
                        address,
                        bytes,
                        "SocketDispatcher.read0",
                    )?;
                    let n = i32::try_from(n).unwrap_or(i32::MAX);
                    return Ok(Some(Value::Int(n)));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Int(-2))); // IOS_UNAVAILABLE
                    }
                    // Clone Arc and drop guard before awaiting to avoid deadlock
                    let stream = stream.clone();
                    drop(handle);
                    match tokio::time::timeout(SIGNAL_CHECK_INTERVAL, stream.readable()).await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            let status = socket_io_status("SocketDispatcher.read0", &error)?;
                            return Ok(Some(Value::Int(i32::try_from(status)?)));
                        }
                        Err(_) => {
                            if socket_wait_interrupted(&thread, fd).await? {
                                return Ok(Some(Value::Int(super::IOS_INTERRUPTED)));
                            }
                        }
                    }
                }
                Err(e) => {
                    let status = socket_io_status("SocketDispatcher.read0", &e)?;
                    return Ok(Some(Value::Int(i32::try_from(status)?)));
                }
            }
        } else if handle.socket_type.as_raw().is_some() {
            let handle = if is_nonblocking {
                handle
            } else {
                drop(handle);
                let status = wait_raw_socket(&thread, fd, false).await?;
                if status < 0 {
                    return Ok(Some(Value::Int(i32::try_from(status)?)));
                }
                let Some(handle) = vm.socket_handles().get(&fd).await else {
                    return Ok(Some(Value::Int(super::IOS_INTERRUPTED)));
                };
                handle
            };
            let mut socket = handle.socket_type.as_raw().ok_or_else(|| {
                InternalError("socket type changed while waiting to read".to_string())
            })?;
            let mut buf = zeroed_buffer(count, "SocketDispatcher.read0")?;
            match std::io::Read::read(&mut socket, &mut buf) {
                Ok(0) => return Ok(Some(Value::Int(-1))),
                Ok(n) => {
                    let bytes = bounds::range_to(&buf, ..n, "SocketDispatcher.read0")?;
                    write_native_bytes(
                        vm.native_memory(),
                        address,
                        bytes,
                        "SocketDispatcher.read0",
                    )?;
                    return Ok(Some(Value::Int(i32::try_from(n)?)));
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Int(super::IOS_UNAVAILABLE)));
                    }
                    drop(handle);
                }
                Err(error) => {
                    let status = socket_io_status("SocketDispatcher.read0", &error)?;
                    return Ok(Some(Value::Int(i32::try_from(status)?)));
                }
            }
        } else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for read".to_string(),
            ));
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[expect(clippy::too_many_lines)]
#[async_method]
pub async fn readv_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let iov_count = parameters.pop_int()?;
    let iov_address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let vm = thread.vm()?;
    let is_nonblocking = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.non_blocking);
    let iov_entries = parse_iovecs(vm.native_memory(), iov_address, iov_count)?;
    let total_len = iov_entries.iter().try_fold(0_usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| InternalError("iovec total length overflow".to_string()))
    })?;
    if total_len == 0 {
        return Ok(Some(Value::Long(0)));
    }

    // Loop with try_read, dropping guard between retries to avoid deadlock
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return readv_file_descriptor(&*vm, fd, &iov_entries).await;
        };

        if let Some(stream) = handle.socket_type.as_tcp_stream() {
            let mut buf = zeroed_buffer(total_len, "SocketDispatcher.readv0")?;
            match stream.try_read(&mut buf) {
                Ok(0) => return Ok(Some(Value::Long(-1))),
                #[expect(clippy::cast_possible_wrap)]
                Ok(n) => {
                    let mut offset = 0usize;
                    for (base, len) in &iov_entries {
                        let to_copy = (*len).min(n.saturating_sub(offset));
                        if to_copy > 0 {
                            let bytes = bounds::range(
                                &buf,
                                offset..offset + to_copy,
                                "SocketDispatcher.readv0",
                            )?;
                            write_native_bytes(
                                vm.native_memory(),
                                *base,
                                bytes,
                                "SocketDispatcher.readv0",
                            )?;
                            offset += to_copy;
                        }
                        if offset >= n {
                            break;
                        }
                    }
                    return Ok(Some(Value::Long(n as i64)));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Long(-2))); // IOS_UNAVAILABLE
                    }
                    let stream = stream.clone();
                    drop(handle);
                    match tokio::time::timeout(SIGNAL_CHECK_INTERVAL, stream.readable()).await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            return Ok(Some(Value::Long(socket_io_status(
                                "SocketDispatcher.readv0",
                                &error,
                            )?)));
                        }
                        Err(_) => {
                            if socket_wait_interrupted(&thread, fd).await? {
                                return Ok(Some(Value::Long(i64::from(super::IOS_INTERRUPTED))));
                            }
                        }
                    }
                }
                Err(e) => {
                    return Ok(Some(Value::Long(socket_io_status(
                        "SocketDispatcher.readv0",
                        &e,
                    )?)));
                }
            }
        } else if handle.socket_type.as_raw().is_some() {
            let handle = if is_nonblocking {
                handle
            } else {
                drop(handle);
                let status = wait_raw_socket(&thread, fd, false).await?;
                if status < 0 {
                    return Ok(Some(Value::Long(status)));
                }
                let Some(handle) = vm.socket_handles().get(&fd).await else {
                    return Ok(Some(Value::Long(i64::from(super::IOS_INTERRUPTED))));
                };
                handle
            };
            let mut socket = handle.socket_type.as_raw().ok_or_else(|| {
                InternalError("socket type changed while waiting to readv".to_string())
            })?;
            let mut buf = zeroed_buffer(total_len, "SocketDispatcher.readv0")?;
            match std::io::Read::read(&mut socket, &mut buf) {
                Ok(0) => return Ok(Some(Value::Long(-1))),
                #[expect(clippy::cast_possible_wrap)]
                Ok(n) => {
                    let mut offset = 0usize;
                    for (base, len) in &iov_entries {
                        let to_copy = (*len).min(n.saturating_sub(offset));
                        if to_copy > 0 {
                            let bytes = bounds::range(
                                &buf,
                                offset..offset + to_copy,
                                "SocketDispatcher.readv0",
                            )?;
                            write_native_bytes(
                                vm.native_memory(),
                                *base,
                                bytes,
                                "SocketDispatcher.readv0",
                            )?;
                            offset += to_copy;
                        }
                        if offset >= n {
                            break;
                        }
                    }
                    return Ok(Some(Value::Long(n as i64)));
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Long(i64::from(super::IOS_UNAVAILABLE))));
                    }
                    drop(handle);
                }
                Err(error) => {
                    return Ok(Some(Value::Long(socket_io_status(
                        "SocketDispatcher.readv0",
                        &error,
                    )?)));
                }
            }
        } else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for readv".to_string(),
            ));
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn write_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let count = usize::try_from(count).map_err(|e| InternalError(e.to_string()))?;
    if count == 0 {
        return Ok(Some(Value::Int(0)));
    }

    let vm = thread.vm()?;
    let is_nonblocking = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.non_blocking);
    let data = native_bytes(
        vm.native_memory(),
        address,
        count,
        "SocketDispatcher.write0",
    )?;

    // Loop with try_write, dropping guard between retries to avoid deadlock
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return write_file_descriptor(&*vm, fd, &data).await;
        };

        if let Some(stream) = handle.socket_type.as_tcp_stream() {
            match stream.try_write(&data) {
                Ok(n) => {
                    let n = i32::try_from(n).map_err(|e| InternalError(e.to_string()))?;
                    return Ok(Some(Value::Int(n)));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Int(-2))); // IOS_UNAVAILABLE
                    }
                    let stream = stream.clone();
                    drop(handle);
                    match tokio::time::timeout(SIGNAL_CHECK_INTERVAL, stream.writable()).await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            let status = socket_io_status("SocketDispatcher.write0", &error)?;
                            return Ok(Some(Value::Int(i32::try_from(status)?)));
                        }
                        Err(_) => {
                            if socket_wait_interrupted(&thread, fd).await? {
                                return Ok(Some(Value::Int(super::IOS_INTERRUPTED)));
                            }
                        }
                    }
                }
                Err(e) => {
                    let status = socket_io_status("SocketDispatcher.write0", &e)?;
                    return Ok(Some(Value::Int(i32::try_from(status)?)));
                }
            }
        } else if handle.socket_type.as_raw().is_some() {
            let handle = if is_nonblocking {
                handle
            } else {
                drop(handle);
                let status = wait_raw_socket(&thread, fd, true).await?;
                if status < 0 {
                    return Ok(Some(Value::Int(i32::try_from(status)?)));
                }
                let Some(handle) = vm.socket_handles().get(&fd).await else {
                    return Ok(Some(Value::Int(super::IOS_INTERRUPTED)));
                };
                handle
            };
            let mut socket = handle.socket_type.as_raw().ok_or_else(|| {
                InternalError("socket type changed while waiting to write".to_string())
            })?;
            match std::io::Write::write(&mut socket, &data) {
                Ok(n) => return Ok(Some(Value::Int(i32::try_from(n)?))),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Int(super::IOS_UNAVAILABLE)));
                    }
                    drop(handle);
                }
                Err(error) => {
                    return Ok(Some(Value::Int(i32::try_from(socket_io_status(
                        "SocketDispatcher.write0",
                        &error,
                    )?)?)));
                }
            }
        } else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for write".to_string(),
            ));
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn writev_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let iov_count = parameters.pop_int()?;
    let iov_address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let vm = thread.vm()?;
    let is_nonblocking = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.non_blocking);
    let iov_entries = parse_iovecs(vm.native_memory(), iov_address, iov_count)?;
    let total_len = iov_entries.iter().try_fold(0_usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| InternalError("iovec total length overflow".to_string()))
    })?;
    if total_len == 0 {
        return Ok(Some(Value::Long(0)));
    }
    let mut data = Vec::new();
    data.try_reserve_exact(total_len)
        .map_err(|error| InternalError(format!("writev buffer allocation failed: {error}")))?;
    for (base, len) in &iov_entries {
        let chunk = native_bytes(vm.native_memory(), *base, *len, "SocketDispatcher.writev0")?;
        data.extend_from_slice(&chunk);
    }

    // Loop with try_write, dropping guard between retries to avoid deadlock
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return writev_file_descriptor(&*vm, fd, &data).await;
        };

        if let Some(stream) = handle.socket_type.as_tcp_stream() {
            match stream.try_write(&data) {
                #[expect(clippy::cast_possible_wrap)]
                Ok(n) => return Ok(Some(Value::Long(n as i64))),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Long(-2))); // IOS_UNAVAILABLE
                    }
                    let stream = stream.clone();
                    drop(handle);
                    match tokio::time::timeout(SIGNAL_CHECK_INTERVAL, stream.writable()).await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            return Ok(Some(Value::Long(socket_io_status(
                                "SocketDispatcher.writev0",
                                &error,
                            )?)));
                        }
                        Err(_) => {
                            if socket_wait_interrupted(&thread, fd).await? {
                                return Ok(Some(Value::Long(i64::from(super::IOS_INTERRUPTED))));
                            }
                        }
                    }
                }
                Err(e) => {
                    return Ok(Some(Value::Long(socket_io_status(
                        "SocketDispatcher.writev0",
                        &e,
                    )?)));
                }
            }
        } else if handle.socket_type.as_raw().is_some() {
            let handle = if is_nonblocking {
                handle
            } else {
                drop(handle);
                let status = wait_raw_socket(&thread, fd, true).await?;
                if status < 0 {
                    return Ok(Some(Value::Long(status)));
                }
                let Some(handle) = vm.socket_handles().get(&fd).await else {
                    return Ok(Some(Value::Long(i64::from(super::IOS_INTERRUPTED))));
                };
                handle
            };
            let mut socket = handle.socket_type.as_raw().ok_or_else(|| {
                InternalError("socket type changed while waiting to writev".to_string())
            })?;
            match std::io::Write::write(&mut socket, &data) {
                Ok(n) => return Ok(Some(Value::Long(i64::try_from(n)?))),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    if is_nonblocking {
                        return Ok(Some(Value::Long(i64::from(super::IOS_UNAVAILABLE))));
                    }
                    drop(handle);
                }
                Err(error) => {
                    return Ok(Some(Value::Long(socket_io_status(
                        "SocketDispatcher.writev0",
                        &error,
                    )?)));
                }
            }
        } else {
            return Err(InternalError(
                "expected TcpStream or Raw socket for writev".to_string(),
            ));
        }
    }
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/ch/SocketDispatcher.close0(I)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    vm.socket_handles().remove(&fd).await;
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.close0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close0_windows_le_v11<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    vm.socket_handles().remove(&fd).await;
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.preClose0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn pre_close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fdo = parameters.pop_reference()?;
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn read0_windows_le_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    read_0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn readv0_windows_le_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    readv_0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn write0_windows_le_v17<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    write_0(thread, parameters).await
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn writev0_windows_le_v17<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    writev_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_family = "wasm"))]
    use ristretto_types::handles::FileHandle;
    #[cfg(not(target_family = "wasm"))]
    use std::io::SeekFrom;

    #[tokio::test]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = readv_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = writev_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn test_file_descriptor_fallback_helpers() -> Result<()> {
        let (vm, _thread) = crate::test::java17_thread().await?;
        let temp_file = tempfile::NamedTempFile::new()?;
        std::fs::write(temp_file.path(), b"abcdef")?;
        let file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(temp_file.path())
            .await?;
        let fd = 4242;
        vm.file_handles()
            .insert(i64::from(fd), FileHandle::from((file, false)))
            .await?;

        let address = vm.native_memory().allocate(6);
        assert_eq!(
            read_file_descriptor(&*vm, fd, address, 6).await?,
            Some(Value::Int(6))
        );
        assert_eq!(vm.native_memory().read_bytes(address, 6), b"abcdef");
        assert_eq!(
            read_file_descriptor(&*vm, fd, address, 1).await?,
            Some(Value::Int(-1))
        );

        managed_files::seek(vm.file_handles(), i64::from(fd), SeekFrom::Start(0)).await?;
        let first = vm.native_memory().allocate(2);
        let second = vm.native_memory().allocate(4);
        let entries = [(first, 2), (second, 4)];
        assert_eq!(
            readv_file_descriptor(&*vm, fd, &entries).await?,
            Some(Value::Long(6))
        );
        assert_eq!(vm.native_memory().read_bytes(first, 2), b"ab");
        assert_eq!(vm.native_memory().read_bytes(second, 4), b"cdef");
        assert_eq!(
            readv_file_descriptor(&*vm, fd, &entries).await?,
            Some(Value::Long(-1))
        );
        assert_eq!(
            readv_file_descriptor(&*vm, fd, &[]).await?,
            Some(Value::Long(0))
        );

        managed_files::seek(vm.file_handles(), i64::from(fd), SeekFrom::Start(0)).await?;
        assert_eq!(
            write_file_descriptor(&*vm, fd, b"123").await?,
            Some(Value::Int(3))
        );
        assert_eq!(
            writev_file_descriptor(&*vm, fd, b"45").await?,
            Some(Value::Long(2))
        );
        assert_eq!(
            writev_file_descriptor(&*vm, fd, &[]).await?,
            Some(Value::Long(0))
        );

        assert!(read_file_descriptor(&*vm, -1, address, 1).await.is_err());
        assert!(readv_file_descriptor(&*vm, -1, &entries).await.is_err());
        assert!(write_file_descriptor(&*vm, -1, b"x").await.is_err());
        assert!(writev_file_descriptor(&*vm, -1, b"x").await.is_err());
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(result.expect("close0"), None);
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0_windows_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            close0_windows_le_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_pre_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pre_close0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(result.expect("pre_close0"), None);
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read0_windows_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read0_windows_le_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_readv0_windows_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = readv0_windows_le_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write0_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write0_windows_le_v17(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_writev0_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = writev0_windows_le_v17(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }
}

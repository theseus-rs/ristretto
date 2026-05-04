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
#[cfg(target_os = "windows")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::native_memory::NativeMemory;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

/// Parses an array of `IOVec` entries from native memory.
/// Each entry matches the native `iovec` layout: a pointer-sized base address
/// followed by a pointer-sized length (e.g. 16 bytes on 64-bit, 8 bytes on 32-bit).
fn parse_iovecs(memory: &NativeMemory, address: i64, count: i32) -> Result<Vec<(i64, usize)>> {
    let ptr_size = std::mem::size_of::<usize>();
    let entry_size = ptr_size * 2;
    let entry_size_i64 = i64::try_from(entry_size).map_err(|e| InternalError(e.to_string()))?;
    let ptr_size_i64 = i64::try_from(ptr_size).map_err(|e| InternalError(e.to_string()))?;
    let mut entries = Vec::new();
    for i in 0..count {
        let entry_addr = address + i64::from(i) * entry_size_i64;
        let base_bytes = memory.read_bytes(entry_addr, ptr_size);
        let len_bytes = memory.read_bytes(entry_addr + ptr_size_i64, ptr_size);
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

async fn read_file_descriptor<V: VM>(
    vm: &V,
    fd: i32,
    address: i64,
    count: usize,
) -> Result<Option<Value>> {
    let mut buf = vec![0u8; count];
    match managed_files::read(vm.file_handles(), i64::from(fd), &mut buf).await {
        Ok(0) if count > 0 => Ok(Some(Value::Int(-1))),
        Ok(n) => {
            if n > 0 {
                vm.native_memory().write_bytes(address, &buf[..n]);
            }
            Ok(Some(Value::Int(i32::try_from(n)?)))
        }
        Err(e) => Err(InternalError(format!("SocketDispatcher.read0: {e}"))),
    }
}

async fn readv_file_descriptor<V: VM>(
    vm: &V,
    fd: i32,
    iov_entries: &[(i64, usize)],
) -> Result<Option<Value>> {
    let chunks: Vec<Vec<u8>> = iov_entries.iter().map(|(_, len)| vec![0u8; *len]).collect();
    if chunks.is_empty() {
        return Ok(Some(Value::Long(0)));
    }

    let (n, returned_chunks) = managed_files::readv(vm.file_handles(), i64::from(fd), chunks)
        .await
        .map_err(|e| InternalError(format!("SocketDispatcher.readv0: {e}")))?;
    if n == 0 {
        return Ok(Some(Value::Long(-1)));
    }

    let mut remaining = n;
    let mut total = 0i64;
    for (index, chunk) in returned_chunks.into_iter().enumerate() {
        if remaining == 0 {
            break;
        }
        let chunk_len = remaining.min(chunk.len());
        if chunk_len > 0 {
            vm.native_memory()
                .write_bytes(iov_entries[index].0, &chunk[..chunk_len]);
            total += i64::try_from(chunk_len)?;
            remaining -= chunk_len;
        }
    }
    Ok(Some(Value::Long(total)))
}

async fn write_file_descriptor<V: VM>(vm: &V, fd: i32, data: &[u8]) -> Result<Option<Value>> {
    let n = managed_files::write(vm.file_handles(), i64::from(fd), data)
        .await
        .map_err(|e| InternalError(format!("SocketDispatcher.write0: {e}")))?;
    Ok(Some(Value::Int(i32::try_from(n)?)))
}

async fn writev_file_descriptor<V: VM>(vm: &V, fd: i32, data: &[u8]) -> Result<Option<Value>> {
    if data.is_empty() {
        return Ok(Some(Value::Long(0)));
    }
    let n = managed_files::write(vm.file_handles(), i64::from(fd), data)
        .await
        .map_err(|e| InternalError(format!("SocketDispatcher.writev0: {e}")))?;
    Ok(Some(Value::Long(i64::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_17)
)]
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
            let mut buf = vec![0u8; count];
            match stream.try_read(&mut buf) {
                Ok(0) => return Ok(Some(Value::Int(-1))),
                Ok(n) => {
                    vm.native_memory().write_bytes(address, &buf[..n]);
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
                    let _ = stream.readable().await;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                    return Ok(Some(Value::Int(-1)));
                }
                Err(e) => {
                    return Err(InternalError(format!("SocketDispatcher.read0: {e}")));
                }
            }
        } else if let Some(socket) = handle.socket_type.as_raw() {
            // Fallback: spawn_blocking for raw sockets
            let cloned = socket
                .try_clone()
                .map_err(|e| InternalError(format!("read: clone: {e}")))?;
            drop(handle);

            let result = tokio::task::spawn_blocking(move || {
                let mut buf = vec![0u8; count];
                match std::io::Read::read(&mut &cloned, &mut buf) {
                    Ok(0) => Ok((-1i32, Vec::new())),
                    Ok(n) => {
                        buf.truncate(n);
                        let n = i32::try_from(n).unwrap_or(i32::MAX);
                        Ok((n, buf))
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        Ok((-2, Vec::new()))
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Ok((-2, Vec::new())),
                    Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                        Ok((-1, Vec::new()))
                    }
                    Err(e) => Err(e),
                }
            })
            .await
            .map_err(|e| InternalError(format!("read: spawn: {e}")))?
            .map_err(|e| InternalError(format!("SocketDispatcher.read0: {e}")))?;

            let (n, data) = result;
            if n > 0 {
                vm.native_memory().write_bytes(address, &data);
            }
            return Ok(Some(Value::Int(n)));
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
    let total_len: usize = iov_entries.iter().map(|(_, len)| len).sum();

    // Loop with try_read, dropping guard between retries to avoid deadlock
    loop {
        let Some(handle) = vm.socket_handles().get(&fd).await else {
            return readv_file_descriptor(&*vm, fd, &iov_entries).await;
        };

        if let Some(stream) = handle.socket_type.as_tcp_stream() {
            let mut buf = vec![0u8; total_len];
            match stream.try_read(&mut buf) {
                Ok(0) => return Ok(Some(Value::Long(-1))),
                #[expect(clippy::cast_possible_wrap)]
                Ok(n) => {
                    let mut offset = 0usize;
                    for (base, len) in &iov_entries {
                        let to_copy = (*len).min(n.saturating_sub(offset));
                        if to_copy > 0 {
                            vm.native_memory()
                                .write_bytes(*base, &buf[offset..offset + to_copy]);
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
                    let _ = stream.readable().await;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                    return Ok(Some(Value::Long(-1)));
                }
                Err(e) => {
                    return Err(InternalError(format!("SocketDispatcher.readv0: {e}")));
                }
            }
        } else if let Some(socket) = handle.socket_type.as_raw() {
            let cloned = socket
                .try_clone()
                .map_err(|e| InternalError(format!("readv: clone: {e}")))?;
            drop(handle);

            let result = tokio::task::spawn_blocking(move || {
                let mut buf = vec![0u8; total_len];
                match std::io::Read::read(&mut &cloned, &mut buf) {
                    Ok(0) => Ok((-1i64, Vec::new())),
                    #[expect(clippy::cast_possible_wrap)]
                    Ok(n) => {
                        buf.truncate(n);
                        Ok((n as i64, buf))
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        Ok((-2, Vec::new()))
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                        Ok((-1, Vec::new()))
                    }
                    Err(e) => Err(e),
                }
            })
            .await
            .map_err(|e| InternalError(format!("readv: spawn: {e}")))?
            .map_err(|e| InternalError(format!("SocketDispatcher.readv0: {e}")))?;

            let (n, data) = result;
            if n > 0 {
                let mut offset = 0usize;
                let data_len = data.len();
                for (base, len) in &iov_entries {
                    let to_copy = (*len).min(data_len.saturating_sub(offset));
                    if to_copy > 0 {
                        vm.native_memory()
                            .write_bytes(*base, &data[offset..offset + to_copy]);
                        offset += to_copy;
                    }
                    if offset >= data_len {
                        break;
                    }
                }
            }
            return Ok(Some(Value::Long(n)));
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

    let vm = thread.vm()?;
    let is_nonblocking = vm
        .socket_handles()
        .get(&fd)
        .await
        .is_some_and(|guard| guard.non_blocking);
    let data = vm.native_memory().read_bytes(address, count);

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
                    let _ = stream.writable().await;
                }
                Err(e) => {
                    return Err(InternalError(format!("SocketDispatcher.write0: {e}")));
                }
            }
        } else if let Some(socket) = handle.socket_type.as_raw() {
            let cloned = socket
                .try_clone()
                .map_err(|e| InternalError(format!("write: clone: {e}")))?;
            drop(handle);

            let n = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned, &data))
                .await
                .map_err(|e| InternalError(format!("write: spawn: {e}")))?
                .map_err(|e| InternalError(format!("SocketDispatcher.write0: {e}")))?;
            let n = i32::try_from(n).map_err(|e| InternalError(e.to_string()))?;
            return Ok(Some(Value::Int(n)));
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
    let mut data = Vec::new();
    for (base, len) in &iov_entries {
        let chunk = vm.native_memory().read_bytes(*base, *len);
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
                    let _ = stream.writable().await;
                }
                Err(e) => {
                    return Err(InternalError(format!("SocketDispatcher.writev0: {e}")));
                }
            }
        } else if let Some(socket) = handle.socket_type.as_raw() {
            let cloned = socket
                .try_clone()
                .map_err(|e| InternalError(format!("writev: clone: {e}")))?;
            drop(handle);

            let n = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned, &data))
                .await
                .map_err(|e| InternalError(format!("writev: spawn: {e}")))?
                .map_err(|e| InternalError(format!("SocketDispatcher.writev0: {e}")))?;
            #[expect(clippy::cast_possible_wrap)]
            return Ok(Some(Value::Long(n as i64)));
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn readv0_windows_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn write0_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn writev0_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
            result.unwrap_err().to_string()
        );
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
        assert_eq!(
            "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
            result.unwrap_err().to_string()
        );
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
        assert_eq!(
            "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
            result.unwrap_err().to_string()
        );
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
        assert_eq!(
            "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
            result.unwrap_err().to_string()
        );
    }
}

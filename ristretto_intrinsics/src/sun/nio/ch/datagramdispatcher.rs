use crate::java::io::socketfiledescriptor::get_fd;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn dup_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_fd_value = parameters.pop()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;

    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    let cloned = socket
        .try_clone()
        .map_err(|e| InternalError(format!("dup: clone: {e}")))?;
    drop(guard);

    let new_fd = vm.next_nio_fd();
    {
        let mut new_fd_guard = new_fd_value.as_reference_mut()?;
        if let Reference::Object(object) = &mut *new_fd_guard {
            object.set_value("fd", Value::Int(new_fd))?;
        }
    }
    vm.socket_handles()
        .insert(new_fd, SocketHandle::new(SocketType::Raw(cloned)))
        .await?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    Any
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
    let cloned_socket = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("read: clone: {e}")))?
    };

    let result = tokio::task::spawn_blocking(move || {
        let mut buf = vec![0u8; count];
        match std::io::Read::read(&mut &cloned_socket, &mut buf) {
            Ok(0) => Ok((-1i32, Vec::new())),
            Ok(n) => {
                buf.truncate(n);
                let n = i32::try_from(n).unwrap_or(i32::MAX);
                Ok((n, buf))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok((-2, Vec::new())),
            Err(e) => Err(e),
        }
    })
    .await
    .map_err(|e| InternalError(format!("read: spawn: {e}")))?
    .map_err(|e| InternalError(format!("DatagramDispatcher.read0: {e}")))?;

    let (n, data) = result;
    if n > 0 {
        vm.native_memory().write_bytes(address, &data);
    }
    Ok(Some(Value::Int(n)))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    Any
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
    let mut total_len: usize = 0;
    let mut iov_entries = Vec::new();
    for i in 0..iov_count {
        let entry_addr = iov_address + i64::from(i) * 16;
        let base_bytes = vm.native_memory().read_bytes(entry_addr, 8);
        let len_bytes = vm.native_memory().read_bytes(entry_addr + 8, 8);
        let base = i64::from_ne_bytes(
            base_bytes
                .try_into()
                .map_err(|_| InternalError("iov base".into()))?,
        );
        let len = i64::from_ne_bytes(
            len_bytes
                .try_into()
                .map_err(|_| InternalError("iov len".into()))?,
        );
        let len_usize = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;
        iov_entries.push((base, len_usize));
        total_len += len_usize;
    }

    let cloned_socket = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("readv: clone: {e}")))?
    };

    let result = tokio::task::spawn_blocking(move || {
        let mut buf = vec![0u8; total_len];
        match std::io::Read::read(&mut &cloned_socket, &mut buf) {
            Ok(0) => Ok((-1i64, Vec::new())),
            #[expect(clippy::cast_possible_wrap)]
            Ok(n) => {
                buf.truncate(n);
                Ok((n as i64, buf))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok((-2, Vec::new())),
            Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => Ok((-1, Vec::new())),
            Err(e) => Err(e),
        }
    })
    .await
    .map_err(|e| InternalError(format!("readv: spawn: {e}")))?
    .map_err(|e| InternalError(format!("DatagramDispatcher.readv0: {e}")))?;

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
    Ok(Some(Value::Long(n)))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    Any
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
    let data = vm.native_memory().read_bytes(address, count);

    let cloned_socket = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("write: clone: {e}")))?
    };

    let n = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned_socket, &data))
        .await
        .map_err(|e| InternalError(format!("write: spawn: {e}")))?
        .map_err(|e| InternalError(format!("DatagramDispatcher.write0: {e}")))?;

    let n = i32::try_from(n).map_err(|e| InternalError(e.to_string()))?;
    Ok(Some(Value::Int(n)))
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    Any
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
    let mut data = Vec::new();
    for i in 0..iov_count {
        let entry_addr = iov_address + i64::from(i) * 16;
        let base_bytes = vm.native_memory().read_bytes(entry_addr, 8);
        let len_bytes = vm.native_memory().read_bytes(entry_addr + 8, 8);
        let base = i64::from_ne_bytes(
            base_bytes
                .try_into()
                .map_err(|_| InternalError("iov base".into()))?,
        );
        let len = i64::from_ne_bytes(
            len_bytes
                .try_into()
                .map_err(|_| InternalError("iov len".into()))?,
        );
        let len_usize = usize::try_from(len).map_err(|e| InternalError(e.to_string()))?;
        let chunk = vm.native_memory().read_bytes(base, len_usize);
        data.extend_from_slice(&chunk);
    }

    let cloned_socket = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        socket
            .try_clone()
            .map_err(|e| InternalError(format!("writev: clone: {e}")))?
    };

    let n = tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned_socket, &data))
        .await
        .map_err(|e| InternalError(format!("writev: spawn: {e}")))?
        .map_err(|e| InternalError(format!("DatagramDispatcher.writev0: {e}")))?;

    #[expect(clippy::cast_possible_wrap)]
    Ok(Some(Value::Long(n as i64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dup_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

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
}

use crate::bounds;
use crate::java::io::socketfiledescriptor::get_fd;
use crate::sun::nio::ch::socketdispatcher::{native_bytes, parse_iovecs, write_native_bytes};
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::Any;
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
#[cfg(not(target_os = "windows"))]
use ristretto_types::handles::{SocketHandle, SocketType};
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn dup_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let target_fd_value = parameters.pop()?;
    let source_fd_value = parameters.pop()?;
    let source_fd = get_fd(&source_fd_value)?;
    let target_fd = get_fd(&target_fd_value)?;
    let vm = thread.vm()?;

    let guard = vm
        .socket_handles()
        .get(&source_fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {source_fd}")))?;
    let Some(socket) = guard.socket_type.as_raw() else {
        return Err(InternalError("expected raw socket".to_string()));
    };
    let cloned = socket
        .try_clone()
        .map_err(|e| InternalError(format!("dup: clone: {e}")))?;
    let timeout = guard.timeout;
    let is_ipv6 = guard.is_ipv6;
    let is_unix = guard.is_unix;
    let non_blocking = guard.non_blocking;
    drop(guard);

    let mut handle = SocketHandle::new(SocketType::Raw(cloned));
    handle.timeout = timeout;
    handle.is_ipv6 = is_ipv6;
    handle.is_unix = is_unix;
    handle.non_blocking = non_blocking;
    // DatagramDispatcher.dup0 has dup2 semantics: replace the target descriptor
    // while retaining its Java-visible descriptor value. Dropping the replaced
    // socket before DatagramChannelImpl rebinds is especially important on macOS.
    vm.socket_handles().insert(target_fd, handle).await?;
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
    if count == 0 {
        return Ok(Some(Value::Int(0)));
    }

    let vm = thread.vm()?;
    let (cloned_socket, non_blocking) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("read: clone: {e}")))?,
            handle.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, false).await?;
        if status < 0 {
            return Ok(Some(Value::Int(i32::try_from(status)?)));
        }
    }

    let result = tokio::task::spawn_blocking(move || {
        super::datagramchannelimpl::receive_connected_datagram(&cloned_socket, count)
    })
    .await
    .map_err(|e| InternalError(format!("read: spawn: {e}")))?;

    let (n, data) = match result {
        Ok((n, data)) => (i32::try_from(n)?, data),
        Err(error) => {
            return Ok(Some(Value::Int(i32::try_from(
                super::datagramchannelimpl::datagram_io_status(&error)?,
            )?)));
        }
    };
    if n > 0 {
        write_native_bytes(
            vm.native_memory(),
            address,
            &data,
            "DatagramDispatcher.read0",
        )?;
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
    let iov_entries = parse_iovecs(vm.native_memory(), iov_address, iov_count)?;
    let total_len = iov_entries.iter().try_fold(0_usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| InternalError("iovec total length overflow".to_string()))
    })?;
    if total_len == 0 {
        return Ok(Some(Value::Long(0)));
    }

    let (cloned_socket, non_blocking) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("readv: clone: {e}")))?,
            handle.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, false).await?;
        if status < 0 {
            return Ok(Some(Value::Long(status)));
        }
    }

    let result = tokio::task::spawn_blocking(move || {
        super::datagramchannelimpl::receive_connected_datagram(&cloned_socket, total_len)
    })
    .await
    .map_err(|e| InternalError(format!("readv: spawn: {e}")))?;

    let (n, data) = match result {
        Ok((n, data)) => (i64::try_from(n)?, data),
        Err(error) => {
            return Ok(Some(Value::Long(
                super::datagramchannelimpl::datagram_io_status(&error)?,
            )));
        }
    };
    if n > 0 {
        let mut offset = 0usize;
        let data_len = data.len();
        for (base, len) in &iov_entries {
            let to_copy = (*len).min(data_len.saturating_sub(offset));
            if to_copy > 0 {
                let bytes =
                    bounds::range(&data, offset..offset + to_copy, "DatagramDispatcher.readv0")?;
                write_native_bytes(
                    vm.native_memory(),
                    *base,
                    bytes,
                    "DatagramDispatcher.readv0",
                )?;
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
    let data = native_bytes(
        vm.native_memory(),
        address,
        count,
        "DatagramDispatcher.write0",
    )?;

    let (cloned_socket, non_blocking) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("write: clone: {e}")))?,
            handle.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, true).await?;
        if status < 0 {
            return Ok(Some(Value::Int(i32::try_from(status)?)));
        }
    }

    let result =
        tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned_socket, &data))
            .await
            .map_err(|e| InternalError(format!("write: spawn: {e}")))?;

    match result {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(error) => Ok(Some(Value::Int(i32::try_from(
            super::datagramchannelimpl::datagram_io_status(&error)?,
        )?))),
    }
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
    let iov_entries = parse_iovecs(vm.native_memory(), iov_address, iov_count)?;
    let total_len = iov_entries.iter().try_fold(0_usize, |total, (_, length)| {
        total
            .checked_add(*length)
            .ok_or_else(|| InternalError("iovec total length overflow".to_string()))
    })?;
    let mut data = Vec::new();
    data.try_reserve_exact(total_len)
        .map_err(|error| InternalError(format!("writev allocation failed: {error}")))?;
    for (base, length) in iov_entries {
        let chunk = native_bytes(
            vm.native_memory(),
            base,
            length,
            "DatagramDispatcher.writev0",
        )?;
        data.extend_from_slice(&chunk);
    }

    let (cloned_socket, non_blocking) = {
        let handle = vm
            .socket_handles()
            .get(&fd)
            .await
            .ok_or_else(|| InternalError(format!("Socket not found for fd {fd}")))?;
        let Some(socket) = handle.socket_type.as_raw() else {
            return Err(InternalError("expected raw socket".to_string()));
        };
        (
            socket
                .try_clone()
                .map_err(|e| InternalError(format!("writev: clone: {e}")))?,
            handle.non_blocking,
        )
    };
    if !non_blocking {
        let status = super::socketdispatcher::wait_raw_socket(&thread, fd, true).await?;
        if status < 0 {
            return Ok(Some(Value::Long(status)));
        }
    }

    let result =
        tokio::task::spawn_blocking(move || std::io::Write::write(&mut &cloned_socket, &data))
            .await
            .map_err(|e| InternalError(format!("writev: spawn: {e}")))?;

    match result {
        Ok(n) => Ok(Some(Value::Long(i64::try_from(n)?))),
        Err(error) => Ok(Some(Value::Long(
            super::datagramchannelimpl::datagram_io_status(&error)?,
        ))),
    }
}

#[cfg(all(test, target_family = "unix", not(target_family = "wasm")))]
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

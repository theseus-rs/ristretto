use crate::sun::nio::fs::managed_files;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_21, JAVA_25};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::io::SeekFrom;
use std::sync::Arc;

/// Extract the integer fd from a `FileDescriptor` Java object value.
fn extract_fd(fd_value: &Value) -> Result<i64> {
    let guard = fd_value.as_reference()?;
    let Reference::Object(object) = &*guard else {
        return Err(InternalError("FileDescriptor: not an object".to_string()));
    };
    let fd = object.value("fd")?.as_i32()?;
    Ok(i64::from(fd))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.allocationGranularity0()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn allocation_granularity_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(4096)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.available0(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn available_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let size = managed_files::metadata(file_handles, fd)
        .await
        .map_err(|e| InternalError(format!("available0: {e}")))?
        .len();
    let pos = managed_files::seek(file_handles, fd, SeekFrom::Current(0))
        .await
        .map_err(|e| InternalError(format!("available0: {e}")))?;
    let available = size.saturating_sub(pos);
    Ok(Some(Value::Int(
        i32::try_from(available).unwrap_or(i32::MAX),
    )))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.closeIntFD(I)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn close_int_fd<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    managed_files::close(vm.file_handles(), i64::from(fd)).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn force_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let metadata_only = parameters.pop_int()? != 0;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    if metadata_only {
        managed_files::sync_data(file_handles, fd)
            .await
            .map_err(|e| InternalError(format!("force0: {e}")))?;
    } else {
        managed_files::sync_all(file_handles, fd)
            .await
            .map_err(|e| InternalError(format!("force0: {e}")))?;
    }
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn is_other_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let metadata = managed_files::metadata(vm.file_handles(), fd)
        .await
        .map_err(|e| InternalError(format!("isOther0: {e}")))?;
    let is_other = !metadata.is_file() && !metadata.is_dir() && !metadata.is_symlink();
    Ok(Some(Value::Int(i32::from(is_other))))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn lock_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let shared = parameters.pop_int()? != 0;
    let _size = parameters.pop_long()?;
    let _pos = parameters.pop_long()?;
    let blocking = parameters.pop_int()? != 0;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let value = managed_files::lock(thread.vm()?.file_handles(), fd, shared, blocking)
        .await
        .map_err(|e| InternalError(format!("lock0: {e}")))?;

    Ok(Some(Value::Int(value)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn map_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_sync = parameters.pop_int()? != 0;
    let length = parameters.pop_long()?;
    let position = parameters.pop_long()?;
    let _prot = parameters.pop_int()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let length =
        u64::try_from(length).map_err(|_| InternalError("map0: negative length".to_string()))?;
    let position = u64::try_from(position)
        .map_err(|_| InternalError("map0: negative position".to_string()))?;

    // Allocate native memory and read file contents into it
    let len =
        usize::try_from(length).map_err(|_| InternalError("map0: length too large".to_string()))?;
    let file_handles = vm.file_handles();
    managed_files::seek(file_handles, fd, SeekFrom::Start(position))
        .await
        .map_err(|e| InternalError(format!("map0: {e}")))?;
    let mut buf = vec![0u8; len];
    let _ = managed_files::read(file_handles, fd, &mut buf)
        .await
        .map_err(|e| InternalError(format!("map0: {e}")))?;
    let address = vm.native_memory().allocate(len);
    vm.native_memory().write_bytes(address, &buf);
    Ok(Some(Value::Long(address)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn pread_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let position = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();

    // Save current position
    let offset = u64::try_from(position)
        .map_err(|_| InternalError("pread0: negative position".to_string()))?;
    let mut buf = vec![0u8; count];
    let n = managed_files::read_at(file_handles, fd, &mut buf, offset)
        .await
        .map_err(|e| InternalError(format!("pread0: {e}")))?;

    if n > 0 {
        vm.native_memory().write_bytes(address, &buf[..n]);
    }
    // Return -1 for EOF per JVM spec
    let result = if n == 0 && count > 0 {
        -1
    } else {
        i32::try_from(n)?
    };
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn pwrite_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let position = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();

    let data = vm.native_memory().read_bytes(address, count);
    // Save current position
    let offset = u64::try_from(position)
        .map_err(|_| InternalError("pwrite0: negative position".to_string()))?;
    let n = managed_files::write_at(file_handles, fd, &data, offset)
        .await
        .map_err(|e| InternalError(format!("pwrite0: {e}")))?;

    Ok(Some(Value::Int(i32::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn read_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let mut buf = vec![0u8; count];

    let vm = thread.vm()?;
    match managed_files::read(vm.file_handles(), fd, &mut buf).await {
        Ok(n) => {
            if n > 0 {
                vm.native_memory().write_bytes(address, &buf[..n]);
            }
            // Return -1 for EOF per JVM spec
            let result = if n == 0 && count > 0 {
                -1
            } else {
                i32::try_from(n)?
            };
            Ok(Some(Value::Int(result)))
        }
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(5 /* EIO */);
            Err(InternalError(format!(
                "UnixFileDispatcherImpl.read0: errno={errno}"
            )))
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn readv_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let vm = thread.vm()?;
    let native_memory = vm.native_memory();
    let file_handles = vm.file_handles();

    let count = usize::try_from(count)?;
    let mut chunks = Vec::with_capacity(count);
    let mut iov_bases = Vec::with_capacity(count);

    for i in 0..count {
        let iov_offset = i64::try_from(i)? * 16;
        let iov_base = native_memory
            .read_i64(address + iov_offset)
            .ok_or_else(|| InternalError("readv0: bad iov_base".to_string()))?;
        let iov_len = native_memory
            .read_i64(address + iov_offset + 8)
            .ok_or_else(|| InternalError("readv0: bad iov_len".to_string()))?;
        let len = usize::try_from(iov_len)?;

        if len > 0 {
            chunks.push(vec![0u8; len]);
            iov_bases.push(iov_base);
        }
    }

    if chunks.is_empty() {
        return Ok(Some(Value::Long(0)));
    }

    let (n, returned_chunks) = managed_files::readv(file_handles, fd, chunks)
        .await
        .map_err(|e| InternalError(format!("readv0: {e}")))?;

    let mut total: i64 = 0;
    let mut remaining = n;
    for (i, chunk) in returned_chunks.into_iter().enumerate() {
        if remaining == 0 {
            break;
        }
        let chunk_len = std::cmp::min(remaining, chunk.len());
        native_memory.write_bytes(iov_bases[i], &chunk[..chunk_len]);
        total += i64::try_from(chunk_len)?;
        remaining -= chunk_len;
    }

    Ok(Some(Value::Long(total)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn release_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_long()?;
    let _pos = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    managed_files::unlock(thread.vm()?.file_handles(), fd)
        .await
        .map_err(|e| InternalError(format!("release0: {e}")))?;

    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn seek_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pos = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let vm = thread.vm()?;
    let result = if pos == -1 {
        // Return current position
        managed_files::seek(vm.file_handles(), fd, SeekFrom::Current(0))
            .await
            .map_err(|e| InternalError(format!("seek0: {e}")))?
    } else {
        let offset = u64::try_from(pos)
            .map_err(|_| InternalError("seek0: negative position".to_string()))?;
        managed_files::seek(vm.file_handles(), fd, SeekFrom::Start(offset))
            .await
            .map_err(|e| InternalError(format!("seek0: {e}")))?
    };
    Ok(Some(Value::Long(i64::try_from(result)?)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_direct_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // O_DIRECT not supported in managed VM; return -1 per JVM convention
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn size_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let size = managed_files::metadata(vm.file_handles(), fd)
        .await
        .map_err(|e| InternalError(format!("size0: {e}")))?
        .len();
    Ok(Some(Value::Long(i64::try_from(size)?)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn truncate_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let size = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;
    let vm = thread.vm()?;
    let size =
        u64::try_from(size).map_err(|_| InternalError("truncate0: negative size".to_string()))?;
    managed_files::set_len(vm.file_handles(), fd, size)
        .await
        .map_err(|e| InternalError(format!("truncate0: {e}")))?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.unmap0(JJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn unmap_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    vm.native_memory().free(address);
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn write_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let data = vm.native_memory().read_bytes(address, count);

    match managed_files::write(vm.file_handles(), fd, &data).await {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(5 /* EIO */);
            Err(InternalError(format!(
                "UnixFileDispatcherImpl.write0: errno={errno}"
            )))
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn writev_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = extract_fd(&fd_value)?;

    let vm = thread.vm()?;
    let native_memory = vm.native_memory();
    let file_handles = vm.file_handles();
    let mut chunks = Vec::new();

    for i in 0..count {
        let iov_offset = i64::from(i) * 16;
        let iov_base = native_memory
            .read_i64(address + iov_offset)
            .ok_or_else(|| InternalError("writev0: bad iov_base".to_string()))?;
        let iov_len = native_memory
            .read_i64(address + iov_offset + 8)
            .ok_or_else(|| InternalError("writev0: bad iov_len".to_string()))?;
        let len = usize::try_from(iov_len)?;
        if len > 0 {
            chunks.push(native_memory.read_bytes(iov_base, len));
        }
    }

    if chunks.is_empty() {
        return Ok(Some(Value::Long(0)));
    }

    let n = managed_files::writev(file_handles, fd, chunks)
        .await
        .map_err(|e| InternalError(format!("writev0: {e}")))?;

    Ok(Some(Value::Long(i64::try_from(n)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allocation_granularity_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = allocation_granularity_0(thread, Parameters::default())
            .await
            .expect("allocation_granularity_0");
        assert_eq!(result, Some(Value::Long(4096)));
    }

    #[tokio::test]
    async fn test_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_int_fd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![Value::Int(-1)]);
        let result = close_int_fd(thread, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = force_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_other_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_other_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lock_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = map_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pread_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pread_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pwrite_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pwrite_0(thread, Parameters::default()).await;
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
    async fn test_release_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_seek_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = seek_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_direct_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_direct_0(thread, Parameters::default())
            .await
            .expect("set_direct_0");
        assert_eq!(result, Some(Value::Int(-1)));
    }

    #[tokio::test]
    async fn test_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = size_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_truncate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = truncate_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unmap_0(thread, Parameters::default()).await;
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

use crate::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::VersionSpecification::{
    Any, Between, Equal, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::io::SeekFrom;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.canTransferToFromOverlappedMap0()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn can_transfer_to_from_overlapped_map_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0))) // false
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    managed_files::close(vm.file_handles(), fd).await;
    // NIO SocketChannel/ServerSocketChannel route their close through
    // FileDispatcherImpl, so also clean up any socket-related state to
    // prevent leaking SocketHandle, SocketMode, SocketTimeout, and
    // SocketDomain entries.
    #[cfg(not(target_family = "wasm"))]
    {
        #[expect(clippy::cast_possible_truncation)]
        vm.socket_handles().remove(&(fd as i32)).await;
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.closeIntFD(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_int_fd<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    managed_files::close(vm.file_handles(), i64::from(fd)).await;
    #[cfg(not(target_family = "wasm"))]
    vm.socket_handles().remove(&fd).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn dup_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dst_fd_value = parameters.pop()?;
    let src_fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let src_fd = file_descriptor_from_java_object(&vm, &src_fd_value)?;

    let new_fd = managed_files::try_clone(vm.file_handles(), vm.resource_manager(), src_fd)
        .await
        .map_err(|e| InternalError(format!("dup0: {e}")))?;

    let new_fd_int = i32::try_from(new_fd)
        .map_err(|_| InternalError("dup0: new fd out of i32 range".to_string()))?;
    let mut guard = dst_fd_value.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "dup0: destination is not an object".to_string(),
        ));
    };
    object.set_value("fd", Value::Int(new_fd_int))?;

    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I",
    Any
)]
#[async_method]
pub async fn force_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let metadata_only = parameters.pop_int()? != 0;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
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

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.init0()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn lock_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let shared = parameters.pop_int()? != 0;
    let _size = parameters.pop_long()?;
    let _pos = parameters.pop_long()?;
    let blocking = parameters.pop_int()? != 0;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    let value = managed_files::lock(vm.file_handles(), fd, shared, blocking)
        .await
        .map_err(|e| InternalError(format!("lock0: {e}")))?;

    Ok(Some(Value::Int(value)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.preClose0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pre_close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pread_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let position = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    let count = usize::try_from(count)?;
    let offset = u64::try_from(position)
        .map_err(|_| InternalError("pread0: negative position".to_string()))?;
    let mut buf = vec![0u8; count];
    let n = managed_files::read_at(vm.file_handles(), fd, &mut buf, offset)
        .await
        .map_err(|e| InternalError(format!("pread0: {e}")))?;

    if n > 0 {
        vm.native_memory().write_bytes(address, &buf[..n]);
    }
    let result = if n == 0 && count > 0 {
        -1
    } else {
        i32::try_from(n)?
    };
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pwrite_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let position = parameters.pop_long()?;
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    let count = usize::try_from(count)?;
    let data = vm.native_memory().read_bytes(address, count);
    let offset = u64::try_from(position)
        .map_err(|_| InternalError("pwrite0: negative position".to_string()))?;
    let n = managed_files::write_at(vm.file_handles(), fd, &data, offset)
        .await
        .map_err(|e| InternalError(format!("pwrite0: {e}")))?;

    Ok(Some(Value::Int(i32::try_from(n)?)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn read_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    let count = usize::try_from(count)?;
    let mut buf = vec![0u8; count];

    match managed_files::read(vm.file_handles(), fd, &mut buf).await {
        Ok(n) => {
            if n > 0 {
                vm.native_memory().write_bytes(address, &buf[..n]);
            }
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
                "FileDispatcherImpl.read0: errno={errno}"
            )))
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn readv_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

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
    "sun/nio/ch/FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn release_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_long()?;
    let _pos = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    managed_files::unlock(vm.file_handles(), fd)
        .await
        .map_err(|e| InternalError(format!("release0: {e}")))?;

    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn seek_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pos = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

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
    "sun/nio/ch/FileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_direct_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn size_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    let size = managed_files::metadata(vm.file_handles(), fd)
        .await
        .map_err(|e| InternalError(format!("size0: {e}")))?
        .len();
    Ok(Some(Value::Long(i64::try_from(size)?)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn transfer_to_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return IOStatus.UNSUPPORTED_CASE (-6) or IOStatus.UNSUPPORTED (-4)
    Ok(Some(Value::Long(-4)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn truncate_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let size = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    let size =
        u64::try_from(size).map_err(|_| InternalError("truncate0: negative size".to_string()))?;
    managed_files::set_len(vm.file_handles(), fd, size)
        .await
        .map_err(|e| InternalError(format!("truncate0: {e}")))?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn write_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

    let count = usize::try_from(count)?;
    let data = vm.native_memory().read_bytes(address, count);

    match managed_files::write(vm.file_handles(), fd, &data).await {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(5 /* EIO */);
            Err(InternalError(format!(
                "FileDispatcherImpl.write0: errno={errno}"
            )))
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn writev_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

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
    async fn test_can_transfer_to_from_overlapped_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = can_transfer_to_from_overlapped_map_0(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
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
    async fn test_dup_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = force_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_lock_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pre_close_0(thread, Parameters::default()).await;
        assert!(result.is_ok());
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
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = transfer_to_0(thread, Parameters::default()).await.unwrap();
        assert_eq!(result, Some(Value::Long(-4)));
    }

    #[tokio::test]
    async fn test_truncate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = truncate_0(thread, Parameters::default()).await;
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

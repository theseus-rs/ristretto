use ristretto_classfile::VersionSpecification::{
    Any, Between, Equal, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use crate::sun::nio::fs::managed_files;

/// Extract the fd int field from a java.io.FileDescriptor object.
fn get_fd(fd_value: &Value) -> Result<i32> {
    let guard = fd_value.as_reference()?;
    let Reference::Object(object) = &*guard else {
        return Err(InternalError(
            "FileDispatcherImpl: not a FileDescriptor object".to_string(),
        ));
    };
    Ok(object.value("fd")?.as_i32()?)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.canTransferToFromOverlappedMap0()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn can_transfer_to_from_overlapped_map_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V", Any)]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    managed_files::close(vm.nio_file_handles(), fd).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.closeIntFD(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_int_fd<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    managed_files::close(vm.nio_file_handles(), fd).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn dup_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Stub: dup not supported
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I",
    Any
)]
#[async_method]
pub async fn force_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 0 for success
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.init0()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.allocationGranularity0()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn allocation_granularity_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Some(Value::Long(65536)))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Some(Value::Long(4096)))
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.maxDirectTransferSize0()I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn max_direct_transfer_size_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.duplicateHandle(J)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn duplicate_handle<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_other_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.available0(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn available_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I",
    Any
)]
#[async_method]
pub async fn lock_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return 0 for success (no lock acquired but no error)
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.preClose0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pre_close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I",
    Any
)]
#[async_method]
pub async fn pread_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Positional read not implemented; return -1 (EOF)
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I",
    Any
)]
#[async_method]
pub async fn pwrite_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I",
    Any
)]
#[async_method]
pub async fn read_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let mut buf = vec![0u8; count];

    let vm = thread.vm()?;
    match managed_files::read(vm.nio_file_handles(), fd, &mut buf).await {
        Ok(n) if n > 0 => {
            vm.native_memory().write_bytes(address, &buf[..n]);
            Ok(Some(Value::Int(i32::try_from(n)?)))
        }
        _ => Ok(Some(Value::Int(-1))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J",
    Any
)]
#[async_method]
pub async fn readv_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(-1)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V",
    Any
)]
#[async_method]
pub async fn release_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J", Any)]
#[async_method]
pub async fn seek_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let offset = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let vm = thread.vm()?;
    let pos = if offset < 0 {
        // Negative offset means get current position
        managed_files::seek(vm.nio_file_handles(), fd, std::io::SeekFrom::Current(0)).await
    } else {
        managed_files::seek(
            vm.nio_file_handles(),
            fd,
            std::io::SeekFrom::Start(u64::try_from(offset)?),
        )
        .await
    };
    match pos {
        Ok(p) => Ok(Some(Value::Long(i64::try_from(p)?))),
        Err(_) => Ok(Some(Value::Long(-1))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn set_direct_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J", Any)]
#[async_method]
pub async fn size_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let vm = thread.vm()?;
    match managed_files::file_size(vm.nio_file_handles(), fd).await {
        Ok(size) => Ok(Some(Value::Long(size))),
        Err(_) => Ok(Some(Value::Long(-1))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn transfer_to_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Return -2 to indicate unsupported (IOUtil.UNSUPPORTED_CASE)
    Ok(Some(Value::Long(-2)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I",
    Any
)]
#[async_method]
pub async fn truncate_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I",
    Any
)]
#[async_method]
pub async fn write_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;

    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let buf = vm.native_memory().read_bytes(address, count);

    match managed_files::write(vm.nio_file_handles(), fd, &buf).await {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(_) => Ok(Some(Value::Int(-1))),
    }
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J",
    Any
)]
#[async_method]
pub async fn writev_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(-1)))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn map_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(InternalError(
        "FileDispatcherImpl.map0: memory mapping not supported".to_string(),
    ))
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.unmap0(JJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn unmap_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_can_transfer_to_from_overlapped_map_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = can_transfer_to_from_overlapped_map_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_dup_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = dup_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_force_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = force_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_lock_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = lock_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_pre_close_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pre_close_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_pread_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pread_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_pwrite_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = pwrite_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_readv_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = readv_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_release_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = release_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_allocation_granularity_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = allocation_granularity_0(thread, Parameters::default()).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_max_direct_transfer_size_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_direct_transfer_size_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_duplicate_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Long(42)]);
        let result = duplicate_handle(thread, params).await?;
        assert_eq!(result, Some(Value::Long(42)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_other_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_other_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_available_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = available_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer_to_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = transfer_to_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(-2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_truncate_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = truncate_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_writev_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writev_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unmap_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = unmap_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }
}

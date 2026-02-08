use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_21, JAVA_25};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use crate::sun::nio::fs::managed_files;

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.allocationGranularity0()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn allocation_granularity_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.allocationGranularity0()J")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.available0(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn available_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.available0(Ljava/io/FileDescriptor;)I")
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
    managed_files::close(vm.nio_file_handles(), fd).await;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn force_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn is_other_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn lock_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn map_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn pread_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn pwrite_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I")
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

    let fd = {
        let guard = fd_value.as_reference()?;
        let Reference::Object(object) = &*guard else {
            return Err(InternalError("read0: not an object".to_string()));
        };
        object.value("fd")?.as_i32()?
    };

    let count = usize::try_from(count)?;
    let mut buf = vec![0u8; count];

    let vm = thread.vm()?;
    match managed_files::read(vm.nio_file_handles(), fd, &mut buf).await {
        Ok(n) => {
            if n > 0 {
                vm.native_memory().write_bytes(address, &buf[..n]);
            }
            Ok(Some(Value::Int(i32::try_from(n)?)))
        }
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(5);
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
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn release_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn seek_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J")
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
    todo!("sun.nio.ch.UnixFileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn size_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn truncate_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.unmap0(JJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn unmap_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.unmap0(JJ)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn write_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I")
}

#[intrinsic_method(
    "sun/nio/ch/UnixFileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn writev_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.allocationGranularity0()J"
    )]
    async fn test_allocation_granularity_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocation_granularity_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.available0(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = available_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_close_int_fd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![Value::Int(-1)]);
        let result = close_int_fd(thread, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I"
    )]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = force_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z"
    )]
    async fn test_is_other_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_other_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I"
    )]
    async fn test_lock_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lock_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J"
    )]
    async fn test_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = map_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I"
    )]
    async fn test_pread_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pread_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I"
    )]
    async fn test_pwrite_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pwrite_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readv_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V"
    )]
    async fn test_release_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J"
    )]
    async fn test_seek_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = seek_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_set_direct_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_direct_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J"
    )]
    async fn test_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I"
    )]
    async fn test_truncate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = truncate_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.unmap0(JJ)I")]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unmap_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writev_0(thread, Parameters::default()).await;
    }
}

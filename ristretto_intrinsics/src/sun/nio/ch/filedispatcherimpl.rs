use ristretto_classfile::VersionSpecification::{
    Any, Between, Equal, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.canTransferToFromOverlappedMap0()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn can_transfer_to_from_overlapped_map_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.canTransferToFromOverlappedMap0()Z");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.closeIntFD(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_int_fd<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.closeIntFD(I)V");
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
    todo!("sun.nio.ch.FileDispatcherImpl.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V");
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
    todo!("sun.nio.ch.FileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I");
}

#[intrinsic_method("sun/nio/ch/FileDispatcherImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
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
pub async fn lock_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I");
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
    todo!("sun.nio.ch.FileDispatcherImpl.preClose0(Ljava/io/FileDescriptor;)V");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pread_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn pwrite_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn read_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn readv_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn release_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn seek_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J");
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
    todo!("sun.nio.ch.FileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn size_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J");
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
    todo!(
        "sun.nio.ch.FileDispatcherImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J"
    );
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn truncate_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn write_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I");
}

#[intrinsic_method(
    "sun/nio/ch/FileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn writev_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.canTransferToFromOverlappedMap0()Z"
    )]
    async fn test_can_transfer_to_from_overlapped_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_transfer_to_from_overlapped_map_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.closeIntFD(I)V")]
    async fn test_close_int_fd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_int_fd(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V"
    )]
    async fn test_dup_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I"
    )]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = force_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I"
    )]
    async fn test_lock_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lock_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.preClose0(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pre_close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I"
    )]
    async fn test_pread_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pread_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I"
    )]
    async fn test_pwrite_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pwrite_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readv_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V"
    )]
    async fn test_release_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J"
    )]
    async fn test_seek_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = seek_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_set_direct_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_direct_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J"
    )]
    async fn test_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J"
    )]
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_to_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I"
    )]
    async fn test_truncate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = truncate_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writev_0(thread, Parameters::default()).await;
    }
}

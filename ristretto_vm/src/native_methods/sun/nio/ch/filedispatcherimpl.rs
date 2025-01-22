use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/FileDispatcherImpl";

/// Register all native methods for `sun.nio.ch.FileDispatcherImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 && registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "setDirect0",
            "(Ljava/io/FileDescriptor;)I",
            set_direct_0,
        );
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "close0", "(Ljava/io/FileDescriptor;)V", close_0);
        registry.register(CLASS_NAME, "closeIntFD", "(I)V", close_int_fd);
        registry.register(CLASS_NAME, "init", "()V", init);
        registry.register(
            CLASS_NAME,
            "lock0",
            "(Ljava/io/FileDescriptor;ZJJZ)I",
            lock_0,
        );
        registry.register(
            CLASS_NAME,
            "preClose0",
            "(Ljava/io/FileDescriptor;)V",
            pre_close_0,
        );
        registry.register(
            CLASS_NAME,
            "pread0",
            "(Ljava/io/FileDescriptor;JIJ)I",
            pread_0,
        );
        registry.register(
            CLASS_NAME,
            "pwrite0",
            "(Ljava/io/FileDescriptor;JIJ)I",
            pwrite_0,
        );
        registry.register(CLASS_NAME, "read0", "(Ljava/io/FileDescriptor;JI)I", read_0);
        registry.register(
            CLASS_NAME,
            "readv0",
            "(Ljava/io/FileDescriptor;JI)J",
            readv_0,
        );
        registry.register(
            CLASS_NAME,
            "release0",
            "(Ljava/io/FileDescriptor;JJ)V",
            release_0,
        );
        registry.register(CLASS_NAME, "seek0", "(Ljava/io/FileDescriptor;J)J", seek_0);
        registry.register(CLASS_NAME, "size0", "(Ljava/io/FileDescriptor;)J", size_0);
        registry.register(
            CLASS_NAME,
            "truncate0",
            "(Ljava/io/FileDescriptor;J)I",
            truncate_0,
        );
        registry.register(
            CLASS_NAME,
            "write0",
            "(Ljava/io/FileDescriptor;JI)I",
            write_0,
        );
        registry.register(
            CLASS_NAME,
            "writev0",
            "(Ljava/io/FileDescriptor;JI)J",
            writev_0,
        );
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(
            CLASS_NAME,
            "canTransferToFromOverlappedMap0",
            "()Z",
            can_transfer_to_from_overlapped_map_0,
        );
        registry.register(
            CLASS_NAME,
            "dup0",
            "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
            dup_0,
        );
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "transferTo0",
            "(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J",
            transfer_to_0,
        );
    }

    #[cfg(target_os = "macos")]
    {
        registry.register(
            CLASS_NAME,
            "force0",
            "(Ljava/io/FileDescriptor;Z)I",
            force_0,
        );
    }
}

#[async_recursion(?Send)]
async fn can_transfer_to_from_overlapped_map_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.canTransferToFromOverlappedMap0()Z");
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.close0(Ljava/io/FileDescriptor;)V");
}

#[async_recursion(?Send)]
async fn close_int_fd(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.closeIntFD(I)V");
}

#[async_recursion(?Send)]
async fn dup_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V");
}

#[cfg(target_os = "macos")]
#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I");
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn lock_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I");
}

#[async_recursion(?Send)]
async fn pre_close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.preClose0(Ljava/io/FileDescriptor;)V");
}

#[async_recursion(?Send)]
async fn pread_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I");
}

#[async_recursion(?Send)]
async fn pwrite_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I");
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I");
}

#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J");
}

#[async_recursion(?Send)]
async fn release_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V");
}

#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J");
}

#[async_recursion(?Send)]
async fn set_direct_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I");
}

#[async_recursion(?Send)]
async fn size_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J");
}

#[async_recursion(?Send)]
async fn transfer_to_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;Z)J");
}

#[async_recursion(?Send)]
async fn truncate_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I");
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I");
}

#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

    #[cfg(target_os = "macos")]
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

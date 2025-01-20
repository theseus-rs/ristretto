use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/UnixFileDispatcherImpl";

/// Register all native methods for `sun.nio.ch.UnixFileDispatcherImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "allocationGranularity0",
        "()J",
        allocation_granularity_0,
    );
    registry.register(CLASS_NAME, "closeIntFD", "(I)V", close_int_fd);
    registry.register(
        CLASS_NAME,
        "force0",
        "(Ljava/io/FileDescriptor;Z)I",
        force_0,
    );
    registry.register(
        CLASS_NAME,
        "lock0",
        "(Ljava/io/FileDescriptor;ZJJZ)I",
        lock_0,
    );
    registry.register(CLASS_NAME, "map0", "(Ljava/io/FileDescriptor;IJJZ)J", map_0);
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
    registry.register(
        CLASS_NAME,
        "setDirect0",
        "(Ljava/io/FileDescriptor;)I",
        set_direct_0,
    );
    registry.register(CLASS_NAME, "size0", "(Ljava/io/FileDescriptor;)J", size_0);
    registry.register(
        CLASS_NAME,
        "truncate0",
        "(Ljava/io/FileDescriptor;J)I",
        truncate_0,
    );
    registry.register(CLASS_NAME, "unmap0", "(JJ)I", unmap_0);
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

#[async_recursion(?Send)]
async fn allocation_granularity_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.allocationGranularity0()J")
}

#[async_recursion(?Send)]
async fn close_int_fd(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.closeIntFD(I)V")
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I")
}

#[async_recursion(?Send)]
async fn lock_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I")
}

#[async_recursion(?Send)]
async fn map_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J")
}

#[async_recursion(?Send)]
async fn pread_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I")
}

#[async_recursion(?Send)]
async fn pwrite_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J")
}

#[async_recursion(?Send)]
async fn release_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J")
}

#[async_recursion(?Send)]
async fn set_direct_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn size_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J")
}

#[async_recursion(?Send)]
async fn truncate_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I")
}

#[async_recursion(?Send)]
async fn unmap_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.unmap0(JJ)I")
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.closeIntFD(I)V"
    )]
    async fn test_close_int_fd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_int_fd(thread, Parameters::default()).await;
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
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixFileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
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

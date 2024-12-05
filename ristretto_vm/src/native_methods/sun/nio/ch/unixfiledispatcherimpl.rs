use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.UnixFileDispatcherImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/UnixFileDispatcherImpl";
    registry.register(
        class_name,
        "allocationGranularity0",
        "()J",
        allocation_granularity_0,
    );
    registry.register(class_name, "closeIntFD", "(I)V", close_int_fd);
    registry.register(
        class_name,
        "force0",
        "(Ljava/io/FileDescriptor;Z)I",
        force_0,
    );
    registry.register(
        class_name,
        "lock0",
        "(Ljava/io/FileDescriptor;ZJJZ)I",
        lock_0,
    );
    registry.register(class_name, "map0", "(Ljava/io/FileDescriptor;IJJZ)J", map_0);
    registry.register(
        class_name,
        "pread0",
        "(Ljava/io/FileDescriptor;JIJ)I",
        pread_0,
    );
    registry.register(
        class_name,
        "pwrite0",
        "(Ljava/io/FileDescriptor;JIJ)I",
        pwrite_0,
    );
    registry.register(class_name, "read0", "(Ljava/io/FileDescriptor;JI)I", read_0);
    registry.register(
        class_name,
        "readv0",
        "(Ljava/io/FileDescriptor;JI)J",
        readv_0,
    );
    registry.register(
        class_name,
        "release0",
        "(Ljava/io/FileDescriptor;JJ)V",
        release_0,
    );
    registry.register(class_name, "seek0", "(Ljava/io/FileDescriptor;J)J", seek_0);
    registry.register(
        class_name,
        "setDirect0",
        "(Ljava/io/FileDescriptor;)I",
        set_direct_0,
    );
    registry.register(class_name, "size0", "(Ljava/io/FileDescriptor;)J", size_0);
    registry.register(
        class_name,
        "truncate0",
        "(Ljava/io/FileDescriptor;J)I",
        truncate_0,
    );
    registry.register(class_name, "unmap0", "(JJ)I", unmap_0);
    registry.register(
        class_name,
        "write0",
        "(Ljava/io/FileDescriptor;JI)I",
        write_0,
    );
    registry.register(
        class_name,
        "writev0",
        "(Ljava/io/FileDescriptor;JI)J",
        writev_0,
    );
}

#[async_recursion(?Send)]
async fn allocation_granularity_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn close_int_fd(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.closeIntFD(I)V")
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.force0(Ljava/io/FileDescriptor;Z)I")
}

#[async_recursion(?Send)]
async fn lock_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.lock0(Ljava/io/FileDescriptor;ZJJZ)I")
}

#[async_recursion(?Send)]
async fn map_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J")
}

#[async_recursion(?Send)]
async fn pread_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I")
}

#[async_recursion(?Send)]
async fn pwrite_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.pwrite0(Ljava/io/FileDescriptor;JIJ)I")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.readv0(Ljava/io/FileDescriptor;JI)J")
}

#[async_recursion(?Send)]
async fn release_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.release0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J")
}

#[async_recursion(?Send)]
async fn set_direct_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.setDirect0(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J")
}

#[async_recursion(?Send)]
async fn truncate_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I")
}

#[async_recursion(?Send)]
async fn unmap_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.unmap0(JJ)I")
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixFileDispatcherImpl.writev0(Ljava/io/FileDescriptor;JI)J")
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.FileDispatcherImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/FileDispatcherImpl";
    registry.register(class_name, "close0", "(Ljava/io/FileDescriptor;)V", close_0);
    registry.register(class_name, "closeIntFD", "(I)V", close_int_fd);
    registry.register(
        class_name,
        "force0",
        "(Ljava/io/FileDescriptor;Z)I",
        force_0,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "lock0",
        "(Ljava/io/FileDescriptor;ZJJZ)I",
        lock_0,
    );
    registry.register(
        class_name,
        "preClose0",
        "(Ljava/io/FileDescriptor;)V",
        pre_close_0,
    );
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
    registry.register(class_name, "size0", "(Ljava/io/FileDescriptor;)J", size_0);
    registry.register(
        class_name,
        "truncate0",
        "(Ljava/io/FileDescriptor;J)I",
        truncate_0,
    );
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn close_int_fd(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lock_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn pre_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn pread_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn pwrite_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn release_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn truncate_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

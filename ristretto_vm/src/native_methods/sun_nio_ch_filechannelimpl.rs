use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.FileChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/FileChannelImpl";
    registry.register(class_name, "initIDs", "()J", init_ids);
    registry.register(class_name, "map0", "(IJJ)J", map_0);
    registry.register(
        class_name,
        "transferTo0",
        "(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
        transfer_to_0,
    );
    registry.register(class_name, "unmap0", "(JJ)I", unmap_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn map_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn transfer_to_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unmap_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

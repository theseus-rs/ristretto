use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDropTargetContextPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDropTargetContextPeer";
    registry.register(class_name, "addTransfer", "(JJJ)V", add_transfer);
    registry.register(class_name, "dropDone", "(JJZZI)V", drop_done);
    registry.register(class_name, "startTransfer", "(JJ)J", start_transfer);
}

#[async_recursion(?Send)]
async fn add_transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn drop_done(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn start_transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.NativeSignalHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/NativeSignalHandler";
    registry.register(class_name, "handle0", "(IJ)V", handle_0);
}

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

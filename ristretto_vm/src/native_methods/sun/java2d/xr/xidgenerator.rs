use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.xr.XIDGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/xr/XIDGenerator";
    registry.register(class_name, "bufferXIDs", "([II)V", buffer_x_ids);
}

#[async_recursion(?Send)]
async fn buffer_x_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XIDGenerator.bufferXIDs([II)V");
}

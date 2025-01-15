use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/vm/vector/VectorSupport";

/// Register all native methods for `jdk.internal.vm.vector.VectorSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getMaxLaneCount",
        "(Ljava/lang/Class;)I",
        get_max_lane_count,
    );
    registry.register(CLASS_NAME, "registerNatives", "()I", register_natives);
}

#[async_recursion(?Send)]
async fn get_max_lane_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

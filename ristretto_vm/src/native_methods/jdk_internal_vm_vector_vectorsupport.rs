use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.vm.vector.VectorSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/vm/vector/VectorSupport";
    registry.register(
        class_name,
        "getMaxLaneCount",
        "(Ljava/lang/Class;)I",
        get_max_lane_count,
    );
    registry.register(class_name, "registerNatives", "()I", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_max_lane_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

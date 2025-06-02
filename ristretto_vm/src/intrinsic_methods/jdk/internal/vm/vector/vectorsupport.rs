use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/vm/vector/VectorSupport";

/// Register all intrinsic methods for `jdk.internal.vm.vector.VectorSupport`.
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
async fn get_max_lane_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

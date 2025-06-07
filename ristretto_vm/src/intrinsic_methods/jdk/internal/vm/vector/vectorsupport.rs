use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_max_lane_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I")
}

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.registerNatives()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.registerNatives()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I"
    )]
    async fn test_get_max_lane_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_lane_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.vector.VectorSupport.registerNatives()I"
    )]
    async fn test_register_natives() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_natives(thread, Parameters::default()).await;
    }
}

use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_max_lane_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I")
}

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.registerNatives()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.registerNatives()I")
}

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.getCPUFeatures()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn get_cpu_features<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.vector.VectorSupport.getCPUFeatures()Ljava/lang/String;")
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

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.vector.VectorSupport.getCPUFeatures()Ljava/lang/String;"
    )]
    async fn test_get_cpu_features() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cpu_features(thread, Parameters::default()).await;
    }
}

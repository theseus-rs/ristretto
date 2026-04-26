use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_max_lane_count<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.registerNatives()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.vector.VectorSupport.registerNatives()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/vm/vector/VectorSupport.getCPUFeatures()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn get_cpu_features<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.vector.VectorSupport.getCPUFeatures()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_max_lane_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_max_lane_count(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.vm.vector.VectorSupport.getMaxLaneCount(Ljava/lang/Class;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_natives() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_natives(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.vm.vector.VectorSupport.registerNatives()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cpu_features() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cpu_features(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.vm.vector.VectorSupport.getCPUFeatures()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}

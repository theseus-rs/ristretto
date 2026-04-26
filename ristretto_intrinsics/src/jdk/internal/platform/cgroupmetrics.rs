use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("jdk/internal/platform/CgroupMetrics.getTotalMemorySize0()J", Any)]
#[async_method]
pub async fn get_total_memory_size0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/platform/CgroupMetrics.getTotalMemorySize0()J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/internal/platform/CgroupMetrics.getTotalSwapSize0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_total_swap_size0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/platform/CgroupMetrics.getTotalSwapSize0()J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/internal/platform/CgroupMetrics.isContainerized0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_containerized0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/platform/CgroupMetrics.isContainerized0()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("jdk/internal/platform/CgroupMetrics.isUseContainerSupport()Z", Any)]
#[async_method]
pub async fn is_use_container_support<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/platform/CgroupMetrics.isUseContainerSupport()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_total_memory_size0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_memory_size0(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/platform/CgroupMetrics.getTotalMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_total_swap_size0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_swap_size0(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/platform/CgroupMetrics.getTotalSwapSize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_containerized0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_containerized0(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/platform/CgroupMetrics.isContainerized0()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_use_container_support() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_use_container_support(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/platform/CgroupMetrics.isUseContainerSupport()Z",
            result.unwrap_err().to_string()
        );
    }
}

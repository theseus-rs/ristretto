use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
#[async_method]
pub async fn get_memory_managers_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
    Any
)]
#[async_method]
pub async fn get_memory_pools_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_memory_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/MemoryImpl.setVerboseGC(Z)V", Any)]
#[async_method]
pub async fn set_verbose_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.management.MemoryImpl.setVerboseGC(Z)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_managers_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_pools_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_memory_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_usage_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_verbose_gc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

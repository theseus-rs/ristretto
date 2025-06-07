use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_memory_pools_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;")
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_memory_usage_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;")
}

#[intrinsic_method("sun/management/MemoryImpl.setVerboseGC(Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_verbose_gc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.setVerboseGC(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_managers_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_pools_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;"
    )]
    async fn test_get_memory_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_usage_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.management.MemoryImpl.setVerboseGC(Z)V")]
    async fn test_set_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_verbose_gc(thread, Parameters::default()).await;
    }
}

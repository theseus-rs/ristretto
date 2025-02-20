use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/MemoryImpl";

/// Register all native methods for `sun.management.MemoryImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getMemoryManagers0",
        "()[Ljava/lang/management/MemoryManagerMXBean;",
        get_memory_managers_0,
    );
    registry.register(
        CLASS_NAME,
        "getMemoryPools0",
        "()[Ljava/lang/management/MemoryPoolMXBean;",
        get_memory_pools_0,
    );
    registry.register(
        CLASS_NAME,
        "getMemoryUsage0",
        "(Z)Ljava/lang/management/MemoryUsage;",
        get_memory_usage_0,
    );
    registry.register(CLASS_NAME, "setVerboseGC", "(Z)V", set_verbose_gc);
}

#[async_recursion(?Send)]
async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )
}

#[async_recursion(?Send)]
async fn get_memory_pools_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;")
}

#[async_recursion(?Send)]
async fn get_memory_usage_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;")
}

#[async_recursion(?Send)]
async fn set_verbose_gc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.MemoryManagerImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/MemoryManagerImpl";
    registry.register(
        class_name,
        "getMemoryPools0",
        "()[Ljava/lang/management/MemoryPoolMXBean;",
        get_memory_pools_0,
    );
}

#[async_recursion(?Send)]
async fn get_memory_pools_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/MemoryManagerImpl";
        assert!(registry
            .method(
                class_name,
                "getMemoryPools0",
                "()[Ljava/lang/management/MemoryPoolMXBean;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_pools_0(thread, Arguments::default()).await;
    }
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/MemoryManagerImpl";

/// Register all native methods for `sun.management.MemoryManagerImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_pools_0(thread, Arguments::default()).await;
    }
}

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

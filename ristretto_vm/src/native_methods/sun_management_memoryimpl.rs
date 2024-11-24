use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.MemoryImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/MemoryImpl";
    registry.register(
        class_name,
        "getMemoryManagers0",
        "()[Ljava/lang/management/MemoryManagerMXBean;",
        get_memory_managers_0,
    );
    registry.register(
        class_name,
        "getMemoryPools0",
        "()[Ljava/lang/management/MemoryPoolMXBean;",
        get_memory_pools_0,
    );
    registry.register(
        class_name,
        "getMemoryUsage0",
        "(Z)Ljava/lang/management/MemoryUsage;",
        get_memory_usage_0,
    );
    registry.register(class_name, "setVerboseGC", "(Z)V", set_verbose_gc);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_memory_pools_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_memory_usage_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_verbose_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

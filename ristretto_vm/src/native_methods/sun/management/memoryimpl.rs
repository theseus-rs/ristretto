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

#[async_recursion(?Send)]
async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )
}

#[async_recursion(?Send)]
async fn get_memory_pools_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;")
}

#[async_recursion(?Send)]
async fn get_memory_usage_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;")
}

#[async_recursion(?Send)]
async fn set_verbose_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.MemoryImpl.setVerboseGC(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/MemoryImpl";
        assert!(registry
            .method(
                class_name,
                "getMemoryManagers0",
                "()[Ljava/lang/management/MemoryManagerMXBean;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getMemoryPools0",
                "()[Ljava/lang/management/MemoryPoolMXBean;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getMemoryUsage0",
                "(Z)Ljava/lang/management/MemoryUsage;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "setVerboseGC", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_managers_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_pools_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;"
    )]
    async fn test_get_memory_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_usage_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.MemoryImpl.setVerboseGC(Z)V")]
    async fn test_set_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_verbose_gc(thread, Arguments::default()).await;
    }
}

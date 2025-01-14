use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.GarbageCollectorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/GarbageCollectorImpl";
    registry.register(
        class_name,
        "getCollectionCount",
        "()J",
        get_collection_count,
    );
    registry.register(class_name, "getCollectionTime", "()J", get_collection_time);
}

#[async_recursion(?Send)]
async fn get_collection_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionCount()J")
}

#[async_recursion(?Send)]
async fn get_collection_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionTime()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/GarbageCollectorImpl";
        assert!(registry
            .method(class_name, "getCollectionCount", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getCollectionTime", "()J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.GarbageCollectorImpl.getCollectionCount()J")]
    async fn test_get_collection_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_collection_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.GarbageCollectorImpl.getCollectionTime()J")]
    async fn test_get_collection_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_collection_time(thread, Arguments::default()).await;
    }
}

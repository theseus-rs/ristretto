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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_collection_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_collection_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

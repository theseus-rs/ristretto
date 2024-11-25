use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Runtime`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Runtime";
    registry.register(
        class_name,
        "availableProcessors",
        "()I",
        available_processors,
    );
    registry.register(class_name, "freeMemory", "()J", free_memory);
    registry.register(class_name, "gc", "()V", gc);
    registry.register(class_name, "maxMemory", "()J", max_memory);
    registry.register(class_name, "totalMemory", "()J", total_memory);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn available_processors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn max_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn total_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

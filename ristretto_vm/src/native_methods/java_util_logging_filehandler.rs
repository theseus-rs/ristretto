use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.logging.FileHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/logging/FileHandler";
    registry.register(class_name, "isSetUID", "()Z", is_set_uid);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_set_uid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

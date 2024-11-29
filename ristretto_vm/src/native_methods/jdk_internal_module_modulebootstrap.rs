use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.module.ModuleBootstrap`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/module/ModuleBootstrap";
    registry.register(class_name, "boot", "()Ljava/lang/ModuleLayer;", boot);
}

#[async_recursion(?Send)]
async fn boot(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: remove this method once the module system is implemented
    Ok(Some(Value::Object(None)))
}

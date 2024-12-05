use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.String`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/String";
    registry.register(class_name, "intern", "()Ljava/lang/String;", intern);
}

#[async_recursion(?Send)]
async fn intern(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let value = arguments.pop()?;
    // TODO: implement proper string interning
    Ok(Some(value))
}

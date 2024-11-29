use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StringCoding`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StringCoding";
    registry.register(class_name, "err", "(Ljava/lang/String;)V", err);
}

#[async_recursion(?Send)]
async fn err(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn intern(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

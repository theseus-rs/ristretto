use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StringUTF16`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StringUTF16";
    registry.register(class_name, "isBigEndian", "()Z", is_big_endian);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_big_endian(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

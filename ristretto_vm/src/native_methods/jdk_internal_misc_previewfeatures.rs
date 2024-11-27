use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.misc.PreviewFeatures`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/PreviewFeatures";
    registry.register(class_name, "isPreviewEnabled", "()Z", is_preview_enabled);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_preview_enabled(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

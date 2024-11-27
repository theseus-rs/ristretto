use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.applescript.AppleScriptEngineFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/applescript/AppleScriptEngineFactory";
    registry.register(class_name, "initNative", "()V", init_native);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

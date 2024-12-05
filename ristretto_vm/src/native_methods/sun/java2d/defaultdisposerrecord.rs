use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.DefaultDisposerRecord`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/DefaultDisposerRecord";
    registry.register(
        class_name,
        "invokeNativeDispose",
        "(JJ)V",
        invoke_native_dispose,
    );
}

#[async_recursion(?Send)]
async fn invoke_native_dispose(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.DefaultDisposerRecord.invokeNativeDispose(JJ)V");
}

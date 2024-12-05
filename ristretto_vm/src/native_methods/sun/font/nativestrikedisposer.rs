use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NativeStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NativeStrikeDisposer";
    registry.register(
        class_name,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V")
}

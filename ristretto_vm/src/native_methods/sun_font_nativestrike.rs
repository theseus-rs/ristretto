use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NativeStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NativeStrike";
    registry.register(
        class_name,
        "createNullScalerContext",
        "()J",
        create_null_scaler_context,
    );
    registry.register(
        class_name,
        "createScalerContext",
        "([BID)J",
        create_scaler_context,
    );
    registry.register(class_name, "getMaxGlyph", "(J)I", get_max_glyph);
}

#[async_recursion(?Send)]
async fn create_null_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn create_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_max_glyph(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

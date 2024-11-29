use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CStrikeDisposer";
    registry.register(
        class_name,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
    registry.register(
        class_name,
        "removeGlyphInfoFromCache",
        "(J)V",
        remove_glyph_info_from_cache,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn remove_glyph_info_from_cache(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

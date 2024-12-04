use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NullFontScaler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NullFontScaler";
    registry.register(class_name, "getGlyphImage", "(JI)J", get_glyph_image);
    registry.register(
        class_name,
        "getNullScalerContext",
        "()J",
        get_null_scaler_context,
    );
}

#[async_recursion(?Send)]
async fn get_glyph_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_null_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

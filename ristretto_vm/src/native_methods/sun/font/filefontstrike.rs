use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.FileFontStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/FileFontStrike";
    registry.register(
        class_name,
        "_getGlyphImageFromWindows",
        "(Ljava/lang/String;IIIZI)J",
        get_glyph_image_from_windows,
    );
    registry.register(class_name, "initNative", "()Z", init_native);
}

#[async_recursion(?Send)]
async fn get_glyph_image_from_windows(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike.initNative()Z")
}

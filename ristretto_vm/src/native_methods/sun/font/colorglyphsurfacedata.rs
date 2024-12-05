use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.ColorGlyphSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/ColorGlyphSurfaceData";
    registry.register(class_name, "initOps", "()V", init_ops);
    registry.register(class_name, "setCurrentGlyph", "(J)V", set_current_glyph);
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.initOps()V")
}

#[async_recursion(?Send)]
async fn set_current_glyph(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V")
}

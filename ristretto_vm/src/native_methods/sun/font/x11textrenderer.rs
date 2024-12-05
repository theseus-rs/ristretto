use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.X11TextRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/X11TextRenderer";
    registry.register(
        class_name,
        "doDrawGlyphList",
        "(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
        do_draw_glyph_list,
    );
}

#[async_recursion(?Send)]
async fn do_draw_glyph_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
    )
}

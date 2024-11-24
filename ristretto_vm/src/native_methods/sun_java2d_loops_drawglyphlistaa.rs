use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.DrawGlyphListAA`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawGlyphListAA";
    registry.register(
        class_name,
        "DrawGlyphListAA",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
        draw_glyph_list_aa,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn draw_glyph_list_aa(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/DrawGlyphList";

/// Register all intrinsic methods for `sun.java2d.loops.DrawGlyphList`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "DrawGlyphList",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
            draw_glyph_list,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "DrawGlyphList",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V",
            draw_glyph_list,
        );
    }
}

#[async_recursion(?Send)]
async fn draw_glyph_list(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawGlyphList.DrawGlyphList(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawGlyphList.DrawGlyphList(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    )]
    async fn test_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list(thread, Parameters::default()).await;
    }
}

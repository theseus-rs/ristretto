use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/DrawGlyphListAA";

/// Register all native methods for `sun.java2d.loops.DrawGlyphListAA`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "DrawGlyphListAA",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
            draw_glyph_list_aa,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "DrawGlyphListAA",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V",
            draw_glyph_list_aa,
        );
    }
}

#[async_recursion(?Send)]
async fn draw_glyph_list_aa(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    )]
    async fn test_draw_glyph_list_aa() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list_aa(thread, Parameters::default()).await;
    }
}

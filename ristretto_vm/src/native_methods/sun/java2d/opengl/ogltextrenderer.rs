use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/OGLTextRenderer";

/// Register all native methods for `sun.java2d.opengl.OGLTextRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "drawGlyphList",
        "(IZZZIFF[J[F)V",
        draw_glyph_list,
    );
}

#[async_recursion(?Send)]
async fn draw_glyph_list(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLTextRenderer.drawGlyphList(IZZZIFF[J[F)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLTextRenderer.drawGlyphList(IZZZIFF[J[F)V"
    )]
    async fn test_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list(thread, Parameters::default()).await;
    }
}

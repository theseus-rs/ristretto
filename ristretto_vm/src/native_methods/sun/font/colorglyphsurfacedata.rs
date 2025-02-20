use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/ColorGlyphSurfaceData";

/// Register all native methods for `sun.font.ColorGlyphSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initOps", "()V", init_ops);
    registry.register(CLASS_NAME, "setCurrentGlyph", "(J)V", set_current_glyph);
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.initOps()V")
}

#[async_recursion(?Send)]
async fn set_current_glyph(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.ColorGlyphSurfaceData.initOps()V")]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V"
    )]
    async fn test_set_current_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_current_glyph(thread, Parameters::default()).await;
    }
}

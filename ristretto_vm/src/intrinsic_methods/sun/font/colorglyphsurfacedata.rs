use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/ColorGlyphSurfaceData.initOps()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_ops(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.initOps()V")
}

#[intrinsic_method(
    "sun/font/ColorGlyphSurfaceData.setCurrentGlyph(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_current_glyph(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

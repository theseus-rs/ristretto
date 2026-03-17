use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn draw_glyph_list_aa_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/loops/DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn draw_glyph_list_aa_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_glyph_list_aa_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_glyph_list_aa_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_draw_glyph_list_aa_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_glyph_list_aa_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn draw_glyph_list_aa_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    );
}

#[intrinsic_method(
    "sun/java2d/loops/DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn draw_glyph_list_aa_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    )]
    async fn test_draw_glyph_list_aa_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list_aa_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawGlyphListAA.DrawGlyphListAA(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V"
    )]
    async fn test_draw_glyph_list_aa_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list_aa_1(thread, Parameters::default()).await;
    }
}

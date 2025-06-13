use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/NativeFont.countGlyphs([BI)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn count_glyphs(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.countGlyphs([BI)I")
}

#[intrinsic_method("sun/font/NativeFont.fontExists([B)Z", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn font_exists(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.fontExists([B)Z")
}

#[intrinsic_method(
    "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_font_metrics(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[intrinsic_method("sun/font/NativeFont.getGlyphAdvance(JI)F", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_advance(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphAdvance(JI)F")
}

#[intrinsic_method("sun/font/NativeFont.getGlyphImage(JI)J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImage(JI)J")
}

#[intrinsic_method(
    "sun/font/NativeFont.getGlyphImageNoDefault(JI)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_image_no_default(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImageNoDefault(JI)J")
}

#[intrinsic_method("sun/font/NativeFont.haveBitmapFonts([B)Z", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn have_bitmap_fonts(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.haveBitmapFonts([B)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeFont.countGlyphs([BI)I")]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = count_glyphs(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeFont.fontExists([B)Z")]
    async fn test_font_exists() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = font_exists(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;"
    )]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeFont.getGlyphAdvance(JI)F")]
    async fn test_get_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_advance(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeFont.getGlyphImage(JI)J")]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeFont.getGlyphImageNoDefault(JI)J"
    )]
    async fn test_get_glyph_image_no_default() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_no_default(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeFont.haveBitmapFonts([B)Z")]
    async fn test_have_bitmap_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = have_bitmap_fonts(thread, Parameters::default()).await;
    }
}

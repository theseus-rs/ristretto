use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/NativeFont";

/// Register all native methods for `sun.font.NativeFont`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "countGlyphs", "([BI)I", count_glyphs);
    registry.register(CLASS_NAME, "fontExists", "([B)Z", font_exists);
    registry.register(
        CLASS_NAME,
        "getFontMetrics",
        "(J)Lsun/font/StrikeMetrics;",
        get_font_metrics,
    );
    registry.register(CLASS_NAME, "getGlyphAdvance", "(JI)F", get_glyph_advance);
    registry.register(CLASS_NAME, "getGlyphImage", "(JI)J", get_glyph_image);
    registry.register(
        CLASS_NAME,
        "getGlyphImageNoDefault",
        "(JI)J",
        get_glyph_image_no_default,
    );
    registry.register(CLASS_NAME, "haveBitmapFonts", "([B)Z", have_bitmap_fonts);
}

#[async_recursion(?Send)]
async fn count_glyphs(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.countGlyphs([BI)I")
}

#[async_recursion(?Send)]
async fn font_exists(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.fontExists([B)Z")
}

#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[async_recursion(?Send)]
async fn get_glyph_advance(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphAdvance(JI)F")
}

#[async_recursion(?Send)]
async fn get_glyph_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImage(JI)J")
}

#[async_recursion(?Send)]
async fn get_glyph_image_no_default(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImageNoDefault(JI)J")
}

#[async_recursion(?Send)]
async fn have_bitmap_fonts(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

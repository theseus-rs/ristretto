use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NativeFont`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NativeFont";
    registry.register(class_name, "countGlyphs", "([BI)I", count_glyphs);
    registry.register(class_name, "fontExists", "([B)Z", font_exists);
    registry.register(
        class_name,
        "getFontMetrics",
        "(J)Lsun/font/StrikeMetrics;",
        get_font_metrics,
    );
    registry.register(class_name, "getGlyphAdvance", "(JI)F", get_glyph_advance);
    registry.register(class_name, "getGlyphImage", "(JI)J", get_glyph_image);
    registry.register(
        class_name,
        "getGlyphImageNoDefault",
        "(JI)J",
        get_glyph_image_no_default,
    );
    registry.register(class_name, "haveBitmapFonts", "([B)Z", have_bitmap_fonts);
}

#[async_recursion(?Send)]
async fn count_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.countGlyphs([BI)I")
}

#[async_recursion(?Send)]
async fn font_exists(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.fontExists([B)Z")
}

#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[async_recursion(?Send)]
async fn get_glyph_advance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphAdvance(JI)F")
}

#[async_recursion(?Send)]
async fn get_glyph_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImage(JI)J")
}

#[async_recursion(?Send)]
async fn get_glyph_image_no_default(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.getGlyphImageNoDefault(JI)J")
}

#[async_recursion(?Send)]
async fn have_bitmap_fonts(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeFont.haveBitmapFonts([B)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/NativeFont";
        assert!(registry
            .method(class_name, "countGlyphs", "([BI)I")
            .is_some());
        assert!(registry.method(class_name, "fontExists", "([B)Z").is_some());
        assert!(registry
            .method(class_name, "getFontMetrics", "(J)Lsun/font/StrikeMetrics;")
            .is_some());
        assert!(registry
            .method(class_name, "getGlyphAdvance", "(JI)F")
            .is_some());
        assert!(registry
            .method(class_name, "getGlyphImage", "(JI)J")
            .is_some());
        assert!(registry
            .method(class_name, "getGlyphImageNoDefault", "(JI)J")
            .is_some());
        assert!(registry
            .method(class_name, "haveBitmapFonts", "([B)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.countGlyphs([BI)I")]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = count_glyphs(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.fontExists([B)Z")]
    async fn test_font_exists() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = font_exists(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;")]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.getGlyphAdvance(JI)F")]
    async fn test_get_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_advance(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.getGlyphImage(JI)J")]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.getGlyphImageNoDefault(JI)J")]
    async fn test_get_glyph_image_no_default() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_no_default(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeFont.haveBitmapFonts([B)Z")]
    async fn test_have_bitmap_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = have_bitmap_fonts(thread, Arguments::default()).await;
    }
}

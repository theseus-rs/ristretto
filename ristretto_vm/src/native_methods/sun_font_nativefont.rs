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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn count_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn font_exists(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_advance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_image_no_default(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn have_bitmap_fonts(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

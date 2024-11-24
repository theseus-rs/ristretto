use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CStrike";
    registry.register(
        class_name,
        "createNativeStrikePtr",
        "(J[D[DII)J",
        create_native_strike_ptr,
    );
    registry.register(
        class_name,
        "disposeNativeStrikePtr",
        "(J)V",
        dispose_native_strike_ptr,
    );
    registry.register(
        class_name,
        "getFontMetrics",
        "(J)Lsun/font/StrikeMetrics;",
        get_font_metrics,
    );
    registry.register(
        class_name,
        "getGlyphImagePtrsNative",
        "(J[J[II)V",
        get_glyph_image_ptrs_native,
    );
    registry.register(
        class_name,
        "getNativeGlyphAdvance",
        "(JI)F",
        get_native_glyph_advance,
    );
    registry.register(
        class_name,
        "getNativeGlyphImageBounds",
        "(JILjava/awt/geom/Rectangle2D$Float;DD)V",
        get_native_glyph_image_bounds,
    );
    registry.register(
        class_name,
        "getNativeGlyphOutline",
        "(JIDD)Ljava/awt/geom/GeneralPath;",
        get_native_glyph_outline,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_native_strike_ptr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn dispose_native_strike_ptr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_image_ptrs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_glyph_advance(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_glyph_image_bounds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_glyph_outline(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

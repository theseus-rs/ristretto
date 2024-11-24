use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.FreetypeFontScaler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/FreetypeFontScaler";
    registry.register(
        class_name,
        "createScalerContextNative",
        "(J[DIIFF)J",
        create_scaler_context_native,
    );
    registry.register(
        class_name,
        "disposeNativeScaler",
        "(Lsun/font/Font2D;J)V",
        dispose_native_scaler,
    );
    registry.register(
        class_name,
        "getFontMetricsNative",
        "(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;",
        get_font_metrics_native,
    );
    registry.register(
        class_name,
        "getGlyphAdvanceNative",
        "(Lsun/font/Font2D;JJI)F",
        get_glyph_advance_native,
    );
    registry.register(
        class_name,
        "getGlyphCodeNative",
        "(Lsun/font/Font2D;JC)I",
        get_glyph_code_native,
    );
    registry.register(
        class_name,
        "getGlyphImageNative",
        "(Lsun/font/Font2D;JJI)J",
        get_glyph_image_native,
    );
    registry.register(
        class_name,
        "getGlyphMetricsNative",
        "(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V",
        get_glyph_metrics_native,
    );
    registry.register(
        class_name,
        "getGlyphOutlineBoundsNative",
        "(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;",
        get_glyph_outline_bounds_native,
    );
    registry.register(
        class_name,
        "getGlyphOutlineNative",
        "(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;",
        get_glyph_outline_native,
    );
    registry.register(
        class_name,
        "getGlyphPointNative",
        "(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;",
        get_glyph_point_native,
    );
    registry.register(
        class_name,
        "getGlyphVectorOutlineNative",
        "(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;",
        get_glyph_vector_outline_native,
    );
    registry.register(
        class_name,
        "getLayoutTableCacheNative",
        "(J)J",
        get_layout_table_cache_native,
    );
    registry.register(
        class_name,
        "getMissingGlyphCodeNative",
        "(J)I",
        get_missing_glyph_code_native,
    );
    registry.register(
        class_name,
        "getNumGlyphsNative",
        "(J)I",
        get_num_glyphs_native,
    );
    registry.register(
        class_name,
        "getUnitsPerEMNative",
        "(J)J",
        get_units_per_em_native,
    );
    registry.register(class_name, "initIDs", "(Ljava/lang/Class;)V", init_ids);
    registry.register(
        class_name,
        "initNativeScaler",
        "(Lsun/font/Font2D;IIZI)J",
        init_native_scaler,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_scaler_context_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn dispose_native_scaler(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_font_metrics_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_advance_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_code_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_image_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_metrics_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_outline_bounds_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_outline_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_point_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_glyph_vector_outline_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_missing_glyph_code_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_num_glyphs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_units_per_em_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_native_scaler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

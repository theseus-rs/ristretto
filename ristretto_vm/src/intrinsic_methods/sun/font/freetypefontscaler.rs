use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/FreetypeFontScaler";

/// Register all intrinsic methods for `sun.font.FreetypeFontScaler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "getLayoutTableCacheNative",
            "(J)J",
            get_layout_table_cache_native,
        );
    }

    registry.register(
        CLASS_NAME,
        "createScalerContextNative",
        "(J[DIIFF)J",
        create_scaler_context_native,
    );
    registry.register(
        CLASS_NAME,
        "disposeNativeScaler",
        "(Lsun/font/Font2D;J)V",
        dispose_native_scaler,
    );
    registry.register(
        CLASS_NAME,
        "getFontMetricsNative",
        "(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;",
        get_font_metrics_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphAdvanceNative",
        "(Lsun/font/Font2D;JJI)F",
        get_glyph_advance_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphCodeNative",
        "(Lsun/font/Font2D;JC)I",
        get_glyph_code_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphImageNative",
        "(Lsun/font/Font2D;JJI)J",
        get_glyph_image_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphMetricsNative",
        "(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V",
        get_glyph_metrics_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphOutlineBoundsNative",
        "(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;",
        get_glyph_outline_bounds_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphOutlineNative",
        "(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;",
        get_glyph_outline_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphPointNative",
        "(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;",
        get_glyph_point_native,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphVectorOutlineNative",
        "(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;",
        get_glyph_vector_outline_native,
    );
    registry.register(
        CLASS_NAME,
        "getMissingGlyphCodeNative",
        "(J)I",
        get_missing_glyph_code_native,
    );
    registry.register(
        CLASS_NAME,
        "getNumGlyphsNative",
        "(J)I",
        get_num_glyphs_native,
    );
    registry.register(
        CLASS_NAME,
        "getUnitsPerEMNative",
        "(J)J",
        get_units_per_em_native,
    );
    registry.register(CLASS_NAME, "initIDs", "(Ljava/lang/Class;)V", init_ids);
    registry.register(
        CLASS_NAME,
        "initNativeScaler",
        "(Lsun/font/Font2D;IIZI)J",
        init_native_scaler,
    );
}

#[async_recursion(?Send)]
async fn create_scaler_context_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.createScalerContextNative(J[DIIFF)J")
}

#[async_recursion(?Send)]
async fn dispose_native_scaler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V")
}

#[async_recursion(?Send)]
async fn get_font_metrics_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;"
    )
}

#[async_recursion(?Send)]
async fn get_glyph_advance_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F")
}

#[async_recursion(?Send)]
async fn get_glyph_code_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I")
}

#[async_recursion(?Send)]
async fn get_glyph_image_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J")
}

#[async_recursion(?Send)]
async fn get_glyph_metrics_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V"
    )
}

#[async_recursion(?Send)]
async fn get_glyph_outline_bounds_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;"
    )
}

#[async_recursion(?Send)]
async fn get_glyph_outline_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;"
    )
}

#[async_recursion(?Send)]
async fn get_glyph_point_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;"
    )
}

#[async_recursion(?Send)]
async fn get_glyph_vector_outline_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;"
    )
}

#[async_recursion(?Send)]
async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getLayoutTableCacheNative(J)J")
}

#[async_recursion(?Send)]
async fn get_missing_glyph_code_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getMissingGlyphCodeNative(J)I")
}

#[async_recursion(?Send)]
async fn get_num_glyphs_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getNumGlyphsNative(J)I")
}

#[async_recursion(?Send)]
async fn get_units_per_em_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getUnitsPerEMNative(J)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_native_scaler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.initNativeScaler(Lsun/font/Font2D;IIZI)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.createScalerContextNative(J[DIIFF)J"
    )]
    async fn test_create_scaler_context_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_scaler_context_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V"
    )]
    async fn test_dispose_native_scaler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_scaler(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;"
    )]
    async fn test_get_font_metrics_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F"
    )]
    async fn test_get_glyph_advance_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_advance_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I"
    )]
    async fn test_get_glyph_code_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_code_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J"
    )]
    async fn test_get_glyph_image_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V"
    )]
    async fn test_get_glyph_metrics_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_metrics_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;"
    )]
    async fn test_get_glyph_outline_bounds_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_outline_bounds_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;"
    )]
    async fn test_get_glyph_outline_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_outline_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;"
    )]
    async fn test_get_glyph_point_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_point_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;"
    )]
    async fn test_get_glyph_vector_outline_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_vector_outline_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getLayoutTableCacheNative(J)J"
    )]
    async fn test_get_layout_table_cache_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_layout_table_cache_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getMissingGlyphCodeNative(J)I"
    )]
    async fn test_get_missing_glyph_code_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_missing_glyph_code_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getNumGlyphsNative(J)I"
    )]
    async fn test_get_num_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_glyphs_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.getUnitsPerEMNative(J)J"
    )]
    async fn test_get_units_per_em_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_units_per_em_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FreetypeFontScaler.initNativeScaler(Lsun/font/Font2D;IIZI)J"
    )]
    async fn test_init_native_scaler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_native_scaler(thread, Parameters::default()).await;
    }
}

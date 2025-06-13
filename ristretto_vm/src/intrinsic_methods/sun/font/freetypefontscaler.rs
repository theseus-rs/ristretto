use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/FreetypeFontScaler.createScalerContextNative(J[DIIFF)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn create_scaler_context_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.createScalerContextNative(J[DIIFF)J")
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn dispose_native_scaler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V")
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_font_metrics_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_advance_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F")
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_code_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I")
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_image_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J")
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_metrics_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_outline_bounds_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_outline_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_point_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_vector_outline_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;"
    )
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getLayoutTableCacheNative(J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_layout_table_cache_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getLayoutTableCacheNative(J)J")
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getMissingGlyphCodeNative(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_missing_glyph_code_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getMissingGlyphCodeNative(J)I")
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getNumGlyphsNative(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_num_glyphs_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getNumGlyphsNative(J)I")
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getUnitsPerEMNative(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_units_per_em_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FreetypeFontScaler.getUnitsPerEMNative(J)J")
}

#[intrinsic_method("sun/font/FreetypeFontScaler.initIDs(Ljava/lang/Class;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.initNativeScaler(Lsun/font/Font2D;IIZI)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init_native_scaler(
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

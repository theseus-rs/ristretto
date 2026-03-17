use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/FreetypeFontScaler.createScalerContextNative(J[DIIFF)J", Any)]
#[async_method]
pub async fn create_scaler_context_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.createScalerContextNative(J[DIIFF)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V",
    Any
)]
#[async_method]
pub async fn dispose_native_scaler<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.disposeNativeScaler(Lsun/font/Font2D;J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;",
    Any
)]
#[async_method]
pub async fn get_font_metrics_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getFontMetricsNative(Lsun/font/Font2D;JJ)Lsun/font/StrikeMetrics;".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F",
    Any
)]
#[async_method]
pub async fn get_glyph_advance_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getGlyphAdvanceNative(Lsun/font/Font2D;JJI)F".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I",
    Any
)]
#[async_method]
pub async fn get_glyph_code_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getGlyphCodeNative(Lsun/font/Font2D;JC)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J",
    Any
)]
#[async_method]
pub async fn get_glyph_image_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getGlyphImageNative(Lsun/font/Font2D;JJI)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V",
    Any
)]
#[async_method]
pub async fn get_glyph_metrics_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getGlyphMetricsNative(Lsun/font/Font2D;JJILjava/awt/geom/Point2D$Float;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;",
    Any
)]
#[async_method]
pub async fn get_glyph_outline_bounds_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getGlyphOutlineBoundsNative(Lsun/font/Font2D;JJI)Ljava/awt/geom/Rectangle2D$Float;".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_method]
pub async fn get_glyph_outline_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getGlyphOutlineNative(Lsun/font/Font2D;JJIFF)Ljava/awt/geom/GeneralPath;".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;",
    Any
)]
#[async_method]
pub async fn get_glyph_point_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getGlyphPointNative(Lsun/font/Font2D;JJII)Ljava/awt/geom/Point2D$Float;".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_method]
pub async fn get_glyph_vector_outline_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FreetypeFontScaler.getGlyphVectorOutlineNative(Lsun/font/Font2D;JJ[IIFF)Ljava/awt/geom/GeneralPath;".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.getLayoutTableCacheNative(J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_layout_table_cache_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getLayoutTableCacheNative(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getMissingGlyphCodeNative(J)I", Any)]
#[async_method]
pub async fn get_missing_glyph_code_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getMissingGlyphCodeNative(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getNumGlyphsNative(J)I", Any)]
#[async_method]
pub async fn get_num_glyphs_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getNumGlyphsNative(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/FreetypeFontScaler.getUnitsPerEMNative(J)J", Any)]
#[async_method]
pub async fn get_units_per_em_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.getUnitsPerEMNative(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/FreetypeFontScaler.initIDs(Ljava/lang/Class;)V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/font/FreetypeFontScaler.initNativeScaler(Lsun/font/Font2D;IIZI)J",
    Any
)]
#[async_method]
pub async fn init_native_scaler<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FreetypeFontScaler.initNativeScaler(Lsun/font/Font2D;IIZI)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_scaler_context_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_scaler_context_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispose_native_scaler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_native_scaler(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_font_metrics_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_metrics_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_advance_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_advance_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_code_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_code_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_image_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_metrics_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_metrics_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_outline_bounds_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_outline_bounds_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_outline_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_outline_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_point_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_point_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_vector_outline_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_vector_outline_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_layout_table_cache_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_layout_table_cache_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_missing_glyph_code_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_missing_glyph_code_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_num_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_glyphs_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_units_per_em_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_units_per_em_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_native_scaler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_scaler(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

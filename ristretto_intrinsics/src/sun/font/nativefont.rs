use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/NativeFont.countGlyphs([BI)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn count_glyphs<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pt_size = parameters.pop_int()?;
    let _platform_name_bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeFont.countGlyphs([BI)I".to_string()).into())
}

#[intrinsic_method("sun/font/NativeFont.fontExists([B)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn font_exists<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xlfd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeFont.fontExists([B)Z".to_string()).into())
}

#[intrinsic_method(
    "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_metrics<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeFont.getGlyphAdvance(JI)F", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_glyph_advance<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NativeFont.getGlyphAdvance(JI)F".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/NativeFont.getGlyphImage(JI)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_glyph_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_scaler_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NativeFont.getGlyphImage(JI)J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/font/NativeFont.getGlyphImageNoDefault(JI)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_glyph_image_no_default<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeFont.getGlyphImageNoDefault(JI)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeFont.haveBitmapFonts([B)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn have_bitmap_fonts<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xlfd = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NativeFont.haveBitmapFonts([B)Z".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/font/NativeFont.countGlyphs([BI)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn count_glyphs_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pt_size = parameters.pop_int()?;
    let _platform_name_bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/font/NativeFont.countGlyphs([BI)I".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/font/NativeFont.fontExists([B)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn font_exists_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xlfd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/font/NativeFont.fontExists([B)Z".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_font_metrics_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeFont.getGlyphAdvance(JI)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_glyph_advance_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/font/NativeFont.getGlyphAdvance(JI)F".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/font/NativeFont.getGlyphImage(JI)J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_glyph_image_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_scaler_context = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/font/NativeFont.getGlyphImage(JI)J".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeFont.getGlyphImageNoDefault(JI)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_glyph_image_no_default_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/NativeFont.getGlyphImageNoDefault(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeFont.haveBitmapFonts([B)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn have_bitmap_fonts_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xlfd = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/font/NativeFont.haveBitmapFonts([B)Z".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = count_glyphs(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.font.NativeFont.countGlyphs([BI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_font_exists() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = font_exists(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.font.NativeFont.fontExists([B)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_font_metrics(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_glyph_advance() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_glyph_advance(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.font.NativeFont.getGlyphAdvance(JI)F",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_glyph_image(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.font.NativeFont.getGlyphImage(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_glyph_image_no_default() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_glyph_image_no_default(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.font.NativeFont.getGlyphImageNoDefault(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_have_bitmap_fonts() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = have_bitmap_fonts(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.font.NativeFont.haveBitmapFonts([B)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_count_glyphs_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = count_glyphs_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/font/NativeFont.countGlyphs([BI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_font_exists_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            font_exists_linux_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/font/NativeFont.fontExists([B)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_font_metrics_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_font_metrics_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_glyph_advance_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_advance_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/font/NativeFont.getGlyphAdvance(JI)F",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_glyph_image_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/font/NativeFont.getGlyphImage(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_glyph_image_no_default_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_no_default_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/font/NativeFont.getGlyphImageNoDefault(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_have_bitmap_fonts_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            have_bitmap_fonts_linux_ge_v11(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun/font/NativeFont.haveBitmapFonts([B)Z",
            result.unwrap_err().to_string()
        );
    }
}

use ristretto_classfile::JAVA_8;
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeFont.countGlyphs([BI)I".to_string()).into())
}

#[intrinsic_method("sun/font/NativeFont.fontExists([B)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn font_exists<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeFont.fontExists([B)Z".to_string()).into())
}

#[intrinsic_method(
    "sun/font/NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_metrics<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeFont.getFontMetrics(J)Lsun/font/StrikeMetrics;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeFont.getGlyphAdvance(JI)F", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_glyph_advance<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NativeFont.getGlyphAdvance(JI)F".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/NativeFont.getGlyphImage(JI)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_glyph_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeFont.getGlyphImageNoDefault(JI)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeFont.haveBitmapFonts([B)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn have_bitmap_fonts<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NativeFont.haveBitmapFonts([B)Z".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = count_glyphs(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_font_exists() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = font_exists(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_metrics(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_advance(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_image_no_default() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_no_default(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_have_bitmap_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = have_bitmap_fonts(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

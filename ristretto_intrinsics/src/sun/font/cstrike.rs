use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CStrike.createNativeStrikePtr(J[D[DII)J", Any)]
#[async_method]
pub async fn create_native_strike_ptr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.createNativeStrikePtr(J[D[DII)J")
}

#[intrinsic_method("sun/font/CStrike.disposeNativeStrikePtr(J)V", Any)]
#[async_method]
pub async fn dispose_native_strike_ptr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.disposeNativeStrikePtr(J)V")
}

#[intrinsic_method("sun/font/CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;", Any)]
#[async_method]
pub async fn get_font_metrics<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[intrinsic_method("sun/font/CStrike.getGlyphImagePtrsNative(J[J[II)V", Any)]
#[async_method]
pub async fn get_glyph_image_ptrs_native<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V")
}

#[intrinsic_method("sun/font/CStrike.getNativeGlyphAdvance(JI)F", Any)]
#[async_method]
pub async fn get_native_glyph_advance<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphAdvance(JI)F")
}

#[intrinsic_method(
    "sun/font/CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V",
    Any
)]
#[async_method]
pub async fn get_native_glyph_image_bounds<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V")
}

#[intrinsic_method(
    "sun/font/CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_method]
pub async fn get_native_glyph_outline<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.createNativeStrikePtr(J[D[DII)J"
    )]
    async fn test_create_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_strike_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CStrike.disposeNativeStrikePtr(J)V")]
    async fn test_dispose_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_strike_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;"
    )]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V"
    )]
    async fn test_get_glyph_image_ptrs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_ptrs_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CStrike.getNativeGlyphAdvance(JI)F")]
    async fn test_get_native_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_advance(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V"
    )]
    async fn test_get_native_glyph_image_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_image_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;"
    )]
    async fn test_get_native_glyph_outline() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_outline(thread, Parameters::default()).await;
    }
}

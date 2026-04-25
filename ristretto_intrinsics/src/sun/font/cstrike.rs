use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CStrike.createNativeStrikePtr(J[D[DII)J", Any)]
#[async_method]
pub async fn create_native_strike_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fm_hint = parameters.pop_int()?;
    let _aa_hint = parameters.pop_int()?;
    let _inv_dev_tx_matrix = parameters.pop_reference()?;
    let _glyph_tx = parameters.pop_reference()?;
    let _native_font_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrike.createNativeStrikePtr(J[D[DII)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CStrike.disposeNativeStrikePtr(J)V", Any)]
#[async_method]
pub async fn dispose_native_strike_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_strike_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CStrike.disposeNativeStrikePtr(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;", Any)]
#[async_method]
pub async fn get_font_metrics<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_strike_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CStrike.getGlyphImagePtrsNative(J[J[II)V", Any)]
#[async_method]
pub async fn get_glyph_image_ptrs_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _uni_codes = parameters.pop_reference()?;
    let _glyph_infos = parameters.pop_reference()?;
    let _native_strike_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CStrike.getNativeGlyphAdvance(JI)F", Any)]
#[async_method]
pub async fn get_native_glyph_advance<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_code = parameters.pop_int()?;
    let _native_strike_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CStrike.getNativeGlyphAdvance(JI)F".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/font/CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V",
    Any
)]
#[async_method]
pub async fn get_native_glyph_image_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg4 = parameters.pop_double()?;
    let _arg3 = parameters.pop_double()?;
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;",
    Any
)]
#[async_method]
pub async fn get_native_glyph_outline<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _glyph_code = parameters.pop_int()?;
    let _native_strike_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_strike_ptr(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.CStrike.createNativeStrikePtr(J[D[DII)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_native_strike_ptr(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.CStrike.disposeNativeStrikePtr(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_metrics(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_glyph_image_ptrs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_ptrs_native(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_native_glyph_advance(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun.font.CStrike.getNativeGlyphAdvance(JI)F",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_glyph_image_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_glyph_image_bounds(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_glyph_outline() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_glyph_outline(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;",
            result.unwrap_err().to_string()
        );
    }
}

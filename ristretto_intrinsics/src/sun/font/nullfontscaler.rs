use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/NullFontScaler.getGlyphImage(JI)J", Any)]
#[async_method]
pub async fn get_glyph_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.NullFontScaler.getGlyphImage(JI)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/NullFontScaler.getNullScalerContext()J", Any)]
#[async_method]
pub async fn get_null_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NullFontScaler.getNullScalerContext()J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_null_scaler_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

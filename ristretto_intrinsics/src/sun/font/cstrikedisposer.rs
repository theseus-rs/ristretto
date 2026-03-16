use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CStrikeDisposer.freeNativeScalerContext(J)V", Any)]
#[async_method]
pub async fn free_native_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrikeDisposer.freeNativeScalerContext(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CStrikeDisposer.removeGlyphInfoFromCache(J)V", Any)]
#[async_method]
pub async fn remove_glyph_info_from_cache<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CStrikeDisposer.removeGlyphInfoFromCache(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_native_scaler_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_glyph_info_from_cache() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_glyph_info_from_cache(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

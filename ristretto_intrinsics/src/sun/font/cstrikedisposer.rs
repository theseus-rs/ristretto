use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CStrikeDisposer.freeNativeScalerContext(J)V", Any)]
#[async_method]
pub async fn free_native_scaler_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrikeDisposer.freeNativeScalerContext(J)V")
}

#[intrinsic_method("sun/font/CStrikeDisposer.removeGlyphInfoFromCache(J)V", Any)]
#[async_method]
pub async fn remove_glyph_info_from_cache<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrikeDisposer.removeGlyphInfoFromCache(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrikeDisposer.freeNativeScalerContext(J)V"
    )]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_native_scaler_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrikeDisposer.removeGlyphInfoFromCache(J)V"
    )]
    async fn test_remove_glyph_info_from_cache() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_glyph_info_from_cache(thread, Parameters::default()).await;
    }
}

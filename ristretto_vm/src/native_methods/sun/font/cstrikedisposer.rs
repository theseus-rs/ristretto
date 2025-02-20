use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/CStrikeDisposer";

/// Register all native methods for `sun.font.CStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
    registry.register(
        CLASS_NAME,
        "removeGlyphInfoFromCache",
        "(J)V",
        remove_glyph_info_from_cache,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrikeDisposer.freeNativeScalerContext(J)V")
}

#[async_recursion(?Send)]
async fn remove_glyph_info_from_cache(
    _thread: Arc<Thread>,
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

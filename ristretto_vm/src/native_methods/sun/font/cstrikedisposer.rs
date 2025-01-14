use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CStrikeDisposer";
    registry.register(
        class_name,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
    registry.register(
        class_name,
        "removeGlyphInfoFromCache",
        "(J)V",
        remove_glyph_info_from_cache,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrikeDisposer.freeNativeScalerContext(J)V")
}

#[async_recursion(?Send)]
async fn remove_glyph_info_from_cache(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrikeDisposer.removeGlyphInfoFromCache(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/CStrikeDisposer";
        assert!(registry
            .method(class_name, "freeNativeScalerContext", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "removeGlyphInfoFromCache", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrikeDisposer.freeNativeScalerContext(J)V")]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_native_scaler_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrikeDisposer.removeGlyphInfoFromCache(J)V")]
    async fn test_remove_glyph_info_from_cache() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_glyph_info_from_cache(thread, Arguments::default()).await;
    }
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/NullFontScaler";

/// Register all native methods for `sun.font.NullFontScaler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getGlyphImage", "(JI)J", get_glyph_image);
    registry.register(
        CLASS_NAME,
        "getNullScalerContext",
        "()J",
        get_null_scaler_context,
    );
}

#[async_recursion(?Send)]
async fn get_glyph_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NullFontScaler.getGlyphImage(JI)J")
}

#[async_recursion(?Send)]
async fn get_null_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NullFontScaler.getNullScalerContext()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NullFontScaler.getGlyphImage(JI)J")]
    async fn test_get_glyph_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NullFontScaler.getNullScalerContext()J"
    )]
    async fn test_get_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_null_scaler_context(thread, Arguments::default()).await;
    }
}

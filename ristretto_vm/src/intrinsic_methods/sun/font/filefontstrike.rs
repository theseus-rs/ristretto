use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/FileFontStrike";

/// Register all intrinsic methods for `sun.font.FileFontStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "_getGlyphImageFromWindows",
        "(Ljava/lang/String;IIIZI)J",
        get_glyph_image_from_windows,
    );
    registry.register(CLASS_NAME, "initNative", "()Z", init_native);
}

#[async_recursion(?Send)]
async fn get_glyph_image_from_windows(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike.initNative()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J"
    )]
    async fn test_get_glyph_image_from_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_from_windows(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.FileFontStrike.initNative()Z")]
    async fn test_init_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_native(thread, Parameters::default()).await;
    }
}

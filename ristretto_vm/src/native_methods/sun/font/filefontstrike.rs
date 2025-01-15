use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/FileFontStrike";

/// Register all native methods for `sun.font.FileFontStrike`.
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
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
        let _ = get_glyph_image_from_windows(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.FileFontStrike.initNative()Z")]
    async fn test_init_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_native(thread, Arguments::default()).await;
    }
}

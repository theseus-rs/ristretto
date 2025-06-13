use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_image_from_windows(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J")
}

#[intrinsic_method("sun/font/FileFontStrike.initNative()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/NullFontScaler.getGlyphImage(JI)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NullFontScaler.getGlyphImage(JI)J")
}

#[intrinsic_method("sun/font/NullFontScaler.getNullScalerContext()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_null_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
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
        let _ = get_glyph_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NullFontScaler.getNullScalerContext()J"
    )]
    async fn test_get_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_null_scaler_context(thread, Parameters::default()).await;
    }
}

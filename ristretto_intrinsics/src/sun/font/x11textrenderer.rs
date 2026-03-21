use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn do_draw_glyph_list<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_draw_glyph_list() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = do_draw_glyph_list(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

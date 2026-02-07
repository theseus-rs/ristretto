use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn do_draw_glyph_list<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
    )]
    async fn test_do_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_glyph_list(thread, Parameters::default()).await;
    }
}

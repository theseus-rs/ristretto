use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLTextRenderer.drawGlyphList(IZZZIFF[J[F)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn draw_glyph_list<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLTextRenderer.drawGlyphList(IZZZIFF[J[F)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLTextRenderer.drawGlyphList(IZZZIFF[J[F)V"
    )]
    async fn test_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list(thread, Parameters::default()).await;
    }
}

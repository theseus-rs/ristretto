use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLRenderer.drawPoly([I[IIZII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn draw_poly<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V"
    )]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_poly(thread, Parameters::default()).await;
    }
}

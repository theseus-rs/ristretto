use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLRenderer.drawPoly([I[IIZII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn draw_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_poly(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

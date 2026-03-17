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
    "sun/awt/CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_bounds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

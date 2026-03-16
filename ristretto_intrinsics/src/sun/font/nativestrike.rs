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
    "sun/font/NativeStrike.createNullScalerContext()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_null_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeStrike.createNullScalerContext()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/NativeStrike.createScalerContext([BID)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeStrike.createScalerContext([BID)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeStrike.getMaxGlyph(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_max_glyph<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeStrike.getMaxGlyph(J)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_null_scaler_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_scaler_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_max_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_max_glyph(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

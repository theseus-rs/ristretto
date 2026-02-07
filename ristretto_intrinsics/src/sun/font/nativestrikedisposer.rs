use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/NativeStrikeDisposer.freeNativeScalerContext(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_native_scaler_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V"
    )]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_native_scaler_context(thread, Parameters::default()).await;
    }
}

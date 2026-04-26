use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/NativeStrikeDisposer.freeNativeScalerContext(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_native_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeStrikeDisposer.freeNativeScalerContext(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn free_native_scaler_context_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/NativeStrikeDisposer.freeNativeScalerContext(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            free_native_scaler_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_free_native_scaler_context_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            free_native_scaler_context_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/font/NativeStrikeDisposer.freeNativeScalerContext(J)V",
            result.unwrap_err().to_string()
        );
    }
}

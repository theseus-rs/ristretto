use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/ForeignLinkerSupport.isSupported0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.ForeignLinkerSupport.isSupported0()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_supported_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

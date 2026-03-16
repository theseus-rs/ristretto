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
    "sun/awt/PlatformGraphicsInfo.isInAquaSession()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_in_aqua_session<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.PlatformGraphicsInfo.isInAquaSession()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_in_aqua_session(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

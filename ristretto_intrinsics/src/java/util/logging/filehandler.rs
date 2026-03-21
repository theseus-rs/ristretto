use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/util/logging/FileHandler.isSetUID()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn is_set_uid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.util.logging.FileHandler.isSetUID()Z".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_set_uid() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_set_uid(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

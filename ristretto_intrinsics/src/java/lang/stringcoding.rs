use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/StringCoding.err(Ljava/lang/String;)V", Equal(JAVA_11))]
#[async_method]
pub async fn err<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.StringCoding.err(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_err() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = err(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

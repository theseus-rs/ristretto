use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/nio/MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn force_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.nio.MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("java/nio/MappedByteBuffer.isLoaded0(JJI)Z", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_loaded_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.nio.MappedByteBuffer.isLoaded0(JJI)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("java/nio/MappedByteBuffer.load0(JJ)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.nio.MappedByteBuffer.load0(JJ)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = force_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_loaded_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

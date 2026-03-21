use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn init_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[J)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn init_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/FileKey.initIDs()V", LessThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_0() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.PollArrayWrapper.interrupt(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.poll0(JIJ)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn poll_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.PollArrayWrapper.poll0(JIJ)I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = interrupt(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = poll_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

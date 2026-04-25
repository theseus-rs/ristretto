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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.PollArrayWrapper.interrupt(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.poll0(JIJ)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn poll_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    let _numfds = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
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
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.PollArrayWrapper.interrupt(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_poll_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = poll_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.PollArrayWrapper.poll0(JIJ)I",
            result.unwrap_err().to_string()
        );
    }
}

use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EventFD.eventfd0()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn eventfd0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EventFD.eventfd0()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EventFD.set0(I)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn set0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _efd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EventFD.set0(I)I".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eventfd0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = eventfd0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EventFD.eventfd0()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/EventFD.set0(I)I",
            result.unwrap_err().to_string()
        );
    }
}

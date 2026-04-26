use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EPollPort.close0(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollPort.close0(I)V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPollPort.drain1(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn drain1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollPort.drain1(I)V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPollPort.interrupt(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollPort.interrupt(I)V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPollPort.socketpair([I)V", Equal(JAVA_8))]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sv = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollPort.socketpair([I)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/EPollPort.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_drain1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drain1(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/EPollPort.drain1(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/EPollPort.interrupt(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socketpair(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/nio/ch/EPollPort.socketpair([I)V",
            result.unwrap_err().to_string()
        );
    }
}

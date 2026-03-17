use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueuePort.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueuePort.close0(I)V".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueuePort.drain1(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn drain_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueuePort.drain1(I)V".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueuePort.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueuePort.interrupt(I)V".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueuePort.socketpair([I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueuePort.socketpair([I)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drain_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = interrupt(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socketpair(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

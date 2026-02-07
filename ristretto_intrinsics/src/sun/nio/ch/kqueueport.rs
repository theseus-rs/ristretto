use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueuePort.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.close0(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.drain1(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn drain_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.drain1(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.interrupt(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.socketpair([I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socketpair<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.socketpair([I)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueuePort.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueuePort.drain1(I)V")]
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueuePort.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueuePort.socketpair([I)V")]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socketpair(thread, Parameters::default()).await;
    }
}

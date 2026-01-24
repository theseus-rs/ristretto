use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueuePort.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.close0(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.drain1(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn drain_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.drain1(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn interrupt(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.interrupt(I)V");
}

#[intrinsic_method("sun/nio/ch/KQueuePort.socketpair([I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn socketpair(
    _thread: Arc<Thread>,
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

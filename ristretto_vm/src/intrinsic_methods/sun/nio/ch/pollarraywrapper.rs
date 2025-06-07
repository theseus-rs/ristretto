use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn interrupt(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.interrupt(I)V")
}

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.poll0(JIJ)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn poll_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.poll0(JIJ)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.PollArrayWrapper.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.PollArrayWrapper.poll0(JIJ)I")]
    async fn test_poll_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll_0(thread, Parameters::default()).await;
    }
}

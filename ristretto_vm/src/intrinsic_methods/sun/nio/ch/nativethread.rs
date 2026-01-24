use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/NativeThread.current()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn current(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.current()J");
}

#[intrinsic_method("sun/nio/ch/NativeThread.current0()J", GreaterThan(JAVA_17))]
#[async_method]
pub(crate) async fn current_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    current(thread, parameters).await
}

#[intrinsic_method("sun/nio/ch/NativeThread.init()V", Any)]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn signal(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.signal(J)V");
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal0(J)V", GreaterThan(JAVA_17))]
#[async_method]
pub(crate) async fn signal_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    signal(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.current()J")]
    async fn test_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.current()J")]
    async fn test_current_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.signal(J)V")]
    async fn test_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.signal(J)V")]
    async fn test_signal_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/UnixDispatcher.close0(Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDispatcher.close0(Ljava/io/FileDescriptor;)V")
}

#[intrinsic_method("sun/nio/ch/UnixDispatcher.init()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn pre_close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDispatcher.close0(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pre_close_0(thread, Parameters::default()).await;
    }
}

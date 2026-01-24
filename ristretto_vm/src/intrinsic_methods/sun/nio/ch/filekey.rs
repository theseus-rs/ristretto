use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn init_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V");
}

#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[J)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub(crate) async fn init_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V");
}

#[intrinsic_method("sun/nio/ch/FileKey.initIDs()V", LessThanOrEqual(JAVA_21))]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_init_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V"
    )]
    async fn test_init_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

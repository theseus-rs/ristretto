use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/FileChannelImpl.initIDs()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJ)J", LessThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn map_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.map0(IJJ)J");
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJZ)J", Equal(JAVA_17))]
#[async_method]
pub(crate) async fn map_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.map0(IJJZ)J");
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.maxDirectTransferSize0()I", Equal(JAVA_17))]
#[async_method]
pub(crate) async fn max_direct_transfer_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I");
}

#[intrinsic_method(
    "sun/nio/ch/FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn transfer_to_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
    );
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.unmap0(JJ)I", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn unmap_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.unmap0(JJ)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileChannelImpl.map0(IJJ)J")]
    async fn test_map_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = map_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileChannelImpl.map0(IJJZ)J")]
    async fn test_map_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = map_0_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I"
    )]
    async fn test_max_direct_transfer_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = max_direct_transfer_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
    )]
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_to_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileChannelImpl.unmap0(JJ)I")]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unmap_0(thread, Parameters::default()).await;
    }
}

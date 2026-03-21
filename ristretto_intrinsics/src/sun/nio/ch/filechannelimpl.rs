use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/FileChannelImpl.initIDs()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJ)J", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn map_0_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.FileChannelImpl.map0(IJJ)J".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJZ)J", Equal(JAVA_17))]
#[async_method]
pub async fn map_0_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.FileChannelImpl.map0(IJJZ)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.maxDirectTransferSize0()I", Equal(JAVA_17))]
#[async_method]
pub async fn max_direct_transfer_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn transfer_to_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.unmap0(JJ)I", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn unmap_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.FileChannelImpl.unmap0(JJ)I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_map_0_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = map_0_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_map_0_1() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = map_0_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_max_direct_transfer_size_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = max_direct_transfer_size_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = transfer_to_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = unmap_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

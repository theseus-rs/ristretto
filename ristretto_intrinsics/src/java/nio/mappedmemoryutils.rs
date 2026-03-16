use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn force_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.isLoaded0(JJJ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_loaded_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("java/nio/MappedMemoryUtils.load0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.nio.MappedMemoryUtils.load0(JJ)V".to_string()).into())
}

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.registerNatives()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/nio/MappedMemoryUtils.unload0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn unload_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.nio.MappedMemoryUtils.unload0(JJ)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = force_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_loaded_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_natives(thread, Parameters::default()).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unload_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

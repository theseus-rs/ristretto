use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn force_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.isLoaded0(JJJ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn is_loaded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")
}

#[intrinsic_method("java/nio/MappedMemoryUtils.load0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.load0(JJ)V")
}

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.registerNatives()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/nio/MappedMemoryUtils.unload0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn unload_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.unload0(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V"
    )]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = force_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")]
    async fn test_is_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loaded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.load0(JJ)V")]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_natives(thread, Parameters::default()).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.unload0(JJ)V")]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_0(thread, Parameters::default()).await;
    }
}

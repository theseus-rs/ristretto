use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/nio/MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn force_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[intrinsic_method("java/nio/MappedByteBuffer.isLoaded0(JJI)Z", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_loaded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.MappedByteBuffer.isLoaded0(JJI)Z")
}

#[intrinsic_method("java/nio/MappedByteBuffer.load0(JJ)V", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedByteBuffer.load0(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V"
    )]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = force_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedByteBuffer.isLoaded0(JJI)Z")]
    async fn test_is_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loaded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedByteBuffer.load0(JJ)V")]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Parameters::default()).await;
    }
}

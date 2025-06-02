use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/nio/MappedByteBuffer";

/// Register all intrinsic methods for `java.nio.MappedByteBuffer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "force0",
        "(Ljava/io/FileDescriptor;JJ)V",
        force_0,
    );
    registry.register(CLASS_NAME, "isLoaded0", "(JJI)Z", is_loaded_0);
    registry.register(CLASS_NAME, "load0", "(JJ)V", load_0);
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn is_loaded_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedByteBuffer.isLoaded0(JJI)Z")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

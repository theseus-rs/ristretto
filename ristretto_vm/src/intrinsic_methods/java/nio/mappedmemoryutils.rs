use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/nio/MappedMemoryUtils";

/// Register all intrinsic methods for `java.nio.MappedMemoryUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_24 {
        registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    }

    registry.register(
        CLASS_NAME,
        "force0",
        "(Ljava/io/FileDescriptor;JJ)V",
        force_0,
    );
    registry.register(CLASS_NAME, "isLoaded0", "(JJJ)Z", is_loaded_0);
    registry.register(CLASS_NAME, "load0", "(JJ)V", load_0);
    registry.register(CLASS_NAME, "unload0", "(JJ)V", unload_0);
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn is_loaded_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.load0(JJ)V")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

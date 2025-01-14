use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.nio.MappedMemoryUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/nio/MappedMemoryUtils";
    registry.register(
        class_name,
        "force0",
        "(Ljava/io/FileDescriptor;JJ)V",
        force_0,
    );
    registry.register(class_name, "isLoaded0", "(JJJ)Z", is_loaded_0);
    registry.register(class_name, "load0", "(JJ)V", load_0);
    registry.register(class_name, "unload0", "(JJ)V", unload_0);
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn is_loaded_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.load0(JJ)V")
}

#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.unload0(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/nio/MappedMemoryUtils";
        assert!(registry
            .method(class_name, "force0", "(Ljava/io/FileDescriptor;JJ)V")
            .is_some());
        assert!(registry.method(class_name, "isLoaded0", "(JJJ)Z").is_some());
        assert!(registry.method(class_name, "load0", "(JJ)V").is_some());
        assert!(registry.method(class_name, "unload0", "(JJ)V").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V"
    )]
    async fn test_force_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = force_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")]
    async fn test_is_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loaded_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.load0(JJ)V")]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.nio.MappedMemoryUtils.unload0(JJ)V")]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_0(thread, Arguments::default()).await;
    }
}

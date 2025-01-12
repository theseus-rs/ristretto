use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.FileCleanable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileCleanable";
    registry.register(class_name, "cleanupClose0", "(IJ)V", cleanup_close_0);
}

#[async_recursion(?Send)]
async fn cleanup_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileCleanable.cleanupClose0(IJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/io/FileCleanable";
        assert!(registry
            .method(class_name, "cleanupClose0", "(IJ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileCleanable.cleanupClose0(IJ)V")]
    async fn test_cleanup_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = cleanup_close_0(thread, Arguments::default()).await;
    }
}

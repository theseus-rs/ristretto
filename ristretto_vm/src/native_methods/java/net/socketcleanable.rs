use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.SocketCleanable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/SocketCleanable";
    registry.register(class_name, "cleanupClose0", "(I)V", cleanup_close_0);
}

#[async_recursion(?Send)]
async fn cleanup_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.SocketCleanable.cleanupClose0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/net/SocketCleanable";
        assert!(registry
            .method(class_name, "cleanupClose0", "(I)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.SocketCleanable.cleanupClose0(I)V")]
    async fn test_cleanup_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = cleanup_close_0(thread, Arguments::default()).await;
    }
}

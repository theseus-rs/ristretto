use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.MacOSXNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/MacOSXNativeDispatcher";
    registry.register(class_name, "normalizepath", "([CI)[C", normalizepath);
}

#[async_recursion(?Send)]
async fn normalizepath(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.MacOSXNativeDispatcher.normalizepath([CI)[C");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/fs/MacOSXNativeDispatcher";
        assert!(registry
            .method(class_name, "normalizepath", "([CI)[C")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.MacOSXNativeDispatcher.normalizepath([CI)[C")]
    async fn test_normalizepath() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = normalizepath(thread, Arguments::default()).await;
    }
}

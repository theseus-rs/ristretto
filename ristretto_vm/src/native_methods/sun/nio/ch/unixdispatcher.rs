use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.UnixDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/UnixDispatcher";
    registry.register(class_name, "close0", "(Ljava/io/FileDescriptor;)V", close_0);
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "preClose0",
        "(Ljava/io/FileDescriptor;)V",
        pre_close_0,
    );
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDispatcher.close0(Ljava/io/FileDescriptor;)V")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn pre_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/UnixDispatcher";
        assert!(registry
            .method(class_name, "close0", "(Ljava/io/FileDescriptor;)V")
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(class_name, "preClose0", "(Ljava/io/FileDescriptor;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.UnixDispatcher.close0(Ljava/io/FileDescriptor;)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V")]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pre_close_0(thread, Arguments::default()).await;
    }
}

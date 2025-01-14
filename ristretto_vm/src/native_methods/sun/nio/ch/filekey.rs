use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.FileKey`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/FileKey";
    registry.register(class_name, "init", "(Ljava/io/FileDescriptor;)V", init);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/FileKey";
        assert!(registry
            .method(class_name, "init", "(Ljava/io/FileDescriptor;)V")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

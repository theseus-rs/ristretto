use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.awt.Cursor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/awt/Cursor";
    registry.register(class_name, "finalizeImpl", "(J)V", finalize_impl);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn finalize_impl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.Cursor.finalizeImpl(J)V")
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
        let class_name = "java/awt/Cursor";
        assert!(registry
            .method(class_name, "finalizeImpl", "(J)V")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.Cursor.finalizeImpl(J)V")]
    async fn test_finalize_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = finalize_impl(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ids(thread, Arguments::default()).await;
    }
}

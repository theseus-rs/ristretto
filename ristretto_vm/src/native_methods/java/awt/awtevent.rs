use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.awt.AWTEvent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/awt/AWTEvent";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "nativeSetSource",
        "(Ljava/awt/peer/ComponentPeer;)V",
        native_set_source,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn native_set_source(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/awt/AWTEvent";
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "nativeSetSource",
                "(Ljava/awt/peer/ComponentPeer;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V"
    )]
    async fn test_native_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_source(thread, Arguments::default()).await;
    }
}

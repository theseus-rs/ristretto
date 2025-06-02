use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/awt/AWTEvent";

/// Register all intrinsic methods for `java.awt.AWTEvent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "nativeSetSource",
        "(Ljava/awt/peer/ComponentPeer;)V",
        native_set_source,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn native_set_source(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V"
    )]
    async fn test_native_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_source(thread, Parameters::default()).await;
    }
}

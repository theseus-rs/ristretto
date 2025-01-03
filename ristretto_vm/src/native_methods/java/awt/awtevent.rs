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

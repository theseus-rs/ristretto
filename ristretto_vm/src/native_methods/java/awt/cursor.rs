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

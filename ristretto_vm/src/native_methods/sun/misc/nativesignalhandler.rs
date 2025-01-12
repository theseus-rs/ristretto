use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.NativeSignalHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/NativeSignalHandler";
    registry.register(class_name, "handle0", "(IJ)V", handle_0);
}

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.NativeSignalHandler.handle0(IJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/NativeSignalHandler";
        assert!(registry.method(class_name, "handle0", "(IJ)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.NativeSignalHandler.handle0(IJ)V")]
    async fn test_handle_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = handle_0(thread, Arguments::default()).await;
    }
}

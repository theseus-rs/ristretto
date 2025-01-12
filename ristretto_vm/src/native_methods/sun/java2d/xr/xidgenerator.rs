use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.xr.XIDGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/xr/XIDGenerator";
    registry.register(class_name, "bufferXIDs", "([II)V", buffer_x_ids);
}

#[async_recursion(?Send)]
async fn buffer_x_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XIDGenerator.bufferXIDs([II)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/xr/XIDGenerator";
        assert!(registry
            .method(class_name, "bufferXIDs", "([II)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.xr.XIDGenerator.bufferXIDs([II)V")]
    async fn test_buffer_x_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = buffer_x_ids(thread, Arguments::default()).await;
    }
}

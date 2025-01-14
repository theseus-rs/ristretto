use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NativeStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NativeStrikeDisposer";
    registry.register(
        class_name,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/NativeStrikeDisposer";
        assert!(registry
            .method(class_name, "freeNativeScalerContext", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V")]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_native_scaler_context(thread, Arguments::default()).await;
    }
}

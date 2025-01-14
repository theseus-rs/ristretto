use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.metal.MTLMaskFill`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLMaskFill";
    registry.register(class_name, "maskFill", "(IIIIIII[B)V", mask_fill);
}

#[async_recursion(?Send)]
async fn mask_fill(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLMaskFill.maskFill(IIIIIII[B)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/metal/MTLMaskFill";
        assert!(registry
            .method(class_name, "maskFill", "(IIIIIII[B)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLMaskFill.maskFill(IIIIIII[B)V")]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_fill(thread, Arguments::default()).await;
    }
}

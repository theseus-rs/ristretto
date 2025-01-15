use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLMaskFill";

/// Register all native methods for `sun.java2d.metal.MTLMaskFill`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "maskFill", "(IIIIIII[B)V", mask_fill);
}

#[async_recursion(?Send)]
async fn mask_fill(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLMaskFill.maskFill(IIIIIII[B)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLMaskFill.maskFill(IIIIIII[B)V"
    )]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_fill(thread, Arguments::default()).await;
    }
}

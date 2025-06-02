use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/NativeStrikeDisposer";

/// Register all intrinsic methods for `sun.font.NativeStrikeDisposer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "freeNativeScalerContext",
        "(J)V",
        free_native_scaler_context,
    );
}

#[async_recursion(?Send)]
async fn free_native_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrikeDisposer.freeNativeScalerContext(J)V"
    )]
    async fn test_free_native_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_native_scaler_context(thread, Parameters::default()).await;
    }
}

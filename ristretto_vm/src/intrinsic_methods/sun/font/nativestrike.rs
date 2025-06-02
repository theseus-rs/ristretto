use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/NativeStrike";

/// Register all intrinsic methods for `sun.font.NativeStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "createNullScalerContext",
        "()J",
        create_null_scaler_context,
    );
    registry.register(
        CLASS_NAME,
        "createScalerContext",
        "([BID)J",
        create_scaler_context,
    );
    registry.register(CLASS_NAME, "getMaxGlyph", "(J)I", get_max_glyph);
}

#[async_recursion(?Send)]
async fn create_null_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createNullScalerContext()J")
}

#[async_recursion(?Send)]
async fn create_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createScalerContext([BID)J")
}

#[async_recursion(?Send)]
async fn get_max_glyph(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.getMaxGlyph(J)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrike.createNullScalerContext()J"
    )]
    async fn test_create_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_null_scaler_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrike.createScalerContext([BID)J"
    )]
    async fn test_create_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_scaler_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeStrike.getMaxGlyph(J)I")]
    async fn test_get_max_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_glyph(thread, Parameters::default()).await;
    }
}

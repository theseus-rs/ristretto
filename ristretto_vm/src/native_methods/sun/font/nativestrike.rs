use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.NativeStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/NativeStrike";
    registry.register(
        class_name,
        "createNullScalerContext",
        "()J",
        create_null_scaler_context,
    );
    registry.register(
        class_name,
        "createScalerContext",
        "([BID)J",
        create_scaler_context,
    );
    registry.register(class_name, "getMaxGlyph", "(J)I", get_max_glyph);
}

#[async_recursion(?Send)]
async fn create_null_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createNullScalerContext()J")
}

#[async_recursion(?Send)]
async fn create_scaler_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createScalerContext([BID)J")
}

#[async_recursion(?Send)]
async fn get_max_glyph(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.getMaxGlyph(J)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/NativeStrike";
        assert!(registry
            .method(class_name, "createNullScalerContext", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "createScalerContext", "([BID)J")
            .is_some());
        assert!(registry.method(class_name, "getMaxGlyph", "(J)I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeStrike.createNullScalerContext()J")]
    async fn test_create_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_null_scaler_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeStrike.createScalerContext([BID)J")]
    async fn test_create_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_scaler_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.NativeStrike.getMaxGlyph(J)I")]
    async fn test_get_max_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_glyph(thread, Arguments::default()).await;
    }
}

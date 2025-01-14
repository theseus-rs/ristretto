use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.CGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsConfig";
    registry.register(
        class_name,
        "nativeGetBounds",
        "(I)Ljava/awt/geom/Rectangle2D;",
        native_get_bounds,
    );
}

#[async_recursion(?Send)]
async fn native_get_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/CGraphicsConfig";
        assert!(registry
            .method(
                class_name,
                "nativeGetBounds",
                "(I)Ljava/awt/geom/Rectangle2D;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_bounds(thread, Arguments::default()).await;
    }
}

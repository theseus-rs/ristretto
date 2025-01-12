use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.X11PMBlitLoops`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/X11PMBlitLoops";
    registry.register(
        class_name,
        "nativeBlit",
        "(JJJLsun/java2d/pipe/Region;IIIIII)V",
        native_blit,
    );
    registry.register(
        class_name,
        "updateBitmask",
        "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
        update_bitmask,
    );
}

#[async_recursion(?Send)]
async fn native_blit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V");
}

#[async_recursion(?Send)]
async fn update_bitmask(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/x11/X11PMBlitLoops";
        assert!(registry
            .method(
                class_name,
                "nativeBlit",
                "(JJJLsun/java2d/pipe/Region;IIIIII)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "updateBitmask",
                "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V"
    )]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V"
    )]
    async fn test_update_bitmask() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bitmask(thread, Arguments::default()).await;
    }
}

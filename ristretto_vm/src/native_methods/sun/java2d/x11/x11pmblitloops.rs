use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/x11/X11PMBlitLoops";

/// Register all native methods for `sun.java2d.x11.X11PMBlitLoops`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeBlit",
        "(JJJLsun/java2d/pipe/Region;IIIIII)V",
        native_blit,
    );
    registry.register(
        CLASS_NAME,
        "updateBitmask",
        "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
        update_bitmask,
    );
}

#[async_recursion(?Send)]
async fn native_blit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V");
}

#[async_recursion(?Send)]
async fn update_bitmask(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V"
    )]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V"
    )]
    async fn test_update_bitmask() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bitmask(thread, Parameters::default()).await;
    }
}

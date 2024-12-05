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

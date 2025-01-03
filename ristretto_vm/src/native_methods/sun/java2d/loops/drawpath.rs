use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.DrawPath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawPath";
    registry.register(
        class_name,
        "DrawPath",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IILjava/awt/geom/Path2D$Float;)V",
        draw_path,
    );
}

#[async_recursion(?Send)]
async fn draw_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.DrawPath.DrawPath(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IILjava/awt/geom/Path2D$Float;)V");
}

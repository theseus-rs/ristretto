use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.DrawPolygons`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawPolygons";
    registry.register(
        class_name,
        "DrawPolygons",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;[I[I[IIIIZ)V",
        draw_polygons,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn draw_polygons(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

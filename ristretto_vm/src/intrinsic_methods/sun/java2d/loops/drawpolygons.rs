use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/DrawPolygons";

/// Register all intrinsic methods for `sun.java2d.loops.DrawPolygons`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "DrawPolygons",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;[I[I[IIIIZ)V",
        draw_polygons,
    );
}

#[async_recursion(?Send)]
async fn draw_polygons(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawPolygons.DrawPolygons(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;[I[I[IIIIZ)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawPolygons.DrawPolygons(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;[I[I[IIIIZ)V"
    )]
    async fn test_draw_polygons() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_polygons(thread, Parameters::default()).await;
    }
}

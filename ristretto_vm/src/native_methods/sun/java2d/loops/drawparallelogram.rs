use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/DrawParallelogram";

/// Register all native methods for `sun.java2d.loops.DrawParallelogram`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "DrawParallelogram",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V",
        draw_parallelogram,
    );
}

#[async_recursion(?Send)]
async fn draw_parallelogram(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawParallelogram.DrawParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawParallelogram.DrawParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V"
    )]
    async fn test_draw_parallelogram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_parallelogram(thread, Parameters::default()).await;
    }
}

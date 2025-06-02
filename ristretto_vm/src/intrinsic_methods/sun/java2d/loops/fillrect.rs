use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/FillRect";

/// Register all intrinsic methods for `sun.java2d.loops.FillRect`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "FillRect",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
        fill_rect,
    );
}

#[async_recursion(?Send)]
async fn fill_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.FillRect.FillRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.FillRect.FillRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    )]
    async fn test_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_rect(thread, Parameters::default()).await;
    }
}

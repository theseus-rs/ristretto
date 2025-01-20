use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/FillPath";

/// Register all native methods for `sun.java2d.loops.FillPath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "FillPath",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IILjava/awt/geom/Path2D$Float;)V",
        fill_path,
    );
}

#[async_recursion(?Send)]
async fn fill_path(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.FillPath.FillPath(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IILjava/awt/geom/Path2D$Float;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.FillPath.FillPath(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IILjava/awt/geom/Path2D$Float;)V"
    )]
    async fn test_fill_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_path(thread, Parameters::default()).await;
    }
}

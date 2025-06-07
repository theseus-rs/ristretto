use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/DrawLine.DrawLine(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn draw_line(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.DrawLine.DrawLine(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.DrawLine.DrawLine(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    )]
    async fn test_draw_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_line(thread, Parameters::default()).await;
    }
}

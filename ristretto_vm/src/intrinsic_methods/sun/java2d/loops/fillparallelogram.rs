use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/FillParallelogram.FillParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDD)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fill_parallelogram(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.FillParallelogram.FillParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDD)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.FillParallelogram.FillParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDD)V"
    )]
    async fn test_fill_parallelogram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_parallelogram(thread, Parameters::default()).await;
    }
}

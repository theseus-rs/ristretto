use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/MaskFill";

/// Register all intrinsic methods for `sun.java2d.loops.MaskFill`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "DrawAAPgram",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V",
        draw_aa_pgram,
    );
    registry.register(
        CLASS_NAME,
        "FillAAPgram",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V",
        fill_aa_pgram,
    );
    registry.register(
        CLASS_NAME,
        "MaskFill",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V",
        mask_fill,
    );
}

#[async_recursion(?Send)]
async fn draw_aa_pgram(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V"
    );
}

#[async_recursion(?Send)]
async fn fill_aa_pgram(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V"
    );
}

#[async_recursion(?Send)]
async fn mask_fill(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V"
    )]
    async fn test_draw_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_aa_pgram(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V"
    )]
    async fn test_fill_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_aa_pgram(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V"
    )]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_fill(thread, Parameters::default()).await;
    }
}

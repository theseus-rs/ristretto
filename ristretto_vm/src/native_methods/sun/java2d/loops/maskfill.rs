use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.MaskFill`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/MaskFill";
    registry.register(
        class_name,
        "DrawAAPgram",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V",
        draw_aa_pgram,
    );
    registry.register(
        class_name,
        "FillAAPgram",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V",
        fill_aa_pgram,
    );
    registry.register(
        class_name,
        "MaskFill",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V",
        mask_fill,
    );
}

#[async_recursion(?Send)]
async fn draw_aa_pgram(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V");
}

#[async_recursion(?Send)]
async fn fill_aa_pgram(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V");
}

#[async_recursion(?Send)]
async fn mask_fill(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/loops/MaskFill";
        assert!(registry
            .method(
                class_name,
                "DrawAAPgram",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "FillAAPgram",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "MaskFill",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V"
    )]
    async fn test_draw_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_aa_pgram(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V"
    )]
    async fn test_fill_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_aa_pgram(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V"
    )]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_fill(thread, Arguments::default()).await;
    }
}

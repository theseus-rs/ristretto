use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.DrawRect`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawRect";
    registry.register(
        class_name,
        "DrawRect",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
        draw_rect,
    );
}

#[async_recursion(?Send)]
async fn draw_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.DrawRect.DrawRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/loops/DrawRect";
        assert!(registry
            .method(
                class_name,
                "DrawRect",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.DrawRect.DrawRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    )]
    async fn test_draw_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_rect(thread, Arguments::default()).await;
    }
}

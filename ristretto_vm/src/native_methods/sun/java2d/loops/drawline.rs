use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.DrawLine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawLine";
    registry.register(
        class_name,
        "DrawLine",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
        draw_line,
    );
}

#[async_recursion(?Send)]
async fn draw_line(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.DrawLine.DrawLine(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/loops/DrawLine";
        assert!(registry
            .method(
                class_name,
                "DrawLine",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.DrawLine.DrawLine(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V"
    )]
    async fn test_draw_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_line(thread, Arguments::default()).await;
    }
}

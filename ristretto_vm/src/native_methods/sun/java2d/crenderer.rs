use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.CRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/CRenderer";
    registry.register(
        class_name,
        "doArc",
        "(Lsun/java2d/SurfaceData;FFFFFFIZ)V",
        do_arc,
    );
    registry.register(
        class_name,
        "doImage",
        "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
        do_image,
    );
    registry.register(
        class_name,
        "doLine",
        "(Lsun/java2d/SurfaceData;FFFF)V",
        do_line,
    );
    registry.register(
        class_name,
        "doOval",
        "(Lsun/java2d/SurfaceData;FFFFZ)V",
        do_oval,
    );
    registry.register(
        class_name,
        "doPoly",
        "(Lsun/java2d/SurfaceData;[I[IIZZ)V",
        do_poly,
    );
    registry.register(
        class_name,
        "doRect",
        "(Lsun/java2d/SurfaceData;FFFFZ)V",
        do_rect,
    );
    registry.register(
        class_name,
        "doRoundRect",
        "(Lsun/java2d/SurfaceData;FFFFFFZ)V",
        do_round_rect,
    );
    registry.register(
        class_name,
        "doShape",
        "(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
        do_shape,
    );
    registry.register(class_name, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn do_arc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V");
}

#[async_recursion(?Send)]
async fn do_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V");
}

#[async_recursion(?Send)]
async fn do_line(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V");
}

#[async_recursion(?Send)]
async fn do_oval(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V");
}

#[async_recursion(?Send)]
async fn do_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_round_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_shape(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V");
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/CRenderer";
        assert!(registry
            .method(class_name, "doArc", "(Lsun/java2d/SurfaceData;FFFFFFIZ)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doImage",
                "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "doLine", "(Lsun/java2d/SurfaceData;FFFF)V")
            .is_some());
        assert!(registry
            .method(class_name, "doOval", "(Lsun/java2d/SurfaceData;FFFFZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "doPoly", "(Lsun/java2d/SurfaceData;[I[IIZZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "doRect", "(Lsun/java2d/SurfaceData;FFFFZ)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doRoundRect",
                "(Lsun/java2d/SurfaceData;FFFFFFZ)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doShape",
                "(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V"
            )
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V")]
    async fn test_do_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_arc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V"
    )]
    async fn test_do_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V")]
    async fn test_do_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_line(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V")]
    async fn test_do_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_oval(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V")]
    async fn test_do_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_poly(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V")]
    async fn test_do_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V")]
    async fn test_do_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_round_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V"
    )]
    async fn test_do_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_shape(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

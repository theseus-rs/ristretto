use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/CRenderer";

/// Register all intrinsic methods for `sun.java2d.CRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "doArc",
        "(Lsun/java2d/SurfaceData;FFFFFFIZ)V",
        do_arc,
    );
    registry.register(
        CLASS_NAME,
        "doImage",
        "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
        do_image,
    );
    registry.register(
        CLASS_NAME,
        "doLine",
        "(Lsun/java2d/SurfaceData;FFFF)V",
        do_line,
    );
    registry.register(
        CLASS_NAME,
        "doOval",
        "(Lsun/java2d/SurfaceData;FFFFZ)V",
        do_oval,
    );
    registry.register(
        CLASS_NAME,
        "doPoly",
        "(Lsun/java2d/SurfaceData;[I[IIZZ)V",
        do_poly,
    );
    registry.register(
        CLASS_NAME,
        "doRect",
        "(Lsun/java2d/SurfaceData;FFFFZ)V",
        do_rect,
    );
    registry.register(
        CLASS_NAME,
        "doRoundRect",
        "(Lsun/java2d/SurfaceData;FFFFFFZ)V",
        do_round_rect,
    );
    registry.register(
        CLASS_NAME,
        "doShape",
        "(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
        do_shape,
    );
    registry.register(CLASS_NAME, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn do_arc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V");
}

#[async_recursion(?Send)]
async fn do_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V"
    );
}

#[async_recursion(?Send)]
async fn do_line(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V");
}

#[async_recursion(?Send)]
async fn do_oval(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V");
}

#[async_recursion(?Send)]
async fn do_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_round_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V");
}

#[async_recursion(?Send)]
async fn do_shape(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V"
    );
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V"
    )]
    async fn test_do_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_arc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V"
    )]
    async fn test_do_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V"
    )]
    async fn test_do_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_line(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V"
    )]
    async fn test_do_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_oval(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V"
    )]
    async fn test_do_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_poly(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V"
    )]
    async fn test_do_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V"
    )]
    async fn test_do_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_round_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V"
    )]
    async fn test_do_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_shape(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

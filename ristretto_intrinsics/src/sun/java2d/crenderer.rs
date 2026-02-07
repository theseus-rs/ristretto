use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V", Any)]
#[async_method]
pub async fn do_arc<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V");
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_image<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V"
    );
}

#[intrinsic_method("sun/java2d/CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V", Any)]
#[async_method]
pub async fn do_line<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V");
}

#[intrinsic_method("sun/java2d/CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_oval<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[intrinsic_method("sun/java2d/CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V", Any)]
#[async_method]
pub async fn do_poly<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V");
}

#[intrinsic_method("sun/java2d/CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_rect<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V");
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V",
    Any
)]
#[async_method]
pub async fn do_round_rect<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V");
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
    Any
)]
#[async_method]
pub async fn do_shape<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V"
    );
}

#[intrinsic_method("sun/java2d/CRenderer.init()V", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

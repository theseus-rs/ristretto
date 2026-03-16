use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V", Any)]
#[async_method]
pub async fn do_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V", Any)]
#[async_method]
pub async fn do_line<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V", Any)]
#[async_method]
pub async fn do_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V",
    Any
)]
#[async_method]
pub async fn do_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
    Any
)]
#[async_method]
pub async fn do_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/CRenderer.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_arc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_line(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_oval(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_poly(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_round_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_shape(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

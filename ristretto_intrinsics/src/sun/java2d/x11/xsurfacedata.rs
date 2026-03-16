use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XCreateGC(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn x_create_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.XCreateGC(J)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XResetClip(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn x_reset_clip<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.XResetClip(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_clip<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_graphics_exposures<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.flushNativeSurface()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn flush_native_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.flushNativeSurface()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.isDrawableValid()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_drawable_valid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.isDrawableValid()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.setInvalid()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn set_invalid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.setInvalid()V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_create_gc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_reset_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_reset_clip(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_set_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_set_clip(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_set_graphics_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_set_graphics_exposures(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_flush_native_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_native_surface(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_drawable_valid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_drawable_valid(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_invalid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_invalid(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

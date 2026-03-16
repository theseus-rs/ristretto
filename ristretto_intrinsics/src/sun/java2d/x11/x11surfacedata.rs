use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_copy_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_foreground<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_xor_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isDgaAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_dga_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_shm_pm_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.isShmPMAvailable()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_set_copy_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_set_copy_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_set_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_set_foreground(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_set_xor_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_set_xor_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_surface(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_dga_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_dga_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_shm_pm_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_shm_pm_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

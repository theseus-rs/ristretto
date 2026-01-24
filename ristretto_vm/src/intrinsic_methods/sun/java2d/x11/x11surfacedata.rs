use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_set_copy_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V")
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_set_foreground(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V")
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_set_xor_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V")
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn init_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V")
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isDgaAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn is_dga_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z")
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn is_shm_pm_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.isShmPMAvailable()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V"
    )]
    async fn test_x_set_copy_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_copy_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V"
    )]
    async fn test_x_set_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_foreground(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V")]
    async fn test_x_set_xor_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_xor_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V"
    )]
    async fn test_init_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z"
    )]
    async fn test_is_dga_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_dga_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11SurfaceData.isShmPMAvailable()Z"
    )]
    async fn test_is_shm_pm_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_shm_pm_available(thread, Parameters::default()).await;
    }
}

use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/x11/X11SurfaceData";

/// Register all intrinsic methods for `sun.java2d.x11.X11SurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "XSetCopyMode", "(J)V", x_set_copy_mode);
    registry.register(CLASS_NAME, "XSetForeground", "(JI)V", x_set_foreground);
    registry.register(CLASS_NAME, "XSetXorMode", "(J)V", x_set_xor_mode);
    registry.register(CLASS_NAME, "initIDs", "(Ljava/lang/Class;Z)V", init_ids);
    registry.register(CLASS_NAME, "initSurface", "(IIIJ)V", init_surface);
    registry.register(CLASS_NAME, "isDgaAvailable", "()Z", is_dga_available);
    registry.register(CLASS_NAME, "isShmPMAvailable", "()Z", is_shm_pm_available);
}

#[async_recursion(?Send)]
async fn x_set_copy_mode(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V")
}

#[async_recursion(?Send)]
async fn x_set_foreground(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V")
}

#[async_recursion(?Send)]
async fn x_set_xor_mode(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_surface(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V")
}

#[async_recursion(?Send)]
async fn is_dga_available(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z")
}

#[async_recursion(?Send)]
async fn is_shm_pm_available(
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

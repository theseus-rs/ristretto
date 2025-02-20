use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/xr/XRSurfaceData";

/// Register all native methods for `sun.java2d.xr.XRSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "XRInitSurface", "(IIIJI)V", xr_init_surface);
    registry.register(CLASS_NAME, "freeXSDOPicture", "(J)V", free_xsdo_picture);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "initXRPicture", "(JI)V", init_xr_picture);
}

#[async_recursion(?Send)]
async fn xr_init_surface(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V");
}

#[async_recursion(?Send)]
async fn free_xsdo_picture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_xr_picture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V"
    )]
    async fn test_xr_init_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_init_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V"
    )]
    async fn test_free_xsdo_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_xsdo_picture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V"
    )]
    async fn test_init_xr_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_xr_picture(thread, Parameters::default()).await;
    }
}

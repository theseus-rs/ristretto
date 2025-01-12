use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.xr.XRSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/xr/XRSurfaceData";
    registry.register(class_name, "XRInitSurface", "(IIIJI)V", xr_init_surface);
    registry.register(class_name, "freeXSDOPicture", "(J)V", free_xsdo_picture);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "initXRPicture", "(JI)V", init_xr_picture);
}

#[async_recursion(?Send)]
async fn xr_init_surface(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V");
}

#[async_recursion(?Send)]
async fn free_xsdo_picture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_xr_picture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/xr/XRSurfaceData";
        assert!(registry
            .method(class_name, "XRInitSurface", "(IIIJI)V")
            .is_some());
        assert!(registry
            .method(class_name, "freeXSDOPicture", "(J)V")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "initXRPicture", "(JI)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V")]
    async fn test_xr_init_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_init_surface(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V")]
    async fn test_free_xsdo_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_xsdo_picture(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V")]
    async fn test_init_xr_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_xr_picture(thread, Arguments::default()).await;
    }
}

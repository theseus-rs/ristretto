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

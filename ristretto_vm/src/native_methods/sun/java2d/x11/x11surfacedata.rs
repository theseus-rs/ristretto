use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.X11SurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/X11SurfaceData";
    registry.register(class_name, "XSetCopyMode", "(J)V", x_set_copy_mode);
    registry.register(class_name, "XSetForeground", "(JI)V", x_set_foreground);
    registry.register(class_name, "XSetXorMode", "(J)V", x_set_xor_mode);
    registry.register(class_name, "initIDs", "(Ljava/lang/Class;Z)V", init_ids);
    registry.register(class_name, "initSurface", "(IIIJ)V", init_surface);
    registry.register(class_name, "isDgaAvailable", "()Z", is_dga_available);
    registry.register(class_name, "isShmPMAvailable", "()Z", is_shm_pm_available);
}

#[async_recursion(?Send)]
async fn x_set_copy_mode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_set_foreground(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_set_xor_mode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_surface(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_dga_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_shm_pm_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

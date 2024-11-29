use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.XSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/XSurfaceData";
    registry.register(class_name, "XCreateGC", "(J)J", x_create_gc);
    registry.register(class_name, "XResetClip", "(J)V", x_reset_clip);
    registry.register(
        class_name,
        "XSetClip",
        "(JIIIILsun/java2d/pipe/Region;)V",
        x_set_clip,
    );
    registry.register(
        class_name,
        "XSetGraphicsExposures",
        "(JZ)V",
        x_set_graphics_exposures,
    );
    registry.register(
        class_name,
        "flushNativeSurface",
        "()V",
        flush_native_surface,
    );
    registry.register(
        class_name,
        "initOps",
        "(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
        init_ops,
    );
    registry.register(class_name, "isDrawableValid", "()Z", is_drawable_valid);
    registry.register(class_name, "setInvalid", "()V", set_invalid);
}

#[async_recursion(?Send)]
async fn x_create_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_reset_clip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_set_clip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_set_graphics_exposures(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn flush_native_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_drawable_valid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_invalid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

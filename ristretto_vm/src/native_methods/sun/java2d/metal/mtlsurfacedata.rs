use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLSurfaceData";

/// Register all native methods for `sun.java2d.metal.MTLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "clearWindow", "()V", clear_window);
    registry.register(
        CLASS_NAME,
        "getMTLTexturePointer",
        "(J)J",
        get_mtl_texture_pointer,
    );
    registry.register(
        CLASS_NAME,
        "initFlipBackbuffer",
        "(J)Z",
        init_flip_backbuffer,
    );
    registry.register(
        CLASS_NAME,
        "initOps",
        "(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
        init_ops,
    );
    registry.register(CLASS_NAME, "initRTexture", "(JZII)Z", init_r_texture);
    registry.register(CLASS_NAME, "initTexture", "(JZII)Z", init_texture);
}

#[async_recursion(?Send)]
async fn clear_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.clearWindow()V");
}

#[async_recursion(?Send)]
async fn get_mtl_texture_pointer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J");
}

#[async_recursion(?Send)]
async fn init_flip_backbuffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z");
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V");
}

#[async_recursion(?Send)]
async fn init_r_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z");
}

#[async_recursion(?Send)]
async fn init_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z");
}

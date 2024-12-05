use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.OGLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/OGLSurfaceData";
    registry.register(class_name, "getTextureID", "(J)I", get_texture_id);
    registry.register(class_name, "getTextureTarget", "(J)I", get_texture_target);
    registry.register(class_name, "initFBObject", "(JZZZII)Z", init_fb_object);
    registry.register(
        class_name,
        "initFlipBackbuffer",
        "(J)Z",
        init_flip_backbuffer,
    );
    registry.register(class_name, "initTexture", "(JZZZII)Z", init_texture);
}

#[async_recursion(?Send)]
async fn get_texture_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I");
}

#[async_recursion(?Send)]
async fn get_texture_target(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I");
}

#[async_recursion(?Send)]
async fn init_fb_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z");
}

#[async_recursion(?Send)]
async fn init_flip_backbuffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z");
}

#[async_recursion(?Send)]
async fn init_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z");
}

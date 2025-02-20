use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/OGLSurfaceData";

/// Register all native methods for `sun.java2d.opengl.OGLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getTextureID", "(J)I", get_texture_id);
    registry.register(CLASS_NAME, "getTextureTarget", "(J)I", get_texture_target);
    registry.register(CLASS_NAME, "initFBObject", "(JZZZII)Z", init_fb_object);
    registry.register(
        CLASS_NAME,
        "initFlipBackbuffer",
        "(J)Z",
        init_flip_backbuffer,
    );
    registry.register(CLASS_NAME, "initTexture", "(JZZZII)Z", init_texture);
}

#[async_recursion(?Send)]
async fn get_texture_id(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I");
}

#[async_recursion(?Send)]
async fn get_texture_target(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I");
}

#[async_recursion(?Send)]
async fn init_fb_object(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z");
}

#[async_recursion(?Send)]
async fn init_flip_backbuffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z");
}

#[async_recursion(?Send)]
async fn init_texture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I"
    )]
    async fn test_get_texture_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_texture_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I"
    )]
    async fn test_get_texture_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_texture_target(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z"
    )]
    async fn test_init_fb_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_fb_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z"
    )]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_flip_backbuffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z"
    )]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_texture(thread, Parameters::default()).await;
    }
}

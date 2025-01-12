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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/OGLSurfaceData";
        assert!(registry
            .method(class_name, "getTextureID", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "getTextureTarget", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "initFBObject", "(JZZZII)Z")
            .is_some());
        assert!(registry
            .method(class_name, "initFlipBackbuffer", "(J)Z")
            .is_some());
        assert!(registry
            .method(class_name, "initTexture", "(JZZZII)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I")]
    async fn test_get_texture_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_texture_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I")]
    async fn test_get_texture_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_texture_target(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z")]
    async fn test_init_fb_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_fb_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z")]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_flip_backbuffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z")]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_texture(thread, Arguments::default()).await;
    }
}

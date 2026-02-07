use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.getTextureID(J)I", Any)]
#[async_method]
pub async fn get_texture_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I");
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.getTextureTarget(J)I", Any)]
#[async_method]
pub async fn get_texture_target<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I");
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFBObject(JZZZII)Z", Any)]
#[async_method]
pub async fn init_fb_object<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z");
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFlipBackbuffer(J)Z", Any)]
#[async_method]
pub async fn init_flip_backbuffer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z");
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initTexture(JZZZII)Z", Any)]
#[async_method]
pub async fn init_texture<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

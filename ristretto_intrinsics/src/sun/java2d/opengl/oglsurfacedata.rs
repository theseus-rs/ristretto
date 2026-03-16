use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.getTextureID(J)I", Any)]
#[async_method]
pub async fn get_texture_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.getTextureTarget(J)I", Any)]
#[async_method]
pub async fn get_texture_target<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFBObject(JZZZII)Z", Any)]
#[async_method]
pub async fn init_fb_object<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFlipBackbuffer(J)Z", Any)]
#[async_method]
pub async fn init_flip_backbuffer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initTexture(JZZZII)Z", Any)]
#[async_method]
pub async fn init_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_texture_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_texture_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_texture_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_texture_target(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_fb_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_fb_object(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_flip_backbuffer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_texture(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

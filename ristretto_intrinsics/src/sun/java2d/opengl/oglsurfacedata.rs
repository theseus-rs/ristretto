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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.getTextureTarget(J)I", Any)]
#[async_method]
pub async fn get_texture_target<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFBObject(JZZZII)Z", Any)]
#[async_method]
pub async fn init_fb_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _tex_rect = parameters.pop_bool()?;
    let _tex_non_pow2 = parameters.pop_bool()?;
    let _is_opaque = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initFlipBackbuffer(J)Z", Any)]
#[async_method]
pub async fn init_flip_backbuffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/OGLSurfaceData.initTexture(JZZZII)Z", Any)]
#[async_method]
pub async fn init_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _tex_rect = parameters.pop_bool()?;
    let _tex_non_pow2 = parameters.pop_bool()?;
    let _is_opaque = parameters.pop_bool()?;
    let _p_data = parameters.pop_long()?;
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
        let result = get_texture_id(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.OGLSurfaceData.getTextureID(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_texture_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_texture_target(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.OGLSurfaceData.getTextureTarget(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_fb_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_fb_object(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.OGLSurfaceData.initFBObject(JZZZII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_flip_backbuffer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.OGLSurfaceData.initFlipBackbuffer(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_texture(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.OGLSurfaceData.initTexture(JZZZII)Z",
            result.unwrap_err().to_string()
        );
    }
}

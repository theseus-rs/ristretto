use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/CGLLayer.blitTexture(J)V", Any)]
#[async_method]
pub async fn blit_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.opengl.CGLLayer.blitTexture(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeCreateLayer()J", Any)]
#[async_method]
pub async fn native_create_layer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.nativeCreateLayer()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeSetScale(JD)V", Any)]
#[async_method]
pub async fn native_set_scale<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V",
    Any
)]
#[async_method]
pub async fn validate<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = blit_texture(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_layer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_scale(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = validate(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

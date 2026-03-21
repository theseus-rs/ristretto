use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/CGLGraphicsConfig.getCGLConfigInfo()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_cgl_config_info_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLGraphicsConfig.getCGLConfigInfo(III)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_cgl_config_info_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.getOGLCapabilities(J)I", Any)]
#[async_method]
pub async fn get_ogl_capabilities<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.initCGL()Z", Any)]
#[async_method]
pub async fn init_cgl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.nativeGetMaxTextureSize()I", Any)]
#[async_method]
pub async fn native_get_max_texture_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_cgl_config_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cgl_config_info_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cgl_config_info_1() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_cgl_config_info_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ogl_capabilities(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_cgl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_cgl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_max_texture_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_max_texture_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

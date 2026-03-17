use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_glx_config_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_ogl_capabilities<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_config<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_glx_config_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glx_config_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ogl_capabilities(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_config(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

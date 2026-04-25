use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/WGLGraphicsConfig.getDefaultPixFmt(I)I", Any)]
#[async_method]
pub async fn get_default_pix_fmt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screennum = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/WGLGraphicsConfig.getDefaultPixFmt(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/opengl/WGLGraphicsConfig.getOGLCapabilities(J)I", Any)]
#[async_method]
pub async fn get_oglcapabilities<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _config_info = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/WGLGraphicsConfig.getOGLCapabilities(J)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/opengl/WGLGraphicsConfig.getWGLConfigInfo(II)J", Any)]
#[async_method]
pub async fn get_wglconfig_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixfmt = parameters.pop_int()?;
    let _screennum = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/WGLGraphicsConfig.getWGLConfigInfo(II)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/opengl/WGLGraphicsConfig.initWGL()Z", Any)]
#[async_method]
pub async fn init_wgl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/WGLGraphicsConfig.initWGL()Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_default_pix_fmt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_pix_fmt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/opengl/WGLGraphicsConfig.getDefaultPixFmt(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_oglcapabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_oglcapabilities(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/opengl/WGLGraphicsConfig.getOGLCapabilities(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_wglconfig_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_wglconfig_info(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/opengl/WGLGraphicsConfig.getWGLConfigInfo(II)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_wgl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_wgl(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/opengl/WGLGraphicsConfig.initWGL()Z",
            result.unwrap_err().to_string()
        );
    }
}

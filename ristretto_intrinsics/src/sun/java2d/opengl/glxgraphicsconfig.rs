use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visualnum = parameters.pop_int()?;
    let _screennum = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _config_info = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ctxinfo = parameters.pop_long()?;
    let _a_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_glxconfig_info_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visualnum = parameters.pop_int()?;
    let _screennum = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_oglcapabilities_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _config_info = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_config_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ctxinfo = parameters.pop_long()?;
    let _a_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_glx_config_info() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_glx_config_info(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_ogl_capabilities(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_config() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_config(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_glxconfig_info_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glxconfig_info_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_oglcapabilities_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_oglcapabilities_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_config_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_config_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V",
            result.unwrap_err().to_string()
        );
    }
}

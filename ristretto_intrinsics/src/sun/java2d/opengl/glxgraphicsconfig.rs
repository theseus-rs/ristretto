use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_glx_config_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J")
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_ogl_capabilities<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I")
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_config<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J"
    )]
    async fn test_get_glx_config_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glx_config_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I"
    )]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_capabilities(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V"
    )]
    async fn test_init_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_config(thread, Parameters::default()).await;
    }
}

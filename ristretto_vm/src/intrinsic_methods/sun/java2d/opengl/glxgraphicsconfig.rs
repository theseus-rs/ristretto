use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getGLXConfigInfo(II)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glx_config_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J")
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.getOGLCapabilities(J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I")
}

#[intrinsic_method(
    "sun/java2d/opengl/GLXGraphicsConfig.initConfig(JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_config(
    _thread: Arc<Thread>,
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

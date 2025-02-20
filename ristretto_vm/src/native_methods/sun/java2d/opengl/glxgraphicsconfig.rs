use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/GLXGraphicsConfig";

/// Register all native methods for `sun.java2d.opengl.GLXGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getGLXConfigInfo", "(II)J", get_glx_config_info);
    registry.register(
        CLASS_NAME,
        "getOGLCapabilities",
        "(J)I",
        get_ogl_capabilities,
    );
    registry.register(CLASS_NAME, "initConfig", "(JJ)V", init_config);
}

#[async_recursion(?Send)]
async fn get_glx_config_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J")
}

#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I")
}

#[async_recursion(?Send)]
async fn init_config(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

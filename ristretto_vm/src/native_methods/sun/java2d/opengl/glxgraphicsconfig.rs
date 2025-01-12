use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.GLXGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/GLXGraphicsConfig";
    registry.register(class_name, "getGLXConfigInfo", "(II)J", get_glx_config_info);
    registry.register(
        class_name,
        "getOGLCapabilities",
        "(J)I",
        get_ogl_capabilities,
    );
    registry.register(class_name, "initConfig", "(JJ)V", init_config);
}

#[async_recursion(?Send)]
async fn get_glx_config_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J")
}

#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I")
}

#[async_recursion(?Send)]
async fn init_config(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/GLXGraphicsConfig";
        assert!(registry
            .method(class_name, "getGLXConfigInfo", "(II)J")
            .is_some());
        assert!(registry
            .method(class_name, "getOGLCapabilities", "(J)I")
            .is_some());
        assert!(registry.method(class_name, "initConfig", "(JJ)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.GLXGraphicsConfig.getGLXConfigInfo(II)J")]
    async fn test_get_glx_config_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glx_config_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.GLXGraphicsConfig.getOGLCapabilities(J)I")]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_capabilities(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.GLXGraphicsConfig.initConfig(JJ)V")]
    async fn test_init_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_config(thread, Arguments::default()).await;
    }
}

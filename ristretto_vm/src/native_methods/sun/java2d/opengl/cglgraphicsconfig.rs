use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/CGLGraphicsConfig";

/// Register all native methods for `sun.java2d.opengl.CGLGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "getCGLConfigInfo",
            "(III)J",
            get_cgl_config_info_1,
        );
    }

    if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "getCGLConfigInfo", "()J", get_cgl_config_info_0);
    }

    registry.register(
        CLASS_NAME,
        "getOGLCapabilities",
        "(J)I",
        get_ogl_capabilities,
    );
    registry.register(CLASS_NAME, "initCGL", "()Z", init_cgl);
    registry.register(
        CLASS_NAME,
        "nativeGetMaxTextureSize",
        "()I",
        native_get_max_texture_size,
    );
}

#[async_recursion(?Send)]
async fn get_cgl_config_info_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo()J")
}

#[async_recursion(?Send)]
async fn get_cgl_config_info_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J")
}

#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I")
}

#[async_recursion(?Send)]
async fn init_cgl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z")
}

#[async_recursion(?Send)]
async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo()J"
    )]
    async fn test_get_cgl_config_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cgl_config_info_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J"
    )]
    async fn test_get_cgl_config_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cgl_config_info_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I"
    )]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_capabilities(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z"
    )]
    async fn test_init_cgl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_cgl(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I"
    )]
    async fn test_native_get_max_texture_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_max_texture_size(thread, Arguments::default()).await;
    }
}

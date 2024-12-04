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
    todo!()
}

#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_config(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

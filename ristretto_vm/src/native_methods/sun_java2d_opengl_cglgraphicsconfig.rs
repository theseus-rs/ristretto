use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.CGLGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/CGLGraphicsConfig";
    registry.register(
        class_name,
        "getCGLConfigInfo",
        "(III)J",
        get_cgl_config_info,
    );
    registry.register(
        class_name,
        "getOGLCapabilities",
        "(J)I",
        get_ogl_capabilities,
    );
    registry.register(class_name, "initCGL", "()Z", init_cgl);
    registry.register(
        class_name,
        "nativeGetMaxTextureSize",
        "()I",
        native_get_max_texture_size,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cgl_config_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_cgl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.CGLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/CGLSurfaceData";
    registry.register(class_name, "clearWindow", "()V", clear_window);
    registry.register(
        class_name,
        "createCGLContextOnSurface",
        "(Lsun/java2d/opengl/CGLSurfaceData;J)J",
        create_cgl_context_on_surface,
    );
    registry.register(class_name, "destroyCGLContext", "(J)V", destroy_cgl_context);
    registry.register(
        class_name,
        "initOps",
        "(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
        init_ops,
    );
    registry.register(
        class_name,
        "makeCGLContextCurrentOnSurface",
        "(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
        make_cgl_context_current_on_surface,
    );
    registry.register(class_name, "validate", "(IIIIZ)V", validate);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn clear_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_cgl_context_on_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn destroy_cgl_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn make_cgl_context_current_on_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

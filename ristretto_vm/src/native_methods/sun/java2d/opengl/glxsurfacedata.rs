use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.GLXSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/GLXSurfaceData";
    registry.register(
        class_name,
        "initOps",
        "(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V");
}

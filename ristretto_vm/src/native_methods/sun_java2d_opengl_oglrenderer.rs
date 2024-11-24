use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.OGLRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/OGLRenderer";
    registry.register(class_name, "drawPoly", "([I[IIZII)V", draw_poly);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn draw_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

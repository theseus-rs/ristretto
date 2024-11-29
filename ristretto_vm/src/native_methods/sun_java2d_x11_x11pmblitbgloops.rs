use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.X11PMBlitBgLoops`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/X11PMBlitBgLoops";
    registry.register(class_name, "nativeBlitBg", "(JJJIIIIIII)V", native_blit_bg);
}

#[async_recursion(?Send)]
async fn native_blit_bg(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

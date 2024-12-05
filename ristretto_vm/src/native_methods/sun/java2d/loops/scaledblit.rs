use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.ScaledBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/ScaledBlit";
    registry.register(class_name, "Scale", "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V", scale);
}

#[async_recursion(?Send)]
async fn scale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V");
}

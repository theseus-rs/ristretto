use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.DefaultMouseInfoPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/DefaultMouseInfoPeer";
    registry.register(
        class_name,
        "fillPointWithCoords",
        "(Ljava/awt/Point;)I",
        fill_point_with_coords,
    );
    registry.register(
        class_name,
        "isWindowUnderMouse",
        "(Ljava/awt/Window;)Z",
        is_window_under_mouse,
    );
}

#[async_recursion(?Send)]
async fn fill_point_with_coords(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_window_under_mouse(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

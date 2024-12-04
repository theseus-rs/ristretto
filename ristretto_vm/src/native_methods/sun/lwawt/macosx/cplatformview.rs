use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPlatformView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPlatformView";
    registry.register(
        class_name,
        "nativeCreateView",
        "(IIIIJ)J",
        native_create_view,
    );
    registry.register(
        class_name,
        "nativeGetLocationOnScreen",
        "(J)Ljava/awt/geom/Rectangle2D;",
        native_get_location_on_screen,
    );
    registry.register(
        class_name,
        "nativeGetNSViewDisplayID",
        "(J)I",
        native_get_ns_view_display_id,
    );
    registry.register(
        class_name,
        "nativeIsViewUnderMouse",
        "(J)Z",
        native_is_view_under_mouse,
    );
    registry.register(
        class_name,
        "nativeSetAutoResizable",
        "(JZ)V",
        native_set_auto_resizable,
    );
}

#[async_recursion(?Send)]
async fn native_create_view(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_location_on_screen(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_ns_view_display_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_is_view_under_mouse(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_auto_resizable(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

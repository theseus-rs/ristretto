use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CTrayIcon`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CTrayIcon";
    registry.register(class_name, "nativeCreate", "()J", native_create);
    registry.register(
        class_name,
        "nativeGetIconLocation",
        "(J)Ljava/awt/geom/Point2D;",
        native_get_icon_location,
    );
    registry.register(
        class_name,
        "nativeSetToolTip",
        "(JLjava/lang/String;)V",
        native_set_tool_tip,
    );
    registry.register(class_name, "setNativeImage", "(JJZ)V", set_native_image);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_icon_location(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_tool_tip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_native_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

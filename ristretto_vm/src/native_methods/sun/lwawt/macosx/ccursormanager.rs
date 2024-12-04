use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CCursorManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CCursorManager";
    registry.register(
        class_name,
        "nativeGetCursorPosition",
        "()Ljava/awt/geom/Point2D;",
        native_get_cursor_position,
    );
    registry.register(
        class_name,
        "nativeSetAllowsCursorSetInBackground",
        "(Z)V",
        native_set_allows_cursor_set_in_background,
    );
    registry.register(
        class_name,
        "nativeSetBuiltInCursor",
        "(ILjava/lang/String;)V",
        native_set_built_in_cursor,
    );
    registry.register(
        class_name,
        "nativeSetCustomCursor",
        "(JDD)V",
        native_set_custom_cursor,
    );
}

#[async_recursion(?Send)]
async fn native_get_cursor_position(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_allows_cursor_set_in_background(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_built_in_cursor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_custom_cursor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

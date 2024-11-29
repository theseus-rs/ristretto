use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDataTransferer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDataTransferer";
    registry.register(
        class_name,
        "formatForIndex",
        "(J)Ljava/lang/String;",
        format_for_index,
    );
    registry.register(
        class_name,
        "nativeDragQueryFile",
        "([B)[Ljava/lang/String;",
        native_drag_query_file,
    );
    registry.register(
        class_name,
        "registerFormatWithPasteboard",
        "(Ljava/lang/String;)J",
        register_format_with_pasteboard,
    );
}

#[async_recursion(?Send)]
async fn format_for_index(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_drag_query_file(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn register_format_with_pasteboard(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

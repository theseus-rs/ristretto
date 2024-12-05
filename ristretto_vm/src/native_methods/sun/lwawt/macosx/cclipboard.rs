use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CClipboard`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CClipboard";
    registry.register(
        class_name,
        "checkPasteboardWithoutNotification",
        "()Z",
        check_pasteboard_without_notification,
    );
    registry.register(
        class_name,
        "declareTypes",
        "([JLsun/awt/datatransfer/SunClipboard;)V",
        declare_types,
    );
    registry.register(class_name, "getClipboardData", "(J)[B", get_clipboard_data);
    registry.register(
        class_name,
        "getClipboardFormats",
        "()[J",
        get_clipboard_formats,
    );
    registry.register(class_name, "setData", "([BJ)V", set_data);
}

#[async_recursion(?Send)]
async fn check_pasteboard_without_notification(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z")
}

#[async_recursion(?Send)]
async fn declare_types(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V")
}

#[async_recursion(?Send)]
async fn get_clipboard_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardData(J)[B")
}

#[async_recursion(?Send)]
async fn get_clipboard_formats(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardFormats()[J")
}

#[async_recursion(?Send)]
async fn set_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.setData([BJ)V")
}

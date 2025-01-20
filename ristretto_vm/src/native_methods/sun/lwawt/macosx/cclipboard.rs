use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CClipboard";

/// Register all native methods for `sun.lwawt.macosx.CClipboard`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "checkPasteboardWithoutNotification",
        "()Z",
        check_pasteboard_without_notification,
    );
    registry.register(
        CLASS_NAME,
        "declareTypes",
        "([JLsun/awt/datatransfer/SunClipboard;)V",
        declare_types,
    );
    registry.register(CLASS_NAME, "getClipboardData", "(J)[B", get_clipboard_data);
    registry.register(
        CLASS_NAME,
        "getClipboardFormats",
        "()[J",
        get_clipboard_formats,
    );
    registry.register(CLASS_NAME, "setData", "([BJ)V", set_data);
}

#[async_recursion(?Send)]
async fn check_pasteboard_without_notification(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z")
}

#[async_recursion(?Send)]
async fn declare_types(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V")
}

#[async_recursion(?Send)]
async fn get_clipboard_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardData(J)[B")
}

#[async_recursion(?Send)]
async fn get_clipboard_formats(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardFormats()[J")
}

#[async_recursion(?Send)]
async fn set_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.setData([BJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z"
    )]
    async fn test_check_pasteboard_without_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_pasteboard_without_notification(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V"
    )]
    async fn test_declare_types() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = declare_types(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.getClipboardData(J)[B"
    )]
    async fn test_get_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.getClipboardFormats()[J"
    )]
    async fn test_get_clipboard_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_formats(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CClipboard.setData([BJ)V")]
    async fn test_set_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_data(thread, Parameters::default()).await;
    }
}

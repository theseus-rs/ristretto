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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CClipboard";
        assert!(registry
            .method(class_name, "checkPasteboardWithoutNotification", "()Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "declareTypes",
                "([JLsun/awt/datatransfer/SunClipboard;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getClipboardData", "(J)[B")
            .is_some());
        assert!(registry
            .method(class_name, "getClipboardFormats", "()[J")
            .is_some());
        assert!(registry.method(class_name, "setData", "([BJ)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z")]
    async fn test_check_pasteboard_without_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_pasteboard_without_notification(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V"
    )]
    async fn test_declare_types() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = declare_types(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CClipboard.getClipboardData(J)[B")]
    async fn test_get_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CClipboard.getClipboardFormats()[J")]
    async fn test_get_clipboard_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_formats(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CClipboard.setData([BJ)V")]
    async fn test_set_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_data(thread, Arguments::default()).await;
    }
}

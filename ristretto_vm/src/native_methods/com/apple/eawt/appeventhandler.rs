use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eawt._AppEventHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppEventHandler";
    registry.register(
        class_name,
        "nativeOpenCocoaAboutWindow",
        "()V",
        native_open_cocoa_about_window,
    );
    registry.register(
        class_name,
        "nativeRegisterForNotification",
        "(I)V",
        native_register_for_notification,
    );
    registry.register(
        class_name,
        "nativeReplyToAppShouldTerminate",
        "(Z)V",
        native_reply_to_app_should_terminate,
    );
}

#[async_recursion(?Send)]
async fn native_open_cocoa_about_window(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V")
}

#[async_recursion(?Send)]
async fn native_register_for_notification(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V")
}

#[async_recursion(?Send)]
async fn native_reply_to_app_should_terminate(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/eawt/_AppEventHandler";
        assert!(registry
            .method(class_name, "nativeOpenCocoaAboutWindow", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeRegisterForNotification", "(I)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeReplyToAppShouldTerminate", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V"
    )]
    async fn test_native_open_cocoa_about_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_open_cocoa_about_window(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V"
    )]
    async fn test_native_register_for_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_register_for_notification(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V"
    )]
    async fn test_native_reply_to_app_should_terminate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_reply_to_app_should_terminate(thread, Arguments::default()).await;
    }
}

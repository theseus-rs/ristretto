use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eawt/_AppEventHandler";

/// Register all native methods for `com.apple.eawt._AppEventHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeOpenCocoaAboutWindow",
        "()V",
        native_open_cocoa_about_window,
    );
    registry.register(
        CLASS_NAME,
        "nativeRegisterForNotification",
        "(I)V",
        native_register_for_notification,
    );
    registry.register(
        CLASS_NAME,
        "nativeReplyToAppShouldTerminate",
        "(Z)V",
        native_reply_to_app_should_terminate,
    );
}

#[async_recursion(?Send)]
async fn native_open_cocoa_about_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V")
}

#[async_recursion(?Send)]
async fn native_register_for_notification(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V")
}

#[async_recursion(?Send)]
async fn native_reply_to_app_should_terminate(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V"
    )]
    async fn test_native_open_cocoa_about_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_open_cocoa_about_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V"
    )]
    async fn test_native_register_for_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_register_for_notification(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V"
    )]
    async fn test_native_reply_to_app_should_terminate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_reply_to_app_should_terminate(thread, Parameters::default()).await;
    }
}

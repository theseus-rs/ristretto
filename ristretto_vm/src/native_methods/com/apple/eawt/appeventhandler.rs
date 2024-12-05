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

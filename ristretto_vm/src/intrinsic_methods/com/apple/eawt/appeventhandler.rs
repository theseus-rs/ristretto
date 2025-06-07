use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/apple/eawt/_AppEventHandler.nativeOpenCocoaAboutWindow()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_open_cocoa_about_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V")
}

#[intrinsic_method(
    "com/apple/eawt/_AppEventHandler.nativeRegisterForNotification(I)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_register_for_notification(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V")
}

#[intrinsic_method(
    "com/apple/eawt/_AppEventHandler.nativeReplyToAppShouldTerminate(Z)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_reply_to_app_should_terminate(
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

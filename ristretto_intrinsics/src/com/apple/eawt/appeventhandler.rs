use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/apple/eawt/_AppEventHandler.nativeOpenCocoaAboutWindow()V", Any)]
#[async_method]
pub async fn native_open_cocoa_about_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eawt/_AppEventHandler.nativeRegisterForNotification(I)V",
    Any
)]
#[async_method]
pub async fn native_register_for_notification<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _notification = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eawt/_AppEventHandler.nativeReplyToAppShouldTerminate(Z)V",
    Any
)]
#[async_method]
pub async fn native_reply_to_app_should_terminate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _should_terminate = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_open_cocoa_about_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_open_cocoa_about_window(thread, Parameters::default()).await;
        assert_eq!(
            "com.apple.eawt._AppEventHandler.nativeOpenCocoaAboutWindow()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_register_for_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_register_for_notification(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.apple.eawt._AppEventHandler.nativeRegisterForNotification(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_reply_to_app_should_terminate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_reply_to_app_should_terminate(thread, Parameters::new(vec![Value::from(false)]))
                .await;
        assert_eq!(
            "com.apple.eawt._AppEventHandler.nativeReplyToAppShouldTerminate(Z)V",
            result.unwrap_err().to_string()
        );
    }
}

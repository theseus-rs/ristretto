use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/_AppMiscHandlers.nativeDisableSuddenTermination()V",
    Any
)]
#[async_method]
pub async fn native_disable_sudden_termination<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eawt/_AppMiscHandlers.nativeEnableSuddenTermination()V",
    Any
)]
#[async_method]
pub async fn native_enable_sudden_termination<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeOpenHelpViewer()V", Any)]
#[async_method]
pub async fn native_open_help_viewer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeRequestActivation(Z)V", Any)]
#[async_method]
pub async fn native_request_activation<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeRequestUserAttention(Z)V", Any)]
#[async_method]
pub async fn native_request_user_attention<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMiscHandlers.nativeRequestUserAttention(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_disable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_disable_sudden_termination(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_enable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_enable_sudden_termination(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_open_help_viewer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_open_help_viewer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_request_activation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_request_activation(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_request_user_attention() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_request_user_attention(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

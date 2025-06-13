use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/_AppMiscHandlers.nativeDisableSuddenTermination()V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_disable_sudden_termination(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V")
}

#[intrinsic_method(
    "com/apple/eawt/_AppMiscHandlers.nativeEnableSuddenTermination()V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_enable_sudden_termination(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V")
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeOpenHelpViewer()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_open_help_viewer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V")
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeRequestActivation(Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_request_activation(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V")
}

#[intrinsic_method("com/apple/eawt/_AppMiscHandlers.nativeRequestUserAttention(Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_request_user_attention(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeRequestUserAttention(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V"
    )]
    async fn test_native_disable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_disable_sudden_termination(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V"
    )]
    async fn test_native_enable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_enable_sudden_termination(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V"
    )]
    async fn test_native_open_help_viewer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_open_help_viewer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V"
    )]
    async fn test_native_request_activation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_request_activation(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeRequestUserAttention(Z)V"
    )]
    async fn test_native_request_user_attention() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_request_user_attention(thread, Parameters::default()).await;
    }
}

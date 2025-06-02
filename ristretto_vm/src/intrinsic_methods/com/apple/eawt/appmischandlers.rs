use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eawt/_AppMiscHandlers";

/// Register all intrinsic methods for `com.apple.eawt._AppMiscHandlers`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeDisableSuddenTermination",
        "()V",
        native_disable_sudden_termination,
    );
    registry.register(
        CLASS_NAME,
        "nativeEnableSuddenTermination",
        "()V",
        native_enable_sudden_termination,
    );
    registry.register(
        CLASS_NAME,
        "nativeOpenHelpViewer",
        "()V",
        native_open_help_viewer,
    );
    registry.register(
        CLASS_NAME,
        "nativeRequestActivation",
        "(Z)V",
        native_request_activation,
    );
    registry.register(
        CLASS_NAME,
        "nativeRequestUserAttention",
        "(Z)V",
        native_request_user_attention,
    );
}

#[async_recursion(?Send)]
async fn native_disable_sudden_termination(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V")
}

#[async_recursion(?Send)]
async fn native_enable_sudden_termination(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V")
}

#[async_recursion(?Send)]
async fn native_open_help_viewer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V")
}

#[async_recursion(?Send)]
async fn native_request_activation(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V")
}

#[async_recursion(?Send)]
async fn native_request_user_attention(
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

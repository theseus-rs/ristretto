use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eawt._AppMiscHandlers`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppMiscHandlers";
    registry.register(
        class_name,
        "nativeDisableSuddenTermination",
        "()V",
        native_disable_sudden_termination,
    );
    registry.register(
        class_name,
        "nativeEnableSuddenTermination",
        "()V",
        native_enable_sudden_termination,
    );
    registry.register(
        class_name,
        "nativeOpenHelpViewer",
        "()V",
        native_open_help_viewer,
    );
    registry.register(
        class_name,
        "nativeRequestActivation",
        "(Z)V",
        native_request_activation,
    );
    registry.register(
        class_name,
        "nativeRequestUserAttention",
        "(Z)V",
        native_request_user_attention,
    );
}

#[async_recursion(?Send)]
async fn native_disable_sudden_termination(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V")
}

#[async_recursion(?Send)]
async fn native_enable_sudden_termination(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V")
}

#[async_recursion(?Send)]
async fn native_open_help_viewer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V")
}

#[async_recursion(?Send)]
async fn native_request_activation(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V")
}

#[async_recursion(?Send)]
async fn native_request_user_attention(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMiscHandlers.nativeRequestUserAttention(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/eawt/_AppMiscHandlers";
        assert!(registry
            .method(class_name, "nativeDisableSuddenTermination", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeEnableSuddenTermination", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeOpenHelpViewer", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeRequestActivation", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeRequestUserAttention", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeDisableSuddenTermination()V"
    )]
    async fn test_native_disable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_disable_sudden_termination(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeEnableSuddenTermination()V"
    )]
    async fn test_native_enable_sudden_termination() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_enable_sudden_termination(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeOpenHelpViewer()V"
    )]
    async fn test_native_open_help_viewer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_open_help_viewer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeRequestActivation(Z)V"
    )]
    async fn test_native_request_activation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_request_activation(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMiscHandlers.nativeRequestUserAttention(Z)V"
    )]
    async fn test_native_request_user_attention() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_request_user_attention(thread, Arguments::default()).await;
    }
}

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
    todo!()
}

#[async_recursion(?Send)]
async fn native_enable_sudden_termination(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_open_help_viewer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_request_activation(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_request_user_attention(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

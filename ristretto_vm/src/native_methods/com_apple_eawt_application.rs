use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eawt.Application`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/Application";
    registry.register(
        class_name,
        "nativeInitializeApplicationDelegate",
        "()V",
        native_initialize_application_delegate,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_initialize_application_delegate(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

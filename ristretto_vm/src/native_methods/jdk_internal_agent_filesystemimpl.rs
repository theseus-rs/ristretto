use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.agent.FileSystemImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/agent/FileSystemImpl";
    registry.register(
        class_name,
        "isAccessUserOnly0",
        "(Ljava/lang/String;)Z",
        is_access_user_only_0,
    );
}

#[async_recursion(?Send)]
async fn is_access_user_only_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

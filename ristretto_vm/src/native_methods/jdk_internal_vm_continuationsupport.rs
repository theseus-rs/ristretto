use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.vm.ContinuationSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/vm/ContinuationSupport";
    registry.register(class_name, "isSupported0", "()Z", is_supported_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_supported_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

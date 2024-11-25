use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Signal";
    registry.register(
        class_name,
        "findSignal0",
        "(Ljava/lang/String;)I",
        find_signal_0,
    );
    registry.register(class_name, "handle0", "(IJ)J", handle_0);
    registry.register(class_name, "raise0", "(I)V", raise_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_signal_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn raise_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

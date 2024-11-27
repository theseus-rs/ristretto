use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Signal";
    registry.register(
        class_name,
        "findSignal",
        "(Ljava/lang/String;)I",
        find_signal,
    );
    registry.register(class_name, "handle0", "(IJ)J", handle_0);
    registry.register(class_name, "raise0", "(I)V", raise_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_signal(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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

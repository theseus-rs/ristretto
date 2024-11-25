use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ref.Reference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/Reference";
    registry.register(
        class_name,
        "getAndClearReferencePendingList",
        "()Ljava/lang/ref/Reference;",
        get_and_clear_reference_pending_list,
    );
    registry.register(
        class_name,
        "hasReferencePendingList",
        "()Z",
        has_reference_pending_list,
    );
    registry.register(
        class_name,
        "waitForReferencePendingList",
        "()V",
        wait_for_reference_pending_list,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_and_clear_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn has_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn wait_for_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

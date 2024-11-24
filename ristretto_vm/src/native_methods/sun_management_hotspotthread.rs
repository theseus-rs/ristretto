use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.HotspotThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/HotspotThread";
    registry.register(
        class_name,
        "getInternalThreadCount",
        "()I",
        get_internal_thread_count,
    );
    registry.register(
        class_name,
        "getInternalThreadTimes0",
        "([Ljava/lang/String;[J)I",
        get_internal_thread_times_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_internal_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_internal_thread_times_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

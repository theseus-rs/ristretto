use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.BsdNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/BsdNativeDispatcher";
    registry.register(class_name, "endfsstat", "(J)V", endfsstat);
    registry.register(
        class_name,
        "fsstatEntry",
        "(JLsun/nio/fs/UnixMountEntry;)I",
        fsstat_entry,
    );
    registry.register(class_name, "getfsstat", "()J", getfsstat);
    registry.register(class_name, "getmntonname0", "(J)[B", getmntonname_0);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn endfsstat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fsstat_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn getfsstat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn getmntonname_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

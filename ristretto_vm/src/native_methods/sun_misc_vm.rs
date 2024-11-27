use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.VM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/VM";
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(
        class_name,
        "latestUserDefinedLoader0",
        "()Ljava/lang/ClassLoader;",
        latest_user_defined_loader_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn latest_user_defined_loader_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

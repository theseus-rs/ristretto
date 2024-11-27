use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ProcessImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ProcessImpl";
    registry.register(
        class_name,
        "forkAndExec",
        "(I[B[B[BI[BI[B[IZ)I",
        fork_and_exec,
    );
    registry.register(class_name, "init", "()V", init);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fork_and_exec(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

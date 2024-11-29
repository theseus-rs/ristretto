use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StackStreamFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackStreamFactory";
    registry.register(
        class_name,
        "checkStackWalkModes",
        "()Z",
        check_stack_walk_modes,
    );
}

#[async_recursion(?Send)]
async fn check_stack_walk_modes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

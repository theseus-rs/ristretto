use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StackStreamFactory$AbstractStackWalker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
    registry.register(
        class_name,
        "callStackWalk",
        "(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
        call_stack_walk,
    );
    registry.register(
        class_name,
        "fetchStackFrames",
        "(JJII[Ljava/lang/Object;)I",
        fetch_stack_frames,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn call_stack_walk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fetch_stack_frames(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

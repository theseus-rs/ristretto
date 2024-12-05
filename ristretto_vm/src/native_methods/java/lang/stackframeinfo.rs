use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StackFrameInfo`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackFrameInfo";
    registry.register(
        class_name,
        "expandStackFrameInfo",
        "()V",
        expand_stack_frame_info,
    );
}

#[async_recursion(?Send)]
async fn expand_stack_frame_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.StackFrameInfo.expandStackFrameInfo()V")
}

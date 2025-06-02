use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StackFrameInfo";

/// Register all intrinsic methods for `java.lang.StackFrameInfo`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "expandStackFrameInfo",
        "()V",
        expand_stack_frame_info,
    );
}

#[async_recursion(?Send)]
async fn expand_stack_frame_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.StackFrameInfo.expandStackFrameInfo()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackFrameInfo.expandStackFrameInfo()V"
    )]
    async fn test_expand_stack_frame_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = expand_stack_frame_info(thread, Parameters::default()).await;
    }
}

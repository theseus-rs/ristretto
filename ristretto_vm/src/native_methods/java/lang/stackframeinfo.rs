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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/StackFrameInfo";
        assert!(registry
            .method(class_name, "expandStackFrameInfo", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackFrameInfo.expandStackFrameInfo()V"
    )]
    async fn test_expand_stack_frame_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = expand_stack_frame_info(thread, Arguments::default()).await;
    }
}

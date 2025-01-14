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
    todo!("java.lang.StackStreamFactory.checkStackWalkModes()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/StackStreamFactory";
        assert!(registry
            .method(class_name, "checkStackWalkModes", "()Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory.checkStackWalkModes()Z"
    )]
    async fn test_check_stack_walk_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_stack_walk_modes(thread, Arguments::default()).await;
    }
}

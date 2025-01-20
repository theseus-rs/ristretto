use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StackStreamFactory";

/// Register all native methods for `java.lang.StackStreamFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "checkStackWalkModes",
        "()Z",
        check_stack_walk_modes,
    );
}

#[async_recursion(?Send)]
async fn check_stack_walk_modes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory.checkStackWalkModes()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory.checkStackWalkModes()Z"
    )]
    async fn test_check_stack_walk_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_stack_walk_modes(thread, Parameters::default()).await;
    }
}

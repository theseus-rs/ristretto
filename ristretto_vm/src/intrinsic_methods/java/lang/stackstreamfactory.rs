use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackStreamFactory.checkStackWalkModes()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_stack_walk_modes(
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

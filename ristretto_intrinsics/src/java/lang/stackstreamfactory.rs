use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

/// Validates the stack walk mode flags. Always returns true as all modes are supported.
#[intrinsic_method(
    "java/lang/StackStreamFactory.checkStackWalkModes()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_stack_walk_modes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_stack_walk_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_stack_walk_modes(thread, Parameters::default())
            .await
            .expect("checkStackWalkModes should succeed");
        assert_eq!(Some(Value::Int(1)), result);
    }
}

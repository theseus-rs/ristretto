use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackStreamFactory.checkStackWalkModes()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_stack_walk_modes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_stack_walk_modes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_stack_walk_modes(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }
}

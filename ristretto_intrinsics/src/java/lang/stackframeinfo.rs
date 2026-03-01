use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackFrameInfo.expandStackFrameInfo()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn expand_stack_frame_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // No-op: fields are eagerly populated during callStackWalk
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_expand_stack_frame_info() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = expand_stack_frame_info(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}

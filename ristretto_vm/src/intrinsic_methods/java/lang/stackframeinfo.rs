use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackFrameInfo.expandStackFrameInfo()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn expand_stack_frame_info(
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

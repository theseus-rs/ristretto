use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/DefaultDisposerRecord.invokeNativeDispose(JJ)V", Any)]
#[async_method]
pub(crate) async fn invoke_native_dispose(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.DefaultDisposerRecord.invokeNativeDispose(JJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.DefaultDisposerRecord.invokeNativeDispose(JJ)V"
    )]
    async fn test_invoke_native_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_native_dispose(thread, Parameters::default()).await;
    }
}

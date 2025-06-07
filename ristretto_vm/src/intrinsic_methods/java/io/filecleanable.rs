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
    "java/io/FileCleanable.cleanupClose0(IJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn cleanup_close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileCleanable.cleanupClose0(IJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileCleanable.cleanupClose0(IJ)V")]
    async fn test_cleanup_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = cleanup_close_0(thread, Parameters::default()).await;
    }
}

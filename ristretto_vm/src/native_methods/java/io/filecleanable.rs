use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/FileCleanable";

/// Register all native methods for `java.io.FileCleanable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "cleanupClose0", "(IJ)V", cleanup_close_0);
}

#[async_recursion(?Send)]
async fn cleanup_close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

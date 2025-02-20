use crate::Result;
use crate::native_methods::registry::{JAVA_11, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/SocketCleanable";

/// Register all native methods for `java.net.SocketCleanable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 && registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "cleanupClose0", "(I)V", cleanup_close_0);
    }
}

#[async_recursion(?Send)]
async fn cleanup_close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.SocketCleanable.cleanupClose0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.SocketCleanable.cleanupClose0(I)V")]
    async fn test_cleanup_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = cleanup_close_0(thread, Parameters::default()).await;
    }
}

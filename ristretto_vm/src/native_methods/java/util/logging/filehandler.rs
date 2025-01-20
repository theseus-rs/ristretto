use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/logging/FileHandler";

/// Register all native methods for `java.util.logging.FileHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "isSetUID", "()Z", is_set_uid);
}

#[async_recursion(?Send)]
async fn is_set_uid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.logging.FileHandler.isSetUID()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.logging.FileHandler.isSetUID()Z")]
    async fn test_is_set_uid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_set_uid(thread, Parameters::default()).await;
    }
}

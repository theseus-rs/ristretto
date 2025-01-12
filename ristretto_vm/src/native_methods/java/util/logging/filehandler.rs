use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.logging.FileHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/logging/FileHandler";
    registry.register(class_name, "isSetUID", "()Z", is_set_uid);
}

#[async_recursion(?Send)]
async fn is_set_uid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.logging.FileHandler.isSetUID()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/util/logging/FileHandler";
        assert!(registry.method(class_name, "isSetUID", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.logging.FileHandler.isSetUID()Z")]
    async fn test_is_set_uid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_set_uid(thread, Arguments::default()).await;
    }
}

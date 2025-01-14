use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StringCoding`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StringCoding";
    registry.register(class_name, "err", "(Ljava/lang/String;)V", err);
}

#[async_recursion(?Send)]
async fn err(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StringCoding.err(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/StringCoding";
        assert!(registry
            .method(class_name, "err", "(Ljava/lang/String;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StringCoding.err(Ljava/lang/String;)V"
    )]
    async fn test_err() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = err(thread, Arguments::default()).await;
    }
}

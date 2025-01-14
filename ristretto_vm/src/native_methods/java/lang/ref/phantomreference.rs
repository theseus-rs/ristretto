use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ref.PhantomReference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/PhantomReference";
    registry.register(
        class_name,
        "refersTo0",
        "(Ljava/lang/Object;)Z",
        refers_to_0,
    );
}

#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/ref/PhantomReference";
        assert!(registry
            .method(class_name, "refersTo0", "(Ljava/lang/Object;)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z"
    )]
    async fn test_refers_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = refers_to_0(thread, Arguments::default()).await;
    }
}

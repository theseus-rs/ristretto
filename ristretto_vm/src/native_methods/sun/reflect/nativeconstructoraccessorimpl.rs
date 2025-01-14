use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.reflect.NativeConstructorAccessorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/NativeConstructorAccessorImpl";
    registry.register(
        class_name,
        "newInstance0",
        "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        new_instance_0,
    );
}

#[async_recursion(?Send)]
async fn new_instance_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.NativeConstructorAccessorImpl.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/reflect/NativeConstructorAccessorImpl";
        assert!(registry
            .method(
                class_name,
                "newInstance0",
                "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.reflect.NativeConstructorAccessorImpl.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_instance_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_instance_0(thread, Arguments::default()).await;
    }
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.reflect.NativeMethodAccessorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/NativeMethodAccessorImpl";
    registry.register(
        class_name,
        "invoke0",
        "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_0,
    );
}

#[async_recursion(?Send)]
async fn invoke_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/reflect/NativeMethodAccessorImpl";
        assert!(registry
            .method(class_name, "invoke0", "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.reflect.NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_0(thread, Arguments::default()).await;
    }
}

use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.reflect.Proxy`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Proxy";
    registry.register(
        class_name,
        "defineClass0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
        define_class_0,
    );
}

#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/reflect/Proxy";
        assert!(registry
            .method(
                class_name,
                "defineClass0",
                "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Arguments::default()).await;
    }
}

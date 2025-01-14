use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectStreamClass`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectStreamClass";
    registry.register(
        class_name,
        "hasStaticInitializer",
        "(Ljava/lang/Class;)Z",
        has_static_initializer,
    );
    registry.register(class_name, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn has_static_initializer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/io/ObjectStreamClass";
        assert!(registry
            .method(class_name, "hasStaticInitializer", "(Ljava/lang/Class;)Z")
            .is_some());
        assert!(registry.method(class_name, "initNative", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z"
    )]
    async fn test_has_static_initializer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = has_static_initializer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

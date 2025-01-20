use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/ObjectStreamClass";

/// Register all native methods for `java.io.ObjectStreamClass`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "hasStaticInitializer",
        "(Ljava/lang/Class;)Z",
        has_static_initializer,
    );
    registry.register(CLASS_NAME, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn has_static_initializer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z"
    )]
    async fn test_has_static_initializer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = has_static_initializer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

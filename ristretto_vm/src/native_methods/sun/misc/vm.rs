use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.VM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/VM";
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(
        class_name,
        "latestUserDefinedLoader0",
        "()Ljava/lang/ClassLoader;",
        latest_user_defined_loader_0,
    );
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn latest_user_defined_loader_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/VM";
        assert!(registry.method(class_name, "initialize", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "latestUserDefinedLoader0",
                "()Ljava/lang/ClassLoader;"
            )
            .is_some());
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = initialize(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;")]
    async fn test_latest_user_defined_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = latest_user_defined_loader_0(thread, Arguments::default()).await;
    }
}

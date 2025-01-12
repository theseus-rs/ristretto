use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eawt.Application`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/Application";
    registry.register(
        class_name,
        "nativeInitializeApplicationDelegate",
        "()V",
        native_initialize_application_delegate,
    );
}

#[async_recursion(?Send)]
async fn native_initialize_application_delegate(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/eawt/Application";
        assert!(registry
            .method(class_name, "nativeInitializeApplicationDelegate", "()V")
            .is_some());
    }

    #[tokio::test]
    async fn test_native_initialize_application_delegate() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_initialize_application_delegate(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

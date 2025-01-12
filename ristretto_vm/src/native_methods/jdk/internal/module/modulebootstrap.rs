use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.module.ModuleBootstrap`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/module/ModuleBootstrap";
    registry.register(class_name, "boot", "()Ljava/lang/ModuleLayer;", boot);
}

#[async_recursion(?Send)]
async fn boot(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: remove this method once the module system is implemented
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/module/ModuleBootstrap";
        assert!(registry
            .method(class_name, "boot", "()Ljava/lang/ModuleLayer;")
            .is_some());
    }

    #[tokio::test]
    async fn test_boot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = boot(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}

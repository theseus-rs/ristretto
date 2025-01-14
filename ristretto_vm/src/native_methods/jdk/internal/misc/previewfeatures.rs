use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.misc.PreviewFeatures`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/PreviewFeatures";
    registry.register(class_name, "isPreviewEnabled", "()Z", is_preview_enabled);
}

#[async_recursion(?Send)]
async fn is_preview_enabled(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let configuration = vm.configuration();
    let preview_features = configuration.preview_features();
    Ok(Some(Value::from(preview_features)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/misc/PreviewFeatures";
        assert!(registry
            .method(class_name, "isPreviewEnabled", "()Z")
            .is_some());
    }

    #[tokio::test]
    async fn test_is_preview_enabled() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = is_preview_enabled(thread, Arguments::default()).await?;
        assert_eq!(value, Some(Value::from(false)));
        Ok(())
    }
}

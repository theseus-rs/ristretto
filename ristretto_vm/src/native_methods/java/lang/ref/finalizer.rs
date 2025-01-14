use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ref.Finalizer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/Finalizer";
    registry.register(
        class_name,
        "isFinalizationEnabled",
        "()Z",
        is_finalization_enabled,
    );
    registry.register(
        class_name,
        "reportComplete",
        "(Ljava/lang/Object;)V",
        report_complete,
    );
}

#[async_recursion(?Send)]
async fn is_finalization_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn report_complete(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/ref/Finalizer";
        assert!(registry
            .method(class_name, "isFinalizationEnabled", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "reportComplete", "(Ljava/lang/Object;)V")
            .is_some());
    }

    #[tokio::test]
    async fn test_is_finalization_enabled() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_finalization_enabled(thread, Arguments::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_report_complete() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = report_complete(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

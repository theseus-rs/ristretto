use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ref/Finalizer";

/// Register all intrinsic methods for `java.lang.ref.Finalizer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "isFinalizationEnabled",
        "()Z",
        is_finalization_enabled,
    );
    registry.register(
        CLASS_NAME,
        "reportComplete",
        "(Ljava/lang/Object;)V",
        report_complete,
    );
}

#[async_recursion(?Send)]
async fn is_finalization_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn report_complete(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_finalization_enabled() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_finalization_enabled(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_report_complete() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = report_complete(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

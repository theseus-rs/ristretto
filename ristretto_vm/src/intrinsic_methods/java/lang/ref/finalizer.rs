use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ref/Finalizer.isFinalizationEnabled()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn is_finalization_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "java/lang/ref/Finalizer.reportComplete(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn report_complete(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

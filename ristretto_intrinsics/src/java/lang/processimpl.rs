use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fork_and_exec<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("java/lang/ProcessImpl.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fork_and_exec() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fork_and_exec(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

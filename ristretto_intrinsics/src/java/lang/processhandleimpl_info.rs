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
    "java/lang/ProcessHandleImpl$Info.info0(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn info_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.lang.ProcessHandleImpl$Info.info0(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl$Info.initIDs()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = info_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}

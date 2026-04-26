use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/misc/VM.initialize()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/misc/VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn latest_user_defined_loader_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_latest_user_defined_loader_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = latest_user_defined_loader_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;",
            result.unwrap_err().to_string()
        );
    }
}

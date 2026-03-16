use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/BsdFileSystem.directCopy0(IIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn direct_copy_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.fs.BsdFileSystem.directCopy0(IJJ)I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_direct_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = direct_copy_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

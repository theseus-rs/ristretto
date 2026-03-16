use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/UnixCopyFile.transfer(IIJ)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn transfer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.fs.UnixCopyFile.transfer(IIJ)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = transfer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.chmod(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn chmod<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.FileSystemPreferences.chmod(Ljava/lang/String;I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I",
    Any
)]
#[async_method]
pub async fn lock_file_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I".to_string(),
    )
    .into())
}

#[intrinsic_method("java/util/prefs/FileSystemPreferences.unlockFile0(I)I", Any)]
#[async_method]
pub async fn unlock_file_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.FileSystemPreferences.unlockFile0(I)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chmod() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = chmod(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_file_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unlock_file_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

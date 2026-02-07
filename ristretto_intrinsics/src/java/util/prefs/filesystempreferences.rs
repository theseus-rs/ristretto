use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.chmod(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn chmod<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.chmod(Ljava/lang/String;I)I")
}

#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I",
    Any
)]
#[async_method]
pub async fn lock_file_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I")
}

#[intrinsic_method("java/util/prefs/FileSystemPreferences.unlockFile0(I)I", Any)]
#[async_method]
pub async fn unlock_file_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.unlockFile0(I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.chmod(Ljava/lang/String;I)I"
    )]
    async fn test_chmod() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = chmod(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I"
    )]
    async fn test_lock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lock_file_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.unlockFile0(I)I"
    )]
    async fn test_unlock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unlock_file_0(thread, Parameters::default()).await;
    }
}

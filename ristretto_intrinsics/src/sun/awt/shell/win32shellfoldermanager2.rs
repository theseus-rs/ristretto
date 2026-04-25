use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/shell/Win32ShellFolderManager2.initializeCom()V", Any)]
#[async_method]
pub async fn initialize_com<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolderManager2.initializeCom()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolderManager2.uninitializeCom()V", Any)]
#[async_method]
pub async fn uninitialize_com<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolderManager2.uninitializeCom()V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_com() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_com(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolderManager2.initializeCom()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_uninitialize_com() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = uninitialize_com(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolderManager2.uninitializeCom()V",
            result.unwrap_err().to_string()
        );
    }
}

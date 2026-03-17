use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_display_name<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/laf/AquaFileView.getNativeLSInfo([BZ)I", Any)]
#[async_method]
pub async fn get_native_ls_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.AquaFileView.getNativeLSInfo([BZ)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativeMachineName()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_machine_name<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.AquaFileView.getNativeMachineName()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_path_for_resolved_alias<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativePathToSharedJDKBundle()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_path_to_shared_jdk_bundle<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.AquaFileView.getNativePathToSharedJDKBundle()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_native_display_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_display_name(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_ls_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_ls_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_machine_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_machine_name(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_path_for_resolved_alias() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_path_for_resolved_alias(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_native_path_to_shared_jdk_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_path_to_shared_jdk_bundle(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

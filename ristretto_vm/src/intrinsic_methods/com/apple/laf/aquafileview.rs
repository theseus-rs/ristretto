use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_native_display_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;")
}

#[intrinsic_method("com/apple/laf/AquaFileView.getNativeLSInfo([BZ)I", Any)]
#[async_method]
pub(crate) async fn get_native_ls_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeLSInfo([BZ)I")
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativeMachineName()Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_native_machine_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeMachineName()Ljava/lang/String;")
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_native_path_for_resolved_alias(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;")
}

#[intrinsic_method(
    "com/apple/laf/AquaFileView.getNativePathToSharedJDKBundle()Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_native_path_to_shared_jdk_bundle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativePathToSharedJDKBundle()Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;"
    )]
    async fn test_get_native_display_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_display_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaFileView.getNativeLSInfo([BZ)I"
    )]
    async fn test_get_native_ls_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_ls_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaFileView.getNativeMachineName()Ljava/lang/String;"
    )]
    async fn test_get_native_machine_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_machine_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;"
    )]
    async fn test_get_native_path_for_resolved_alias() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_path_for_resolved_alias(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaFileView.getNativePathToSharedJDKBundle()Ljava/lang/String;"
    )]
    async fn test_get_native_path_to_shared_jdk_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_path_to_shared_jdk_bundle(thread, Parameters::default()).await;
    }
}

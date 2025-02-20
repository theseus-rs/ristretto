use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/laf/AquaFileView";

/// Register all native methods for `com.apple.laf.AquaFileView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getNativeDisplayName",
        "([BZ)Ljava/lang/String;",
        get_native_display_name,
    );
    registry.register(CLASS_NAME, "getNativeLSInfo", "([BZ)I", get_native_ls_info);
    registry.register(
        CLASS_NAME,
        "getNativeMachineName",
        "()Ljava/lang/String;",
        get_native_machine_name,
    );
    registry.register(
        CLASS_NAME,
        "getNativePathForResolvedAlias",
        "([BZ)Ljava/lang/String;",
        get_native_path_for_resolved_alias,
    );
    registry.register(
        CLASS_NAME,
        "getNativePathToSharedJDKBundle",
        "()Ljava/lang/String;",
        get_native_path_to_shared_jdk_bundle,
    );
}

#[async_recursion(?Send)]
async fn get_native_display_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeDisplayName([BZ)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_native_ls_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeLSInfo([BZ)I")
}

#[async_recursion(?Send)]
async fn get_native_machine_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativeMachineName()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_native_path_for_resolved_alias(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaFileView.getNativePathForResolvedAlias([BZ)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_native_path_to_shared_jdk_bundle(
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

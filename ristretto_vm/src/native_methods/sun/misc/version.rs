use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/Version";

/// Register all native methods for `sun.misc.Version`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getJdkSpecialVersion",
        "()Ljava/lang/String;",
        get_jdk_special_version,
    );
    registry.register(CLASS_NAME, "getJdkVersionInfo", "()V", get_jdk_version_info);
    registry.register(
        CLASS_NAME,
        "getJvmSpecialVersion",
        "()Ljava/lang/String;",
        get_jvm_special_version,
    );
    registry.register(CLASS_NAME, "getJvmVersionInfo", "()Z", get_jvm_version_info);
}

#[async_recursion(?Send)]
async fn get_jdk_special_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_jdk_version_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkVersionInfo()V")
}

#[async_recursion(?Send)]
async fn get_jvm_special_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_jvm_version_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJvmVersionInfo()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;"
    )]
    async fn test_get_jdk_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jdk_special_version(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Version.getJdkVersionInfo()V")]
    async fn test_get_jdk_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jdk_version_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;"
    )]
    async fn test_get_jvm_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jvm_special_version(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Version.getJvmVersionInfo()Z")]
    async fn test_get_jvm_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jvm_version_info(thread, Parameters::default()).await;
    }
}

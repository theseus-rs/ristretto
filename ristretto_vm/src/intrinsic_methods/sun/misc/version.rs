use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/Version.getJdkSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_jdk_special_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;")
}

#[intrinsic_method("sun/misc/Version.getJdkVersionInfo()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn get_jdk_version_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkVersionInfo()V")
}

#[intrinsic_method(
    "sun/misc/Version.getJvmSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_jvm_special_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;")
}

#[intrinsic_method("sun/misc/Version.getJvmVersionInfo()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn get_jvm_version_info(
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

use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/Version.getJdkSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_jdk_special_version<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/misc/Version.getJdkVersionInfo()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_jdk_version_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.misc.Version.getJdkVersionInfo()V".to_string()).into())
}

#[intrinsic_method(
    "sun/misc/Version.getJvmSpecialVersion()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_jvm_special_version<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/misc/Version.getJvmVersionInfo()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_jvm_version_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.misc.Version.getJvmVersionInfo()Z".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_jdk_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_jdk_special_version(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_jdk_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_jdk_version_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_jvm_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_jvm_special_version(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_jvm_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_jvm_version_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

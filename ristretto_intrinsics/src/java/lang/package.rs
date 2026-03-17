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
    "java/lang/Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_system_package_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/lang/Package.getSystemPackages0()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_system_packages_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.Package.getSystemPackages0()[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_system_package_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_package_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_system_packages_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_packages_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

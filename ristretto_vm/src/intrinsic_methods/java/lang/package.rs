use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_package_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;")
}

#[intrinsic_method(
    "java/lang/Package.getSystemPackages0()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_packages_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Package.getSystemPackages0()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_package_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Package.getSystemPackages0()[Ljava/lang/String;"
    )]
    async fn test_get_system_packages_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_packages_0(thread, Parameters::default()).await;
    }
}

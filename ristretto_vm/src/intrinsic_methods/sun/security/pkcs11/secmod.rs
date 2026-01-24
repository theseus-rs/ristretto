use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssGetLibraryHandle(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn nss_get_library_handle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn nss_get_module_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn nss_initialize(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z")
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssLoadLibrary(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn nss_load_library(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssVersionCheck(JLjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn nss_version_check(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J"
    )]
    async fn test_nss_get_library_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_get_library_handle(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;"
    )]
    async fn test_nss_get_module_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_get_module_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z"
    )]
    async fn test_nss_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_initialize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J"
    )]
    async fn test_nss_load_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_load_library(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z"
    )]
    async fn test_nss_version_check() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_version_check(thread, Parameters::default()).await;
    }
}

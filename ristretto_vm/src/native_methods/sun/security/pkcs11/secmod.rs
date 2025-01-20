use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/pkcs11/Secmod";

/// Register all native methods for `sun.security.pkcs11.Secmod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nssGetLibraryHandle",
        "(Ljava/lang/String;)J",
        nss_get_library_handle,
    );
    registry.register(
        CLASS_NAME,
        "nssGetModuleList",
        "(JLjava/lang/String;)Ljava/lang/Object;",
        nss_get_module_list,
    );
    registry.register(
        CLASS_NAME,
        "nssInitialize",
        "(Ljava/lang/String;JLjava/lang/String;Z)Z",
        nss_initialize,
    );
    registry.register(
        CLASS_NAME,
        "nssLoadLibrary",
        "(Ljava/lang/String;)J",
        nss_load_library,
    );
    registry.register(
        CLASS_NAME,
        "nssVersionCheck",
        "(JLjava/lang/String;)Z",
        nss_version_check,
    );
}

#[async_recursion(?Send)]
async fn nss_get_library_handle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn nss_get_module_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn nss_initialize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z")
}

#[async_recursion(?Send)]
async fn nss_load_library(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn nss_version_check(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

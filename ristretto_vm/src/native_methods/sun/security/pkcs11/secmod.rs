use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.pkcs11.Secmod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/pkcs11/Secmod";
    registry.register(
        class_name,
        "nssGetLibraryHandle",
        "(Ljava/lang/String;)J",
        nss_get_library_handle,
    );
    registry.register(
        class_name,
        "nssGetModuleList",
        "(JLjava/lang/String;)Ljava/lang/Object;",
        nss_get_module_list,
    );
    registry.register(
        class_name,
        "nssInitialize",
        "(Ljava/lang/String;JLjava/lang/String;Z)Z",
        nss_initialize,
    );
    registry.register(
        class_name,
        "nssLoadLibrary",
        "(Ljava/lang/String;)J",
        nss_load_library,
    );
    registry.register(
        class_name,
        "nssVersionCheck",
        "(JLjava/lang/String;)Z",
        nss_version_check,
    );
}

#[async_recursion(?Send)]
async fn nss_get_library_handle(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn nss_get_module_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn nss_initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z")
}

#[async_recursion(?Send)]
async fn nss_load_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn nss_version_check(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/pkcs11/Secmod";
        assert!(registry
            .method(class_name, "nssGetLibraryHandle", "(Ljava/lang/String;)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nssGetModuleList",
                "(JLjava/lang/String;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nssInitialize",
                "(Ljava/lang/String;JLjava/lang/String;Z)Z"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nssLoadLibrary", "(Ljava/lang/String;)J")
            .is_some());
        assert!(registry
            .method(class_name, "nssVersionCheck", "(JLjava/lang/String;)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J"
    )]
    async fn test_nss_get_library_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_get_library_handle(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;"
    )]
    async fn test_nss_get_module_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_get_module_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z"
    )]
    async fn test_nss_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_initialize(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J")]
    async fn test_nss_load_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_load_library(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z")]
    async fn test_nss_version_check() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = nss_version_check(thread, Arguments::default()).await;
    }
}

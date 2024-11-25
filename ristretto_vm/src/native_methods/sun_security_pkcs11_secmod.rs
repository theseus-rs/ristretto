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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nss_get_library_handle(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nss_get_module_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nss_initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nss_load_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nss_version_check(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

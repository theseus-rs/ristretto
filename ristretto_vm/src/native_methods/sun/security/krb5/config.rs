use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.krb5.Config`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/krb5/Config";
    registry.register(
        class_name,
        "getWindowsDirectory",
        "(Z)Ljava/lang/String;",
        get_windows_directory,
    );
}

#[async_recursion(?Send)]
async fn get_windows_directory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

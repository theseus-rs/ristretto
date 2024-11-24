use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.krb5.SCDynamicStoreConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/krb5/SCDynamicStoreConfig";
    registry.register(
        class_name,
        "getKerberosConfig",
        "()Ljava/util/Hashtable;",
        get_kerberos_config,
    );
    registry.register(
        class_name,
        "installNotificationCallback",
        "()V",
        install_notification_callback,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_kerberos_config(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn install_notification_callback(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

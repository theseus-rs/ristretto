use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.krb5.Credentials`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/krb5/Credentials";
    registry.register(
        class_name,
        "acquireDefaultNativeCreds",
        "([I)Lsun/security/krb5/Credentials;",
        acquire_default_native_creds,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn acquire_default_native_creds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

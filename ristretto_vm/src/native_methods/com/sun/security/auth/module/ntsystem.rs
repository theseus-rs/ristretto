use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.security.auth.module.NTSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/security/auth/module/NTSystem";
    registry.register(class_name, "getCurrent", "(Z)V", get_current);
    registry.register(
        class_name,
        "getImpersonationToken0",
        "()J",
        get_impersonation_token_0,
    );
}

#[async_recursion(?Send)]
async fn get_current(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_impersonation_token_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

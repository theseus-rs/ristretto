use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/security/auth/module/NTSystem";

/// Register all native methods for `com.sun.security.auth.module.NTSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getCurrent", "(Z)V", get_current);
    registry.register(
        CLASS_NAME,
        "getImpersonationToken0",
        "()J",
        get_impersonation_token_0,
    );
}

#[async_recursion(?Send)]
async fn get_current(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.NTSystem.getCurrent(Z)V")
}

#[async_recursion(?Send)]
async fn get_impersonation_token_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.NTSystem.getImpersonationToken0()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getCurrent(Z)V"
    )]
    async fn test_get_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_current(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getImpersonationToken0()J"
    )]
    async fn test_get_impersonation_token_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_impersonation_token_0(thread, Parameters::default()).await;
    }
}

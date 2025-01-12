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
    todo!("com.sun.security.auth.module.NTSystem.getCurrent(Z)V")
}

#[async_recursion(?Send)]
async fn get_impersonation_token_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.NTSystem.getImpersonationToken0()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/security/auth/module/NTSystem";
        assert!(registry.method(class_name, "getCurrent", "(Z)V").is_some());
        assert!(registry
            .method(class_name, "getImpersonationToken0", "()J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getCurrent(Z)V"
    )]
    async fn test_get_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_current(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getImpersonationToken0()J"
    )]
    async fn test_get_impersonation_token_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_impersonation_token_0(thread, Arguments::default()).await;
    }
}

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

#[async_recursion(?Send)]
async fn acquire_default_native_creds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/krb5/Credentials";
        assert!(registry
            .method(
                class_name,
                "acquireDefaultNativeCreds",
                "([I)Lsun/security/krb5/Credentials;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.krb5.Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;"
    )]
    async fn test_acquire_default_native_creds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = acquire_default_native_creds(thread, Arguments::default()).await;
    }
}

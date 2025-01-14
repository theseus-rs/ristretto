use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.ec.ECDHKeyAgreement`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/ec/ECDHKeyAgreement";
    registry.register(class_name, "deriveKey", "([B[B[B)[B", derive_key);
}

#[async_recursion(?Send)]
async fn derive_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/ec/ECDHKeyAgreement";
        assert!(registry
            .method(class_name, "deriveKey", "([B[B[B)[B")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B")]
    async fn test_derive_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = derive_key(thread, Arguments::default()).await;
    }
}

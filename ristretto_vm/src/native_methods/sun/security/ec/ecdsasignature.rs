use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.ec.ECDSASignature`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/ec/ECDSASignature";
    registry.register(class_name, "signDigest", "([B[B[B[BI)[B", sign_digest);
    registry.register(
        class_name,
        "verifySignedDigest",
        "([B[B[B[B)Z",
        verify_signed_digest,
    );
}

#[async_recursion(?Send)]
async fn sign_digest(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B")
}

#[async_recursion(?Send)]
async fn verify_signed_digest(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/ec/ECDSASignature";
        assert!(registry
            .method(class_name, "signDigest", "([B[B[B[BI)[B")
            .is_some());
        assert!(registry
            .method(class_name, "verifySignedDigest", "([B[B[B[B)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B")]
    async fn test_sign_digest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sign_digest(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z")]
    async fn test_verify_signed_digest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = verify_signed_digest(thread, Arguments::default()).await;
    }
}

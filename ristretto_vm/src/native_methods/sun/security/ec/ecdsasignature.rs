use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/ec/ECDSASignature";

/// Register all native methods for `sun.security.ec.ECDSASignature`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "signDigest", "([B[B[B[BI)[B", sign_digest);
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B"
    )]
    async fn test_sign_digest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sign_digest(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z"
    )]
    async fn test_verify_signed_digest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = verify_signed_digest(thread, Arguments::default()).await;
    }
}

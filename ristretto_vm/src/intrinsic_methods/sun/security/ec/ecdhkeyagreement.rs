use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/ec/ECDHKeyAgreement";

/// Register all intrinsic methods for `sun.security.ec.ECDHKeyAgreement`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "deriveKey", "([B[B[B)[B", derive_key);
}

#[async_recursion(?Send)]
async fn derive_key(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B"
    )]
    async fn test_derive_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = derive_key(thread, Parameters::default()).await;
    }
}

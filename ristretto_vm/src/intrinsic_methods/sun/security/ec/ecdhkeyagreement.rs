use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/security/ec/ECDHKeyAgreement.deriveKey([B[B[B)[B", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn derive_key(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

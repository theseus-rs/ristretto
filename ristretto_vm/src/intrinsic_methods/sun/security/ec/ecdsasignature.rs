use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/ec/ECDSASignature.signDigest([B[B[B[BI)[B",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn sign_digest(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B")
}

#[intrinsic_method(
    "sun/security/ec/ECDSASignature.verifySignedDigest([B[B[B[B)Z",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn verify_signed_digest(
    _thread: Arc<Thread>,
    _parameters: Parameters,
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
        let _ = sign_digest(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z"
    )]
    async fn test_verify_signed_digest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = verify_signed_digest(thread, Parameters::default()).await;
    }
}

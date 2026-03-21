use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/ec/ECDSASignature.signDigest([B[B[B[BI)[B",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn sign_digest<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/ec/ECDSASignature.verifySignedDigest([B[B[B[B)Z",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn verify_signed_digest<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_digest() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = sign_digest(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_signed_digest() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = verify_signed_digest(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

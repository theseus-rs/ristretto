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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timing = parameters.pop_int()?;
    let _seed = parameters.pop_reference()?;
    let _encoded_params = parameters.pop_reference()?;
    let _s = parameters.pop_reference()?;
    let _digest = parameters.pop_reference()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _encoded_params = parameters.pop_reference()?;
    let _w = parameters.pop_reference()?;
    let _digest = parameters.pop_reference()?;
    let _signature = parameters.pop_reference()?;
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
        let result = sign_digest(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.ec.ECDSASignature.signDigest([B[B[B[BI)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_verify_signed_digest() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = verify_signed_digest(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.ec.ECDSASignature.verifySignedDigest([B[B[B[B)Z",
            result.unwrap_err().to_string()
        );
    }
}

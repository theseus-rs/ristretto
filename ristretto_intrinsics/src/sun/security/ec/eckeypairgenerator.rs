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
    "sun/security/ec/ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn generate_ec_key_pair<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/ec/ECKeyPairGenerator.isCurveSupported([B)Z",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn is_curve_supported<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.ec.ECKeyPairGenerator.isCurveSupported([B)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_ec_key_pair() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = generate_ec_key_pair(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_curve_supported() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = is_curve_supported(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}

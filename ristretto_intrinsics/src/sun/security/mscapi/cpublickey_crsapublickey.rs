use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/mscapi/CPublicKey$CRSAPublicKey.getExponent([B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_exponent<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_key_blob = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CPublicKey$CRSAPublicKey.getExponent([B)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CPublicKey$CRSAPublicKey.getModulus([B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_modulus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_key_blob = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CPublicKey$CRSAPublicKey.getModulus([B)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_exponent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_exponent(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/security/mscapi/CPublicKey$CRSAPublicKey.getExponent([B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_modulus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_modulus(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/security/mscapi/CPublicKey$CRSAPublicKey.getModulus([B)[B",
            result.unwrap_err().to_string()
        );
    }
}

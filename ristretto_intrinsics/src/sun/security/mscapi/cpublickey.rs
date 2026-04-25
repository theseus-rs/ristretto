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
    "sun/security/mscapi/CPublicKey.getPublicKeyBlob(JJ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_public_key_blob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CPublicKey.getPublicKeyBlob(JJ)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_public_key_blob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_public_key_blob(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CPublicKey.getPublicKeyBlob(JJ)[B",
            result.unwrap_err().to_string()
        );
    }
}

use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/mscapi/CRSACipher.cngEncryptDecrypt([I[BIJZ)[B",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn cng_encrypt_decrypt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _do_encrypt = parameters.pop_bool()?;
    let _h_key = parameters.pop_long()?;
    let _j_data_size = parameters.pop_int()?;
    let _j_data = parameters.pop_reference()?;
    let _j_result_status = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CRSACipher.cngEncryptDecrypt([I[BIJZ)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CRSACipher.encryptDecrypt([I[BIJZ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn encrypt_decrypt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _do_encrypt = parameters.pop_bool()?;
    let _h_key = parameters.pop_long()?;
    let _j_data_size = parameters.pop_int()?;
    let _j_data = parameters.pop_reference()?;
    let _j_result_status = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CRSACipher.encryptDecrypt([I[BIJZ)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_cng_encrypt_decrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cng_encrypt_decrypt(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CRSACipher.cngEncryptDecrypt([I[BIJZ)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_encrypt_decrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = encrypt_decrypt(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CRSACipher.encryptDecrypt([I[BIJZ)[B",
            result.unwrap_err().to_string()
        );
    }
}

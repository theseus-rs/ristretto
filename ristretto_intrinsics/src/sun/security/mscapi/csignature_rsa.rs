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
    "sun/security/mscapi/CSignature$RSA.generatePublicKeyBlob(I[B[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_public_key_blob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_public_exponent = parameters.pop_reference()?;
    let _j_modulus = parameters.pop_reference()?;
    let _j_key_bit_length = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CSignature$RSA.generatePublicKeyBlob(I[B[B)[B".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_public_key_blob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_public_key_blob(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature$RSA.generatePublicKeyBlob(I[B[B)[B",
            result.unwrap_err().to_string()
        );
    }
}

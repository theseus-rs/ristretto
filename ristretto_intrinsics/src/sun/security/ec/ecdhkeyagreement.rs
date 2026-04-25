use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/security/ec/ECDHKeyAgreement.deriveKey([B[B[B)[B", Equal(JAVA_11))]
#[async_method]
pub async fn derive_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _encoded_params = parameters.pop_reference()?;
    let _w = parameters.pop_reference()?;
    let _s = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_derive_key() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = derive_key(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.ec.ECDHKeyAgreement.deriveKey([B[B[B)[B",
            result.unwrap_err().to_string()
        );
    }
}

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
    "sun/security/mscapi/CKeyPairGenerator$RSA.generateCKeyPair(Ljava/lang/String;ILjava/lang/String;)Lsun/security/mscapi/CKeyPair;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_ckey_pair<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key_container_name = parameters.pop_reference()?;
    let _key_size = parameters.pop_int()?;
    let _alg = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CKeyPairGenerator$RSA.generateCKeyPair(Ljava/lang/String;ILjava/lang/String;)Lsun/security/mscapi/CKeyPair;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_ckey_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_ckey_pair(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyPairGenerator$RSA.generateCKeyPair(Ljava/lang/String;ILjava/lang/String;)Lsun/security/mscapi/CKeyPair;",
            result.unwrap_err().to_string()
        );
    }
}

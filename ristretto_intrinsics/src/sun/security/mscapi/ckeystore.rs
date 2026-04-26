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
    "sun/security/mscapi/CKeyStore.destroyKeyContainer(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn destroy_key_container<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key_container_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKeyStore.destroyKeyContainer(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.generateRSAPrivateKeyBlob(I[B[B[B[B[B[B[B[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_rsaprivate_key_blob<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_crt_coefficient = parameters.pop_reference()?;
    let _j_exponent_q = parameters.pop_reference()?;
    let _j_exponent_p = parameters.pop_reference()?;
    let _j_prime_q = parameters.pop_reference()?;
    let _j_prime_p = parameters.pop_reference()?;
    let _j_private_exponent = parameters.pop_reference()?;
    let _j_public_exponent = parameters.pop_reference()?;
    let _j_modulus = parameters.pop_reference()?;
    let _j_key_bit_length = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKeyStore.generateRSAPrivateKeyBlob(I[B[B[B[B[B[B[B[B)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.loadKeysOrCertificateChains(Ljava/lang/String;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn load_keys_or_certificate_chains<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_cert_store_location = parameters.pop_int()?;
    let _j_cert_store_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKeyStore.loadKeysOrCertificateChains(Ljava/lang/String;I)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.removeCertificate(Ljava/lang/String;Ljava/lang/String;[BI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn remove_certificate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_cert_encoding_size = parameters.pop_int()?;
    let _j_cert_encoding = parameters.pop_reference()?;
    let _j_cert_alias_name = parameters.pop_reference()?;
    let _j_cert_store_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKeyStore.removeCertificate(Ljava/lang/String;Ljava/lang/String;[BI)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.removeCngKey(J)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn remove_cng_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _k = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKeyStore.removeCngKey(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.storeCertificate(Ljava/lang/String;Ljava/lang/String;[BIJJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn store_certificate<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    let _j_cert_encoding_size = parameters.pop_int()?;
    let _j_cert_encoding = parameters.pop_reference()?;
    let _j_cert_alias_name = parameters.pop_reference()?;
    let _j_cert_store_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CKeyStore.storeCertificate(Ljava/lang/String;Ljava/lang/String;[BIJJ)V".to_string()).into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKeyStore.storePrivateKey(Ljava/lang/String;[BLjava/lang/String;I)Lsun/security/mscapi/CPrivateKey;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn store_private_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key_size = parameters.pop_int()?;
    let _key_container_name = parameters.pop_reference()?;
    let _key_blob = parameters.pop_reference()?;
    let _alg = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CKeyStore.storePrivateKey(Ljava/lang/String;[BLjava/lang/String;I)Lsun/security/mscapi/CPrivateKey;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_destroy_key_container() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            destroy_key_container(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.destroyKeyContainer(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_rsaprivate_key_blob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_rsaprivate_key_blob(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.generateRSAPrivateKeyBlob(I[B[B[B[B[B[B[B[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_load_keys_or_certificate_chains() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_keys_or_certificate_chains(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.loadKeysOrCertificateChains(Ljava/lang/String;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_certificate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_certificate(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.removeCertificate(Ljava/lang/String;Ljava/lang/String;[BI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_cng_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_cng_key(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.removeCngKey(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_store_certificate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = store_certificate(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.storeCertificate(Ljava/lang/String;Ljava/lang/String;[BIJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_store_private_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = store_private_key(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKeyStore.storePrivateKey(Ljava/lang/String;[BLjava/lang/String;I)Lsun/security/mscapi/CPrivateKey;",
            result.unwrap_err().to_string()
        );
    }
}

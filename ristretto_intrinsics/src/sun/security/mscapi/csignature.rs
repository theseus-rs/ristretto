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
    "sun/security/mscapi/CSignature.importECPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn import_ecpublic_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key_size = parameters.pop_int()?;
    let _key_blob = parameters.pop_reference()?;
    let _alg = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CSignature.importECPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;".to_string()).into())
}
#[intrinsic_method(
    "sun/security/mscapi/CSignature.importPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn import_public_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key_size = parameters.pop_int()?;
    let _key_blob = parameters.pop_reference()?;
    let _alg = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CSignature.importPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;".to_string()).into())
}
#[intrinsic_method(
    "sun/security/mscapi/CSignature.signCngHash(I[BIILjava/lang/String;JJ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn sign_cng_hash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    let _j_hash_algorithm = parameters.pop_reference()?;
    let _salt_len = parameters.pop_int()?;
    let _j_hash_size = parameters.pop_int()?;
    let _j_hash = parameters.pop_reference()?;
    let _type_ = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CSignature.signCngHash(I[BIILjava/lang/String;JJ)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CSignature.signHash(Z[BILjava/lang/String;JJ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn sign_hash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    let _j_hash_algorithm = parameters.pop_reference()?;
    let _j_hash_size = parameters.pop_int()?;
    let _j_hash = parameters.pop_reference()?;
    let _no_hash_oid = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CSignature.signHash(Z[BILjava/lang/String;JJ)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CSignature.verifyCngSignedHash(I[BI[BIILjava/lang/String;JJ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn verify_cng_signed_hash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    let _j_hash_algorithm = parameters.pop_reference()?;
    let _salt_len = parameters.pop_int()?;
    let _j_signed_hash_size = parameters.pop_int()?;
    let _j_signed_hash = parameters.pop_reference()?;
    let _j_hash_size = parameters.pop_int()?;
    let _j_hash = parameters.pop_reference()?;
    let _type_ = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CSignature.verifyCngSignedHash(I[BI[BIILjava/lang/String;JJ)Z"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CSignature.verifySignedHash([BILjava/lang/String;[BIJJ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn verify_signed_hash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    let _j_signed_hash_size = parameters.pop_int()?;
    let _j_signed_hash = parameters.pop_reference()?;
    let _j_hash_algorithm = parameters.pop_reference()?;
    let _j_hash_size = parameters.pop_int()?;
    let _j_hash = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CSignature.verifySignedHash([BILjava/lang/String;[BIJJ)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_import_ecpublic_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = import_ecpublic_key(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.importECPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_import_public_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = import_public_key(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.importPublicKey(Ljava/lang/String;[BI)Lsun/security/mscapi/CPublicKey;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_sign_cng_hash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sign_cng_hash(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.signCngHash(I[BIILjava/lang/String;JJ)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_sign_hash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sign_hash(
            thread,
            Parameters::new(vec![
                Value::from(false),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.signHash(Z[BILjava/lang/String;JJ)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_verify_cng_signed_hash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = verify_cng_signed_hash(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.verifyCngSignedHash(I[BI[BIILjava/lang/String;JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_verify_signed_hash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = verify_signed_hash(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CSignature.verifySignedHash([BILjava/lang/String;[BIJJ)Z",
            result.unwrap_err().to_string()
        );
    }
}

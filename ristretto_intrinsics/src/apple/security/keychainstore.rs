use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "apple/security/KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J",
    Any
)]
#[async_method]
pub async fn add_item_to_keychain<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg3 = parameters.pop_reference()?;
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_bool()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/security/KeychainStore._getEncodedKeyData(J[C)[B", Any)]
#[async_method]
pub async fn get_encoded_key_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _password = parameters.pop_reference()?;
    let _sec_key_ref = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._getEncodedKeyData(J[C)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/security/KeychainStore._releaseKeychainItemRef(J)V", Any)]
#[async_method]
pub async fn release_keychain_item_ref<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keychain_item_ref = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._releaseKeychainItemRef(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/security/KeychainStore._removeItemFromKeychain(J)I", Any)]
#[async_method]
pub async fn remove_item_from_keychain<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cert_ref = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._removeItemFromKeychain(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain()V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn scan_keychain_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._scanKeychain()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain(Ljava/lang/String;)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn scan_keychain_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_item_to_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_item_to_keychain(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::from(false),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_encoded_key_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_encoded_key_data(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "apple.security.KeychainStore._getEncodedKeyData(J[C)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_release_keychain_item_ref() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_keychain_item_ref(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.security.KeychainStore._releaseKeychainItemRef(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_remove_item_from_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_item_from_keychain(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.security.KeychainStore._removeItemFromKeychain(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_scan_keychain_0() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = scan_keychain_0(thread, Parameters::default()).await;
        assert_eq!(
            "apple.security.KeychainStore._scanKeychain()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_scan_keychain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scan_keychain_1(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}

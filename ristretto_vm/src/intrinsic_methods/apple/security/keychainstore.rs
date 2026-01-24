use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "apple/security/KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J",
    Any
)]
#[async_method]
pub(crate) async fn add_item_to_keychain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J")
}

#[intrinsic_method("apple/security/KeychainStore._getEncodedKeyData(J[C)[B", Any)]
#[async_method]
pub(crate) async fn get_encoded_key_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._getEncodedKeyData(J[C)[B")
}

#[intrinsic_method("apple/security/KeychainStore._releaseKeychainItemRef(J)V", Any)]
#[async_method]
pub(crate) async fn release_keychain_item_ref(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._releaseKeychainItemRef(J)V")
}

#[intrinsic_method("apple/security/KeychainStore._removeItemFromKeychain(J)I", Any)]
#[async_method]
pub(crate) async fn remove_item_from_keychain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._removeItemFromKeychain(J)I")
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain()V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn scan_keychain_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._scanKeychain()V")
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain(Ljava/lang/String;)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub(crate) async fn scan_keychain_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J"
    )]
    async fn test_add_item_to_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_item_to_keychain(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._getEncodedKeyData(J[C)[B"
    )]
    async fn test_get_encoded_key_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_encoded_key_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._releaseKeychainItemRef(J)V"
    )]
    async fn test_release_keychain_item_ref() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_keychain_item_ref(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._removeItemFromKeychain(J)I"
    )]
    async fn test_remove_item_from_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_item_from_keychain(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.security.KeychainStore._scanKeychain()V")]
    async fn test_scan_keychain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = scan_keychain_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V"
    )]
    async fn test_scan_keychain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = scan_keychain_1(thread, Parameters::default()).await;
    }
}

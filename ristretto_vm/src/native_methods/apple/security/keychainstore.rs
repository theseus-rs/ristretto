use crate::Result;
use crate::native_methods::registry::{JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/security/KeychainStore";

/// Register all native methods for `apple.security.KeychainStore`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_21 {
        registry.register(CLASS_NAME, "_scanKeychain", "()V", scan_keychain);
    } else {
        registry.register(
            CLASS_NAME,
            "_scanKeychain",
            "(Ljava/lang/String;)V",
            scan_keychain,
        );
    }

    registry.register(
        CLASS_NAME,
        "_addItemToKeychain",
        "(Ljava/lang/String;Z[B[C)J",
        add_item_to_keychain,
    );
    registry.register(
        CLASS_NAME,
        "_getEncodedKeyData",
        "(J[C)[B",
        get_encoded_key_data,
    );
    registry.register(
        CLASS_NAME,
        "_releaseKeychainItemRef",
        "(J)V",
        release_keychain_item_ref,
    );
    registry.register(
        CLASS_NAME,
        "_removeItemFromKeychain",
        "(J)I",
        remove_item_from_keychain,
    );
}

#[async_recursion(?Send)]
async fn add_item_to_keychain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J")
}

#[async_recursion(?Send)]
async fn get_encoded_key_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._getEncodedKeyData(J[C)[B")
}

#[async_recursion(?Send)]
async fn release_keychain_item_ref(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._releaseKeychainItemRef(J)V")
}

#[async_recursion(?Send)]
async fn remove_item_from_keychain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._removeItemFromKeychain(J)I")
}

#[async_recursion(?Send)]
async fn scan_keychain(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V"
    )]
    async fn test_scan_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = scan_keychain(thread, Parameters::default()).await;
    }
}

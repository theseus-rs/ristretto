use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_22: Version = Version::Java22 { minor: 0 };

/// Register all native methods for `apple.security.KeychainStore`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/security/KeychainStore";
    let java_version = registry.java_version();

    if java_version <= &JAVA_22 {
        registry.register(class_name, "_scanKeychain", "()V", scan_keychain);
    } else {
        registry.register(
            class_name,
            "_scanKeychain",
            "(Ljava/lang/String;)V",
            scan_keychain,
        );
    }

    registry.register(
        class_name,
        "_addItemToKeychain",
        "(Ljava/lang/String;Z[B[C)J",
        add_item_to_keychain,
    );
    registry.register(
        class_name,
        "_getEncodedKeyData",
        "(J[C)[B",
        get_encoded_key_data,
    );
    registry.register(
        class_name,
        "_releaseKeychainItemRef",
        "(J)V",
        release_keychain_item_ref,
    );
    registry.register(
        class_name,
        "_removeItemFromKeychain",
        "(J)I",
        remove_item_from_keychain,
    );
}

#[async_recursion(?Send)]
async fn add_item_to_keychain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J")
}

#[async_recursion(?Send)]
async fn get_encoded_key_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._getEncodedKeyData(J[C)[B")
}

#[async_recursion(?Send)]
async fn release_keychain_item_ref(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._releaseKeychainItemRef(J)V")
}

#[async_recursion(?Send)]
async fn remove_item_from_keychain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._removeItemFromKeychain(J)I")
}

#[async_recursion(?Send)]
async fn scan_keychain(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java22 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "apple/security/KeychainStore";
        assert!(registry
            .method(class_name, "_scanKeychain", "()V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "_addItemToKeychain",
                "(Ljava/lang/String;Z[B[C)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "_getEncodedKeyData", "(J[C)[B")
            .is_some());
        assert!(registry
            .method(class_name, "_releaseKeychainItemRef", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "_removeItemFromKeychain", "(J)I")
            .is_some());
    }

    #[test]
    fn test_register_java_23() {
        let mut registry = MethodRegistry::new(&Version::Java23 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "apple/security/KeychainStore";
        assert!(registry
            .method(class_name, "_scanKeychain", "(Ljava/lang/String;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J"
    )]
    async fn test_add_item_to_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_item_to_keychain(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._getEncodedKeyData(J[C)[B"
    )]
    async fn test_get_encoded_key_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_encoded_key_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._releaseKeychainItemRef(J)V"
    )]
    async fn test_release_keychain_item_ref() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_keychain_item_ref(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._removeItemFromKeychain(J)I"
    )]
    async fn test_remove_item_from_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_item_from_keychain(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.security.KeychainStore._scanKeychain(Ljava/lang/String;)V"
    )]
    async fn test_scan_keychain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = scan_keychain(thread, Arguments::default()).await;
    }
}

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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn add_item_to_keychain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_encoded_key_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn release_keychain_item_ref(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn remove_item_from_keychain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn scan_keychain(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

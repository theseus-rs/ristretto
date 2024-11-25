use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `sun.security.pkcs11.wrapper.PKCS11`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/pkcs11/wrapper/PKCS11";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 {
        registry.register(
            class_name,
            "C_GCMDecryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_decrypt_init_with_retry,
        );
        registry.register(
            class_name,
            "C_GCMEncryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_encrypt_init_with_retry,
        );
    }

    if java_version >= JAVA_18 {
        registry.register(class_name, "C_SessionCancel", "(JJ)V", c_session_cancel);
    }
    if java_version <= JAVA_18 {
        registry.register(class_name, "disconnect", "()V", disconnect);
    } else {
        registry.register(class_name, "disconnect", "(J)V", disconnect);
    }

    registry.register(class_name, "C_CloseSession", "(J)V", c_close_session);
    registry.register(
        class_name,
        "C_CopyObject",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_copy_object,
    );
    registry.register(
        class_name,
        "C_CreateObject",
        "(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_create_object,
    );
    registry.register(class_name, "C_Decrypt", "(JJ[BIIJ[BII)I", c_decrypt);
    registry.register(class_name, "C_DecryptFinal", "(JJ[BII)I", c_decrypt_final);
    registry.register(
        class_name,
        "C_DecryptInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_decrypt_init,
    );
    registry.register(
        class_name,
        "C_DecryptUpdate",
        "(JJ[BIIJ[BII)I",
        c_decrypt_update,
    );
    registry.register(class_name, "C_DeriveKey", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J", c_derive_key);
    registry.register(class_name, "C_DestroyObject", "(JJ)V", c_destroy_object);
    registry.register(class_name, "C_DigestFinal", "(J[BII)I", c_digest_final);
    registry.register(
        class_name,
        "C_DigestInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V",
        c_digest_init,
    );
    registry.register(class_name, "C_DigestKey", "(JJ)V", c_digest_key);
    registry.register(
        class_name,
        "C_DigestSingle",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I",
        c_digest_single,
    );
    registry.register(class_name, "C_DigestUpdate", "(JJ[BII)V", c_digest_update);
    registry.register(class_name, "C_Encrypt", "(JJ[BIIJ[BII)I", c_encrypt);
    registry.register(class_name, "C_EncryptFinal", "(JJ[BII)I", c_encrypt_final);
    registry.register(
        class_name,
        "C_EncryptInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_encrypt_init,
    );
    registry.register(
        class_name,
        "C_EncryptUpdate",
        "(JJ[BIIJ[BII)I",
        c_encrypt_update,
    );
    registry.register(
        class_name,
        "C_Finalize",
        "(Ljava/lang/Object;)V",
        c_finalize,
    );
    registry.register(class_name, "C_FindObjects", "(JJ)[J", c_find_objects);
    registry.register(
        class_name,
        "C_FindObjectsFinal",
        "(J)V",
        c_find_objects_final,
    );
    registry.register(
        class_name,
        "C_FindObjectsInit",
        "(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_find_objects_init,
    );
    registry.register(
        class_name,
        "C_GenerateKey",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_generate_key,
    );
    registry.register(class_name, "C_GenerateKeyPair", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J", c_generate_key_pair);
    registry.register(class_name, "C_GenerateRandom", "(J[B)V", c_generate_random);
    registry.register(
        class_name,
        "C_GetAttributeValue",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_get_attribute_value,
    );
    registry.register(
        class_name,
        "C_GetInfo",
        "()Lsun/security/pkcs11/wrapper/CK_INFO;",
        c_get_info,
    );
    registry.register(
        class_name,
        "C_GetMechanismInfo",
        "(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;",
        c_get_mechanism_info,
    );
    registry.register(
        class_name,
        "C_GetMechanismList",
        "(J)[J",
        c_get_mechanism_list,
    );
    registry.register(
        class_name,
        "C_GetOperationState",
        "(J)[B",
        c_get_operation_state,
    );
    registry.register(
        class_name,
        "C_GetSessionInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;",
        c_get_session_info,
    );
    registry.register(
        class_name,
        "C_GetSlotInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;",
        c_get_slot_info,
    );
    registry.register(class_name, "C_GetSlotList", "(Z)[J", c_get_slot_list);
    registry.register(
        class_name,
        "C_GetTokenInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;",
        c_get_token_info,
    );
    registry.register(
        class_name,
        "C_Initialize",
        "(Ljava/lang/Object;)V",
        c_initialize,
    );
    registry.register(class_name, "C_Login", "(JJ[C)V", c_login);
    registry.register(class_name, "C_Logout", "(J)V", c_logout);
    registry.register(
        class_name,
        "C_OpenSession",
        "(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J",
        c_open_session,
    );
    registry.register(class_name, "C_SeedRandom", "(J[B)V", c_seed_random);
    registry.register(
        class_name,
        "C_SetAttributeValue",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_set_attribute_value,
    );
    registry.register(
        class_name,
        "C_SetOperationState",
        "(J[BJJ)V",
        c_set_operation_state,
    );
    registry.register(class_name, "C_Sign", "(J[B)[B", c_sign);
    registry.register(class_name, "C_SignFinal", "(JI)[B", c_sign_final);
    registry.register(
        class_name,
        "C_SignInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_sign_init,
    );
    registry.register(class_name, "C_SignRecover", "(J[BII[BII)I", c_sign_recover);
    registry.register(
        class_name,
        "C_SignRecoverInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_sign_recover_init,
    );
    registry.register(class_name, "C_SignUpdate", "(JJ[BII)V", c_sign_update);
    registry.register(class_name, "C_UnwrapKey", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J", c_unwrap_key);
    registry.register(class_name, "C_Verify", "(J[B[B)V", c_verify);
    registry.register(class_name, "C_VerifyFinal", "(J[B)V", c_verify_final);
    registry.register(
        class_name,
        "C_VerifyInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_verify_init,
    );
    registry.register(
        class_name,
        "C_VerifyRecover",
        "(J[BII[BII)I",
        c_verify_recover,
    );
    registry.register(
        class_name,
        "C_VerifyRecoverInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_verify_recover_init,
    );
    registry.register(class_name, "C_VerifyUpdate", "(JJ[BII)V", c_verify_update);
    registry.register(
        class_name,
        "C_WrapKey",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B",
        c_wrap_key,
    );
    registry.register(
        class_name,
        "connect",
        "(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;",
        connect,
    );
    registry.register(
        class_name,
        "createNativeKey",
        "(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J",
        create_native_key,
    );
    registry.register(class_name, "finalizeLibrary", "()V", finalize_library);
    registry.register(class_name, "freeMechanism", "(J)J", free_mechanism);
    registry.register(
        class_name,
        "getNativeKeyInfo",
        "(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B",
        get_native_key_info,
    );
    registry.register(class_name, "initializeLibrary", "(Z)V", initialize_library);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_close_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_copy_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_create_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_decrypt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_decrypt_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_decrypt_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_decrypt_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_derive_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_destroy_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_digest_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_digest_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_digest_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_digest_single(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_digest_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_encrypt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_encrypt_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_encrypt_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_encrypt_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_finalize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_find_objects(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_find_objects_final(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_find_objects_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_gcm_decrypt_init_with_retry(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_gcm_encrypt_init_with_retry(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_generate_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_generate_key_pair(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_generate_random(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_attribute_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_mechanism_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_mechanism_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_operation_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_session_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_slot_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_slot_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_get_token_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_login(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_logout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_open_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_seed_random(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_session_cancel(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_set_attribute_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_set_operation_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign_recover(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign_recover_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_sign_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_unwrap_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify_recover(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify_recover_init(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_verify_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn c_wrap_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_native_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn disconnect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn finalize_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_mechanism(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_native_key_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn initialize_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

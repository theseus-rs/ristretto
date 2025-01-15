use crate::arguments::Arguments;
use crate::native_methods::registry::{
    MethodRegistry, JAVA_11, JAVA_17, JAVA_18, JAVA_21, JAVA_22,
};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/pkcs11/wrapper/PKCS11";

/// Register all native methods for `sun.security.pkcs11.wrapper.PKCS11`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "C_GCMDecryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_decrypt_init_with_retry,
        );
        registry.register(
            CLASS_NAME,
            "C_GCMEncryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_encrypt_init_with_retry,
        );
    }

    if registry.java_major_version() >= JAVA_11 && registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "connect",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            connect,
        );
    }

    if registry.java_major_version() >= JAVA_18 {
        registry.register(CLASS_NAME, "C_SessionCancel", "(JJ)V", c_session_cancel);
    }
    if registry.java_major_version() <= JAVA_18 {
        registry.register(CLASS_NAME, "disconnect", "()V", disconnect);
    } else {
        registry.register(CLASS_NAME, "disconnect", "(J)V", disconnect);
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "C_GCMDecryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_decrypt_init_with_retry,
        );
        registry.register(
            CLASS_NAME,
            "C_GCMEncryptInitWithRetry",
            "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            c_gcm_encrypt_init_with_retry,
        );
    }

    if registry.java_major_version() <= JAVA_22 {
        registry.register(CLASS_NAME, "freeMechanism", "(J)J", free_mechanism);
    }

    registry.register(CLASS_NAME, "C_CloseSession", "(J)V", c_close_session);
    registry.register(
        CLASS_NAME,
        "C_CopyObject",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_copy_object,
    );
    registry.register(
        CLASS_NAME,
        "C_CreateObject",
        "(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_create_object,
    );
    registry.register(CLASS_NAME, "C_Decrypt", "(JJ[BIIJ[BII)I", c_decrypt);
    registry.register(CLASS_NAME, "C_DecryptFinal", "(JJ[BII)I", c_decrypt_final);
    registry.register(
        CLASS_NAME,
        "C_DecryptInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_decrypt_init,
    );
    registry.register(
        CLASS_NAME,
        "C_DecryptUpdate",
        "(JJ[BIIJ[BII)I",
        c_decrypt_update,
    );
    registry.register(CLASS_NAME, "C_DeriveKey", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J", c_derive_key);
    registry.register(CLASS_NAME, "C_DestroyObject", "(JJ)V", c_destroy_object);
    registry.register(CLASS_NAME, "C_DigestFinal", "(J[BII)I", c_digest_final);
    registry.register(
        CLASS_NAME,
        "C_DigestInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V",
        c_digest_init,
    );
    registry.register(CLASS_NAME, "C_DigestKey", "(JJ)V", c_digest_key);
    registry.register(
        CLASS_NAME,
        "C_DigestSingle",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I",
        c_digest_single,
    );
    registry.register(CLASS_NAME, "C_DigestUpdate", "(JJ[BII)V", c_digest_update);
    registry.register(CLASS_NAME, "C_Encrypt", "(JJ[BIIJ[BII)I", c_encrypt);
    registry.register(CLASS_NAME, "C_EncryptFinal", "(JJ[BII)I", c_encrypt_final);
    registry.register(
        CLASS_NAME,
        "C_EncryptInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_encrypt_init,
    );
    registry.register(
        CLASS_NAME,
        "C_EncryptUpdate",
        "(JJ[BIIJ[BII)I",
        c_encrypt_update,
    );
    registry.register(
        CLASS_NAME,
        "C_Finalize",
        "(Ljava/lang/Object;)V",
        c_finalize,
    );
    registry.register(CLASS_NAME, "C_FindObjects", "(JJ)[J", c_find_objects);
    registry.register(
        CLASS_NAME,
        "C_FindObjectsFinal",
        "(J)V",
        c_find_objects_final,
    );
    registry.register(
        CLASS_NAME,
        "C_FindObjectsInit",
        "(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_find_objects_init,
    );
    registry.register(
        CLASS_NAME,
        "C_GenerateKey",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
        c_generate_key,
    );
    registry.register(CLASS_NAME, "C_GenerateKeyPair", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J", c_generate_key_pair);
    registry.register(CLASS_NAME, "C_GenerateRandom", "(J[B)V", c_generate_random);
    registry.register(
        CLASS_NAME,
        "C_GetAttributeValue",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_get_attribute_value,
    );
    registry.register(
        CLASS_NAME,
        "C_GetInfo",
        "()Lsun/security/pkcs11/wrapper/CK_INFO;",
        c_get_info,
    );
    registry.register(
        CLASS_NAME,
        "C_GetMechanismInfo",
        "(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;",
        c_get_mechanism_info,
    );
    registry.register(
        CLASS_NAME,
        "C_GetMechanismList",
        "(J)[J",
        c_get_mechanism_list,
    );
    registry.register(
        CLASS_NAME,
        "C_GetOperationState",
        "(J)[B",
        c_get_operation_state,
    );
    registry.register(
        CLASS_NAME,
        "C_GetSessionInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;",
        c_get_session_info,
    );
    registry.register(
        CLASS_NAME,
        "C_GetSlotInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;",
        c_get_slot_info,
    );
    registry.register(CLASS_NAME, "C_GetSlotList", "(Z)[J", c_get_slot_list);
    registry.register(
        CLASS_NAME,
        "C_GetTokenInfo",
        "(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;",
        c_get_token_info,
    );
    registry.register(
        CLASS_NAME,
        "C_Initialize",
        "(Ljava/lang/Object;)V",
        c_initialize,
    );
    registry.register(CLASS_NAME, "C_Login", "(JJ[C)V", c_login);
    registry.register(CLASS_NAME, "C_Logout", "(J)V", c_logout);
    registry.register(
        CLASS_NAME,
        "C_OpenSession",
        "(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J",
        c_open_session,
    );
    registry.register(CLASS_NAME, "C_SeedRandom", "(J[B)V", c_seed_random);
    registry.register(
        CLASS_NAME,
        "C_SetAttributeValue",
        "(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
        c_set_attribute_value,
    );
    registry.register(
        CLASS_NAME,
        "C_SetOperationState",
        "(J[BJJ)V",
        c_set_operation_state,
    );
    registry.register(CLASS_NAME, "C_Sign", "(J[B)[B", c_sign);
    registry.register(CLASS_NAME, "C_SignFinal", "(JI)[B", c_sign_final);
    registry.register(
        CLASS_NAME,
        "C_SignInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_sign_init,
    );
    registry.register(CLASS_NAME, "C_SignRecover", "(J[BII[BII)I", c_sign_recover);
    registry.register(
        CLASS_NAME,
        "C_SignRecoverInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_sign_recover_init,
    );
    registry.register(CLASS_NAME, "C_SignUpdate", "(JJ[BII)V", c_sign_update);
    registry.register(CLASS_NAME, "C_UnwrapKey", "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J", c_unwrap_key);
    registry.register(CLASS_NAME, "C_Verify", "(J[B[B)V", c_verify);
    registry.register(CLASS_NAME, "C_VerifyFinal", "(J[B)V", c_verify_final);
    registry.register(
        CLASS_NAME,
        "C_VerifyInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_verify_init,
    );
    registry.register(
        CLASS_NAME,
        "C_VerifyRecover",
        "(J[BII[BII)I",
        c_verify_recover,
    );
    registry.register(
        CLASS_NAME,
        "C_VerifyRecoverInit",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
        c_verify_recover_init,
    );
    registry.register(CLASS_NAME, "C_VerifyUpdate", "(JJ[BII)V", c_verify_update);
    registry.register(
        CLASS_NAME,
        "C_WrapKey",
        "(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B",
        c_wrap_key,
    );
    registry.register(
        CLASS_NAME,
        "connect",
        "(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;",
        connect,
    );
    registry.register(
        CLASS_NAME,
        "createNativeKey",
        "(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J",
        create_native_key,
    );
    registry.register(CLASS_NAME, "finalizeLibrary", "()V", finalize_library);
    registry.register(
        CLASS_NAME,
        "getNativeKeyInfo",
        "(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B",
        get_native_key_info,
    );
    registry.register(CLASS_NAME, "initializeLibrary", "(Z)V", initialize_library);
}

#[async_recursion(?Send)]
async fn c_close_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_CloseSession(J)V")
}

#[async_recursion(?Send)]
async fn c_copy_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J")
}

#[async_recursion(?Send)]
async fn c_create_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J")
}

#[async_recursion(?Send)]
async fn c_decrypt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_decrypt_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_decrypt_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_decrypt_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_derive_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J")
}

#[async_recursion(?Send)]
async fn c_destroy_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V")
}

#[async_recursion(?Send)]
async fn c_digest_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I")
}

#[async_recursion(?Send)]
async fn c_digest_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V")
}

#[async_recursion(?Send)]
async fn c_digest_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V")
}

#[async_recursion(?Send)]
async fn c_digest_single(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I")
}

#[async_recursion(?Send)]
async fn c_digest_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V")
}

#[async_recursion(?Send)]
async fn c_encrypt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_encrypt_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_encrypt_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_encrypt_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I")
}

#[async_recursion(?Send)]
async fn c_finalize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn c_find_objects(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J")
}

#[async_recursion(?Send)]
async fn c_find_objects_final(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V")
}

#[async_recursion(?Send)]
async fn c_find_objects_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V")
}

#[async_recursion(?Send)]
async fn c_gcm_decrypt_init_with_retry(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V")
}

#[async_recursion(?Send)]
async fn c_gcm_encrypt_init_with_retry(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V")
}

#[async_recursion(?Send)]
async fn c_generate_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J")
}

#[async_recursion(?Send)]
async fn c_generate_key_pair(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J")
}

#[async_recursion(?Send)]
async fn c_generate_random(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V")
}

#[async_recursion(?Send)]
async fn c_get_attribute_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V")
}

#[async_recursion(?Send)]
async fn c_get_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;")
}

#[async_recursion(?Send)]
async fn c_get_mechanism_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;")
}

#[async_recursion(?Send)]
async fn c_get_mechanism_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J")
}

#[async_recursion(?Send)]
async fn c_get_operation_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B")
}

#[async_recursion(?Send)]
async fn c_get_session_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;")
}

#[async_recursion(?Send)]
async fn c_get_slot_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;")
}

#[async_recursion(?Send)]
async fn c_get_slot_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J")
}

#[async_recursion(?Send)]
async fn c_get_token_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;")
}

#[async_recursion(?Send)]
async fn c_initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn c_login(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V")
}

#[async_recursion(?Send)]
async fn c_logout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V")
}

#[async_recursion(?Send)]
async fn c_open_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J")
}

#[async_recursion(?Send)]
async fn c_seed_random(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V")
}

#[async_recursion(?Send)]
async fn c_session_cancel(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V")
}

#[async_recursion(?Send)]
async fn c_set_attribute_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V")
}

#[async_recursion(?Send)]
async fn c_set_operation_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V")
}

#[async_recursion(?Send)]
async fn c_sign(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B")
}

#[async_recursion(?Send)]
async fn c_sign_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B")
}

#[async_recursion(?Send)]
async fn c_sign_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_sign_recover(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I")
}

#[async_recursion(?Send)]
async fn c_sign_recover_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_sign_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V")
}

#[async_recursion(?Send)]
async fn c_unwrap_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J")
}

#[async_recursion(?Send)]
async fn c_verify(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V")
}

#[async_recursion(?Send)]
async fn c_verify_final(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V")
}

#[async_recursion(?Send)]
async fn c_verify_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_verify_recover(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I")
}

#[async_recursion(?Send)]
async fn c_verify_recover_init(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V")
}

#[async_recursion(?Send)]
async fn c_verify_update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V")
}

#[async_recursion(?Send)]
async fn c_wrap_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B")
}

#[async_recursion(?Send)]
async fn connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;")
}

#[async_recursion(?Send)]
async fn create_native_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J")
}

#[async_recursion(?Send)]
async fn disconnect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.disconnect()V")
}

#[async_recursion(?Send)]
async fn finalize_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V")
}

#[async_recursion(?Send)]
async fn free_mechanism(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.freeMechanism(J)J")
}

#[async_recursion(?Send)]
async fn get_native_key_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B")
}

#[async_recursion(?Send)]
async fn initialize_library(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.initializeLibrary(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_CloseSession(J)V"
    )]
    async fn test_c_close_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_close_session(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_copy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_copy_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_create_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_create_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I"
    )]
    async fn test_c_decrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I"
    )]
    async fn test_c_decrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_decrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I"
    )]
    async fn test_c_decrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_derive_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_derive_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V"
    )]
    async fn test_c_destroy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_destroy_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I"
    )]
    async fn test_c_digest_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V"
    )]
    async fn test_c_digest_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V"
    )]
    async fn test_c_digest_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I"
    )]
    async fn test_c_digest_single() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_single(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V"
    )]
    async fn test_c_digest_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I"
    )]
    async fn test_c_encrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I"
    )]
    async fn test_c_encrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_encrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I"
    )]
    async fn test_c_encrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V"
    )]
    async fn test_c_finalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_finalize(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J"
    )]
    async fn test_c_find_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V"
    )]
    async fn test_c_find_objects_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_find_objects_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )]
    async fn test_c_gcm_decrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_gcm_decrypt_init_with_retry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )]
    async fn test_c_gcm_encrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_gcm_encrypt_init_with_retry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_generate_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J"
    )]
    async fn test_c_generate_key_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_key_pair(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V"
    )]
    async fn test_c_generate_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_random(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_get_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_attribute_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;"
    )]
    async fn test_c_get_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;"
    )]
    async fn test_c_get_mechanism_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_mechanism_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J"
    )]
    async fn test_c_get_mechanism_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_mechanism_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B"
    )]
    async fn test_c_get_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_operation_state(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;"
    )]
    async fn test_c_get_session_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_session_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;"
    )]
    async fn test_c_get_slot_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_slot_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J"
    )]
    async fn test_c_get_slot_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_slot_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;"
    )]
    async fn test_c_get_token_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_token_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V"
    )]
    async fn test_c_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_initialize(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V"
    )]
    async fn test_c_login() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_login(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V"
    )]
    async fn test_c_logout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_logout(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J"
    )]
    async fn test_c_open_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_open_session(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V"
    )]
    async fn test_c_seed_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_seed_random(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V"
    )]
    async fn test_c_session_cancel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_session_cancel(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_set_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_set_attribute_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V"
    )]
    async fn test_c_set_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_set_operation_state(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B"
    )]
    async fn test_c_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B"
    )]
    async fn test_c_sign_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_sign_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I"
    )]
    async fn test_c_sign_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_recover(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_sign_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_recover_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V"
    )]
    async fn test_c_sign_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_unwrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_unwrap_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V"
    )]
    async fn test_c_verify() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V"
    )]
    async fn test_c_verify_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_final(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_verify_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I"
    )]
    async fn test_c_verify_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_recover(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_verify_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_recover_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V"
    )]
    async fn test_c_verify_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B"
    )]
    async fn test_c_wrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_wrap_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;"
    )]
    async fn test_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J"
    )]
    async fn test_create_native_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_key(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V"
    )]
    async fn test_finalize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = finalize_library(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B"
    )]
    async fn test_get_native_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_key_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.initializeLibrary(Z)V"
    )]
    async fn test_initialize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_library(thread, Arguments::default()).await;
    }
}

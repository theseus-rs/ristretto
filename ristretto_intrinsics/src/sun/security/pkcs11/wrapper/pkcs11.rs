use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CloseSession(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_close_session<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_CloseSession(J)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_copy_object<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_create_object<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Decrypt(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptFinal(JJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_derive_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DestroyObject(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_destroy_object<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestFinal(J[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestKey(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_single<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Encrypt(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptFinal(JJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Finalize(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_finalize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjects(JJ)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjectsFinal(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_gcm_decrypt_init_with_retry<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_gcm_encrypt_init_with_retry<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_key_pair<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateRandom(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_random<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_attribute_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_mechanism_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetMechanismList(J)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_mechanism_list<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetOperationState(J)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_operation_state<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_session_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_slot_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSlotList(Z)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_slot_list<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_token_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Initialize(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_initialize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Login(JJ[C)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_login<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Logout(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_logout<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_open_session<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SeedRandom(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_seed_random<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SessionCancel(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn c_session_cancel<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_set_attribute_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SetOperationState(J[BJJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_set_operation_state<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Sign(J[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignFinal(JI)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignRecover(J[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_recover<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_recover_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_unwrap_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Verify(J[B[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyFinal(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_final<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyRecover(J[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_recover<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_recover_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_update<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_wrap_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn connect_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_native_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.disconnect()V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn disconnect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.disconnect()V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.disconnect(J)V",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn disconnect_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.disconnect(J)V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.finalizeLibrary()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn finalize_library<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.freeMechanism(J)J",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn free_mechanism<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.pkcs11.wrapper.PKCS11.freeMechanism(J)J")
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_key_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B"
    )
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.initializeLibrary(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize_library<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let _ = c_close_session(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_copy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_copy_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_create_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_create_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I"
    )]
    async fn test_c_decrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I"
    )]
    async fn test_c_decrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_decrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I"
    )]
    async fn test_c_decrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_decrypt_update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_derive_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_derive_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V"
    )]
    async fn test_c_destroy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_destroy_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I"
    )]
    async fn test_c_digest_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V"
    )]
    async fn test_c_digest_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V"
    )]
    async fn test_c_digest_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I"
    )]
    async fn test_c_digest_single() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_single(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V"
    )]
    async fn test_c_digest_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_digest_update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I"
    )]
    async fn test_c_encrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I"
    )]
    async fn test_c_encrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_encrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I"
    )]
    async fn test_c_encrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_encrypt_update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V"
    )]
    async fn test_c_finalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_finalize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J"
    )]
    async fn test_c_find_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V"
    )]
    async fn test_c_find_objects_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_find_objects_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_find_objects_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )]
    async fn test_c_gcm_decrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_gcm_decrypt_init_with_retry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V"
    )]
    async fn test_c_gcm_encrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_gcm_encrypt_init_with_retry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_generate_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J"
    )]
    async fn test_c_generate_key_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_key_pair(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V"
    )]
    async fn test_c_generate_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_generate_random(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_get_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_attribute_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;"
    )]
    async fn test_c_get_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;"
    )]
    async fn test_c_get_mechanism_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_mechanism_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J"
    )]
    async fn test_c_get_mechanism_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_mechanism_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B"
    )]
    async fn test_c_get_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_operation_state(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;"
    )]
    async fn test_c_get_session_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_session_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;"
    )]
    async fn test_c_get_slot_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_slot_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J"
    )]
    async fn test_c_get_slot_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_slot_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;"
    )]
    async fn test_c_get_token_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_get_token_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V"
    )]
    async fn test_c_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_initialize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V"
    )]
    async fn test_c_login() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_login(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V"
    )]
    async fn test_c_logout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_logout(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J"
    )]
    async fn test_c_open_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_open_session(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V"
    )]
    async fn test_c_seed_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_seed_random(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V"
    )]
    async fn test_c_session_cancel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_session_cancel(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V"
    )]
    async fn test_c_set_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_set_attribute_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V"
    )]
    async fn test_c_set_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_set_operation_state(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B"
    )]
    async fn test_c_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B"
    )]
    async fn test_c_sign_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_sign_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I"
    )]
    async fn test_c_sign_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_recover(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_sign_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_recover_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V"
    )]
    async fn test_c_sign_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_sign_update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J"
    )]
    async fn test_c_unwrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_unwrap_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V"
    )]
    async fn test_c_verify() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V"
    )]
    async fn test_c_verify_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_final(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_verify_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I"
    )]
    async fn test_c_verify_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_recover(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V"
    )]
    async fn test_c_verify_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_recover_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V"
    )]
    async fn test_c_verify_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_verify_update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B"
    )]
    async fn test_c_wrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = c_wrap_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;"
    )]
    async fn test_connect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J"
    )]
    async fn test_create_native_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_key(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.disconnect()V"
    )]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.disconnect(J)V"
    )]
    async fn test_disconnect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V"
    )]
    async fn test_finalize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = finalize_library(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.freeMechanism(J)J"
    )]
    async fn test_free_mechanism() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_mechanism(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B"
    )]
    async fn test_get_native_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_key_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.pkcs11.wrapper.PKCS11.initializeLibrary(Z)V"
    )]
    async fn test_initialize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_library(thread, Parameters::default()).await;
    }
}

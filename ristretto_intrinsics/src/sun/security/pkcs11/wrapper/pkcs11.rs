use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CloseSession(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_close_session<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_CloseSession(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_copy_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_object = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_create_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Decrypt(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptFinal(JJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_decrypt_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_derive_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_base_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DestroyObject(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_destroy_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_object = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestFinal(J[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _digest_len = parameters.pop_int()?;
    let _digest_ofs = parameters.pop_int()?;
    let _p_digest = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestKey(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_single<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _digest_len = parameters.pop_int()?;
    let _digest_ofs = parameters.pop_int()?;
    let _digest = parameters.pop_reference()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_DigestUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_digest_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Encrypt(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptFinal(JJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_encrypt_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_ofs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _direct_out = parameters.pop_long()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Finalize(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_finalize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_reserved = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjects(JJ)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ul_max_object_count = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjectsFinal(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_find_objects_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_gcm_decrypt_init_with_retry<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_normative_ver_first = parameters.pop_bool()?;
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_gcm_encrypt_init_with_retry<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_normative_ver_first = parameters.pop_bool()?;
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_key_pair<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_private_key_template = parameters.pop_reference()?;
    let _p_public_key_template = parameters.pop_reference()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GenerateRandom(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_generate_random<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _random_data = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_attribute_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_object = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_mechanism_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_long()?;
    let _slot_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetMechanismList(J)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_mechanism_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _slot_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetOperationState(J)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_operation_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_session_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_slot_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _slot_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetSlotList(Z)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_slot_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token_present = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_get_token_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _slot_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Initialize(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_init_args = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Login(JJ[C)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_login<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_pin = parameters.pop_reference()?;
    let _user_type = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Logout(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_logout<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_open_session<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_notify = parameters.pop_reference()?;
    let _j_application = parameters.pop_reference()?;
    let _j_flags = parameters.pop_long()?;
    let _j_slot_id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SeedRandom(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_seed_random<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_seed = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SessionCancel(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn c_session_cancel<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_set_attribute_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _h_object = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SetOperationState(J[BJJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_set_operation_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_authentication_key = parameters.pop_long()?;
    let _h_encryption_key = parameters.pop_long()?;
    let _p_operation_state = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Sign(J[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignFinal(JI)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _expected_len = parameters.pop_int()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignRecover(J[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_recover<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_oufs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_recover_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_SignUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_sign_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_unwrap_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_template = parameters.pop_reference()?;
    let _p_wrapped_key = parameters.pop_reference()?;
    let _h_unwrapping_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_Verify(J[B[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_signature = parameters.pop_reference()?;
    let _p_data = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyFinal(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_final<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_signature = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyRecover(J[BII[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_recover<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_len = parameters.pop_int()?;
    let _out_oufs = parameters.pop_int()?;
    let _out = parameters.pop_reference()?;
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_recover_init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_VerifyUpdate(JJ[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_verify_update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_len = parameters.pop_int()?;
    let _in_ofs = parameters.pop_int()?;
    let _in_ = parameters.pop_reference()?;
    let _direct_in = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn c_wrap_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let _h_wrapping_key = parameters.pop_long()?;
    let _p_mechanism = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_get_function_list = parameters.pop_reference()?;
    let _j_pkcs11_module_path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn connect_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_get_function_list = parameters.pop_reference()?;
    let _j_pkcs11_module_path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_native_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_wrapping_mech = parameters.pop_reference()?;
    let _h_wrapping_key = parameters.pop_long()?;
    let _key_info = parameters.pop_reference()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.disconnect()V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn disconnect_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.disconnect()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.disconnect(J)V",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn disconnect_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_native_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.disconnect(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.finalizeLibrary()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn finalize_library<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.freeMechanism(J)J",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn free_mechanism<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_mechanism = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.freeMechanism(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_key_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_wrapping_mech = parameters.pop_reference()?;
    let _h_wrapping_key = parameters.pop_long()?;
    let _h_key = parameters.pop_long()?;
    let _h_session = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B".to_string()).into())
}

#[intrinsic_method(
    "sun/security/pkcs11/wrapper/PKCS11.initializeLibrary(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize_library<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _debug = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.wrapper.PKCS11.initializeLibrary(Z)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_c_close_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_close_session(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_CloseSession(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_copy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_copy_object(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_CopyObject(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_create_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_create_object(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_CreateObject(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_decrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_decrypt(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Decrypt(JJ[BIIJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_decrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_decrypt_final(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DecryptFinal(JJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_decrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_decrypt_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DecryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_decrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_decrypt_update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DecryptUpdate(JJ[BIIJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_derive_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_derive_key(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DeriveKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_destroy_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_destroy_object(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DestroyObject(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_digest_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_digest_final(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DigestFinal(J[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_digest_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_digest_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DigestInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_digest_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_digest_key(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DigestKey(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_digest_single() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_digest_single(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DigestSingle(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[BII[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_digest_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_digest_update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_DigestUpdate(JJ[BII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_encrypt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_encrypt(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Encrypt(JJ[BIIJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_encrypt_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_encrypt_final(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_EncryptFinal(JJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_encrypt_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_encrypt_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_EncryptInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_encrypt_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_encrypt_update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_EncryptUpdate(JJ[BIIJ[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_finalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_finalize(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Finalize(Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_find_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_find_objects(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_FindObjects(JJ)[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_find_objects_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_find_objects_final(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsFinal(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_find_objects_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_find_objects_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_FindObjectsInit(J[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_gcm_decrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_gcm_decrypt_init_with_retry(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GCMDecryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_gcm_encrypt_init_with_retry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_gcm_encrypt_init_with_retry(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GCMEncryptInitWithRetry(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_generate_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_generate_key(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GenerateKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_generate_key_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_generate_key_pair(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GenerateKeyPair(JLsun/security/pkcs11/wrapper/CK_MECHANISM;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_generate_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_generate_random(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GenerateRandom(J[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_attribute_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_info(thread, Parameters::default()).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetInfo()Lsun/security/pkcs11/wrapper/CK_INFO;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_mechanism_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_mechanism_info(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismInfo(JJ)Lsun/security/pkcs11/wrapper/CK_MECHANISM_INFO;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_mechanism_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_mechanism_list(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetMechanismList(J)[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_operation_state(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetOperationState(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_session_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_session_info(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetSessionInfo(J)Lsun/security/pkcs11/wrapper/CK_SESSION_INFO;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_slot_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_slot_info(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetSlotInfo(J)Lsun/security/pkcs11/wrapper/CK_SLOT_INFO;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_slot_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_slot_list(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetSlotList(Z)[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_get_token_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_get_token_info(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_GetTokenInfo(J)Lsun/security/pkcs11/wrapper/CK_TOKEN_INFO;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_initialize(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Initialize(Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_login() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_login(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Login(JJ[C)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_logout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_logout(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Logout(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_open_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_open_session(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_OpenSession(JJLjava/lang/Object;Lsun/security/pkcs11/wrapper/CK_NOTIFY;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_seed_random() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_seed_random(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SeedRandom(J[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_session_cancel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_session_cancel(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SessionCancel(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_set_attribute_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_set_attribute_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SetAttributeValue(JJ[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_set_operation_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_set_operation_state(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SetOperationState(J[BJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_sign(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Sign(J[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            c_sign_final(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SignFinal(JI)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_sign_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SignInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_sign_recover(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SignRecover(J[BII[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_sign_recover_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SignRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_sign_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_sign_update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_SignUpdate(JJ[BII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_unwrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_unwrap_key(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_UnwrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J[B[Lsun/security/pkcs11/wrapper/CK_ATTRIBUTE;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_Verify(J[B[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify_final() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify_final(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_VerifyFinal(J[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_VerifyInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify_recover() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify_recover(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecover(J[BII[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify_recover_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify_recover_init(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_VerifyRecoverInit(JLsun/security/pkcs11/wrapper/CK_MECHANISM;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_verify_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_verify_update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_VerifyUpdate(JJ[BII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_c_wrap_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = c_wrap_key(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.C_WrapKey(JLsun/security/pkcs11/wrapper/CK_MECHANISM;JJ)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = connect_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_connect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.connect(Ljava/lang/String;Ljava/lang/String;)Lsun/security/pkcs11/wrapper/CK_VERSION;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_native_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_key(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.createNativeKey(J[BJLsun/security/pkcs11/wrapper/CK_MECHANISM;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = disconnect_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.disconnect()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_disconnect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disconnect_1(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.disconnect(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_finalize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = finalize_library(thread, Parameters::default()).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.finalizeLibrary()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_mechanism() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = free_mechanism(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.freeMechanism(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_key_info(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.getNativeKeyInfo(JJJLsun/security/pkcs11/wrapper/CK_MECHANISM;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_initialize_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_library(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.security.pkcs11.wrapper.PKCS11.initializeLibrary(Z)V",
            result.unwrap_err().to_string()
        );
    }
}

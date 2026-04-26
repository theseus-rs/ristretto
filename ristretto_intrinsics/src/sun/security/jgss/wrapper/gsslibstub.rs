use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
    Any
)]
#[async_method]
pub async fn accept_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_reference()?;
    let _in_token = parameters.pop_reference()?;
    let _cb = parameters.pop_reference()?;
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B".to_string()).into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.acquireCred(JII)J", Any)]
#[async_method]
pub async fn acquire_cred<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _usage = parameters.pop_int()?;
    let _lifetime = parameters.pop_int()?;
    let _p_name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.canonicalizeName(J)J", Any)]
#[async_method]
pub async fn canonicalize_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.compareName(JJ)Z", Any)]
#[async_method]
pub async fn compare_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_name2 = parameters.pop_long()?;
    let _p_name1 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.deleteContext(J)J", Any)]
#[async_method]
pub async fn delete_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.displayName(J)[Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn display_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.exportContext(J)[B", Any)]
#[async_method]
pub async fn export_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.exportName(J)[B", Any)]
#[async_method]
pub async fn export_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn get_context_mech<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getContextName(JZ)J", Any)]
#[async_method]
pub async fn get_context_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_src = parameters.pop_bool()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getContextTime(J)I", Any)]
#[async_method]
pub async fn get_context_time<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredName(J)J", Any)]
#[async_method]
pub async fn get_cred_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredTime(J)I", Any)]
#[async_method]
pub async fn get_cred_time<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredUsage(J)I", Any)]
#[async_method]
pub async fn get_cred_usage<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getMechPtr([B)J", Any)]
#[async_method]
pub async fn get_mech_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _oid_der_encoding = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getMic(JI[B)[B", Any)]
#[async_method]
pub async fn get_mic<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _msg = parameters.pop_reference()?;
    let _qop = parameters.pop_int()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;",
    Any
)]
#[async_method]
pub async fn import_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _inter_proc_token = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;".to_string()).into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J",
    Any
)]
#[async_method]
pub async fn import_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_reference()?;
    let _name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn indicate_mechs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.init(Ljava/lang/String;Z)Z", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_debug = parameters.pop_bool()?;
    let _jlib_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
    Any
)]
#[async_method]
pub async fn init_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_reference()?;
    let _in_token = parameters.pop_reference()?;
    let _cb = parameters.pop_reference()?;
    let _target_name = parameters.pop_long()?;
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B".to_string()).into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.inquireContext(J)[J", Any)]
#[async_method]
pub async fn inquire_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn inquire_names_for_mech<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.releaseCred(J)J", Any)]
#[async_method]
pub async fn release_cred<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_cred = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.releaseName(J)V", Any)]
#[async_method]
pub async fn release_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B",
    Any
)]
#[async_method]
pub async fn unwrap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_reference()?;
    let _msg_token = parameters.pop_reference()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V",
    Any
)]
#[async_method]
pub async fn verify_mic<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_reference()?;
    let _msg = parameters.pop_reference()?;
    let _token = parameters.pop_reference()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B",
    Any
)]
#[async_method]
pub async fn wrap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_reference()?;
    let _msg = parameters.pop_reference()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.wrapSizeLimit(JIII)I", Any)]
#[async_method]
pub async fn wrap_size_limit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_size = parameters.pop_int()?;
    let _qop = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _p_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.jgss.wrapper.GSSLibStub.wrapSizeLimit(JIII)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept_context(
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
            "sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_acquire_cred() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = acquire_cred(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_canonicalize_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = canonicalize_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_compare_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_name(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_delete_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delete_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_display_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_export_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = export_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_export_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = export_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_context_mech() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_context_mech(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_context_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_context_name(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_context_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_context_time(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cred_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cred_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cred_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cred_time(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cred_usage() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cred_usage(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_mech_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_mech_ptr(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_mic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_mic(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_import_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = import_context(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_import_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = import_name(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_indicate_mechs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = indicate_mechs(thread, Parameters::default()).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_context(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_inquire_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inquire_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_inquire_names_for_mech() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inquire_names_for_mech(thread, Parameters::default()).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_release_cred() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_cred(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_release_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_unwrap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unwrap(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_verify_mic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = verify_mic(
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
            "sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_wrap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wrap(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_wrap_size_limit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wrap_size_limit(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.jgss.wrapper.GSSLibStub.wrapSizeLimit(JIII)I",
            result.unwrap_err().to_string()
        );
    }
}

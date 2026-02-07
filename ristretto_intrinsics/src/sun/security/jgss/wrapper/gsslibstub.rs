use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
    Any
)]
#[async_method]
pub async fn accept_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.acquireCred(JII)J", Any)]
#[async_method]
pub async fn acquire_cred<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.canonicalizeName(J)J", Any)]
#[async_method]
pub async fn canonicalize_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.compareName(JJ)Z", Any)]
#[async_method]
pub async fn compare_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.deleteContext(J)J", Any)]
#[async_method]
pub async fn delete_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.displayName(J)[Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn display_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.exportContext(J)[B", Any)]
#[async_method]
pub async fn export_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.exportName(J)[B", Any)]
#[async_method]
pub async fn export_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn get_context_mech<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getContextName(JZ)J", Any)]
#[async_method]
pub async fn get_context_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getContextTime(J)I", Any)]
#[async_method]
pub async fn get_context_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredName(J)J", Any)]
#[async_method]
pub async fn get_cred_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredTime(J)I", Any)]
#[async_method]
pub async fn get_cred_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getCredUsage(J)I", Any)]
#[async_method]
pub async fn get_cred_usage<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getMechPtr([B)J", Any)]
#[async_method]
pub async fn get_mech_ptr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.getMic(JI[B)[B", Any)]
#[async_method]
pub async fn get_mic<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;",
    Any
)]
#[async_method]
pub async fn import_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;"
    )
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J",
    Any
)]
#[async_method]
pub async fn import_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn indicate_mechs<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.init(Ljava/lang/String;Z)Z", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
    Any
)]
#[async_method]
pub async fn init_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.inquireContext(J)[J", Any)]
#[async_method]
pub async fn inquire_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;",
    Any
)]
#[async_method]
pub async fn inquire_names_for_mech<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.releaseCred(J)J", Any)]
#[async_method]
pub async fn release_cred<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.releaseName(J)V", Any)]
#[async_method]
pub async fn release_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B",
    Any
)]
#[async_method]
pub async fn unwrap<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V",
    Any
)]
#[async_method]
pub async fn verify_mic<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V")
}

#[intrinsic_method(
    "sun/security/jgss/wrapper/GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B",
    Any
)]
#[async_method]
pub async fn wrap<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[intrinsic_method("sun/security/jgss/wrapper/GSSLibStub.wrapSizeLimit(JIII)I", Any)]
#[async_method]
pub async fn wrap_size_limit<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.wrapSizeLimit(JIII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )]
    async fn test_accept_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J"
    )]
    async fn test_acquire_cred() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = acquire_cred(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J"
    )]
    async fn test_canonicalize_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z"
    )]
    async fn test_compare_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J"
    )]
    async fn test_delete_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = delete_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;"
    )]
    async fn test_display_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = display_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B"
    )]
    async fn test_export_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = export_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B"
    )]
    async fn test_export_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = export_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;"
    )]
    async fn test_get_context_mech() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_context_mech(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J"
    )]
    async fn test_get_context_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_context_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I"
    )]
    async fn test_get_context_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_context_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J"
    )]
    async fn test_get_cred_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cred_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I"
    )]
    async fn test_get_cred_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cred_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I"
    )]
    async fn test_get_cred_usage() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cred_usage(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J"
    )]
    async fn test_get_mech_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mech_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B"
    )]
    async fn test_get_mic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;"
    )]
    async fn test_import_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = import_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J"
    )]
    async fn test_import_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = import_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;"
    )]
    async fn test_indicate_mechs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = indicate_mechs(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z"
    )]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )]
    async fn test_init_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J"
    )]
    async fn test_inquire_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inquire_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;"
    )]
    async fn test_inquire_names_for_mech() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inquire_names_for_mech(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J"
    )]
    async fn test_release_cred() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_cred(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V"
    )]
    async fn test_release_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B"
    )]
    async fn test_unwrap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unwrap(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V"
    )]
    async fn test_verify_mic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = verify_mic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B"
    )]
    async fn test_wrap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wrap(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.jgss.wrapper.GSSLibStub.wrapSizeLimit(JIII)I"
    )]
    async fn test_wrap_size_limit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wrap_size_limit(thread, Parameters::default()).await;
    }
}

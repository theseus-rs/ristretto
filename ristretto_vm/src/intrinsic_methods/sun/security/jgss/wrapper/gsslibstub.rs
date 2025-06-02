use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/jgss/wrapper/GSSLibStub";

/// Register all intrinsic methods for `sun.security.jgss.wrapper.GSSLibStub`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "acceptContext",
        "(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
        accept_context,
    );
    registry.register(CLASS_NAME, "acquireCred", "(JII)J", acquire_cred);
    registry.register(CLASS_NAME, "canonicalizeName", "(J)J", canonicalize_name);
    registry.register(CLASS_NAME, "compareName", "(JJ)Z", compare_name);
    registry.register(CLASS_NAME, "deleteContext", "(J)J", delete_context);
    registry.register(
        CLASS_NAME,
        "displayName",
        "(J)[Ljava/lang/Object;",
        display_name,
    );
    registry.register(CLASS_NAME, "exportContext", "(J)[B", export_context);
    registry.register(CLASS_NAME, "exportName", "(J)[B", export_name);
    registry.register(
        CLASS_NAME,
        "getContextMech",
        "(J)Lorg/ietf/jgss/Oid;",
        get_context_mech,
    );
    registry.register(CLASS_NAME, "getContextName", "(JZ)J", get_context_name);
    registry.register(CLASS_NAME, "getContextTime", "(J)I", get_context_time);
    registry.register(CLASS_NAME, "getCredName", "(J)J", get_cred_name);
    registry.register(CLASS_NAME, "getCredTime", "(J)I", get_cred_time);
    registry.register(CLASS_NAME, "getCredUsage", "(J)I", get_cred_usage);
    registry.register(CLASS_NAME, "getMechPtr", "([B)J", get_mech_ptr);
    registry.register(CLASS_NAME, "getMic", "(JI[B)[B", get_mic);
    registry.register(
        CLASS_NAME,
        "importContext",
        "([B)Lsun/security/jgss/wrapper/NativeGSSContext;",
        import_context,
    );
    registry.register(
        CLASS_NAME,
        "importName",
        "([BLorg/ietf/jgss/Oid;)J",
        import_name,
    );
    registry.register(
        CLASS_NAME,
        "indicateMechs",
        "()[Lorg/ietf/jgss/Oid;",
        indicate_mechs,
    );
    registry.register(CLASS_NAME, "init", "(Ljava/lang/String;Z)Z", init);
    registry.register(
        CLASS_NAME,
        "initContext",
        "(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
        init_context,
    );
    registry.register(CLASS_NAME, "inquireContext", "(J)[J", inquire_context);
    registry.register(
        CLASS_NAME,
        "inquireNamesForMech",
        "()[Lorg/ietf/jgss/Oid;",
        inquire_names_for_mech,
    );
    registry.register(CLASS_NAME, "releaseCred", "(J)J", release_cred);
    registry.register(CLASS_NAME, "releaseName", "(J)V", release_name);
    registry.register(
        CLASS_NAME,
        "unwrap",
        "(J[BLorg/ietf/jgss/MessageProp;)[B",
        unwrap,
    );
    registry.register(
        CLASS_NAME,
        "verifyMic",
        "(J[B[BLorg/ietf/jgss/MessageProp;)V",
        verify_mic,
    );
    registry.register(
        CLASS_NAME,
        "wrap",
        "(J[BLorg/ietf/jgss/MessageProp;)[B",
        wrap,
    );
    registry.register(CLASS_NAME, "wrapSizeLimit", "(JIII)I", wrap_size_limit);
}

#[async_recursion(?Send)]
async fn accept_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )
}

#[async_recursion(?Send)]
async fn acquire_cred(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J")
}

#[async_recursion(?Send)]
async fn canonicalize_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J")
}

#[async_recursion(?Send)]
async fn compare_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z")
}

#[async_recursion(?Send)]
async fn delete_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J")
}

#[async_recursion(?Send)]
async fn display_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn export_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B")
}

#[async_recursion(?Send)]
async fn export_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B")
}

#[async_recursion(?Send)]
async fn get_context_mech(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn get_context_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J")
}

#[async_recursion(?Send)]
async fn get_context_time(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I")
}

#[async_recursion(?Send)]
async fn get_cred_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J")
}

#[async_recursion(?Send)]
async fn get_cred_time(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I")
}

#[async_recursion(?Send)]
async fn get_cred_usage(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I")
}

#[async_recursion(?Send)]
async fn get_mech_ptr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J")
}

#[async_recursion(?Send)]
async fn get_mic(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B")
}

#[async_recursion(?Send)]
async fn import_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;"
    )
}

#[async_recursion(?Send)]
async fn import_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J")
}

#[async_recursion(?Send)]
async fn indicate_mechs(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z")
}

#[async_recursion(?Send)]
async fn init_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B"
    )
}

#[async_recursion(?Send)]
async fn inquire_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J")
}

#[async_recursion(?Send)]
async fn inquire_names_for_mech(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn release_cred(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J")
}

#[async_recursion(?Send)]
async fn release_name(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V")
}

#[async_recursion(?Send)]
async fn unwrap(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[async_recursion(?Send)]
async fn verify_mic(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V")
}

#[async_recursion(?Send)]
async fn wrap(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[async_recursion(?Send)]
async fn wrap_size_limit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

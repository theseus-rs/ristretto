use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.jgss.wrapper.GSSLibStub`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/jgss/wrapper/GSSLibStub";
    registry.register(
        class_name,
        "acceptContext",
        "(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
        accept_context,
    );
    registry.register(class_name, "acquireCred", "(JII)J", acquire_cred);
    registry.register(class_name, "canonicalizeName", "(J)J", canonicalize_name);
    registry.register(class_name, "compareName", "(JJ)Z", compare_name);
    registry.register(class_name, "deleteContext", "(J)J", delete_context);
    registry.register(
        class_name,
        "displayName",
        "(J)[Ljava/lang/Object;",
        display_name,
    );
    registry.register(class_name, "exportContext", "(J)[B", export_context);
    registry.register(class_name, "exportName", "(J)[B", export_name);
    registry.register(
        class_name,
        "getContextMech",
        "(J)Lorg/ietf/jgss/Oid;",
        get_context_mech,
    );
    registry.register(class_name, "getContextName", "(JZ)J", get_context_name);
    registry.register(class_name, "getContextTime", "(J)I", get_context_time);
    registry.register(class_name, "getCredName", "(J)J", get_cred_name);
    registry.register(class_name, "getCredTime", "(J)I", get_cred_time);
    registry.register(class_name, "getCredUsage", "(J)I", get_cred_usage);
    registry.register(class_name, "getMechPtr", "([B)J", get_mech_ptr);
    registry.register(class_name, "getMic", "(JI[B)[B", get_mic);
    registry.register(
        class_name,
        "importContext",
        "([B)Lsun/security/jgss/wrapper/NativeGSSContext;",
        import_context,
    );
    registry.register(
        class_name,
        "importName",
        "([BLorg/ietf/jgss/Oid;)J",
        import_name,
    );
    registry.register(
        class_name,
        "indicateMechs",
        "()[Lorg/ietf/jgss/Oid;",
        indicate_mechs,
    );
    registry.register(class_name, "init", "(Ljava/lang/String;Z)Z", init);
    registry.register(
        class_name,
        "initContext",
        "(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B",
        init_context,
    );
    registry.register(class_name, "inquireContext", "(J)[J", inquire_context);
    registry.register(
        class_name,
        "inquireNamesForMech",
        "()[Lorg/ietf/jgss/Oid;",
        inquire_names_for_mech,
    );
    registry.register(class_name, "releaseCred", "(J)J", release_cred);
    registry.register(class_name, "releaseName", "(J)V", release_name);
    registry.register(
        class_name,
        "unwrap",
        "(J[BLorg/ietf/jgss/MessageProp;)[B",
        unwrap,
    );
    registry.register(
        class_name,
        "verifyMic",
        "(J[B[BLorg/ietf/jgss/MessageProp;)V",
        verify_mic,
    );
    registry.register(
        class_name,
        "wrap",
        "(J[BLorg/ietf/jgss/MessageProp;)[B",
        wrap,
    );
    registry.register(class_name, "wrapSizeLimit", "(JIII)I", wrap_size_limit);
}

#[async_recursion(?Send)]
async fn accept_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.acceptContext(JLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B")
}

#[async_recursion(?Send)]
async fn acquire_cred(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.acquireCred(JII)J")
}

#[async_recursion(?Send)]
async fn canonicalize_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.canonicalizeName(J)J")
}

#[async_recursion(?Send)]
async fn compare_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.compareName(JJ)Z")
}

#[async_recursion(?Send)]
async fn delete_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.deleteContext(J)J")
}

#[async_recursion(?Send)]
async fn display_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.displayName(J)[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn export_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportContext(J)[B")
}

#[async_recursion(?Send)]
async fn export_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.exportName(J)[B")
}

#[async_recursion(?Send)]
async fn get_context_mech(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextMech(J)Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn get_context_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextName(JZ)J")
}

#[async_recursion(?Send)]
async fn get_context_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getContextTime(J)I")
}

#[async_recursion(?Send)]
async fn get_cred_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredName(J)J")
}

#[async_recursion(?Send)]
async fn get_cred_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredTime(J)I")
}

#[async_recursion(?Send)]
async fn get_cred_usage(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getCredUsage(J)I")
}

#[async_recursion(?Send)]
async fn get_mech_ptr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMechPtr([B)J")
}

#[async_recursion(?Send)]
async fn get_mic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.getMic(JI[B)[B")
}

#[async_recursion(?Send)]
async fn import_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.importContext([B)Lsun/security/jgss/wrapper/NativeGSSContext;")
}

#[async_recursion(?Send)]
async fn import_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.importName([BLorg/ietf/jgss/Oid;)J")
}

#[async_recursion(?Send)]
async fn indicate_mechs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.indicateMechs()[Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.init(Ljava/lang/String;Z)Z")
}

#[async_recursion(?Send)]
async fn init_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.initContext(JJLorg/ietf/jgss/ChannelBinding;[BLsun/security/jgss/wrapper/NativeGSSContext;)[B")
}

#[async_recursion(?Send)]
async fn inquire_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireContext(J)[J")
}

#[async_recursion(?Send)]
async fn inquire_names_for_mech(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.inquireNamesForMech()[Lorg/ietf/jgss/Oid;")
}

#[async_recursion(?Send)]
async fn release_cred(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseCred(J)J")
}

#[async_recursion(?Send)]
async fn release_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.releaseName(J)V")
}

#[async_recursion(?Send)]
async fn unwrap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.unwrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[async_recursion(?Send)]
async fn verify_mic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.verifyMic(J[B[BLorg/ietf/jgss/MessageProp;)V")
}

#[async_recursion(?Send)]
async fn wrap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.wrap(J[BLorg/ietf/jgss/MessageProp;)[B")
}

#[async_recursion(?Send)]
async fn wrap_size_limit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.jgss.wrapper.GSSLibStub.wrapSizeLimit(JIII)I")
}

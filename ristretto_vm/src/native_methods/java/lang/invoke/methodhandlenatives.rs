use crate::Error::InternalError;
use crate::Result;
use crate::native_methods::registry::{JAVA_8, JAVA_11, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classloader::{Class, Object, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/invoke/MethodHandleNatives";

bitflags! {
    /// Method name flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MemberNameFlags: i32 {
        /// method (not constructor)
        const IS_METHOD = 0x0001_0000;
        /// constructor
        const IS_CONSTRUCTOR = 0x0002_0000;
        /// field
        const IS_FIELD = 0x0004_0000;
        /// nested type
        const IS_TYPE = 0x0008_0000;
        /// @CallerSensitive annotation detected
        const CALLER_SENSITIVE = 0x001_00000;
        /// trusted final field
        const TRUSTED_FINAL = 0x002_00000;
        /// refKind
        const REFERENCE_KIND_SHIFT = 24;
        /// 0x0F00_0000 >> REFERENCE_KIND_SHIFT
        const REFERENCE_KIND_MASK = 0x0F00_0000 >> 24;
    }
}

bitflags! {
    /// Lookup mode flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct LookupModeFlags: i32 {
        const MODULE = 16;
        const UNCONDITIONAL = 32;
        const TRUSTED = -1;
    }
}

/// Register all native methods for `java.lang.invoke.MethodHandleNatives`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "getConstant", "(I)I", get_constant);
        registry.register(
            CLASS_NAME,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;",
            resolve_0,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "clearCallSiteContext",
            "(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V",
            clear_call_site_context,
        );
        registry.register(
            CLASS_NAME,
            "copyOutBootstrapArguments",
            "(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V",
            copy_out_bootstrap_arguments,
        );
    }

    if registry.java_major_version() == JAVA_11 {
        registry.register(
            CLASS_NAME,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Z)Ljava/lang/invoke/MemberName;",
            resolve_1,
        );
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "getMembers", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I", get_members);
    }
    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;",
            resolve_2,
        );
    }

    registry.register(
        CLASS_NAME,
        "expand",
        "(Ljava/lang/invoke/MemberName;)V",
        expand,
    );
    registry.register(
        CLASS_NAME,
        "getMemberVMInfo",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        get_member_vm_info,
    );
    registry.register(
        CLASS_NAME,
        "getNamedCon",
        "(I[Ljava/lang/Object;)I",
        get_named_con,
    );
    registry.register(
        CLASS_NAME,
        "init",
        "(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
        init,
    );
    registry.register(
        CLASS_NAME,
        "objectFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        object_field_offset,
    );
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "setCallSiteTargetNormal",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_normal,
    );
    registry.register(
        CLASS_NAME,
        "setCallSiteTargetVolatile",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_volatile,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldBase",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        static_field_base,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        static_field_offset,
    );
}

#[async_recursion(?Send)]
async fn clear_call_site_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V"
    )
}

#[async_recursion(?Send)]
async fn copy_out_bootstrap_arguments(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn expand(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V")
}

#[async_recursion(?Send)]
async fn get_constant(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getConstant(I)I")
}

#[async_recursion(?Send)]
async fn get_member_vm_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_members(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I"
    )
}

#[async_recursion(?Send)]
async fn get_named_con(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn object_field_offset(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

async fn resolve(
    thread: Arc<Thread>,
    member_self: Object,
    _caller: Option<Arc<Class>>,
    _lookup_mode: i32,
    speculative_resolve: bool,
) -> Result<Option<Value>> {
    let class_object: Object = member_self.value("clazz")?.try_into()?;
    let name = member_self.value("name")?;
    let flags = member_self.value("flags")?.to_int()?;
    let member_name_flags = MemberNameFlags::from_bits_truncate(flags);

    let (method_name, method_descriptor, method_parameters) = if member_name_flags
        .contains(MemberNameFlags::IS_METHOD)
        || member_name_flags.contains(MemberNameFlags::IS_CONSTRUCTOR)
    {
        let method_type = member_self.value("type")?;
        let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
        let ptypes_method = method_type_class.try_get_method("ptypes", "()[Ljava/lang/Class;")?;
        let parameter_types = thread
            .try_execute(
                &method_type_class,
                &ptypes_method,
                vec![method_type.clone()],
            )
            .await?;
        (
            "getMethod",
            "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;",
            vec![Value::from(class_object), name, parameter_types],
        )
    } else if member_name_flags.contains(MemberNameFlags::IS_FIELD) {
        (
            "getField",
            "(Ljava/lang/String;)Ljava/lang/reflect/Field;",
            vec![Value::from(class_object)],
        )
    } else {
        return Err(InternalError(format!(
            "Unsupported member name flag: {member_name_flags:?}"
        )));
    };

    let class = thread.class("java.lang.Class").await?;
    let get_method = class.try_get_method(method_name, method_descriptor)?;
    match thread
        .try_execute(&class, &get_method, method_parameters)
        .await
    {
        Ok(object) => {
            let object: Object = object.try_into()?;
            let modifiers: i32 = object.value("modifiers")?.to_int()?;
            let flags = flags | modifiers;
            member_self.set_value("flags", Value::from(flags))?;
            Ok(Some(member_self.into()))
        }
        Err(error) => {
            if speculative_resolve {
                Ok(None)
            } else {
                Err(error)
            }
        }
    }
}

#[async_recursion(?Send)]
async fn resolve_0(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop_object()?;
    resolve(thread, member_self, caller, -1, true).await
}

#[async_recursion(?Send)]
async fn resolve_1(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop_object()?;
    resolve(thread, member_self, caller, -1, speculative_resolve).await
}

#[async_recursion(?Send)]
async fn resolve_2(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let lookup_mode = parameters.pop_int()?;
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop_object()?;
    resolve(
        thread,
        member_self,
        caller,
        lookup_mode,
        speculative_resolve,
    )
    .await
}

#[async_recursion(?Send)]
async fn set_call_site_target_normal(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )
}

#[async_recursion(?Send)]
async fn set_call_site_target_volatile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )
}

#[async_recursion(?Send)]
async fn static_field_base(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn static_field_offset(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V"
    )]
    async fn test_clear_call_site_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_call_site_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V"
    )]
    async fn test_copy_out_bootstrap_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_out_bootstrap_arguments(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V"
    )]
    async fn test_expand() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = expand(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getConstant(I)I"
    )]
    async fn test_get_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_constant(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )]
    async fn test_get_member_vm_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_member_vm_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I"
    )]
    async fn test_get_members() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_members(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I"
    )]
    async fn test_get_named_con() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_named_con(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J"
    )]
    async fn test_object_field_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = object_field_offset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )]
    async fn test_set_call_site_target_normal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_call_site_target_normal(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )]
    async fn test_set_call_site_target_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_call_site_target_volatile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )]
    async fn test_static_field_base() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_base(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J"
    )]
    async fn test_static_field_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_offset(thread, Parameters::default()).await;
    }
}

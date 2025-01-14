use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Class, Object, Value};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.lang.invoke.MethodHandleNatives`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/MethodHandleNatives";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(class_name, "getConstant", "(I)I", get_constant);
        registry.register(
            class_name,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;",
            resolve_0,
        );
    } else {
        registry.register(
            class_name,
            "clearCallSiteContext",
            "(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V",
            clear_call_site_context,
        );
        registry.register(
            class_name,
            "copyOutBootstrapArguments",
            "(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V",
            copy_out_bootstrap_arguments,
        );
        registry.register(
            class_name,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Z)Ljava/lang/invoke/MemberName;",
            resolve_1,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "resolve",
            "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;",
            resolve_2,
        );
    }

    if java_version <= JAVA_20 {
        registry.register(class_name, "getMembers", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I", get_members);
    }

    registry.register(
        class_name,
        "expand",
        "(Ljava/lang/invoke/MemberName;)V",
        expand,
    );
    registry.register(
        class_name,
        "getMemberVMInfo",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        get_member_vm_info,
    );
    registry.register(
        class_name,
        "getNamedCon",
        "(I[Ljava/lang/Object;)I",
        get_named_con,
    );
    registry.register(
        class_name,
        "init",
        "(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
        init,
    );
    registry.register(
        class_name,
        "objectFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        object_field_offset,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setCallSiteTargetNormal",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_normal,
    );
    registry.register(
        class_name,
        "setCallSiteTargetVolatile",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_volatile,
    );
    registry.register(
        class_name,
        "staticFieldBase",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        static_field_base,
    );
    registry.register(
        class_name,
        "staticFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        static_field_offset,
    );
}

#[async_recursion(?Send)]
async fn clear_call_site_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V")
}

#[async_recursion(?Send)]
async fn copy_out_bootstrap_arguments(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn expand(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V")
}

#[async_recursion(?Send)]
async fn get_constant(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getConstant(I)I")
}

#[async_recursion(?Send)]
async fn get_member_vm_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_members(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I")
}

#[async_recursion(?Send)]
async fn get_named_con(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn object_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

async fn resolve(
    thread: Arc<Thread>,
    member_self: Object,
    _caller: Option<Arc<Class>>,
    _lookup_mode: i32,
    _speculative_resolve: bool,
) -> Result<Option<Value>> {
    let class_object: Object = member_self.value("clazz")?.try_into()?;
    let class_name: String = class_object.value("name")?.try_into()?;
    let class = thread.class(class_name).await?;
    let name: String = member_self.value("name")?.try_into()?;

    let method_type = member_self.value("type")?;
    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    let method_type_descriptor_method =
        method_type_class.try_get_method("toMethodDescriptorString", "()Ljava/lang/String;")?;
    let descriptor: String = thread
        .try_execute(
            &method_type_class,
            &method_type_descriptor_method,
            vec![method_type],
        )
        .await?
        .try_into()?;

    // Lookup modes:
    // LM_MODULE = 16;
    // LM_UNCONDITIONAL = 32;
    // LM_TRUSTED = -1;

    let method = class.try_get_method(name, descriptor)?;
    let access_flags = i32::from(method.access_flags().bits());
    let flags: i32 = member_self.value("flags")?.try_into()?;
    let flags = flags | access_flags;
    member_self.set_value("flags", Value::from(flags))?;

    let resolved_method_name_class = thread.class("java.lang.invoke.ResolvedMethodName").await?;
    let resolved_method_name = Object::new(resolved_method_name_class)?;
    // TODO: the ResolvedMethodName class appears to be a holder for the JVM-internal class and
    // method references.  This should evaluated to determine if it is necessary to implement.
    member_self.set_value("method", Value::from(resolved_method_name))?;

    let member_self = Value::from(member_self);
    Ok(Some(member_self))
}

#[async_recursion(?Send)]
async fn resolve_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let caller = match arguments.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = arguments.pop_object()?;
    resolve(thread, member_self, caller, -1, true).await
}

#[async_recursion(?Send)]
async fn resolve_1(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let speculative_resolve = arguments.pop_int()? != 0;
    let caller = match arguments.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = arguments.pop_object()?;
    resolve(thread, member_self, caller, -1, speculative_resolve).await
}

#[async_recursion(?Send)]
async fn resolve_2(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let speculative_resolve = arguments.pop_int()? != 0;
    let lookup_mode = arguments.pop_int()?;
    let caller = match arguments.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = arguments.pop_object()?;
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
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V")
}

#[async_recursion(?Send)]
async fn set_call_site_target_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V")
}

#[async_recursion(?Send)]
async fn static_field_base(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn static_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java20 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/invoke/MethodHandleNatives";
        assert!(registry
            .method(
                class_name,
                "clearCallSiteContext",
                "(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "copyOutBootstrapArguments",
                "(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "expand", "(Ljava/lang/invoke/MemberName;)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getMemberVMInfo",
                "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getMembers",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getNamedCon", "(I[Ljava/lang/Object;)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "init",
                "(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "objectFieldOffset",
                "(Ljava/lang/invoke/MemberName;)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "resolve",
                "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Z)Ljava/lang/invoke/MemberName;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "resolve",
                "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setCallSiteTargetNormal",
                "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setCallSiteTargetVolatile",
                "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "staticFieldBase",
                "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "staticFieldOffset",
                "(Ljava/lang/invoke/MemberName;)J"
            )
            .is_some());
    }
    #[test]
    fn test_register_java_8() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/invoke/MethodHandleNatives";
        assert!(registry.method(class_name, "getConstant", "(I)I").is_some());
        assert!(registry
            .method(
                class_name,
                "resolve",
                "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V"
    )]
    async fn test_clear_call_site_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_call_site_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V"
    )]
    async fn test_copy_out_bootstrap_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_out_bootstrap_arguments(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V"
    )]
    async fn test_expand() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = expand(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getConstant(I)I"
    )]
    async fn test_get_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_constant(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )]
    async fn test_get_member_vm_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_member_vm_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I"
    )]
    async fn test_get_members() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_members(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I"
    )]
    async fn test_get_named_con() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_named_con(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J"
    )]
    async fn test_object_field_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = object_field_offset(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )]
    async fn test_set_call_site_target_normal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_call_site_target_normal(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V"
    )]
    async fn test_set_call_site_target_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_call_site_target_volatile(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )]
    async fn test_static_field_base() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_base(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J"
    )]
    async fn test_static_field_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_offset(thread, Arguments::default()).await;
    }
}

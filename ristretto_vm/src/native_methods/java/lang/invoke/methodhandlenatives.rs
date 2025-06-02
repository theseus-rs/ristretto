use crate::Error::InternalError;
use crate::Result;
use crate::native_methods::registry::{JAVA_8, JAVA_11, JAVA_17, JAVA_21, MethodRegistry};
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
        if registry.java_major_version() <= JAVA_21 {
            registry.register(
                CLASS_NAME,
                "clearCallSiteContext",
                "(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V",
                clear_call_site_context,
            );
        }

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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop_object()?;
    let class_object: Object = member_name.value("clazz")?.try_into()?;
    let class = class_object.class();
    let field_name: String = member_name.value("name")?.try_into()?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
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
    _speculative_resolve: bool,
) -> Result<Option<Value>> {
    let class_object: Object = member_self.value("clazz")?.try_into()?;
    let class_name: String = class_object.value("name")?.try_into()?;
    let class = thread.class(class_name).await?;
    let name = member_self.value("name")?;
    let flags = member_self.value("flags")?.to_int()?;
    let member_name_flags = MemberNameFlags::from_bits_truncate(flags);

    if member_name_flags.contains(MemberNameFlags::IS_METHOD)
        || member_name_flags.contains(MemberNameFlags::IS_CONSTRUCTOR)
    {
        let method_type: Object = member_self.value("type")?.try_into()?;
        let parameter_types = method_type.value("ptypes")?;
        let parameters: Vec<Value> = parameter_types.try_into()?;
        let mut parameter_descriptors = Vec::with_capacity(parameters.len());
        for parameter in parameters {
            let class_object: Object = parameter.try_into()?;
            let class_name: String = class_object.value("name")?.try_into()?;
            let descriptor = Class::to_descriptor(&class_name);
            parameter_descriptors.push(descriptor);
        }
        let return_type: Object = method_type.value("rtype")?.try_into()?;
        let return_class_name: String = return_type.value("name")?.try_into()?;
        let return_descriptor = Class::to_descriptor(&return_class_name);

        let method_name: String = name.try_into()?;
        let method_descriptor = format!("({}){return_descriptor}", parameter_descriptors.concat());
        let method = class.try_get_method(method_name, method_descriptor)?;
        let method_access_flags = method.access_flags().bits();
        let modifiers = i32::from(method_access_flags);
        let flags = flags | modifiers;
        member_self.set_value("flags", Value::from(flags))?;
        Ok(Some(member_self.into()))
    } else if member_name_flags.contains(MemberNameFlags::IS_FIELD) {
        let field_name: String = name.try_into()?;
        let field = class_object.field(&field_name)?;
        let field_access_flags = field.access_flags().bits();
        let modifiers = i32::from(field_access_flags);
        let flags = flags | modifiers;
        member_self.set_value("flags", Value::from(flags))?;
        Ok(Some(member_self.into()))
    } else {
        Err(InternalError(format!(
            "Unsupported member name flag: {member_name_flags:?}"
        )))
    }
}

#[async_recursion(?Send)]
async fn resolve_0(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    // Correct parameter order: pop MemberName first, then caller (Class)
    let member_self = parameters.pop_object()?;
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller: Arc<Class> = caller.try_into()?;
            Some(caller)
        }
        Err(_) => None,
    };
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let call_site = parameters.pop_object()?;
    let method_handle = parameters.pop_object()?;
    call_site.set_value("target", Value::from(method_handle))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_call_site_target_volatile(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let call_site = parameters.pop_object()?;
    let method_handle = parameters.pop_object()?;
    call_site.set_value("target", Value::from(method_handle))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn static_field_base(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop_object()?;
    let class = member_name.value("clazz")?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn static_field_offset(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop_object()?;
    let class_object: Object = member_name.value("clazz")?.try_into()?;
    let class = class_object.class();
    let field_name: String = member_name.value("name")?.try_into()?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;
    use ristretto_classloader::Reference;

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
    async fn test_object_field_offset() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Reference::from(Object::new(class_object)?);
        member_name.set_value("clazz", Value::from(class))?;
        let value_string = "value".to_object(&vm).await?;
        member_name.set_value("name", value_string)?;
        parameters.push_reference(Some(Reference::from(member_name)));
        let result = object_field_offset(thread, parameters).await?;
        assert_eq!(Some(Value::Long(7)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_normal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Reference::from(Object::new(method_handle_class)?);
        parameters.push_reference(Some(method_handle));
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site = Reference::from(Object::new(call_site_class)?);
        parameters.push_reference(Some(call_site));
        let result = set_call_site_target_normal(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_volatile() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Reference::from(Object::new(method_handle_class)?);
        parameters.push_reference(Some(method_handle));
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site = Reference::from(Object::new(call_site_class)?);
        parameters.push_reference(Some(call_site));
        let result = set_call_site_target_volatile(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_base() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Reference::from(Object::new(class_object)?);
        member_name.set_value("clazz", Value::from(class.clone()))?;
        parameters.push_reference(Some(Reference::from(member_name)));
        let result = static_field_base(thread, parameters).await?;
        assert_eq!(Some(Value::from(class)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_offset() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Reference::from(Object::new(class_object)?);
        member_name.set_value("clazz", Value::from(class))?;
        let value_string = "MAX_VALUE".to_object(&vm).await?;
        member_name.set_value("name", value_string)?;
        parameters.push_reference(Some(Reference::from(member_name)));
        let result = static_field_offset(thread, parameters).await?;
        assert_eq!(Some(Value::Long(2)), result);
        Ok(())
    }
}

use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::{
    Any, Between, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{FieldAccessFlags, JAVA_8, JAVA_11, JAVA_17, JAVA_21, MethodAccessFlags};
use ristretto_classloader::Error::IllegalAccessError;
use ristretto_classloader::{Class, Method, Object, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

bitflags! {
    /// Member name flags.
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
    ///
    /// # References
    ///
    /// - [java.lang.invoke.MethodHandles.Lookup fields](https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/invoke/MethodHandles.Lookup.html#field-summary)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct LookupModeFlags: i32 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const PACKAGE = 0x0008;
        const MODULE = 0x0010;
        const UNCONDITIONAL = 0x0020;
        const ORIGINAL = 0x0040;
        const TRUSTED = -1;
    }
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn clear_call_site_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.clearCallSiteContext(Ljava/lang/invoke/MethodHandleNatives$CallSiteContext;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn copy_out_bootstrap_arguments(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn expand(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getConstant(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_constant(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getConstant(I)I")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_member_vm_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_members(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_named_con(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.init(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn object_field_offset(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop_object()?;
    let class_object: Object = member_name.value("clazz")?.try_into()?;
    let class = get_class(&thread, &class_object).await?;
    let field_name: String = member_name.value("name")?.try_into()?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

/// Resolves a method handle for the `DirectMethodHandle$Holder` class; this is a special case that
/// handles the method resolution for direct method handles in the JVM.
fn resolve_direct_method_handle_holder(
    class: Arc<Class>,
    method_name: &str,
    method_descriptor: &str,
) -> Result<Arc<Method>> {
    if let Ok(method) = class.try_get_method(method_name, method_descriptor) {
        return Ok(method);
    }
    let method_descriptor = "([Ljava/lang/Object;)Ljava/lang/Object;";
    let method = class.try_get_method(method_name, method_descriptor)?;
    Ok(method)
}

#[intrinsic_method("java/lang/invoke/MethodHandleNatives.registerNatives()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

pub(crate) async fn resolve(
    thread: Arc<Thread>,
    member_self: Object,
    caller: Option<Arc<Class>>,
    lookup_mode_flags: &LookupModeFlags,
    speculative_resolve: bool,
) -> Result<Option<Value>> {
    let class_object: Object = member_self.value("clazz")?.try_into()?;
    let class_name: String = class_object.value("name")?.try_into()?;
    let class = thread.class(class_name.clone()).await?;
    let name = member_self.value("name")?;
    let flags = member_self.value("flags")?.to_int()?;
    let member_name_flags = MemberNameFlags::from_bits_truncate(flags);

    // Handle methods/constructors
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
        let class_name: String = class_object.value("name")?.try_into()?;
        let return_type: Object = method_type.value("rtype")?.try_into()?;
        let return_class_name: String = return_type.value("name")?.try_into()?;
        let return_descriptor = Class::to_descriptor(&return_class_name);

        let method_name: String = name.try_into()?;
        let method_descriptor = format!("({}){return_descriptor}", parameter_descriptors.concat());
        let method = if &class_name == "java.lang.invoke.DirectMethodHandle$Holder" {
            resolve_direct_method_handle_holder(class.clone(), &method_name, &method_descriptor)?
        } else {
            class.try_get_method(method_name.clone(), method_descriptor.clone())?
        };

        // Access control enforcement
        let method_access_flags = method.access_flags();
        if !check_method_access(caller, &class, method_access_flags, lookup_mode_flags)? {
            return if speculative_resolve {
                // If speculative, return None (fail silently)
                Ok(None)
            } else {
                Err(IllegalAccessError(format!(
                    "member is {}: {}.{}{}",
                    if method_access_flags.contains(MethodAccessFlags::PRIVATE) {
                        "private"
                    } else {
                        "inaccessible"
                    },
                    class_name,
                    method_name,
                    method_descriptor,
                ))
                .into())
            };
        }

        let modifiers = i32::from(method_access_flags.bits());
        let flags = flags | modifiers;
        member_self.set_value("flags", Value::from(flags))?;
        Ok(Some(member_self.into()))
    }
    // Handle fields (for both normal field and VarHandle)
    else if member_name_flags.contains(MemberNameFlags::IS_FIELD) {
        let field_name: String = name.try_into()?;
        let field = class.declared_field(&field_name)?;
        let field_access_flags = field.access_flags();
        if !check_field_access(caller, &class, field_access_flags, lookup_mode_flags)? {
            return if speculative_resolve {
                Ok(None)
            } else {
                Err(IllegalAccessError(format!(
                    "member is {}: {}.{}",
                    if field_access_flags.contains(FieldAccessFlags::PRIVATE) {
                        "private"
                    } else {
                        "inaccessible"
                    },
                    class_name,
                    field_name,
                ))
                .into())
            };
        }
        let modifiers = i32::from(field_access_flags.bits());
        let flags = flags | modifiers;
        member_self.set_value("flags", Value::from(flags))?;
        Ok(Some(member_self.into()))
    } else {
        Err(InternalError(format!(
            "Unsupported member name flag: {member_name_flags:?}"
        )))
    }
}

/// Returns true if `caller` is permitted to access a method of `declaring` with the given access
/// flags.
///
/// # References
///
/// - [JLS §6.6 Access Control](https://docs.oracle.com/javase/specs/jls/se24/html/jls-6.html#jls-6.6)
pub fn check_method_access(
    caller: Option<Arc<Class>>,
    declaring: &Arc<Class>,
    method_access_flags: &MethodAccessFlags,
    lookup_mode_flags: &LookupModeFlags,
) -> Result<bool> {
    if lookup_mode_flags.contains(LookupModeFlags::TRUSTED) {
        return Ok(true);
    }

    // 1. PUBLIC: accessible everywhere
    if method_access_flags.contains(MethodAccessFlags::PUBLIC) {
        return Ok(true);
    }

    let Some(ref caller) = caller else {
        return Ok(false);
    };

    // 2. PRIVATE: accessible only from the same class
    if method_access_flags.contains(MethodAccessFlags::PRIVATE) {
        if Arc::ptr_eq(caller, declaring) {
            return Ok(true);
        }
        return Ok(false);
    }

    // 3. PROTECTED: accessible to subclasses (and package; Java's rules)
    if method_access_flags.contains(MethodAccessFlags::PROTECTED) {
        if caller.package() == declaring.package() || caller.is_subclass_of(declaring)? {
            return Ok(true);
        }
        return Ok(false);
    }

    // 4. PACKAGE-PRIVATE (default): accessible to same-package classes only
    // If not public/private/protected, then it's package-private
    if caller.package() == declaring.package() {
        return Ok(true);
    }

    // Not accessible
    Ok(false)
}

/// Returns true if `caller` is permitted to access a field of `declaring` with the given access
/// flags.
///
/// # References
///
/// - [JLS §6.6 Access Control](https://docs.oracle.com/javase/specs/jls/se24/html/jls-6.html#jls-6.6)
pub fn check_field_access(
    caller: Option<Arc<Class>>,
    declaring: &Arc<Class>,
    field_access_flags: &FieldAccessFlags,
    lookup_mode_flags: &LookupModeFlags,
) -> Result<bool> {
    if lookup_mode_flags.contains(LookupModeFlags::TRUSTED) {
        return Ok(true);
    }

    // 1. PUBLIC: accessible everywhere
    if field_access_flags.contains(FieldAccessFlags::PUBLIC) {
        return Ok(true);
    }

    let Some(ref caller) = caller else {
        return Ok(false);
    };

    // 2. PRIVATE: accessible only from the same class
    if field_access_flags.contains(FieldAccessFlags::PRIVATE) {
        if Arc::ptr_eq(caller, declaring) {
            return Ok(true);
        }
        return Ok(false);
    }

    // 3. PROTECTED: accessible to subclasses (and package; Java's rules)
    if field_access_flags.contains(FieldAccessFlags::PROTECTED) {
        if caller.package() == declaring.package() || caller.is_subclass_of(declaring)? {
            return Ok(true);
        }
        return Ok(false);
    }

    // 4. PACKAGE-PRIVATE (default): accessible to same-package classes only
    // If not public/private/protected, then it's package-private
    if caller.package() == declaring.package() {
        return Ok(true);
    }

    // Not accessible
    Ok(false)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn resolve_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Correct parameter order: pop MemberName first, then caller (Class)
    let member_self = parameters.pop_object()?;
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller = get_class(&thread, &caller).await?;
            Some(caller)
        }
        Err(_) => None,
    };
    resolve(
        thread,
        member_self,
        caller,
        &LookupModeFlags::empty(),
        false,
    )
    .await
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Z)Ljava/lang/invoke/MemberName;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn resolve_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller = get_class(&thread, &caller).await?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop_object()?;
    resolve(
        thread,
        member_self,
        caller,
        &LookupModeFlags::empty(),
        speculative_resolve,
    )
    .await
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn resolve_2(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let lookup_mode = LookupModeFlags::from_bits_truncate(parameters.pop_int()?);
    let caller = match parameters.pop_object() {
        Ok(caller) => {
            let caller = get_class(&thread, &caller).await?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop_object()?;
    resolve(
        thread,
        member_self,
        caller,
        &lookup_mode,
        speculative_resolve,
    )
    .await
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_call_site_target_normal(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let call_site = parameters.pop_object()?;
    let method_handle = parameters.pop_object()?;
    call_site.set_value("target", Value::from(method_handle))?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_call_site_target_volatile(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let call_site = parameters.pop_object()?;
    let method_handle = parameters.pop_object()?;
    call_site.set_value("target", Value::from(method_handle))?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_base(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop_object()?;
    let class = member_name.value("clazz")?;
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_offset(
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Reference::from(Object::new(class_object)?);
        member_name.set_value("clazz", Value::from(class))?;
        let value_string = "value".to_object(&thread).await?;
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Reference::from(Object::new(class_object)?);
        member_name.set_value("clazz", Value::from(class))?;
        let value_string = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", value_string)?;
        parameters.push_reference(Some(Reference::from(member_name)));
        let result = static_field_offset(thread, parameters).await?;
        assert_eq!(Some(Value::Long(2)), result);
        Ok(())
    }
}

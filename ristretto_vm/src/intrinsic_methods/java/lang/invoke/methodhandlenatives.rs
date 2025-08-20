use crate::Error::InternalError;
use crate::intrinsic_methods::java::lang::class::get_class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::{
    Any, Between, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{
    FieldAccessFlags, JAVA_8, JAVA_11, JAVA_17, JAVA_21, MethodAccessFlags, ReferenceKind,
};
use ristretto_classloader::Error::IllegalAccessError;
use ristretto_classloader::{Class, Method, Value};
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
        const REFERENCE_KIND_MASK = 0x0F00_0000 >> MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
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
    let member_name = parameters.pop()?;
    let (class_object, field_name) = {
        let member_name = member_name.as_object_ref()?;
        let class_object = member_name.value("clazz")?;
        let field_name = member_name.value("name")?.as_string()?;
        (class_object, field_name)
    };
    let class = get_class(&thread, &class_object).await?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

/// Resolves synthetic methods for `...$Holder` classes.
#[expect(clippy::needless_pass_by_value)]
fn resolve_holder_methods(
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
    member_self: Value,
    caller: Option<Arc<Class>>,
    lookup_mode_flags: &LookupModeFlags,
    speculative_resolve: bool,
) -> Result<Option<Value>> {
    let (class_object, name, flags) = {
        let member_self = member_self.as_object_ref()?;
        let class_object = member_self.value("clazz")?;
        let name = member_self.value("name")?;
        let flags = member_self.value("flags")?.as_i32()?;
        (class_object, name, flags)
    };
    let class_name = {
        let class_object = class_object.as_object_ref()?;
        class_object.value("name")?.as_string()?
    };
    let class = thread.class(class_name).await?;
    let member_name_flags = MemberNameFlags::from_bits_truncate(flags);

    if member_name_flags.contains(MemberNameFlags::IS_METHOD)
        || member_name_flags.contains(MemberNameFlags::IS_CONSTRUCTOR)
    {
        resolve_method(
            &thread,
            member_self,
            &caller,
            lookup_mode_flags,
            speculative_resolve,
            &name,
            flags,
            &class,
        )
        .await
    } else if member_name_flags.contains(MemberNameFlags::IS_FIELD) {
        resolve_field(
            &thread,
            member_self,
            caller,
            lookup_mode_flags,
            speculative_resolve,
            name,
            flags,
            &class,
        )
        .await
    } else {
        Err(InternalError(format!(
            "Unsupported member name flag: {member_name_flags:?}"
        )))
    }
}

/// Resolves a method in the given class, checking access permissions and returning the member self
/// if successful.
#[expect(clippy::too_many_arguments)]
async fn resolve_method(
    thread: &Thread,
    member_self: Value,
    caller: &Option<Arc<Class>>,
    lookup_mode_flags: &LookupModeFlags,
    speculative_resolve: bool,
    name: &Value,
    flags: i32,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    let _reference_kind = get_reference_kind(flags)?;
    let method_type = {
        let member_self = member_self.as_object_ref()?;
        member_self.value("type")?
    };

    let (parameter_descriptors, return_descriptor) = {
        let method_type = method_type.as_object_ref()?;
        let parameter_types = method_type.value("ptypes")?;
        let parameters: Vec<Value> = parameter_types.try_into()?;
        let mut parameter_descriptors = Vec::with_capacity(parameters.len());
        for parameter in parameters {
            let class_object = parameter.as_object_ref()?;
            let class_name = class_object.value("name")?.as_string()?;
            let descriptor = Class::convert_to_descriptor(&class_name);
            parameter_descriptors.push(descriptor);
        }
        let return_type = method_type.value("rtype")?;
        let return_type = return_type.as_object_ref()?;
        let return_class_name = return_type.value("name")?.as_string()?;
        let return_descriptor = Class::convert_to_descriptor(&return_class_name);
        (parameter_descriptors, return_descriptor)
    };

    let method_name = name.as_string()?;
    let method_descriptor = format!("({}){return_descriptor}", parameter_descriptors.concat());
    let method = match class.name() {
        "java.lang.invoke.DelegatingMethodHandle$Holder"
        | "java.lang.invoke.DirectMethodHandle$Holder"
        | "java.lang.invoke.Invokers$Holder" => {
            resolve_holder_methods(class.clone(), &method_name, &method_descriptor)?
        }
        _ => class.try_get_method(&method_name, &method_descriptor)?,
    };

    // Access control enforcement
    let method_access_flags = method.access_flags();
    if !check_method_access(caller, class, *method_access_flags, *lookup_mode_flags)? {
        return if speculative_resolve {
            // If speculative, return None (fail silently)
            Ok(None)
        } else {
            Err(IllegalAccessError(format!(
                "member is {}: {}.{method_name}{method_descriptor}",
                if method_access_flags.contains(MethodAccessFlags::PRIVATE) {
                    "private"
                } else {
                    "inaccessible"
                },
                class.name(),
            ))
            .into())
        };
    }

    let modifiers = i32::from(method_access_flags.bits());
    let flags = flags | modifiers;
    {
        let vm = thread.vm()?;
        let member_handles = vm.member_handles();
        let method_signature =
            format!("{}.{}{}", class.name(), method.name(), method.descriptor(),);
        member_handles
            .insert(method_signature, method.into())
            .await?;
        let vmindex = method_descriptor.to_object(thread).await?;
        let mut member_self = member_self.as_object_mut()?;
        member_self.set_value("flags", Value::from(flags))?;
        member_self.set_value("vmindex", vmindex)?;
    }
    Ok(Some(member_self))
}

/// Resolves a field in the given class, checking access permissions and returning the member self
/// if successful.
#[expect(clippy::too_many_arguments)]
async fn resolve_field(
    thread: &Thread,
    member_self: Value,
    caller: Option<Arc<Class>>,
    lookup_mode_flags: &LookupModeFlags,
    speculative_resolve: bool,
    name: Value,
    flags: i32,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    let _reference_kind = get_reference_kind(flags)?;
    let field_name = name.as_string()?;
    let field = class.declared_field(&field_name)?;
    let field_access_flags = field.access_flags();
    if !check_field_access(caller, class, *field_access_flags, *lookup_mode_flags)? {
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
                class.name(),
                field_name,
            ))
            .into())
        };
    }
    let modifiers = i32::from(field_access_flags.bits());
    let flags = flags | modifiers;
    {
        let vm = thread.vm()?;
        let member_handles = vm.member_handles();
        let field_offset = class.field_offset(&field_name)?;
        let field_signature = format!("{}.{field_name}", class.name(),);
        member_handles
            .insert(field_signature.clone(), field_offset.into())
            .await?;
        let vmindex = field_signature.to_object(thread).await?;
        let mut member_self = member_self.as_object_mut()?;
        member_self.set_value("flags", Value::from(flags))?;
        member_self.set_value("vmindex", vmindex)?;
    }
    Ok(Some(member_self))
}

/// Extracts the reference kind from the flags of a member name.
fn get_reference_kind(flags: i32) -> Result<ReferenceKind> {
    let flags = flags as u32;
    let shift = MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
    let mask = MemberNameFlags::REFERENCE_KIND_MASK.bits() as u32;
    let reference_kind = ((flags >> shift) & mask) as u8;
    ReferenceKind::try_from(reference_kind).map_err(Into::into)
}

/// Returns true if `caller` is permitted to access a method of `declaring` with the given access
/// flags.
///
/// # References
///
/// - [JLS §6.6 Access Control](https://docs.oracle.com/javase/specs/jls/se24/html/jls-6.html#jls-6.6)
pub fn check_method_access(
    caller: &Option<Arc<Class>>,
    declaring: &Arc<Class>,
    method_access_flags: MethodAccessFlags,
    lookup_mode_flags: LookupModeFlags,
) -> Result<bool> {
    if lookup_mode_flags.contains(LookupModeFlags::TRUSTED) {
        return Ok(true);
    }

    // 1. PUBLIC: accessible everywhere
    if method_access_flags.contains(MethodAccessFlags::PUBLIC) {
        return Ok(true);
    }

    let Some(caller) = caller else {
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
#[expect(clippy::needless_pass_by_value)]
pub fn check_field_access(
    caller: Option<Arc<Class>>,
    declaring: &Arc<Class>,
    field_access_flags: FieldAccessFlags,
    lookup_mode_flags: LookupModeFlags,
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
    let member_self = parameters.pop()?;
    let caller = match parameters.pop() {
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
    let caller = match parameters.pop() {
        Ok(caller) => {
            let caller = get_class(&thread, &caller).await?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop()?;
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
    let caller = match parameters.pop() {
        Ok(caller) => {
            let caller = get_class(&thread, &caller).await?;
            Some(caller)
        }
        Err(_) => None,
    };
    let member_self = parameters.pop()?;
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
    let call_site = parameters.pop()?;
    let mut call_site = call_site.as_object_mut()?;
    let method_handle = parameters.pop()?;
    call_site.set_value("target", method_handle)?;
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
    let call_site = parameters.pop()?;
    let mut call_site = call_site.as_object_mut()?;
    let method_handle = parameters.pop()?;
    call_site.set_value("target", method_handle)?;
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
    let member_name = parameters.pop()?;
    let member_name = member_name.as_object_ref()?;
    let class = member_name.value("clazz")?;
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_offset(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop()?;
    let (class_object, field_name) = {
        let member_name = member_name.as_object_ref()?;
        let class_object = member_name.value("clazz")?;
        let field_name = member_name.value("name")?.as_string()?;
        (class_object, field_name)
    };
    let class = get_class(&thread, &class_object).await?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;
    use ristretto_classloader::Object;

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
        let mut member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Value::from(Object::new(class_object)?);
        member_name.set_value("clazz", class)?;
        let value_string = "value".to_object(&thread).await?;
        member_name.set_value("name", value_string)?;
        parameters.push(Value::from(member_name));
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

    #[test]
    fn test_get_reference_kind() -> Result<()> {
        assert_eq!(
            get_reference_kind(0x0601_0000)?,
            ReferenceKind::InvokeStatic
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_normal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Value::from(Object::new(method_handle_class)?);
        parameters.push(method_handle);
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site = Value::from(Object::new(call_site_class)?);
        parameters.push(call_site);
        let result = set_call_site_target_normal(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_volatile() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Value::from(Object::new(method_handle_class)?);
        parameters.push(method_handle);
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site = Value::from(Object::new(call_site_class)?);
        parameters.push(call_site);
        let result = set_call_site_target_volatile(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_base() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Value::from(Object::new(class_object)?);
        member_name.set_value("clazz", class.clone())?;
        parameters.push(Value::from(member_name));
        let result = static_field_base(thread, parameters).await?;
        assert_eq!(Some(class), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;
        let class_object = thread.class("java.lang.Integer").await?;
        let class = Value::from(Object::new(class_object)?);
        member_name.set_value("clazz", class)?;
        let value_string = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", value_string)?;
        parameters.push(Value::from(member_name));
        let result = static_field_offset(thread, parameters).await?;
        assert_eq!(Some(Value::Long(2)), result);
        Ok(())
    }
}

use crate::java::lang::class::get_class;
use crate::jdk::internal::misc::r#unsafe::STATIC_FIELD_OFFSET_MASK;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::{
    Any, Between, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{
    Constant, ConstantPool, FieldAccessFlags, FieldType, JAVA_8, JAVA_11, JAVA_17, JAVA_21,
    MethodAccessFlags, ReferenceKind,
};
use ristretto_classloader::Error::IllegalAccessError;
use ristretto_classloader::{Class, Method, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Parameters;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{JavaObject, Result};
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
        /// Search superclasses
        const SUPERCLASSES = 0x0010_0000;
        /// Search interfaces
        const INTERFACES = 0x0020_0000;
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
    /// - [java.lang.invoke.MethodHandles.Lookup fields](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/lang/invoke/MethodHandles.Lookup.html#field-summary)
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
#[async_method]
pub async fn clear_call_site_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // This method is called to clear the context for a call site, typically during garbage
    // collection or when invalidating call sites. The CallSiteContext object contains references
    // that may need to be cleared. This is currently a no-op as ristretto does not currently
    // maintain separate call site context tracking.
    let _context = parameters.pop()?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn copy_out_bootstrap_arguments<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let if_not_available = parameters.pop()?;
    let resolve = parameters.pop_int()? != 0;
    let pos = parameters.pop_int()?;
    let buffer = parameters.pop()?;
    let end = parameters.pop_int()?;
    let start = parameters.pop_int()?;
    let index_info = parameters.pop()?;
    let caller = parameters.pop()?;

    // Get the class from the caller
    let class = get_class(&thread, &caller).await?;
    let constant_pool = class.constant_pool();

    // Get the index info array (contains bootstrap argument indices as int[])
    let index_info_array: Vec<i32> = {
        let index_info_vec = index_info.as_int_vec_ref()?;
        index_info_vec.to_vec()
    };

    // Copy arguments from start to end
    let start = usize::try_from(start)?;
    let end = usize::try_from(end)?;
    let mut pos = usize::try_from(pos)?;
    let mut values = Vec::new();

    for i in start..end {
        let cp_index = if let Some(idx) = index_info_array.get(i) {
            u16::try_from(*idx)?
        } else {
            continue;
        };

        let value = if resolve {
            // Resolve the constant pool entry to a Java object
            match constant_pool.try_get(cp_index) {
                Ok(constant) => resolve_constant_to_value(&thread, constant_pool, constant).await?,
                Err(_) => if_not_available.clone(),
            }
        } else {
            // Return the raw constant pool index or if_not_available
            if_not_available.clone()
        };
        values.push(value);
    }

    // Get mutable reference to the output buffer
    let mut buffer_reference = buffer.as_reference_mut()?;
    let buffer_array = match &mut *buffer_reference {
        Reference::Array(arr) => &mut arr.elements,
        _ => return Err(InternalError("buf is not an array".to_string())),
    };

    for value in values {
        if pos >= buffer_array.len() {
            break;
        }
        buffer_array[pos] = value;
        pos += 1;
    }

    Ok(None)
}

/// Resolves a constant pool entry to a Java Value.
///
/// This helper function converts constant pool entries to their corresponding Java object
/// representations for use with bootstrap method arguments.
async fn resolve_constant_to_value<T: Thread + 'static>(
    thread: &T,
    constant_pool: &ConstantPool,
    constant: &Constant,
) -> Result<Value> {
    match constant {
        Constant::Integer(value) => Ok(Value::from(*value)),
        Constant::Float(value) => Ok(Value::from(*value)),
        Constant::Long(value) => Ok(Value::from(*value)),
        Constant::Double(value) => Ok(Value::from(*value)),
        Constant::String(utf8_index) => {
            let string = constant_pool.try_get_utf8(*utf8_index)?;
            string.to_object(thread).await
        }
        Constant::Class(name_index) => {
            let class_name = constant_pool.try_get_utf8(*name_index)?;
            let class = thread.class(class_name).await?;
            class.to_object(thread).await
        }
        Constant::MethodType(descriptor_index) => {
            let descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
            resolve_method_type(thread, descriptor).await
        }
        Constant::MethodHandle {
            reference_kind,
            reference_index,
        } => resolve_method_handle(thread, constant_pool, reference_kind, *reference_index).await,
        Constant::Dynamic {
            bootstrap_method_attr_index: _,
            name_and_type_index: _,
        } => {
            // Dynamic constants are resolved lazily via bootstrap methods. For now, return null as
            // a placeholder; full implementation would require invoking the bootstrap method
            // similar to invokedynamic.
            // TODO: Implement full dynamic constant resolution via bootstrap method invocation
            Ok(Value::Object(None))
        }
        _ => Err(InternalError(format!(
            "Unsupported constant type for bootstrap argument: {constant:?}"
        ))),
    }
}

/// Resolves a `MethodType` from a method descriptor string.
async fn resolve_method_type<T: Thread + 'static>(thread: &T, descriptor: &str) -> Result<Value> {
    let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;

    // Get return type class
    let return_class = if let Some(ret_type) = ret {
        let class_name = ret_type.class_name();
        let class = thread.class(&class_name).await?;
        class.to_object(thread).await?
    } else {
        let void_class = thread.class("void").await?;
        void_class.to_object(thread).await?
    };

    // Build MethodType using methodType factory method
    if params.is_empty() {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_class])
            .await
    } else {
        // Create parameter class array
        let class_array_class = thread.class("[Ljava/lang/Class;").await?;
        let mut param_classes = Vec::with_capacity(params.len());
        for param in params {
            let class_name = param.class_name();
            let class = thread.class(&class_name).await?;
            param_classes.push(class.to_object(thread).await?);
        }
        let reference = Reference::try_from((class_array_class, param_classes))?;
        let param_array = Value::new_object(thread.vm()?.garbage_collector(), reference);

        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_class, param_array])
            .await
    }
}

/// Resolves a `MethodHandle` from a constant pool `MethodHandle` entry.
#[expect(clippy::too_many_lines)]
async fn resolve_method_handle<T: Thread + 'static>(
    thread: &T,
    constant_pool: &ConstantPool,
    reference_kind: &ReferenceKind,
    reference_index: u16,
) -> Result<Value> {
    let target = constant_pool.try_get(reference_index)?;

    // Extract class name, member name, and descriptor from the target constant
    let (class_name, member_name, member_descriptor, is_method) = match target {
        Constant::FieldRef {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.try_get_class(*class_index)?;
            let (name_index, type_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let name = constant_pool.try_get_utf8(*name_index)?;
            let descriptor = constant_pool.try_get_utf8(*type_index)?;
            (class_name, name, descriptor, false)
        }
        Constant::MethodRef {
            class_index,
            name_and_type_index,
        }
        | Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.try_get_class(*class_index)?;
            let (name_index, type_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let name = constant_pool.try_get_utf8(*name_index)?;
            let descriptor = constant_pool.try_get_utf8(*type_index)?;
            (class_name, name, descriptor, true)
        }
        Constant::MethodHandle {
            reference_kind: nested_reference_kind,
            reference_index: nested_reference_index,
        } => {
            return Box::pin(resolve_method_handle(
                thread,
                constant_pool,
                nested_reference_kind,
                *nested_reference_index,
            ))
            .await;
        }
        _ => {
            return Err(InternalError(format!(
                "Unsupported MethodHandle target constant type: {target:?}"
            )));
        }
    };

    // Get the class and lookup object
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(thread).await?;
    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;

    // Get a trusted lookup for bootstrap resolution
    let method_handles_class = thread.class("java.lang.invoke.MethodHandles").await?;
    let lookup_method = method_handles_class
        .try_get_method("lookup", "()Ljava/lang/invoke/MethodHandles$Lookup;")?;
    let empty_args: &[Value] = &[];
    let lookup = thread
        .try_execute(&method_handles_class, &lookup_method, empty_args)
        .await?;

    // Build the Java String and MethodType for the member
    let name_value = member_name.to_object(thread).await?;
    let method_type = if is_method {
        resolve_method_type(thread, member_descriptor).await?
    } else {
        // not used for fields
        Value::Object(None)
    };

    // Select and call the right Lookup method based on ReferenceKind
    match (reference_kind, is_method) {
        (ReferenceKind::InvokeStatic, true) => {
            let find_method = lookup_class.try_get_method(
                "findStatic",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, method_type],
                )
                .await
        }
        (ReferenceKind::InvokeVirtual | ReferenceKind::InvokeInterface, true) => {
            let find_method = lookup_class.try_get_method(
                "findVirtual",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, method_type],
                )
                .await
        }
        (ReferenceKind::InvokeSpecial, true) => {
            let find_method = lookup_class.try_get_method(
                "findSpecial",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[
                        lookup,
                        class_object.clone(),
                        name_value,
                        method_type,
                        class_object,
                    ],
                )
                .await
        }
        (ReferenceKind::NewInvokeSpecial, true) => {
            let find_method = lookup_class.try_get_method(
                "findConstructor",
                "(Ljava/lang/Class;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, method_type],
                )
                .await
        }
        (ReferenceKind::GetField, false) => {
            let find_method = lookup_class.try_get_method(
                "findGetter",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(&field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, field_class_object],
                )
                .await
        }
        (ReferenceKind::GetStatic, false) => {
            let find_method = lookup_class.try_get_method(
                "findStaticGetter",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(&field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, field_class_object],
                )
                .await
        }
        (ReferenceKind::PutField, false) => {
            let find_method = lookup_class.try_get_method(
                "findSetter",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(&field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, field_class_object],
                )
                .await
        }
        (ReferenceKind::PutStatic, false) => {
            let find_method = lookup_class.try_get_method(
                "findStaticSetter",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(&field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &find_method,
                    &[lookup, class_object, name_value, field_class_object],
                )
                .await
        }
        _ => Err(InternalError(format!(
            "Unsupported method handle reference kind: {reference_kind:?} is_method: {is_method}"
        ))),
    }
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.expand(Ljava/lang/invoke/MemberName;)V",
    Any
)]
#[async_method]
pub async fn expand<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop()?;
    resolve(thread, member_name, None, &LookupModeFlags::empty(), false).await?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getConstant(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_constant<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _which = parameters.pop_int()?;
    // Constants defined in MethodHandleNatives:
    // GC_COUNT_MAX = 0: Maximum garbage collection count (0 = no limit)
    // Other indices are reserved and return 0
    let result = 0;
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn get_member_vm_info<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_name = parameters.pop()?;
    let (vmindex, flags) = {
        let member_name_ref = member_name.as_object_ref()?;
        let vmindex = member_name_ref.value("vmindex")?;
        let flags = member_name_ref.value("flags")?;
        (vmindex, flags)
    };

    let vmindex = vmindex.to_object(&thread).await?;
    let flags = flags.to_object(&thread).await?;
    let object_array_class = thread.class("[Ljava/lang/Object;").await?;
    let values = vec![vmindex, flags];
    let reference = Reference::try_from((object_array_class, values))?;
    let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(array))
}

/// Recursively collects all interfaces implemented by a class.
///
/// This includes directly implemented interfaces and all superinterfaces.
fn collect_interfaces(class: &Arc<Class>, result: &mut Vec<Arc<Class>>) -> Result<()> {
    let interfaces = class.interfaces()?;
    for interface in interfaces {
        // Avoid duplicates
        if !result.iter().any(|c| Arc::ptr_eq(c, &interface)) {
            result.push(interface.clone());
            // Recursively collect superinterfaces
            collect_interfaces(&interface, result)?;
        }
    }
    // Also collect interfaces from parent class
    if let Some(parent) = class.parent()? {
        collect_interfaces(&parent, result)?;
    }
    Ok(())
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getMembers(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn get_members<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let results = parameters.pop()?;
    let skip = parameters.pop_int()?;
    let _caller = parameters.pop()?;
    let match_flags = parameters.pop_int()?;
    let match_sig = parameters.pop()?;
    let match_name = parameters.pop()?;
    let defc = parameters.pop()?;

    let class = get_class(&thread, &defc).await?;
    let match_name = if match_name.is_null() {
        None
    } else {
        Some(match_name.as_string()?)
    };
    let match_sig = if match_sig.is_null() {
        None
    } else {
        Some(match_sig.as_string()?)
    };

    let match_flags = MemberNameFlags::from_bits_truncate(match_flags);

    let mut members = Vec::new();
    let mut classes_to_search = Vec::new();
    classes_to_search.push(class.clone());

    // Add superclasses to search if SUPERCLASSES flag is set
    if match_flags.contains(MemberNameFlags::SUPERCLASSES) {
        let mut current_class = class.clone();
        while let Some(parent) = current_class.parent()? {
            classes_to_search.push(parent.clone());
            current_class = parent;
        }
    }

    // Add interfaces to search if INTERFACES flag is set
    if match_flags.contains(MemberNameFlags::INTERFACES) {
        collect_interfaces(&class, &mut classes_to_search)?;
    }

    for class in classes_to_search {
        if match_flags.contains(MemberNameFlags::IS_FIELD) {
            for field in class.declared_fields() {
                if let Some(ref name) = match_name
                    && field.name() != name
                {
                    continue;
                }
                if let Some(ref sig) = match_sig
                    && field.field_type().to_string() != *sig
                {
                    continue;
                }
                members.push((
                    class.clone(),
                    field.name().to_string(),
                    field.field_type().to_string(),
                    field.access_flags().bits(),
                    MemberNameFlags::IS_FIELD,
                ));
            }
        }

        if match_flags.contains(MemberNameFlags::IS_METHOD)
            || match_flags.contains(MemberNameFlags::IS_CONSTRUCTOR)
        {
            for method in class.methods() {
                let method_name = method.name();
                let flags = if method_name == "<init>" {
                    if !match_flags.contains(MemberNameFlags::IS_CONSTRUCTOR) {
                        continue;
                    }
                    MemberNameFlags::IS_CONSTRUCTOR
                } else {
                    if !match_flags.contains(MemberNameFlags::IS_METHOD) {
                        continue;
                    }
                    if method_name == "<clinit>" {
                        continue;
                    }
                    MemberNameFlags::IS_METHOD
                };

                if let Some(ref name) = match_name
                    && method_name != name
                {
                    continue;
                }
                if let Some(ref sig) = match_sig
                    && method.descriptor() != sig
                {
                    continue;
                }
                members.push((
                    class.clone(),
                    method_name.to_string(),
                    method.descriptor().to_string(),
                    method.access_flags().bits(),
                    flags,
                ));
            }
        }
    }

    let results_len = {
        let results_guard = results.as_reference()?;
        match &*results_guard {
            Reference::Array(arr) => arr.elements.len(),
            _ => return Err(InternalError("results is not an array".to_string())),
        }
    };

    let skip = usize::try_from(skip).unwrap_or(0);
    let mut resolved_members = Vec::new();

    for (class, name, descriptor, modifiers, flags) in
        members.into_iter().skip(skip).take(results_len)
    {
        let class_val = class.to_object(&thread).await?;
        let name_val = name.to_object(&thread).await?;
        let type_val = descriptor.to_object(&thread).await?;
        let flags_val = Value::Int(i32::from(modifiers) | flags.bits());
        resolved_members.push((class_val, name_val, type_val, flags_val));
    }

    let mut results_guard = results.as_reference_mut()?;
    let results_array = match &mut *results_guard {
        Reference::Array(arr) => &mut arr.elements,
        _ => return Err(InternalError("results is not an array".to_string())),
    };

    let mut count = 0;
    for (class_val, name_val, type_val, flags_val) in resolved_members {
        if count >= results_array.len() {
            break;
        }

        if let Some(Value::Object(Some(member_name_ref))) = results_array.get(count) {
            let mut member_name = member_name_ref.write();
            if let Reference::Object(ref mut object) = *member_name {
                object.set_value("clazz", class_val)?;
                object.set_value("name", name_val)?;
                object.set_value("type", type_val)?;
                object.set_value("flags", flags_val)?;
            }
        }
        count += 1;
    }

    Ok(Some(Value::Int(i32::try_from(count)?)))
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I",
    Any
)]
#[async_method]
pub async fn get_named_con<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let box_array = parameters.pop()?;
    let which = parameters.pop_int()?;

    // Named constants - these are internal VM constants that can be queried by index
    // The name is stored in box_array[0] if found
    // Returns the constant value, or -1 if not found

    let constants: &[(&str, i32)] = &[
        ("GC_COUNT_MAX", 0),
        ("MN_IS_METHOD", 0x0001_0000),
        ("MN_IS_CONSTRUCTOR", 0x0002_0000),
        ("MN_IS_FIELD", 0x0004_0000),
        ("MN_IS_TYPE", 0x0008_0000),
        ("MN_CALLER_SENSITIVE", 0x0010_0000),
        ("MN_REFERENCE_KIND_SHIFT", 24),
        ("MN_REFERENCE_KIND_MASK", 0x0F00_0000),
        ("MN_SEARCH_SUPERCLASSES", 0x0010_0000),
        ("MN_SEARCH_INTERFACES", 0x0020_0000),
        ("T_BOOLEAN", 4),
        ("T_CHAR", 5),
        ("T_FLOAT", 6),
        ("T_DOUBLE", 7),
        ("T_BYTE", 8),
        ("T_SHORT", 9),
        ("T_INT", 10),
        ("T_LONG", 11),
        ("T_OBJECT", 12),
        ("T_ARRAY", 13),
        ("T_VOID", 14),
        ("T_ADDRESS", 15),
        ("T_NARROWOOP", 16),
        ("T_METADATA", 17),
        ("T_NARROWKLASS", 18),
        ("T_CONFLICT", 19),
        ("REF_getField", 1),
        ("REF_getStatic", 2),
        ("REF_putField", 3),
        ("REF_putStatic", 4),
        ("REF_invokeVirtual", 5),
        ("REF_invokeStatic", 6),
        ("REF_invokeSpecial", 7),
        ("REF_newInvokeSpecial", 8),
        ("REF_invokeInterface", 9),
    ];

    let which_usize = usize::try_from(which).unwrap_or(usize::MAX);
    let (name, value) = if let Some((name, value)) = constants.get(which_usize) {
        (Some(*name), *value)
    } else {
        (None, -1)
    };

    if let Some(name) = name {
        if !box_array.is_null() {
            let name_value = name.to_object(&thread).await?;
            let mut array_ref = box_array.as_reference_mut()?;
            if let Reference::Array(arr) = &mut *array_ref
                && !arr.elements.is_empty()
            {
                arr.elements[0] = name_value;
            }
        }
        Ok(Some(Value::Int(value)))
    } else {
        Ok(Some(Value::Int(-1)))
    }
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.init(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
    Any
)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ref_object = parameters.pop()?;
    let member_name = parameters.pop()?;

    if ref_object.is_null() {
        return Ok(None);
    }

    let ref_class_name = {
        let ref_object = ref_object.as_object_ref()?;
        ref_object.class().name().to_string()
    };

    match ref_class_name.as_str() {
        "java/lang/reflect/Method" => {
            init_from_method(&thread, member_name, ref_object).await?;
        }
        "java/lang/reflect/Constructor" => {
            init_from_constructor(&thread, member_name, ref_object).await?;
        }
        "java/lang/reflect/Field" => {
            init_from_field(&thread, &member_name, &ref_object)?;
        }
        _ => {
            // Unknown ref type, leave MemberName uninitialized
        }
    }

    Ok(None)
}

/// Initializes a `MemberName` from a java.lang.reflect.Method object.
async fn init_from_method<T: Thread + 'static>(
    thread: &T,
    member_name: Value,
    method_ref: Value,
) -> Result<()> {
    let (clazz, name, modifiers, parameter_types, return_type) = {
        let method_object = method_ref.as_object_ref()?;
        let clazz = method_object.value("clazz")?;
        let name = method_object.value("name")?;
        let modifiers = method_object.value("modifiers")?.as_i32()?;
        let parameter_types = method_object.value("parameterTypes")?;
        let return_type = method_object.value("returnType")?;
        (clazz, name, modifiers, parameter_types, return_type)
    };

    // Build the method type from parameter and return types
    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    let method_type = if parameter_types.is_null() {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_type])
            .await?
    } else {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_type, parameter_types])
            .await?
    };

    let is_static = (modifiers & i32::from(MethodAccessFlags::STATIC.bits())) != 0;
    let reference_kind = i32::from(if is_static {
        ReferenceKind::InvokeStatic.kind()
    } else {
        ReferenceKind::InvokeVirtual.kind()
    });
    let flags = MemberNameFlags::IS_METHOD.bits() | modifiers | (reference_kind << 24);

    let mut member_name_object = member_name.as_object_mut()?;
    member_name_object.set_value("clazz", clazz)?;
    member_name_object.set_value("name", name)?;
    member_name_object.set_value("type", method_type)?;
    member_name_object.set_value("flags", Value::Int(flags))?;

    Ok(())
}

/// Initializes a `MemberName` from a java.lang.reflect.Constructor object.
async fn init_from_constructor<T: Thread + 'static>(
    thread: &T,
    member_name: Value,
    constructor_ref: Value,
) -> Result<()> {
    let (clazz, modifiers, parameter_types) = {
        let constructor_object = constructor_ref.as_object_ref()?;
        let clazz = constructor_object.value("clazz")?;
        let modifiers = constructor_object.value("modifiers")?.as_i32()?;
        let parameter_types = constructor_object.value("parameterTypes")?;
        (clazz, modifiers, parameter_types)
    };

    // Constructor name is always "<init>"
    let name = "<init>".to_object(thread).await?;

    // Return type is void
    let void_class = thread.class("void").await?;
    let return_type = void_class.to_object(thread).await?;

    // Build the method type from parameter and return types
    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    let method_type = if parameter_types.is_null() {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_type])
            .await?
    } else {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        thread
            .try_execute(&method_type_class, &method, &[return_type, parameter_types])
            .await?
    };

    // REF_newInvokeSpecial = 8 for constructors
    let flags = MemberNameFlags::IS_CONSTRUCTOR.bits() | modifiers | (8 << 24);

    let mut member_name_object = member_name.as_object_mut()?;
    member_name_object.set_value("clazz", clazz)?;
    member_name_object.set_value("name", name)?;
    member_name_object.set_value("type", method_type)?;
    member_name_object.set_value("flags", Value::Int(flags))?;

    Ok(())
}

/// Initializes a `MemberName` from a java.lang.reflect.Field object.
fn init_from_field<T: Thread + 'static>(
    _thread: &T,
    member_name: &Value,
    field_ref: &Value,
) -> Result<()> {
    let (clazz, name, field_type, modifiers) = {
        let field_object = field_ref.as_object_ref()?;
        let clazz = field_object.value("clazz")?;
        let name = field_object.value("name")?;
        let field_type = field_object.value("type")?;
        let modifiers = field_object.value("modifiers")?.as_i32()?;
        (clazz, name, field_type, modifiers)
    };

    // Default to getField/getStatic based on static modifier
    let is_static = (modifiers & i32::from(FieldAccessFlags::STATIC.bits())) != 0;
    let ref_kind = i32::from(if is_static {
        ReferenceKind::GetStatic.kind()
    } else {
        ReferenceKind::GetField.kind()
    });
    let flags = MemberNameFlags::IS_FIELD.bits() | modifiers | (ref_kind << 24);

    let mut member_name_object = member_name.as_object_mut()?;
    member_name_object.set_value("clazz", clazz)?;
    member_name_object.set_value("name", name)?;
    member_name_object.set_value("type", field_type)?;
    member_name_object.set_value("flags", Value::Int(flags))?;

    Ok(())
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J",
    Any
)]
#[async_method]
pub async fn object_field_offset<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
    // For holder classes, the methods may not exist in the class file.
    // They are provided by intrinsics and we need to create synthetic methods.
    let class_name = class.name();

    // Check if the class is a holder class
    let is_holder = matches!(
        class_name,
        "java/lang/invoke/DirectMethodHandle$Holder"
            | "java/lang/invoke/DelegatingMethodHandle$Holder"
            | "java/lang/invoke/Invokers$Holder"
            | "java/lang/invoke/LambdaForm$Holder"
            | "java/lang/invoke/VarHandleGuards"
    ) || class_name.starts_with("java/lang/invoke/LambdaForm$");

    // First try to get the method from the class
    if let Ok(method) = class.try_get_method(method_name, method_descriptor) {
        return Ok(method);
    }

    // Try with the polymorphic signature
    let polymorphic_descriptor = "([Ljava/lang/Object;)Ljava/lang/Object;";
    if let Ok(method) = class.try_get_method(method_name, polymorphic_descriptor) {
        return Ok(method);
    }

    // For holder classes, create a synthetic method for intrinsics
    if is_holder {
        use ristretto_classfile::{FieldType, MethodAccessFlags};

        let definition = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC
                | MethodAccessFlags::STATIC
                | MethodAccessFlags::NATIVE,
            name_index: 0,
            descriptor_index: 0,
            attributes: Vec::new(),
        };

        let descriptor_to_use = polymorphic_descriptor;
        let (parameters, return_type) = FieldType::parse_method_descriptor(descriptor_to_use)?;

        let method = ristretto_classloader::Method::new_synthetic(
            definition,
            method_name.to_string(),
            descriptor_to_use.to_string(),
            parameters,
            return_type,
        );

        return Ok(Arc::new(method));
    }

    // If not a holder class, return the original error
    class.try_get_method(method_name, method_descriptor)?;
    unreachable!()
}

#[intrinsic_method("java/lang/invoke/MethodHandleNatives.registerNatives()V", Any)]
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

/// Searches for a lambda method in interface classes.
///
/// Lambda methods in interface default methods (like `Function.andThen`) are private instance
/// methods on the interface itself. When the `MemberName.clazz` is incorrectly set to `Object`,
/// we need to find the actual interface that defines the lambda method.
///
/// We use the method's parameter types to identify candidate interfaces, since lambda methods
/// often have the interface type as their first parameter (for captured `this`).
async fn find_lambda_method_in_interfaces<T: Thread + 'static>(
    thread: &T,
    parameter_descriptors: &[String],
    method_name: &str,
    method_descriptor: &str,
) -> Result<Option<(Arc<Class>, Arc<Method>)>> {
    // Extract interface class names from parameter descriptors
    // Lambda methods typically capture 'this' as the first parameter
    for param_desc in parameter_descriptors {
        // Check if this is an object type descriptor (L...;)
        if param_desc.starts_with('L') && param_desc.ends_with(';') {
            let class_name = &param_desc[1..param_desc.len() - 1];

            // Try to load the class and check if it has the lambda method
            if let Ok(candidate_class) = thread.class(class_name).await {
                // Check if this class has the lambda method
                if let Ok(method) = candidate_class.try_get_method(method_name, method_descriptor) {
                    return Ok(Some((candidate_class, method)));
                }

                // Also check interfaces of the class
                if let Ok(interfaces) = candidate_class.interfaces() {
                    for interface in interfaces {
                        if let Ok(method) = interface.try_get_method(method_name, method_descriptor)
                        {
                            return Ok(Some((interface, method)));
                        }
                    }
                }
            }
        }
    }

    // Also try common functional interfaces based on the method name pattern
    // e.g., lambda$andThen$0 is likely from Function, IntUnaryOperator, etc.
    let potential_interfaces = get_potential_interfaces_from_method_name(method_name);
    for interface_name in potential_interfaces {
        if let Ok(interface_class) = thread.class(interface_name).await
            && let Ok(method) = interface_class.try_get_method(method_name, method_descriptor)
        {
            return Ok(Some((interface_class, method)));
        }
    }

    Ok(None)
}

/// Returns potential interface class names based on a lambda method name.
///
/// Lambda methods follow the pattern `lambda$<method>$<number>`, where `<method>` is the
/// name of the method that created the lambda. We can use this to guess the interface.
fn get_potential_interfaces_from_method_name(method_name: &str) -> Vec<&'static str> {
    // Extract the method name from the lambda pattern: lambda$methodName$number
    let parts: Vec<&str> = method_name.split('$').collect();
    if parts.len() < 2 {
        return Vec::new();
    }

    let origin_method = parts[1];

    // Map common method names to their likely interfaces
    match origin_method {
        "andThen" => vec![
            "java/util/function/Function",
            "java/util/function/BiFunction",
            "java/util/function/Consumer",
            "java/util/function/BiConsumer",
            "java/util/function/IntUnaryOperator",
            "java/util/function/LongUnaryOperator",
            "java/util/function/DoubleUnaryOperator",
            "java/util/function/IntConsumer",
            "java/util/function/LongConsumer",
            "java/util/function/DoubleConsumer",
        ],
        "compose" => vec![
            "java/util/function/Function",
            "java/util/function/IntUnaryOperator",
            "java/util/function/LongUnaryOperator",
            "java/util/function/DoubleUnaryOperator",
        ],
        "negate" => vec![
            "java/util/function/Predicate",
            "java/util/function/IntPredicate",
            "java/util/function/LongPredicate",
            "java/util/function/DoublePredicate",
        ],
        "and" | "or" => vec![
            "java/util/function/Predicate",
            "java/util/function/IntPredicate",
            "java/util/function/LongPredicate",
            "java/util/function/DoublePredicate",
            "java/util/function/BiPredicate",
        ],
        "identity" => vec![
            "java/util/function/Function",
            "java/util/function/UnaryOperator",
            "java/util/function/IntUnaryOperator",
            "java/util/function/LongUnaryOperator",
            "java/util/function/DoubleUnaryOperator",
        ],
        _ => Vec::new(),
    }
}

/// Searches for an interface method based on parameter types.
///
/// This is used when a method on Object cannot be found, but might be an interface method
/// like `Collection.stream()`. We search interfaces that could define the method.
async fn find_interface_method_from_params<T: Thread + 'static>(
    thread: &T,
    parameter_descriptors: &[String],
    method_name: &str,
    method_descriptor: &str,
) -> Result<Option<(Arc<Class>, Arc<Method>)>> {
    // Search interfaces based on parameter types
    for param_desc in parameter_descriptors {
        if param_desc.starts_with('L') && param_desc.ends_with(';') {
            let class_name = &param_desc[1..param_desc.len() - 1];

            if let Ok(candidate_class) = thread.class(class_name).await {
                // Check if this class or its interfaces have the method
                if let Ok(method) = candidate_class.try_get_method(method_name, method_descriptor) {
                    return Ok(Some((candidate_class, method)));
                }

                // Check interfaces of the class
                if let Ok(interfaces) = candidate_class.interfaces() {
                    for interface in interfaces {
                        if let Ok(method) = interface.try_get_method(method_name, method_descriptor)
                        {
                            return Ok(Some((interface, method)));
                        }
                        // Also check super-interfaces
                        if let Some(result) = search_interface_hierarchy_for_method(
                            &interface,
                            method_name,
                            method_descriptor,
                        )
                        .await?
                        {
                            return Ok(Some(result));
                        }
                    }
                }
            }
        }
    }

    // Try common interfaces based on method name
    let common_interfaces = get_common_interfaces_for_method(method_name);
    for interface_name in common_interfaces {
        if let Ok(interface_class) = thread.class(interface_name).await
            && let Ok(method) = interface_class.try_get_method(method_name, method_descriptor)
        {
            return Ok(Some((interface_class, method)));
        }
    }

    Ok(None)
}

/// Recursively search interface hierarchy for a method.
#[async_method]
async fn search_interface_hierarchy_for_method(
    interface: &Arc<Class>,
    method_name: &str,
    method_descriptor: &str,
) -> Result<Option<(Arc<Class>, Arc<Method>)>> {
    if let Ok(method) = interface.try_get_method(method_name, method_descriptor) {
        return Ok(Some((interface.clone(), method)));
    }

    if let Ok(super_interfaces) = interface.interfaces() {
        for super_interface in super_interfaces {
            if let Some(result) = search_interface_hierarchy_for_method(
                &super_interface,
                method_name,
                method_descriptor,
            )
            .await?
            {
                return Ok(Some(result));
            }
        }
    }

    Ok(None)
}

/// Returns common interface names for a given method name.
fn get_common_interfaces_for_method(method_name: &str) -> Vec<&'static str> {
    match method_name {
        "stream" | "parallelStream" => {
            vec!["java/util/Collection", "java/util/List", "java/util/Set"]
        }
        "iterator" | "forEach" | "spliterator" => {
            vec!["java/lang/Iterable", "java/util/Collection"]
        }
        "getTypeName" => vec!["java/lang/Class", "java/lang/reflect/Type"],
        _ => Vec::new(),
    }
}

/// Search all loaded interfaces for a method.
/// This is a fallback for when parameter-based search fails,
/// used for default interface methods with no parameters.
async fn search_all_loaded_interfaces<T: Thread + 'static>(
    thread: &T,
    method_name: &str,
    method_descriptor: &str,
) -> Result<Option<(Arc<Class>, Arc<Method>)>> {
    let vm = thread.vm()?;
    let class_loader_lock = vm.class_loader();
    let class_loader = class_loader_lock.read().await;

    // Get all loaded classes and search for interfaces with the method
    for class in class_loader.loaded_classes().await {
        if class.is_interface()
            && let Ok(method) = class.try_get_method(method_name, method_descriptor)
        {
            // Check if it's a default method (not abstract)
            if !method.access_flags().contains(MethodAccessFlags::ABSTRACT) {
                return Ok(Some((class.clone(), method)));
            }
        }
    }

    Ok(None)
}

/// Resolve a member name to a method, field, or constructor.
///
/// # Errors
/// Returns an error if the member cannot be resolved.
pub async fn resolve<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
    let class = thread.class(&class_name).await?;
    let member_name_flags = MemberNameFlags::from_bits_truncate(flags);

    if member_name_flags.contains(MemberNameFlags::IS_METHOD)
        || member_name_flags.contains(MemberNameFlags::IS_CONSTRUCTOR)
    {
        resolve_method(
            &thread,
            member_self,
            caller.as_ref(),
            *lookup_mode_flags,
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
            *lookup_mode_flags,
            speculative_resolve,
            &name,
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
#[expect(clippy::too_many_lines)]
async fn resolve_method<T: Thread + 'static>(
    thread: &T,
    member_self: Value,
    caller: Option<&Arc<Class>>,
    lookup_mode_flags: LookupModeFlags,
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
        let class_name = {
            let method_type_ref = method_type.as_object_ref()?;
            method_type_ref.class().name().to_string()
        };

        if class_name == "java/lang/invoke/MethodType" {
            let method_type_ref = method_type.as_object_ref()?;
            let parameter_types = method_type_ref.value("ptypes")?;
            let parameters: Vec<Value> = parameter_types.try_into()?;
            let mut parameter_descriptors = Vec::with_capacity(parameters.len());
            for parameter in parameters {
                let class_object = parameter.as_object_ref()?;
                let class_name = class_object.value("name")?.as_string()?;
                let descriptor = Class::convert_to_descriptor(&class_name);
                parameter_descriptors.push(descriptor);
            }
            let return_type = method_type_ref.value("rtype")?;
            let return_type = return_type.as_object_ref()?;
            let return_class_name = return_type.value("name")?.as_string()?;
            let return_descriptor = Class::convert_to_descriptor(&return_class_name);
            (parameter_descriptors, return_descriptor)
        } else if class_name.starts_with('[') {
            let method_type_ref = method_type.as_reference()?;
            let (_class, elements) = method_type_ref.as_class_vec_ref()?;
            if elements.is_empty() {
                return Err(InternalError("Invalid type array".to_string()));
            }
            // elements[0] is return type (Class)
            // elements[1..] are parameter types (Class)
            let return_type = elements[0].as_object_ref()?;
            let return_class_name = return_type.value("name")?.as_string()?;
            let return_descriptor = Class::convert_to_descriptor(&return_class_name);

            let mut parameter_descriptors = Vec::with_capacity(elements.len() - 1);
            for parameter in elements.iter().skip(1) {
                let class_object = parameter.as_object_ref()?;
                let class_name = class_object.value("name")?.as_string()?;
                let descriptor = Class::convert_to_descriptor(&class_name);
                parameter_descriptors.push(descriptor);
            }
            (parameter_descriptors, return_descriptor)
        } else if class_name == "java/lang/String" {
            let descriptor = method_type.as_string()?;
            let (params, ret) = FieldType::parse_method_descriptor(&descriptor)?;
            let parameter_descriptors: Vec<String> =
                params.iter().map(FieldType::descriptor).collect();
            let return_descriptor = ret.map_or_else(|| "V".to_string(), |r| r.descriptor());
            (parameter_descriptors, return_descriptor)
        } else {
            return Err(InternalError(format!("Unsupported type: {class_name}")));
        }
    };

    let method_name = name.as_string()?;
    let method_descriptor = format!("({}){return_descriptor}", parameter_descriptors.concat());
    let (resolved_class, method, is_lambda_method) = match class.name() {
        "java/lang/invoke/DelegatingMethodHandle$Holder"
        | "java/lang/invoke/DirectMethodHandle$Holder"
        | "java/lang/invoke/Invokers$Holder"
        | "java/lang/invoke/MethodHandle"
        | "java/lang/invoke/VarHandle" => (
            class.clone(),
            resolve_holder_methods(class.clone(), &method_name, &method_descriptor)?,
            false,
        ),
        _ => {
            // First try the specified class
            match class.try_get_method(&method_name, &method_descriptor) {
                Ok(m) => (class.clone(), m, false),
                Err(_)
                    if method_name.starts_with("lambda$") && class.name() == "java/lang/Object" =>
                {
                    // For lambda methods on Object, the MemberName.clazz was incorrectly set.
                    // Search for the method in interfaces based on the method signature.
                    // Lambda methods in interface default methods typically have the interface
                    // type as their first parameter.
                    if let Some((found_class, found_method)) = find_lambda_method_in_interfaces(
                        thread,
                        &parameter_descriptors,
                        &method_name,
                        &method_descriptor,
                    )
                    .await?
                    {
                        // Update the MemberName's clazz field to the correct class
                        let class_object = found_class.to_object(thread).await?;
                        {
                            let mut member_self_mut = member_self.as_object_mut()?;
                            member_self_mut.set_value("clazz", class_object)?;
                        }
                        // Lambda methods are private but invoked through method handles
                        // which already have appropriate access, so skip access check
                        (found_class, found_method, true)
                    } else {
                        return Err(InternalError(format!(
                            "Lambda method not found: {}.{method_name}{method_descriptor}",
                            class.name()
                        )));
                    }
                }
                Err(_) if class.name() == "java/lang/Object" => {
                    // For non-lambda methods on Object (like interface methods),
                    // search for the method in interfaces based on parameter types.
                    // This handles cases like Collection.stream() invoked via method handles.
                    if let Some((found_class, found_method)) = find_interface_method_from_params(
                        thread,
                        &parameter_descriptors,
                        &method_name,
                        &method_descriptor,
                    )
                    .await?
                    {
                        // Update the MemberName's clazz field to the correct class
                        let class_object = found_class.to_object(thread).await?;
                        {
                            let mut member_self_mut = member_self.as_object_mut()?;
                            member_self_mut.set_value("clazz", class_object)?;
                        }
                        (found_class, found_method, false)
                    } else if let Some((found_class, found_method)) =
                        search_all_loaded_interfaces(thread, &method_name, &method_descriptor)
                            .await?
                    {
                        // Fallback: search all loaded interfaces for the method
                        // This handles default methods with no parameters
                        let class_object = found_class.to_object(thread).await?;
                        {
                            let mut member_self_mut = member_self.as_object_mut()?;
                            member_self_mut.set_value("clazz", class_object)?;
                        }
                        (found_class, found_method, false)
                    } else {
                        return Err(InternalError(format!(
                            "Interface method not found: {}.{method_name}{method_descriptor}",
                            class.name()
                        )));
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    // Access control enforcement (skip for lambda methods)
    let method_access_flags = method.access_flags();
    if !is_lambda_method
        && !check_method_access(
            caller,
            &resolved_class,
            *method_access_flags,
            lookup_mode_flags,
        )?
    {
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
                resolved_class.name(),
            ))
            .into())
        };
    }

    let modifiers = i32::from(method_access_flags.bits());
    let flags = flags | modifiers;
    {
        // vmindex is used by OpenJDK MethodHandle implementation.
        // For methods, it's typically a vtable index or similar.
        // Since we intercept execution, we can use 0 or a placeholder.
        // Must be a boxed Long object to match the field type Ljava/lang/Object;
        let vmindex = 0i64.to_object(thread).await?;
        let mut member_self = member_self.as_object_mut()?;
        member_self.set_value("flags", Value::from(flags))?;
        member_self.set_value("vmindex", vmindex)?;
    }
    Ok(Some(member_self))
}

/// Resolves a field in the given class, checking access permissions and returning the member self
/// if successful.
#[expect(clippy::too_many_arguments)]
async fn resolve_field<T: Thread + 'static>(
    thread: &T,
    member_self: Value,
    caller: Option<Arc<Class>>,
    lookup_mode_flags: LookupModeFlags,
    speculative_resolve: bool,
    name: &Value,
    flags: i32,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    let _reference_kind = get_reference_kind(flags)?;
    let field_name = name.as_string()?;
    let field = class.declared_field(&field_name)?;
    let field_access_flags = field.access_flags();
    if !check_field_access(caller, class, *field_access_flags, lookup_mode_flags)? {
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
        let field_offset = class.field_offset(&field_name)?;
        let vmindex = (i64::try_from(field_offset)?).to_object(thread).await?;
        let mut member_self = member_self.as_object_mut()?;
        member_self.set_value("flags", Value::from(flags))?;
        member_self.set_value("vmindex", vmindex)?;
    }
    Ok(Some(member_self))
}

/// Extracts the reference kind from the flags of a member name.
#[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn get_reference_kind(flags: i32) -> Result<ReferenceKind> {
    let flags = flags as u32;
    let shift = MemberNameFlags::REFERENCE_KIND_SHIFT.bits();
    let mask = MemberNameFlags::REFERENCE_KIND_MASK.bits() as u32;
    let reference_kind = ((flags >> shift) & mask) as u8;
    ReferenceKind::try_from(reference_kind).map_err(Into::into)
}

/// Returns the nest host class for a given class.
///
/// If the class has a `NestHost` attribute, returns the nest host class. Otherwise, the class is its
/// own nest host.
fn get_nest_host(class: &Arc<Class>) -> Arc<Class> {
    let class_file = class.class_file();
    for attribute in &class_file.attributes {
        if let Attribute::NestHost {
            name_index: _,
            host_class_index,
        } = attribute
        {
            // Try to get the host class name from constant pool
            if let Ok(host_class_name) = class_file.constant_pool.try_get_class(*host_class_index) {
                // For now, just check the name - proper implementation would load the class. If the
                // name matches, return the class itself as a simplification
                if host_class_name == class.name() {
                    return Arc::clone(class);
                }
            }
        }
    }
    // Class is its own nest host
    Arc::clone(class)
}

/// Checks if two classes are nestmates (share the same nest host).
///
/// In Java 11+, nestmates can access each other's private members.
fn are_nestmates(class1: &Arc<Class>, class2: &Arc<Class>) -> bool {
    // Same class is always a nestmate of itself
    if Arc::ptr_eq(class1, class2) {
        return true;
    }

    // Check if classes share the same nest host
    let host1 = get_nest_host(class1);
    let host2 = get_nest_host(class2);

    // Compare by class name since we might not have the same Arc instance
    host1.name() == host2.name()
}

/// Returns true if `caller` is permitted to access a method of `declaring` with the given access
/// flags.
///
/// # Errors
/// Returns an error if access flags cannot be evaluated.
///
/// # References
///
/// - [JLS §6.6](https://docs.oracle.com/javase/specs/jls/se25/html/jls-6.html#jls-6.6)
pub fn check_method_access(
    caller: Option<&Arc<Class>>,
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

    // 2. PRIVATE: accessible only from the same class or nestmates (Java 11+)
    if method_access_flags.contains(MethodAccessFlags::PRIVATE) {
        if Arc::ptr_eq(caller, declaring) {
            return Ok(true);
        }
        // Check nestmate access (Java 11+ feature)
        if lookup_mode_flags.contains(LookupModeFlags::PRIVATE) && are_nestmates(caller, declaring)
        {
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
/// # Errors
/// Returns an error if access flags cannot be evaluated.
///
/// # References
///
/// - [JLS §6.6](https://docs.oracle.com/javase/specs/jls/se25/html/jls-6.html#jls-6.6)
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

    // 2. PRIVATE: accessible only from the same class or nestmates (Java 11+)
    if field_access_flags.contains(FieldAccessFlags::PRIVATE) {
        if Arc::ptr_eq(caller, declaring) {
            return Ok(true);
        }
        // Check nestmate access (Java 11+ feature)
        if lookup_mode_flags.contains(LookupModeFlags::PRIVATE) && are_nestmates(caller, declaring)
        {
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
#[async_method]
pub async fn resolve_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Correct parameter order: pop MemberName first, then caller (Class)
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
        false,
    )
    .await
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Z)Ljava/lang/invoke/MemberName;",
    Any
)]
#[async_method]
pub async fn resolve_1<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let caller = parameters.pop()?;
    let caller = if caller.is_null() {
        None
    } else {
        let caller = get_class(&thread, &caller).await?;
        Some(caller)
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
#[async_method]
pub async fn resolve_2<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let speculative_resolve = parameters.pop_bool()?;
    let lookup_mode = LookupModeFlags::from_bits_truncate(parameters.pop_int()?);
    let caller = parameters.pop()?;
    let caller = if caller.is_null() {
        None
    } else {
        let caller = get_class(&thread, &caller).await?;
        Some(caller)
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
#[async_method]
pub async fn set_call_site_target_normal<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn set_call_site_target_volatile<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn static_field_base<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn static_field_offset<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
    // Mark this as a static field offset by setting the high bit
    let offset = offset | STATIC_FIELD_OFFSET_MASK;
    Ok(Some(Value::Long(offset)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_clear_call_site_context() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None));
        let result = clear_call_site_context(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_out_bootstrap_arguments() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let mut parameters = Parameters::default();

        let string_class = thread.class("java.lang.String").await?;
        let caller = string_class.to_object(&thread).await?;

        let index_info = Value::new_object(collector, Reference::from(vec![0i32]));

        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let reference = Reference::try_from((object_array_class, vec![Value::Object(None)]))?;
        let buffer = Value::new_object(collector, reference);

        // Function pops: if_not_available, resolve, pos, buf, end, start, index_info, caller
        // So we push in reverse: caller, index_info, start, end, buf, pos, resolve, if_not_available
        parameters.push(caller); // caller
        parameters.push(index_info); // indexInfo
        parameters.push(Value::Int(0)); // start
        parameters.push(Value::Int(0)); // end (0 means no elements to copy)
        parameters.push(buffer); // buffer
        parameters.push(Value::Int(0)); // pos
        parameters.push(Value::Int(0)); // resolve = false
        parameters.push(Value::Object(None)); // ifNotAvailable

        let result = copy_out_bootstrap_arguments(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_expand() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;

        // Use to_object to create a proper java.lang.Class object with the "name" field set
        let class_object = thread.class("java.lang.Integer").await?;
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;

        // Use MAX_VALUE as a real existing field
        let name = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", name)?;

        let type_object = Value::Object(None);
        member_name.set_value("type", type_object)?;

        let flags = MemberNameFlags::IS_FIELD.bits()
            | i32::from(FieldAccessFlags::PUBLIC.bits())
            | i32::from(FieldAccessFlags::STATIC.bits())
            | i32::from(FieldAccessFlags::FINAL.bits())
            | (i32::from(ReferenceKind::GetStatic.kind()) << 24);
        member_name.set_value("flags", Value::Int(flags))?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        let result = expand(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_constant() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // GC_COUNT_MAX constant index
        parameters.push(Value::Int(0));
        let result = get_constant(thread, parameters).await?;
        // GC_COUNT_MAX = 0 (no limit)
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_member_vm_info() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;

        let vmindex = "test_signature".to_object(&thread).await?;
        member_name.set_value("vmindex", vmindex)?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));

        let result = get_member_vm_info(thread, parameters).await?;

        if let Some(Value::Object(Some(reference))) = result {
            let array_value = Value::Object(Some(reference));
            let array: Vec<Value> = array_value.try_into().expect("array");
            assert_eq!(2, array.len());
            assert!(matches!(array[0], Value::Object(Some(_))));
        } else {
            panic!("Expected Object array result");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_members() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let mut parameters = Parameters::default();

        let defc_class = thread.class("java.lang.Integer").await?;
        let defc = defc_class.to_object(&thread).await?;

        let match_name = Value::Object(None);
        let match_sig = Value::Object(None);
        let match_flags = Value::Int(MemberNameFlags::IS_FIELD.bits());
        let caller = Value::Object(None);
        let skip = Value::Int(0);

        let member_name_array_class = thread.class("[Ljava/lang/invoke/MemberName;").await?;
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut elements = Vec::new();
        for _ in 0..10 {
            elements.push(Value::new_object(
                collector,
                Reference::Object(Object::new(member_name_class.clone())?),
            ));
        }
        let results = {
            let reference = Reference::try_from((member_name_array_class, elements))?;
            Value::new_object(collector, reference)
        };

        parameters.push(defc);
        parameters.push(match_name);
        parameters.push(match_sig);
        parameters.push(match_flags);
        parameters.push(caller);
        parameters.push(skip);
        parameters.push(results);

        let result = get_members(thread, parameters).await?;

        if let Some(Value::Int(count)) = result {
            assert!(count > 0);
        } else {
            panic!("Expected int result");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_named_con() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();

        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let reference = Reference::try_from((object_array_class, vec![Value::Object(None)]))?;
        let box_array = Value::new_object(thread.vm()?.garbage_collector(), reference);

        // which = 0 (GC_COUNT_MAX)
        parameters.push(Value::Int(0));
        parameters.push(box_array);

        let result = get_named_con(thread, parameters).await?;
        // Index 0 is GC_COUNT_MAX with value 0
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_named_con_unknown() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();

        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let reference = Reference::try_from((object_array_class, vec![Value::Object(None)]))?;
        let box_array = Value::new_object(thread.vm()?.garbage_collector(), reference);

        // unknown index
        parameters.push(Value::Int(999));
        parameters.push(box_array);

        let result = get_named_con(thread, parameters).await?;
        // Unknown index returns -1
        assert_eq!(Some(Value::Int(-1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();

        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        parameters.push(Value::Object(None));

        let result = init(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_with_field() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let collector = vm.garbage_collector();
        let mut parameters = Parameters::default();

        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let member_name = Object::new(member_name_class)?;

        let field_class = thread.class("java.lang.reflect.Field").await?;
        let mut field_object = Object::new(field_class)?;

        let declaring_class = thread.class("java.lang.Integer").await?;
        let declaring_class_object = declaring_class.to_object(&thread).await?;
        field_object.set_value("clazz", declaring_class_object)?;

        let field_name = "MAX_VALUE".to_object(&thread).await?;
        field_object.set_value("name", field_name)?;

        let int_class = thread.class("int").await?;
        let int_class_object = int_class.to_object(&thread).await?;
        field_object.set_value("type", int_class_object)?;

        // PUBLIC | STATIC | FINAL
        let modifiers =
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL;
        field_object.set_value("modifiers", Value::Int(i32::from(modifiers.bits())))?;

        parameters.push(Value::new_object(collector, Reference::Object(member_name)));
        parameters.push(Value::new_object(
            collector,
            Reference::Object(field_object),
        ));

        let result = init(thread, parameters).await?;
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
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;
        let value_string = "value".to_object(&thread).await?;
        member_name.set_value("name", value_string)?;
        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        let result = object_field_offset(thread, parameters).await?;
        assert_eq!(Some(Value::Long(5)), result);
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
        let reference_kind = MemberNameFlags::IS_METHOD.bits()
            | (i32::from(ReferenceKind::InvokeStatic.kind()) << 24);

        assert_eq!(
            get_reference_kind(reference_kind)?,
            ReferenceKind::InvokeStatic
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_normal() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Value::new_object(
            collector,
            Reference::Object(Object::new(method_handle_class)?),
        );
        parameters.push(method_handle);
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site =
            Value::new_object(collector, Reference::Object(Object::new(call_site_class)?));
        parameters.push(call_site);
        let result = set_call_site_target_normal(thread, parameters).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_call_site_target_volatile() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let mut parameters = Parameters::default();
        let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
        let method_handle = Value::new_object(
            collector,
            Reference::Object(Object::new(method_handle_class)?),
        );
        parameters.push(method_handle);
        let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
        let call_site =
            Value::new_object(collector, Reference::Object(Object::new(call_site_class)?));
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
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class.clone())?;
        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
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
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;
        let value_string = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", value_string)?;
        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        let result = static_field_offset(thread, parameters).await?;
        // The offset has the STATIC_FIELD_OFFSET_MASK set to indicate it's a static field
        assert_eq!(Some(Value::Long(2 | STATIC_FIELD_OFFSET_MASK)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_resolve_0_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;

        // Use to_object to create a proper java.lang.Class object
        let class_object = thread.class("java.lang.Integer").await?;
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;

        let name = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", name)?;

        let type_object = Value::Object(None);
        member_name.set_value("type", type_object)?;

        let flags = MemberNameFlags::IS_FIELD.bits()
            | i32::from(FieldAccessFlags::PUBLIC.bits())
            | i32::from(FieldAccessFlags::STATIC.bits())
            | i32::from(FieldAccessFlags::FINAL.bits())
            | (i32::from(ReferenceKind::GetStatic.kind()) << 24);
        member_name.set_value("flags", Value::Int(flags))?;

        let caller_class = thread.class("java.lang.Object").await?;
        let caller = caller_class.to_object(&thread).await?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        parameters.push(caller);

        let result = resolve_0(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_resolve_1_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;

        let class_object = thread.class("java.lang.Integer").await?;
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;

        let name = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", name)?;

        let type_object = Value::Object(None);
        member_name.set_value("type", type_object)?;

        let flags = MemberNameFlags::IS_FIELD.bits()
            | i32::from(FieldAccessFlags::PUBLIC.bits())
            | i32::from(FieldAccessFlags::STATIC.bits())
            | i32::from(FieldAccessFlags::FINAL.bits())
            | (i32::from(ReferenceKind::GetStatic.kind()) << 24);
        member_name.set_value("flags", Value::Int(flags))?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        // caller (null)
        parameters.push(Value::Object(None));
        // speculative_resolve
        parameters.push(Value::from(false));

        let result = resolve_1(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_resolve_2_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        let member_name_class = thread.class("java.lang.invoke.MemberName").await?;
        let mut member_name = Object::new(member_name_class)?;

        let class_object = thread.class("java.lang.Integer").await?;
        let class = class_object.to_object(&thread).await?;
        member_name.set_value("clazz", class)?;

        let name = "MAX_VALUE".to_object(&thread).await?;
        member_name.set_value("name", name)?;

        let type_object = Value::Object(None);
        member_name.set_value("type", type_object)?;

        let flags = MemberNameFlags::IS_FIELD.bits()
            | i32::from(FieldAccessFlags::PUBLIC.bits())
            | i32::from(FieldAccessFlags::STATIC.bits())
            | i32::from(FieldAccessFlags::FINAL.bits())
            | (i32::from(ReferenceKind::GetStatic.kind()) << 24);
        member_name.set_value("flags", Value::Int(flags))?;

        parameters.push(Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(member_name),
        ));
        // caller (null)
        parameters.push(Value::Object(None));
        // lookup_mode (TRUSTED = -1 for full access)
        parameters.push(Value::Int(-1));
        // speculative_resolve
        parameters.push(Value::from(false));

        let result = resolve_2(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }
}

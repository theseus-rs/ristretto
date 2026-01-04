//! Implements the JVM `invokedynamic` instruction and its resolution process.
//!
//! # Overview
//!
//! The `invokedynamic` instruction enables dynamic method invocation in the JVM, supporting
//! features such as lambda expressions, method references, and dynamic language implementations.
//! Unlike other invocation instructions, `invokedynamic` does not statically link to a method at
//! compile time. Instead, it uses a *bootstrap method* to resolve the call site at runtime,
//! allowing for flexible and efficient dynamic dispatch.
//!
//! # Resolution Process
//!
//! The resolution of an `invokedynamic` call site involves several steps:
//!
//! 1. **Instruction Decoding:** The JVM reads the `invokedynamic` instruction and extracts its
//!    constant pool index.
//! 2. **Bootstrap Method Lookup:** The constant pool entry provides a reference to a bootstrap
//!    method and the method signature for the call site.
//! 3. **Argument Preparation:** The JVM prepares arguments for the bootstrap method, including a
//!    `MethodHandles.Lookup` object, the method name, the method type, and any additional static
//!    arguments.
//! 4. **Bootstrap Method Invocation:** The bootstrap method is invoked to produce a `CallSite`
//!    object, which encapsulates the target method handle for the call site.
//! 5. **Call Site Caching:** The resolved `CallSite` and its target are cached for efficient
//!    subsequent invocations.
//! 6. **Dynamic Invocation:** The target method handle is invoked with the runtime arguments.
//!
//! ## Step 1: Instruction Encounter and Validation
//!
//! 1.1 JVM encounters invokedynamic instruction during bytecode execution
//!
//! 1.2 Extract the constant pool index from the instruction operand;
//!     occurs when the `ClassFile` is read and `Instruction::Invokedynamic` is created
//!
//! 1.3 Resolve the `CONSTANT_InvokeDynamic_info` entry at that index
//!
//! 1.4 Validate that the entry contains: (e.g. `ClassFile.verify()`, also occurs at runtime)
//!   - `bootstrap_method_attr_index` (points to `BootstrapMethods` attribute)
//!   - `name_and_type_index` (method name and descriptor)
//!
//! ## Step 2: Bootstrap Method Resolution
//!
//! 2.1 Use `bootstrap_method_attr_index` to locate the specific bootstrap method entry
//!
//! 2.2 The bootstrap method entry contains:
//!   - `bootstrap_method_ref` (`CONSTANT_MethodHandle_info` index)
//!   - `num_bootstrap_arguments` (count of static arguments)
//!   - `bootstrap_arguments[]` (array of constant pool indices)
//!
//! 2.3 Extract method name from `name_and_type_index`:
//!   - Resolve `CONSTANT_NameAndType_info` entry
//!   - Extract UTF8 string for method name
//!
//! 2.4 Validate bootstrap method signature matches required pattern:
//!     (`MethodHandles.Lookup`, `String`, `MethodType`, ...additionalArgs) -> `CallSite`
//!
//! ## Step 3: Argument Preparation
//!
//! 3.1 Create MethodHandles.Lookup object:
//!   - Set lookup class to the class containing the invokedynamic instruction
//!   - Set access modes based on the calling class's access rights
//!   - Lookup modes include: MODULE, PACKAGE, PROTECTED, PUBLIC
//!
//! 3.2 Create `MethodType` from method descriptor:
//!   - Parse method descriptor string (e.g., "(Ljava/lang/String;I)V")
//!   - Resolve all parameter and return types
//!   - Construct `MethodType` object with resolved types
//!
//! 3.3 Resolve the `bootstrap_method_ref` to get the actual `MethodHandle`
//!
//! 3.4 Prepare additional static arguments:
//!   - For each `bootstrap_arguments[i]`:
//!     - Resolve constant pool entry at that index
//!     - Convert to appropriate Java object based on constant type:
//!       * `CONSTANT_String` -> `String`
//!       * `CONSTANT_Class` -> `Class<?>`
//!       * `CONSTANT_Integer` -> `Integer`
//!       * `CONSTANT_Long` -> `Long`
//!       * `CONSTANT_Float` -> `Float`
//!       * `CONSTANT_Double` -> `Double`
//!       * `CONSTANT_MethodHandle` -> `MethodHandle`
//!       * `CONSTANT_MethodType` -> `MethodType`
//!
//! ## Step 4: Bootstrap Method Invocation
//!
//! 4.1 Construct argument array for bootstrap method:
//!   ```text
//!   Object[] args = {
//!       lookupObject,           // MethodHandles.Lookup
//!       methodName,             // String
//!       methodType,             // MethodType
//!       staticArg1,             // First additional argument (if any)
//!       staticArg2,             // Second additional argument (if any)
//!       ...                     // Remaining static arguments
//!   }
//!   ```
//!
//! 4.2 Invoke bootstrap method using `MethodHandle.invoke()`:
//!   ```text
//!   try {
//!       CallSite result = (CallSite) bootstrapMethodHandle.invoke(args);
//!   } catch (Throwable t) {
//!       throw new BootstrapMethodError("Bootstrap method failed", t);
//!   }
//!   ```
//!
//! 4.3 Validate returned `CallSite`:
//!   - Must not be null
//!   - `CallSite.type()` must exactly match the expected `MethodType`
//!   - If validation fails, throw `BootstrapMethodError`
//!
//! ## Step 5: Call Site Linkage and Caching
//!
//! 5.1 Extract target `MethodHandle` from `CallSite`:
//!     `MethodHandle` target = `callSite.getTarget()`;
//!
//! 5.2 Validate target `MethodHandle`:
//!   - Must not be null
//!   - `target.type()` must exactly match expected `MethodType`
//!   - If validation fails, throw `BootstrapMethodError`
//!
//! 5.3 Create call site cache entry:
//!   - Store `CallSite` object indexed by the invokedynamic instruction location
//!   - Store target `MethodHandle` for fast access
//!   - Mark call site as resolved
//!
//! 5.4 Set up call site invalidation handling:
//!   - If `CallSite` is `MutableCallSite` or `VolatileCallSite`:
//!     - Register for target change notifications
//!     - Set up invalidation callbacks for JIT compiler
//!
//! ## Step 6: Method Invocation Setup
//!
//! 6.1 Prepare for actual method invocation:
//!   - Stack frame contains the runtime arguments for the dynamic method
//!   - Target `MethodHandle` is now available for invocation
//!
//! 6.2 Configure JIT compilation hints:
//!   - Mark call site for potential inlining
//!   - If `ConstantCallSite`, mark target as stable for aggressive optimization
//!   - If mutable call site, set up guard conditions for speculative inlining
//!
//! 6.3 Execute the target method:
//!   - Use `MethodHandle.invoke()` or `invokeExact()`
//!   - Pass runtime arguments from the current stack frame
//!   - Handle return value according to method descriptor
//!
//! # References
//!
//! - [JVMS §6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic)
//! - [JVMS §4.7.23](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23)

use crate::Error::InternalError;
use crate::JavaError::BootstrapMethodError;
use crate::assignable::Assignable;
use crate::call_site_cache::CallSiteKey;
use crate::frame::{ExecutionResult, Frame};
use crate::intrinsic_methods::java::lang::invoke::methodhandle::call_method_handle_target;
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{Constant, ConstantPool, FieldType, ReferenceKind};
use ristretto_classloader::{Class, Method, Reference, Value};
use std::sync::Arc;
use tracing::debug;

/// **Step 1.3** Resolve the `CONSTANT_InvokeDynamic_info` entry at that index
///
/// Extracts bootstrap method information from the constant pool for dynamic invocation.
///
/// This function retrieves the bootstrap method attribute index and name/type index from the
/// constant pool entry referenced by `method_index`.
///
/// # Errors
///
/// Returns an error if the constant pool entry is not a valid `InvokeDynamic` entry or if the
/// bootstrap method attribute index or name/type index cannot be retrieved.
fn get_bootstrap_method_attribute_name_and_type(
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<(u16, u16)> {
    if let Constant::InvokeDynamic {
        bootstrap_method_attr_index,
        name_and_type_index,
    } = constant_pool.try_get(method_index)?
    {
        Ok((*bootstrap_method_attr_index, *name_and_type_index))
    } else {
        Err(BootstrapMethodError(format!(
            "Invalid constant pool index for invokedynamic: {method_index}"
        ))
        .into())
    }
}

/// **Step 2.1** Use `bootstrap_method_attr_index` to locate the specific bootstrap method entry
///
/// Get the bootstrap method definition for the specified class.
///
/// This function retrieves a specific `BootstrapMethod` from the class's bootstrap methods
/// attribute using the provided index. Bootstrap methods are used in the Java Virtual Machine
/// to support dynamic invocation through the `invokedynamic` instruction.
///
/// # Errors
///
/// Returns an error if the bootstrap method attribute index is invalid or if the bootstrap method
/// cannot be found in the class's attributes.
fn get_bootstrap_method_attribute(
    frame: &Frame,
    bootstrap_method_attr_index: u16,
) -> Result<BootstrapMethod> {
    let class = frame.class();
    // Step 1.4 Validate that the entry contains:
    //     - bootstrap_method_attr_index (points to BootstrapMethods attribute)
    class
        .class_file()
        .attributes
        .iter()
        .find_map(|attribute| {
            if let Attribute::BootstrapMethods { methods, .. } = attribute {
                methods.get(bootstrap_method_attr_index as usize).cloned()
            } else {
                None
            }
        })
        .ok_or_else(|| {
            BootstrapMethodError(format!(
                "No bootstrap methods found for class {} at index {}",
                class.name(),
                bootstrap_method_attr_index
            ))
            .into()
        })
}

/// **Step 2.2 / 2.3 / 2.4** The bootstrap method entry contains:
///   - `bootstrap_method_ref` (`CONSTANT_MethodHandle_info` index)
///   - `num_bootstrap_arguments` (count of static arguments)
///   - `bootstrap_arguments[]` (array of constant pool indices)
///
/// Resolves the bootstrap method reference and retrieves the method handle, class, and method name
/// and method descriptor for the invokedynamic instruction.
///
/// # Errors
///
/// Returns an error if the bootstrap method reference cannot be resolved, if the method signature
/// does not match the expected pattern, or if any constant pool entries are invalid.
async fn resolve_bootstrap_method<'a>(
    thread: &Arc<Thread>,
    constant_pool: &'a ConstantPool,
    bootstrap_method_attribute: &BootstrapMethod,
) -> Result<(&'a ReferenceKind, Arc<Class>, &'a str, Arc<Method>)> {
    let (reference_kind, method_ref) =
        constant_pool.try_get_method_handle(bootstrap_method_attribute.bootstrap_method_ref)?;
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(*method_ref)?;
    let bootstrap_class_name = constant_pool.try_get_class(*class_index)?;
    let bootstrap_class = thread.class(bootstrap_class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;

    // 2.3 Extract method name from name_and_type_index:
    //   - Resolve CONSTANT_NameAndType_info entry
    //   - Extract UTF8 string for method name
    let bootstrap_method_name = constant_pool.try_get_utf8(*name_index)?;

    let bootstrap_method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let bootstrap_method =
        bootstrap_class.try_get_method(bootstrap_method_name, bootstrap_method_descriptor)?;

    // 2.4 Validate bootstrap method signature matches required pattern:
    //     (MethodHandles.Lookup, String, MethodType|TypeDescriptor, ...additionalArgs) -> CallSite|Object
    //
    // Bootstrap methods can use either:
    // - MethodType for standard lambda/method reference bootstrap methods
    // - TypeDescriptor for record-related bootstrap methods (ObjectMethods.bootstrap)
    //
    // Return types can be:
    // - CallSite for standard bootstrap methods
    // - Object for record-related bootstrap methods (which return MethodHandle)
    let is_valid_third_arg = bootstrap_method_descriptor.contains("Ljava/lang/invoke/MethodType;")
        || bootstrap_method_descriptor.contains("Ljava/lang/invoke/TypeDescriptor;");
    let is_valid_return = bootstrap_method_descriptor.ends_with(")Ljava/lang/invoke/CallSite;")
        || bootstrap_method_descriptor.ends_with(")Ljava/lang/Object;");

    if !bootstrap_method_descriptor
        .starts_with("(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;")
        || !is_valid_third_arg
        || !is_valid_return
    {
        return Err(BootstrapMethodError(format!(
            "Invalid bootstrap method descriptor: {bootstrap_method_descriptor}"
        ))
        .into());
    }

    Ok((
        reference_kind,
        bootstrap_class,
        bootstrap_method_descriptor,
        bootstrap_method,
    ))
}

/// **Step 3.1** Create MethodHandles.Lookup object:
///   - Set lookup class to the class containing the invokedynamic instruction
///   - Set access modes based on the calling class's access rights
///   - Lookup modes include: MODULE, PACKAGE, PROTECTED, PUBLIC
///
/// Resolves the `MethodHandles.Lookup` object for the current thread.
///
/// # Errors
///
/// Returns an error if the `MethodHandles` class cannot be found or if the lookup method cannot be
/// resolved.
async fn get_method_handles_lookup(thread: &Thread) -> Result<Value> {
    let method_handles_class = thread.class("java.lang.invoke.MethodHandles").await?;
    let lookup_method = method_handles_class
        .try_get_method("lookup", "()Ljava/lang/invoke/MethodHandles$Lookup;")?;
    let lookup = thread
        .try_execute(&method_handles_class, &lookup_method, &[] as &[Value])
        .await?;
    Ok(lookup)
}

/// Creates a Lookup object with full access rights for the specified class.
/// This is used for bootstrap method invocation where we need a properly privileged lookup.
///
/// # Errors
///
/// Returns an error if the Lookup cannot be created.
async fn get_private_lookup(thread: &Thread, caller_class: &Arc<Class>) -> Result<Value> {
    let caller_class_object = caller_class.to_object(thread).await?;
    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;

    // Construct lookup directly with TRUSTED mode (-1) to bypass all access checks
    // The TRUSTED mode (-1) is what IMPL_LOOKUP uses and is required for
    // bootstrap method invocation where the lookup needs access to JDK internals.
    let mut lookup_instance = ristretto_classloader::Object::new(lookup_class.clone())?;
    lookup_instance.set_value("lookupClass", caller_class_object.clone())?;
    lookup_instance.set_value("prevLookupClass", Value::Object(None))?;
    lookup_instance.set_value("allowedModes", Value::Int(-1))?;

    debug!(
        "Created trusted lookup for class '{}' with allowedModes=-1",
        caller_class.name()
    );

    Ok(Value::from(lookup_instance))
}

/// **For Step 3.2** Resolves a Java class object corresponding to a field type.
///
/// This function takes an optional `FieldType` and returns the Java class object (as a `Value`)
/// that represents that type in the JVM. For primitive types and arrays, this returns the
/// corresponding class objects (like `java.lang.Integer.TYPE` for `int`). For reference types, it
/// loads the class for the specified type.
///
/// # Errors
///
/// Returns an error if the class cannot be resolved or if the field type is invalid.
async fn get_field_type_class(thread: &Thread, field_type: Option<FieldType>) -> Result<Value> {
    let class_name = if let Some(field_type) = field_type {
        field_type.class_name()
    } else {
        "void".to_string()
    };
    let class = thread.class(class_name).await?;
    class.to_object(thread).await
}

/// **Step 3.2** Create `MethodType` from method descriptor:
///   - Parse method descriptor string (e.g., "(Ljava/lang/String;I)V")
///   - Resolve all parameter and return types
///   - Construct `MethodType` object with resolved types
///
/// Constructs a `java.lang.invoke.MethodType` object from a method descriptor string.
///
/// This function parses the provided method descriptor into its constituent parts (argument types
/// and return type), creates `Class` objects for each type, and then invokes the appropriate
/// `MethodType.methodType()` factory method to create a `MethodType` instance.
///
/// # Errors
///
/// Returns an error if the method descriptor is invalid or if any of the field types cannot be
/// resolved.
async fn get_method_type(thread: &Thread, method_descriptor: &str) -> Result<Value> {
    let (argument_types, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
    let return_class = get_field_type_class(thread, return_type).await?;

    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    if argument_types.is_empty() {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        return thread
            .try_execute(&method_type_class, &method, &[return_class])
            .await;
    }

    let first_argument = get_field_type_class(thread, argument_types.first().cloned()).await?;
    let mut argument_classes: Vec<Value> = Vec::with_capacity(argument_types.len() - 1);
    for argument_type in argument_types.iter().skip(1) {
        let argument_class = get_field_type_class(thread, Some(argument_type.clone())).await?;
        argument_classes.push(argument_class);
    }
    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let reference_array = Reference::try_from((class_array, argument_classes))?;
    let arguments = Value::from(reference_array);

    let method = method_type_class.try_get_method(
        "methodType",
        "(Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
    )?;
    let arguments = vec![return_class, first_argument, arguments];
    thread
        .try_execute(&method_type_class, &method, &arguments)
        .await
}

/// Construct a `java.lang.invoke.MethodHandle` instance for a `CONSTANT_MethodHandle`
///
/// This function creates a `MethodHandle` object based on the provided reference kind and index. It
/// resolves the target constant from the constant pool and retrieves the class and method or field
/// name/type information. It then uses the `MethodHandles.Lookup` object to find the corresponding
/// method handle.
///
/// # Errors
///
/// Returns an error if the target constant is not a valid field or method reference, or if the
/// class or method/field cannot be resolved.
#[expect(clippy::too_many_lines)]
pub async fn get_method_handle(
    thread: &Arc<Thread>,
    constant_pool: &ConstantPool,
    reference_kind: &ReferenceKind,
    reference_index: u16,
) -> Result<Value> {
    // 1. Lookup the referenced constant
    let target = constant_pool.try_get(reference_index)?;

    // 2. Get (class, name, descriptor, type) for the target
    let (class_name, member_name, member_descriptor, is_method) = match target {
        Constant::FieldRef {
            class_index,
            name_and_type_index,
        } => {
            let class_name = constant_pool.try_get_class(*class_index)?;
            let (field_name_index, field_descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*field_name_index)?;
            let field_descriptor = constant_pool.try_get_utf8(*field_descriptor_index)?;
            (class_name, field_name, field_descriptor, false)
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
            let (method_name_index, method_descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let method_name = constant_pool.try_get_utf8(*method_name_index)?;
            let method_descriptor = constant_pool.try_get_utf8(*method_descriptor_index)?;
            (class_name, method_name, method_descriptor, true)
        }
        // Method handles can also reference other method handles recursively
        Constant::MethodHandle {
            reference_kind: nested_reference_kind,
            reference_index: nested_reference_index,
        } => {
            // Recursively resolve the nested method handle reference
            return Box::pin(get_method_handle(
                thread,
                constant_pool,
                nested_reference_kind,
                *nested_reference_index,
            ))
            .await;
        }
        _ => {
            return Err(InternalError(format!(
                "Unsupported MethodHandle target constant type at index {reference_index}: {target:?}",
            )));
        }
    };

    // 3. Get the class and lookup object
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(thread).await?;
    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;

    // Try to get IMPL_LOOKUP which is a properly initialized trusted lookup.
    // IMPL_LOOKUP has access to all classes including java.lang.invoke internals.
    let lookup = if let Ok(impl_lookup) = lookup_class.static_value("IMPL_LOOKUP") {
        if impl_lookup.is_null() {
            // Fallback: create a trusted lookup directly
            let mut trusted_lookup = ristretto_classloader::Object::new(lookup_class.clone())?;
            trusted_lookup.set_value("lookupClass", class_object.clone())?;
            trusted_lookup.set_value("prevLookupClass", Value::Object(None))?;
            trusted_lookup.set_value("allowedModes", Value::Int(-1))?;
            Value::from(trusted_lookup)
        } else {
            impl_lookup
        }
    } else {
        // Fallback: create a trusted lookup directly
        let mut trusted_lookup = ristretto_classloader::Object::new(lookup_class.clone())?;
        trusted_lookup.set_value("lookupClass", class_object.clone())?;
        trusted_lookup.set_value("prevLookupClass", Value::Object(None))?;
        trusted_lookup.set_value("allowedModes", Value::Int(-1))?;
        Value::from(trusted_lookup)
    };

    // 4. Build the Java String and MethodType for the member
    let name_value = member_name.to_object(thread).await?;
    let method_type = if is_method {
        get_method_type(thread, member_descriptor).await?
    } else {
        // not used for fields
        Value::Object(None)
    };

    // 5. Select and call the right Lookup method based on ReferenceKind
    let method_handle = match (reference_kind, is_method) {
        (ReferenceKind::InvokeStatic, true) => {
            let method_handle = lookup_class.try_get_method(
                "findStatic",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, method_type],
                )
                .await?
        }
        (ReferenceKind::InvokeVirtual | ReferenceKind::InvokeInterface, true) => {
            let method_handle = lookup_class.try_get_method(
                "findVirtual",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, method_type],
                )
                .await?
        }
        (ReferenceKind::InvokeSpecial, true) => {
            let method_handle = lookup_class.try_get_method(
                "findSpecial",
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[
                        lookup.clone(),
                        class_object.clone(),
                        name_value,
                        method_type.clone(),
                        class_object,
                    ],
                )
                .await?
        }
        (ReferenceKind::NewInvokeSpecial, true) => {
            let method_handle = lookup_class.try_get_method(
                "findConstructor",
                "(Ljava/lang/Class;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            )?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, method_type],
                )
                .await?
        }
        (ReferenceKind::GetField | ReferenceKind::GetStatic, false) => {
            let method_name = if *reference_kind == ReferenceKind::GetField {
                "findGetter"
            } else {
                "findStaticGetter"
            };
            let method_handle = lookup_class.try_get_method(
                method_name,
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            // field type from descriptor
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, field_class_object],
                )
                .await?
        }
        (ReferenceKind::PutField | ReferenceKind::PutStatic, false) => {
            let method_name = if *reference_kind == ReferenceKind::PutField {
                "findSetter"
            } else {
                "findStaticSetter"
            };
            let method_handle = lookup_class.try_get_method(
                method_name,
                "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;",
            )?;
            let field_type = FieldType::parse(member_descriptor)?.class_name();
            let field_class = thread.class(field_type).await?;
            let field_class_object = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, field_class_object],
                )
                .await?
        }

        _ => {
            return Err(InternalError(format!(
                "Unsupported method handle reference kind: {reference_kind:?} is_method: {is_method}"
            )));
        }
    };

    Ok(method_handle)
}

/// **Step 3.4** Prepare additional static arguments:
///   - For each `bootstrap_arguments[i]`:
///     - Resolve constant pool entry at that index
///     - Convert to appropriate Java object based on constant type:
///       * `CONSTANT_String` -> `String`
///       * `CONSTANT_Class` -> `Class<?>`
///       * `CONSTANT_Integer` -> `Integer`
///       * `CONSTANT_Long` -> `Long`
///       * `CONSTANT_Float` -> `Float`
///       * `CONSTANT_Double` -> `Double`
///       * `CONSTANT_MethodHandle` -> `MethodHandle`
///       * `CONSTANT_MethodType` -> `MethodType`
///
/// Appends bootstrap method static arguments to the arguments vector for invokedynamic resolution.
///
/// In the JVM specification, bootstrap methods can have additional static arguments that are stored
/// in the constant pool. This function resolves those arguments from the constant pool and creates
/// arguments vector.
///
/// # Errors
///
/// Returns an error if any of the constant pool entries are invalid or if the conversion to Java
/// objects fails.
async fn resolve_static_bootstrap_arguments(
    thread: &Arc<Thread>,
    constant_pool: &ConstantPool,
    bootstrap_method: &BootstrapMethod,
) -> Result<Vec<Value>> {
    let mut arguments = Vec::with_capacity(bootstrap_method.arguments.len());
    for argument in &bootstrap_method.arguments {
        let constant = constant_pool.try_get(*argument)?;
        match constant {
            Constant::Integer(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Float(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Long(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::Double(value) => {
                arguments.push(Value::from(*value));
            }
            Constant::String(value) => {
                let string = constant_pool.try_get_utf8(*value)?;
                let value = string.to_object(thread).await?;
                arguments.push(value);
            }
            Constant::Class(class_index) => {
                let class_name = constant_pool.try_get_utf8(*class_index)?;
                let class = thread.class(class_name).await?;
                let class_object = class.to_object(thread).await?;
                arguments.push(class_object);
            }
            Constant::MethodType(descriptor_index) => {
                let descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
                let method_type = get_method_type(thread, descriptor).await?;
                arguments.push(method_type);
            }
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                let method_handle =
                    get_method_handle(thread, constant_pool, reference_kind, *reference_index)
                        .await?;
                arguments.push(method_handle);
            }
            _ => {
                return Err(BootstrapMethodError(format!(
                    "Invalid bootstrap argument type at constant pool index {} (tag: {})",
                    argument,
                    constant.tag()
                ))
                .into());
            }
        }
    }
    Ok(arguments)
}

/// Adjusts static arguments for bootstrap methods with varargs parameters.
///
/// If the bootstrap method's last parameter is an array type (varargs), this function
/// collects the trailing static arguments into an array. This is required for bootstrap
/// methods like `ObjectMethods.bootstrap` which expects:
/// `(Lookup, String, TypeDescriptor, Class, String, MethodHandle...)Object`
///
/// # Arguments
///
/// * `thread` - The current thread
/// * `bootstrap_method_descriptor` - The descriptor of the bootstrap method
/// * `static_arguments` - The resolved static arguments
///
/// # Returns
///
/// The adjusted static arguments with varargs collected into an array if needed.
async fn adjust_varargs_static_arguments(
    thread: &Arc<Thread>,
    bootstrap_method_descriptor: &str,
    mut static_arguments: Vec<Value>,
) -> Result<Vec<Value>> {
    debug!(
        "adjust_varargs_static_arguments: descriptor={}, static_arguments.len()={}",
        bootstrap_method_descriptor,
        static_arguments.len()
    );

    // Parse the bootstrap method descriptor to get parameter types
    let (param_types, _) = FieldType::parse_method_descriptor(bootstrap_method_descriptor)?;

    // The first 3 params are always: Lookup, String, MethodType/TypeDescriptor
    // The remaining params come from static_arguments
    let static_param_types: Vec<_> = param_types.into_iter().skip(3).collect();

    debug!(
        "adjust_varargs_static_arguments: static_param_types.len()={}",
        static_param_types.len()
    );

    if static_param_types.is_empty() {
        return Ok(static_arguments);
    }

    // Check if the last parameter is an array type (varargs)
    if let Some(FieldType::Array(component_type)) = static_param_types.last() {
        // Count fixed static parameters (all except the varargs array)
        let fixed_count = static_param_types.len() - 1;

        debug!(
            "adjust_varargs_static_arguments: varargs detected, fixed_count={}, static_arguments.len()={}",
            fixed_count,
            static_arguments.len()
        );

        if static_arguments.len() >= fixed_count {
            // Collect the varargs into an array
            let varargs: Vec<Value> = static_arguments.drain(fixed_count..).collect();

            debug!(
                "adjust_varargs_static_arguments: collected {} varargs",
                varargs.len()
            );

            // Create the array type name
            let array_type_name = format!("[{}", component_type.descriptor());
            let array_class = thread.class(&array_type_name).await?;

            // Box primitives if the array is Object[] (needed for varargs like Object...)
            let boxed_varargs = if array_type_name == "[Ljava/lang/Object;" {
                let mut boxed = Vec::with_capacity(varargs.len());
                for val in varargs {
                    boxed.push(val.to_object(thread).await?);
                }
                boxed
            } else {
                varargs
            };

            // Convert Values directly to the array (preserving Gc pointers)
            let array_ref = Reference::try_from((array_class, boxed_varargs))?;
            static_arguments.push(Value::from(array_ref));

            debug!(
                "adjust_varargs_static_arguments: final static_arguments.len()={}",
                static_arguments.len()
            );
        }
    }

    Ok(static_arguments)
}

/// **Step 4.1 / 4.2 / 4.3** Resolves the call site object for an invokedynamic instruction.
///
/// This function implements the bootstrap method resolution and call site linking process as
/// specified in the JVM specification for the invokedynamic instruction. It:
///
/// 1. Extracts bootstrap method information from the constant pool
/// 2. Resolves the bootstrap method reference into a method handle
/// 3. Creates the necessary arguments for the bootstrap method:
///    - The caller's lookup object (with appropriate access rights)
///    - The method name for the dynamic call site
///    - The method type for the dynamic call site
///    - Any additional static arguments from the constant pool
/// 4. Invokes the bootstrap method to obtain and return a call site object
///
/// # Errors
///
/// Returns an error if any of the following conditions are met:
/// - The bootstrap method cannot be found or resolved
/// - The method name or descriptor cannot be resolved
/// - The bootstrap method invocation fails
/// - Recursive call site resolution is detected
///
/// # References
///
/// - [JVMS §4.7.23](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23)
/// - [JVMS §5.4.3.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.6)
async fn resolve_call_site(frame: &Frame, method_index: u16) -> Result<Value> {
    let thread = frame.thread()?;
    let current_class = frame.class();
    let cache_key = CallSiteKey::new(current_class.name().to_string(), method_index);

    debug!(
        "resolve_call_site: Attempting to resolve call site for class '{}' at CP index {}",
        cache_key.class_name, cache_key.instruction_index
    );

    // Check if we can access the VM and cache
    let vm = thread.vm()?;
    let call_site_cache = vm.call_site_cache();

    debug!("resolve_call_site: Got VM and cache references successfully");

    call_site_cache
        .resolve_with_cache(cache_key, || async {
            debug!(
                "resolve_call_site: Actually resolving (not cached) for class '{}' at CP index {method_index}",
                current_class.name(),
            );
            resolve_call_site_uncached(frame, method_index).await
        })
        .await
}

/// Internal function that performs the actual call site resolution without caching.
async fn resolve_call_site_uncached(frame: &Frame, method_index: u16) -> Result<Value> {
    let thread = frame.thread()?;
    let current_class = frame.class();
    let constant_pool = current_class.constant_pool();

    // Get the bootstrap method attribute name and type
    let (bootstrap_method_attr_index, name_and_type_index) =
        get_bootstrap_method_attribute_name_and_type(constant_pool, method_index)?;
    // 1.4 Validate that the entry contains:
    //     - name_and_type_index (method name and descriptor)
    let (bootstrap_method_attr_name_index, bootstrap_method_attr_descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let bootstrap_method_attr_name =
        constant_pool.try_get_utf8(*bootstrap_method_attr_name_index)?;
    let bootstrap_method_attr_descriptor =
        constant_pool.try_get_utf8(*bootstrap_method_attr_descriptor_index)?;

    // Get the bootstrap method
    let bootstrap_method_attribute =
        get_bootstrap_method_attribute(frame, bootstrap_method_attr_index)?;

    // 2.2 The bootstrap method entry contains:
    //     - bootstrap_method_ref (CONSTANT_MethodHandle_info index)
    //     - num_bootstrap_arguments (count of static arguments)
    //     - bootstrap_arguments[] (array of constant pool indices)
    let (reference_kind, bootstrap_class, _bootstrap_method_descriptor, bootstrap_method) =
        resolve_bootstrap_method(&thread, constant_pool, &bootstrap_method_attribute).await?;

    debug!(
        "invokedynamic: bootstrap method({reference_kind}): {}.{}{}",
        bootstrap_class.name(),
        bootstrap_method.name(),
        bootstrap_method.descriptor()
    );

    // 3.1 Create MethodHandles.Lookup object for the calling class with full access rights
    let caller_class = frame.class();
    let caller_lookup = get_private_lookup(&thread, caller_class).await?;

    // Debug: verify the lookup has the correct allowedModes
    if let Ok(lookup_obj) = caller_lookup.as_object_ref()
        && let Ok(modes) = lookup_obj.value("allowedModes")
        && let Ok(modes_int) = modes.as_i32()
    {
        debug!(
            "Bootstrap lookup for '{}': allowedModes={modes_int}",
            caller_class.name()
        );
    }

    // 3.2 Prepare method name and method type for the invokedynamic call site
    let method_name = bootstrap_method_attr_name.to_object(&thread).await?;
    let method_type = get_method_type(&thread, bootstrap_method_attr_descriptor).await?;

    // 3.4 Resolve static arguments
    let static_arguments =
        resolve_static_bootstrap_arguments(&thread, constant_pool, &bootstrap_method_attribute)
            .await?;

    // Adjust static arguments for varargs bootstrap methods (e.g., ObjectMethods.bootstrap)
    let bootstrap_method_descriptor = bootstrap_method.descriptor();
    let static_arguments =
        adjust_varargs_static_arguments(&thread, bootstrap_method_descriptor, static_arguments)
            .await?;

    // 4.1 Construct argument array for bootstrap method:
    let mut arguments = vec![caller_lookup, method_name, method_type];
    arguments.extend(static_arguments);

    // 4.2 Invoke bootstrap method directly (bypassing MethodHandle)
    // This ensures the bootstrap method has a proper Java frame on the stack
    let call_site_result = thread
        .execute(&bootstrap_class, &bootstrap_method, &arguments)
        .await?
        .ok_or_else(|| BootstrapMethodError("Bootstrap method returned null".to_string()))?;

    // 4.3 Validate returned CallSite:
    validate_call_site(&thread, bootstrap_method_attr_descriptor, &call_site_result).await?;

    Ok(call_site_result)
}

/// **Step 4.2** Invokes the bootstrap method using direct method handle execution.
///
/// # Errors
///
/// Returns an error if the bootstrap method invocation fails or if the method handle is not valid.
async fn invoke_bootstrap_method(
    thread: &Arc<Thread>,
    method_handle: Value,
    mut arguments: Vec<Value>,
) -> Result<Value> {
    // Validate method handle is not null
    if let Value::Object(None) = method_handle {
        return Err(BootstrapMethodError("Bootstrap method handle is null".to_string()).into());
    }

    // Try to get the LambdaForm's vmentry first
    let (form, member) = {
        let mh_ref = method_handle.as_object_ref()?;
        let form = mh_ref.value("form").ok();
        let member = mh_ref.value("member").ok();
        (form, member)
    };

    // Check if we have a LambdaForm with a vmentry
    if let Some(ref form_val) = form
        && !form_val.is_null()
    {
        let vmentry = {
            let form_ref = form_val.as_object_ref()?;
            form_ref.value("vmentry").ok()
        };

        if let Some(ref vmentry_val) = vmentry
            && !vmentry_val.is_null()
        {
            // Prepend the MethodHandle to arguments for LambdaForm invocation
            arguments.insert(0, method_handle.clone());
            let result = call_method_handle_target(thread.clone(), vmentry_val, arguments).await?;

            // Validate call site result is not null
            if let Value::Object(None) = result {
                return Err(
                    BootstrapMethodError("Bootstrap method returned null".to_string()).into(),
                );
            }

            return Ok(result);
        }
    }

    // Fallback: use the member field directly
    if let Some(ref member_val) = member
        && !member_val.is_null()
    {
        let result = call_method_handle_target(thread.clone(), member_val, arguments).await?;

        // Validate call site result is not null
        if let Value::Object(None) = result {
            return Err(BootstrapMethodError("Bootstrap method returned null".to_string()).into());
        }

        return Ok(result);
    }

    Err(BootstrapMethodError("MethodHandle has neither vmentry nor member".to_string()).into())
}

/// **Step 4.3** Validates the `CallSite` returned by the bootstrap method.
///
/// Validate returned `CallSite` or `MethodHandle`:
///   - Must not be null
///   - For `CallSite`: `CallSite.type()` must exactly match the expected `MethodType`
///   - For `MethodHandle`: also valid (used by record-related bootstrap methods)
///   - If validation fails, throw `BootstrapMethodError`
///
/// # Errors
///
/// Returns an error if the result is null, if it does not match the expected type, or if it is
/// neither a `CallSite` nor a `MethodHandle`.
async fn validate_call_site(
    thread: &Arc<Thread>,
    bootstrap_method_descriptor: &str,
    call_site: &Value,
) -> Result<()> {
    if call_site.is_null() {
        return Err(BootstrapMethodError("Bootstrap method returned null".to_string()).into());
    } else if !call_site.is_object() {
        return Err(
            BootstrapMethodError("Bootstrap method did not return an object".to_string()).into(),
        );
    }

    // Check if the returned object is a CallSite or MethodHandle
    let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
    let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;
    let object_class = {
        let object = call_site.as_object_ref()?;
        object.class().clone()
    };

    let is_call_site = call_site_class
        .is_assignable_from(thread, &object_class)
        .await?;
    let is_method_handle = method_handle_class
        .is_assignable_from(thread, &object_class)
        .await?;

    if !is_call_site && !is_method_handle {
        return Err(BootstrapMethodError(format!(
            "Bootstrap method returned object of type {} which is neither a CallSite nor a MethodHandle",
            object_class.name()
        ))
        .into());
    }

    // Only validate CallSite.type() for CallSite results (not for MethodHandle)
    if is_call_site {
        let type_method =
            call_site_class.try_get_method("type", "()Ljava/lang/invoke/MethodType;")?;
        let call_site_type = thread
            .try_execute(
                &call_site_class,
                &type_method,
                std::slice::from_ref(call_site),
            )
            .await?;

        // Get the descriptor string from the CallSite's MethodType
        let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
        let to_descriptor =
            method_type_class.try_get_method("toMethodDescriptorString", "()Ljava/lang/String;")?;
        let call_site_descriptor = thread
            .try_execute(&method_type_class, &to_descriptor, &[call_site_type])
            .await?;
        let call_site_descriptor_str = call_site_descriptor.as_string()?;

        // Compare descriptor strings
        if call_site_descriptor_str != bootstrap_method_descriptor {
            return Err(BootstrapMethodError(format!(
                "CallSite type '{call_site_descriptor_str}' does not match expected descriptor '{bootstrap_method_descriptor}'"
            ))
            .into());
        }
    }
    Ok(())
}

/// Executes the `invokedynamic` JVM instruction for dynamic method invocation.
///
/// The `invokedynamic` instruction is how Java supports dynamic languages and lambda
/// expressions. Unlike other invoke instructions, it does not statically link to a specific method
/// implementation. Instead, it:
///
/// 1. Resolves a call site object through a bootstrap method specified in the class file
/// 2. Dynamically links the call site to a target method implementation
/// 3. Executes the linked method with arguments from the operand stack
/// 4. Caches the linking for subsequent invocations of the same call site
///
/// This instruction enables several Java language features:
/// - Lambda expressions and method references
/// - Dynamic language support on the JVM
/// - Improved performance for interface method invocation
/// - Efficient implementation of functional interfaces
///
/// # Errors
///
/// Returns an error if the call site cannot be resolved, if the bootstrap method fails, or if the
/// target method handle cannot be invoked.
///
/// # References
///
/// - [JVMS §6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic)
#[inline]
pub(crate) async fn invokedynamic(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let current_class = frame.class();

    debug!(
        "invokedynamic: Starting for class '{}' at CP index {}",
        current_class.name(),
        method_index
    );

    // Step 1: Resolve the call site (this may be cached on subsequent calls)
    let call_site_or_method_handle = resolve_call_site(frame, method_index).await?;

    // Step 2: Extract the target MethodHandle from the CallSite or use it directly if it's already a MethodHandle
    let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
    let method_handle_class = thread.class("java.lang.invoke.MethodHandle").await?;

    let object_class = {
        let object = call_site_or_method_handle.as_object_ref()?;
        object.class().clone()
    };

    let target_method_handle = if call_site_class
        .is_assignable_from(&thread, &object_class)
        .await?
    {
        // It's a CallSite - extract the target MethodHandle
        // Use the actual runtime class (e.g., ConstantCallSite) not the abstract CallSite class
        let get_target_method =
            object_class.try_get_method("getTarget", "()Ljava/lang/invoke/MethodHandle;")?;
        thread
            .try_execute(
                &object_class,
                &get_target_method,
                &[call_site_or_method_handle],
            )
            .await?
    } else if method_handle_class
        .is_assignable_from(&thread, &object_class)
        .await?
    {
        // It's already a MethodHandle (from record-related bootstrap methods)
        call_site_or_method_handle
    } else {
        return Err(BootstrapMethodError(format!(
            "Bootstrap method returned object of type {} which is neither a CallSite nor a MethodHandle",
            object_class.name()
        ))
        .into());
    };

    // Step 3: Get the method type to determine argument count and types
    let current_class = frame.class();
    let constant_pool = current_class.constant_pool();
    let (_, name_and_type_index) =
        get_bootstrap_method_attribute_name_and_type(constant_pool, method_index)?;
    let (_, bootstrap_method_attr_descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*bootstrap_method_attr_descriptor_index)?;

    // Parse the method descriptor to determine argument count
    let (argument_types, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;

    // Step 4: Get parameters from the operand stack
    let mut parameters = stack.drain_last(argument_types.len());

    // Step 5: Invoke the target MethodHandle
    // Try to get the LambdaForm's vmentry first
    let (form, member) = {
        let mh_ref = target_method_handle.as_object_ref()?;
        let form = mh_ref.value("form").ok();
        let member = mh_ref.value("member").ok();
        (form, member)
    };

    let result = if let Some(ref form_val) = form
        && !form_val.is_null()
    {
        let vmentry = {
            let form_ref = form_val.as_object_ref()?;
            form_ref.value("vmentry").ok()
        };

        if let Some(ref vmentry_val) = vmentry
            && !vmentry_val.is_null()
        {
            // Prepend the MethodHandle to parameters for LambdaForm invocation
            parameters.insert(0, target_method_handle.clone());
            call_method_handle_target(thread.clone(), vmentry_val, parameters).await?
        } else if let Some(ref member_val) = member
            && !member_val.is_null()
        {
            call_method_handle_target(thread.clone(), member_val, parameters).await?
        } else {
            return Err(BootstrapMethodError(
                "MethodHandle has neither vmentry nor member".to_string(),
            )
            .into());
        }
    } else if let Some(ref member_val) = member
        && !member_val.is_null()
    {
        call_method_handle_target(thread.clone(), member_val, parameters).await?
    } else {
        return Err(BootstrapMethodError(
            "MethodHandle has neither vmentry nor member".to_string(),
        )
        .into());
    };

    // Step 6: Handle the return value based on the method descriptor
    if let Some(_return_type) = return_type {
        // Method has a return value - push it onto the operand stack
        stack.push(result)?;
    }

    Ok(ExecutionResult::Continue)
}

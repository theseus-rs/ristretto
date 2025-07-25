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
//! - [JVM Specification §6.5](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic)
//! - [JVM Specification §4.7.23](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.23)

use crate::Error::InternalError;
use crate::JavaError::BootstrapMethodError;
use crate::assignable::Assignable;
use crate::frame::{ExecutionResult, Frame};
use crate::intrinsic_methods::java::lang::invoke::methodhandle::call_method_handle_target;
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{Constant, ConstantPool, FieldType, ReferenceKind};
use ristretto_classloader::{Class, ConcurrentVec, Method, ObjectArray, Reference, Value};
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
    //     (MethodHandles.Lookup, String, MethodType, ...additionalArgs) -> CallSite
    if !bootstrap_method_descriptor.starts_with(
        "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;",
    ) || !bootstrap_method_descriptor.ends_with(")Ljava/lang/invoke/CallSite;")
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
    let argument_classes = ConcurrentVec::from(Vec::with_capacity(argument_types.len() - 1));
    for argument_type in argument_types.iter().skip(1) {
        let argument_class = get_field_type_class(thread, Some(argument_type.clone())).await?;
        let argument_reference: Reference = argument_class.try_into()?;
        argument_classes.push(Some(argument_reference))?;
    }
    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let reference_array = Reference::Array(ObjectArray {
        class: class_array,
        elements: argument_classes,
    });
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
    let lookup_object = get_method_handles_lookup(thread).await?;
    // We use the "trusted" lookup for bootstraps
    let lookup = thread
        .try_execute(
            &lookup_class,
            &lookup_class.try_get_method(
                "in",
                "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;",
            )?,
            &[lookup_object, class_object.clone()],
        )
        .await?;

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
        (ReferenceKind::InvokeVirtual, true) => {
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
            let field_class_obj = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, field_class_obj],
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
            let field_class_obj = field_class.to_object(thread).await?;
            thread
                .try_execute(
                    &lookup_class,
                    &method_handle,
                    &[lookup.clone(), class_object, name_value, field_class_obj],
                )
                .await?
        }
        (ReferenceKind::InvokeInterface, true) => {
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
/// 4. Invokes the bootstrap method to obtain and return a callsite object
///
/// # Errors
///
/// Returns an error if any of the following conditions are met:
/// - The bootstrap method cannot be found or resolved
/// - The method name or descriptor cannot be resolved
/// - The bootstrap method invocation fails
///
/// # References
///
/// - [JVM Specification §4.7.3](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.23)
/// - [JVM Specification §5.4.3.6](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.6)
pub async fn resolve_call_site(frame: &Frame, method_index: u16) -> Result<Value> {
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

    // 3.1 Create MethodHandles.Lookup object for the calling class
    let lookup = get_method_handles_lookup(&thread).await?;
    let caller_class = frame.class();
    let caller_class_object = caller_class.to_object(&thread).await?;
    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;
    let lookup_in_method = lookup_class.try_get_method(
        "in",
        "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;",
    )?;
    let caller_lookup = thread
        .try_execute(
            &lookup_class,
            &lookup_in_method,
            &[lookup, caller_class_object],
        )
        .await?;

    // 3.2 Prepare method name and method type for the invokedynamic call site
    let method_name = bootstrap_method_attr_name.to_object(&thread).await?;
    let method_type = get_method_type(&thread, bootstrap_method_attr_descriptor).await?;

    // 3.4 Resolve static arguments
    let static_arguments =
        resolve_static_bootstrap_arguments(&thread, constant_pool, &bootstrap_method_attribute)
            .await?;

    // 4.1 Construct argument array for bootstrap method:
    let mut arguments = vec![caller_lookup, method_name, method_type];
    arguments.extend(static_arguments);

    // 4.2 Invoke bootstrap method using the bootstrap method handle directly
    let bootstrap_method_handle = get_method_handle(
        &thread,
        constant_pool,
        reference_kind,
        bootstrap_method_attribute.bootstrap_method_ref,
    )
    .await?;

    let call_site_result =
        invoke_bootstrap_method(&thread, bootstrap_method_handle, arguments).await?;

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
    arguments: Vec<Value>,
) -> Result<Value> {
    // Validate method handle is not null
    if let Value::Object(None) = method_handle {
        return Err(BootstrapMethodError("Bootstrap method handle is null".to_string()).into());
    }

    // Extract the target member from the method handle and call it directly
    // to avoid infinite recursion through thread.try_execute()
    let target_member: ristretto_classloader::Object = method_handle.try_into()?;
    let member: ristretto_classloader::Object = target_member.value("member")?.try_into()?;

    let result = call_method_handle_target(thread.clone(), member, arguments).await?;

    // Validate call site result is not null
    if let Value::Object(None) = result {
        return Err(BootstrapMethodError("Bootstrap method returned null".to_string()).into());
    }

    Ok(result)
}

/// **Step 4.3** Validates the `CallSite` returned by the bootstrap method.
///
/// Validate returned `CallSite`:
///   - Must not be null
///   - `CallSite.type()` must exactly match the expected `MethodType`
///   - If validation fails, throw `BootstrapMethodError`
///
/// # Errors
///
/// Returns an error if the `CallSite` is null, if it does not match the expected type, or if it does
/// not implement the `CallSite` interface.
async fn validate_call_site(
    thread: &Arc<Thread>,
    bootstrap_method_descriptor: &str,
    call_site: &Value,
) -> Result<()> {
    if let Value::Object(None) = call_site {
        return Err(
            BootstrapMethodError("Bootstrap method returned null CallSite".to_string()).into(),
        );
    }

    // Validate that the returned object is actually a CallSite
    let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
    let call_site_reference: Reference = call_site.clone().try_into()?;

    if let Reference::Object(object) = call_site_reference {
        // Check if the object's class is assignable from CallSite class
        let object_class = object.class();
        if !call_site_class
            .is_assignable_from(thread, object_class)
            .await?
        {
            return Err(BootstrapMethodError(format!(
                "Bootstrap method returned object of type {} which is not a CallSite",
                object_class.name()
            ))
            .into());
        }
    } else {
        return Err(
            BootstrapMethodError("Bootstrap method did not return an object".to_string()).into(),
        );
    }

    // Validate CallSite.type() matches expected MethodType
    let type_method = call_site_class.try_get_method("type", "()Ljava/lang/invoke/MethodType;")?;
    let call_site_type = thread
        .try_execute(&call_site_class, &type_method, &[call_site.clone()])
        .await?;
    let expected_method_type = get_method_type(thread, bootstrap_method_descriptor).await?;

    // Compare the method types
    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    let equals_method = method_type_class.try_get_method("equals", "(Ljava/lang/Object;)Z")?;
    let types_equal = thread
        .try_execute(
            &method_type_class,
            &equals_method,
            &[call_site_type, expected_method_type],
        )
        .await?;

    if let Value::Int(0) = types_equal {
        return Err(BootstrapMethodError(
            "CallSite type does not match expected MethodType".to_string(),
        )
        .into());
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
/// - [JVM Specification §6.5](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic)
#[inline]
pub(crate) async fn invokedynamic(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;

    // Step 1: Resolve the call site (this may be cached on subsequent calls)
    let call_site = resolve_call_site(frame, method_index).await?;

    // Step 2: Extract the target MethodHandle from the CallSite
    let call_site_class = thread.class("java.lang.invoke.CallSite").await?;
    let get_target_method =
        call_site_class.try_get_method("getTarget", "()Ljava/lang/invoke/MethodHandle;")?;
    let target_method_handle = thread
        .try_execute(&call_site_class, &get_target_method, &[call_site])
        .await?;

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
    let parameters = stack.drain_last(argument_types.len());

    // Step 5: Invoke the target MethodHandle directly using call_method_handle_target
    // to avoid infinite recursion through thread.try_execute()
    let target_member: ristretto_classloader::Object = target_method_handle.try_into()?;
    let member: ristretto_classloader::Object = target_member.value("member")?.try_into()?;

    let result = call_method_handle_target(thread.clone(), member, parameters).await?;

    // Step 6: Handle the return value based on the method descriptor
    if let Some(_return_type) = return_type {
        // Method has a return value - push it onto the operand stack
        stack.push(result)?;
    }

    Ok(ExecutionResult::Continue)
}

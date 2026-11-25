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
//!    object, which encapsulates the
//!    target method handle for the call site.
//! 5. **Call Site Caching:** The resolved `CallSite` is cached
//!    for efficient subsequent invocations.
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
//! ## Step 3: Argument Preparation
//!
//! 3.1 Create MethodHandles.Lookup object:
//!   - Set lookup class to the class containing the invokedynamic instruction
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
//! 4.2 Invoke the bootstrap handle through `MethodHandle.invokeWithArguments`.
//!
//! 4.3 Validate the returned `CallSite`:
//!   - Must not be null
//!   - For `CallSite`: `CallSite.type()` must exactly match the expected `MethodType`
//!   - If validation fails, throw `BootstrapMethodError`
//!
//! ## Step 5: Call Site Linkage and Caching
//!
//! 5.1 Extract target `MethodHandle`:
//!   - If the result is a `CallSite`: `MethodHandle` target = `callSite.getTarget()`;
//!
//! 5.2 Validate target `MethodHandle`:
//!   - Must not be null
//!   - For `CallSite` results: `target.type()` must exactly match expected `MethodType`
//!   - If validation fails, throw `BootstrapMethodError`
//!
//! 5.3 Create call site cache entry:
//!   - Store result indexed by class identity and invokedynamic instruction index
//!   - Mark call site as resolved
//!   - Subsequent invocations use the cached result
//!
//! ## Step 6: Method Invocation
//!
//! 6.1 Prepare for actual method invocation:
//!   - Parse the method descriptor to determine argument count and types
//!   - Pop runtime arguments from the operand stack
//!
//! 6.2 Execute the target method:
//!   - Resolve the target via the `MethodHandle`'s `LambdaForm` vmentry or `MemberName` member
//!   - Pass runtime arguments from the current stack frame
//!   - Handle return value according to method descriptor
//!
//! # References
//!
//! - [JVMS §6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic)
//! - [JVMS §4.7.23](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23)

use crate::JavaError::BootstrapMethodError;
use crate::call_site_cache::CallSiteKey;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{ClassAccessFlags, Constant, ConstantPool, FieldType, JavaStr};
use ristretto_classloader::{Class, Reference, Value};
use ristretto_intrinsics::call_method_handle_target;
use ristretto_intrinsics::java::lang::invoke::methodhandlenatives::{
    LookupModeFlags, check_method_access,
};
use ristretto_types::Assignable;
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
    constant_pool: &ConstantPool<'_>,
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
    class: &Class,
    bootstrap_method_attr_index: u16,
) -> Result<BootstrapMethod> {
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

// A loader's ClassNotFoundException is the cause of NoClassDefFoundError
// when a symbolic class reference is resolved for a bootstrap.
async fn resolve_bootstrap_class(
    thread: &Thread,
    caller: &Arc<Class>,
    name: &JavaStr,
) -> Result<Arc<Class>> {
    let cause = match thread.load_referenced_class(caller, name).await {
        Ok(class) => {
            // An array has the accessibility of its innermost component.
            let mut element = class.clone();
            while element.is_array() {
                element = thread.array_component_class(&element).await?;
            }
            if !element.is_primitive() && !Arc::ptr_eq(caller, &element) {
                if !element
                    .class_file()
                    .access_flags
                    .contains(ClassAccessFlags::PUBLIC)
                    && !caller.same_runtime_package(&element)?
                {
                    return Err(crate::JavaError::IllegalAccessError(format!(
                        "{} cannot access {}",
                        caller.name(),
                        element.name()
                    ))
                    .into());
                }
                let source = caller.module_name()?;
                let target = element.module_name()?;
                if source != target {
                    thread.vm()?.module_system().require_access(
                        source.as_deref(),
                        target.as_deref(),
                        element.name(),
                    )?;
                }
            }
            return Ok(class);
        }
        Err(crate::Error::ClassLoaderError(ristretto_classloader::Error::ClassNotFound(
            message,
        ))) => {
            super::convert_error_to_throwable(
                thread,
                crate::JavaError::ClassNotFoundException(message).into(),
            )
            .await?
        }
        Err(error @ crate::Error::JavaError(crate::JavaError::ClassNotFoundException(_))) => {
            super::convert_error_to_throwable(thread, error).await?
        }
        Err(crate::Error::Throwable(cause)) => {
            let class = cause.as_object_ref()?.class().clone();
            if !thread
                .class("java/lang/ClassNotFoundException")
                .await?
                .is_assignable_from(thread, &class)
                .await?
            {
                return Err(crate::Error::Throwable(cause));
            }
            cause
        }
        Err(error) => return Err(error),
    };
    let error = thread
        .object(
            "java/lang/NoClassDefFoundError",
            "Ljava/lang/String;",
            &[name.to_object(thread).await?],
        )
        .await?;
    let error = thread
        .try_invoke(
            "java/lang/Throwable",
            "initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
            &[error, cause],
        )
        .await?;
    Err(crate::Error::Throwable(error))
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
async fn get_field_type_class(
    thread: &Thread,
    caller: &Arc<Class>,
    field_type: Option<FieldType>,
) -> Result<Value> {
    let class_name = if let Some(field_type) = field_type {
        field_type.class_name()
    } else {
        "void".to_string()
    };
    // Per JVM spec §5.4.3.5, resolving field/method types for method handles and
    // method types should NOT trigger class initialization.
    let class_name_java = JavaStr::cow_from_str(&class_name);
    let class = resolve_bootstrap_class(thread, caller, &class_name_java).await?;
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
pub(super) async fn get_method_type(
    thread: &Thread,
    caller: &Arc<Class>,
    method_descriptor: &str,
) -> Result<Value> {
    let method_descriptor = JavaStr::cow_from_str(method_descriptor);
    let (argument_types, return_type) = FieldType::parse_method_descriptor(&method_descriptor)?;
    let return_class = get_field_type_class(thread, caller, return_type).await?;

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

    let first_argument =
        get_field_type_class(thread, caller, argument_types.first().cloned()).await?;
    let mut argument_classes: Vec<Value> = Vec::with_capacity(argument_types.len() - 1);
    for argument_type in argument_types.iter().skip(1) {
        let argument_class =
            get_field_type_class(thread, caller, Some(argument_type.clone())).await?;
        argument_classes.push(argument_class);
    }
    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let reference_array = Reference::try_from((class_array, argument_classes))?;
    let arguments = Value::new_object(thread.vm()?.garbage_collector(), reference_array);

    let method = method_type_class.try_get_method(
        "methodType",
        "(Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
    )?;
    let arguments = vec![return_class, first_argument, arguments];
    thread
        .try_execute(&method_type_class, &method, &arguments)
        .await
}

/// Resolve the symbolic member before loading the classes in its descriptor.
async fn link_method_handle(
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
    index: u16,
) -> Result<Value> {
    let pool = caller.constant_pool();
    let (reference_kind, reference_index) = pool.try_get_method_handle(index)?;
    let target = pool.try_get(*reference_index)?;
    let (class_index, name_and_type_index) = match target {
        Constant::FieldRef {
            class_index,
            name_and_type_index,
        }
        | Constant::MethodRef {
            class_index,
            name_and_type_index,
        }
        | Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        } => (*class_index, *name_and_type_index),
        _ => {
            return Err(
                ristretto_classfile::Error::InvalidConstantPoolIndexType(*reference_index).into(),
            );
        }
    };
    let owner = resolve_bootstrap_class(thread, caller, pool.try_get_class(class_index)?).await?;
    let (name_index, descriptor_index) = pool.try_get_name_and_type(name_and_type_index)?;
    let member_name = pool.try_get_utf8(*name_index)?;
    let descriptor = pool.try_get_utf8(*descriptor_index)?;
    let name = member_name.to_str_lossy();
    let member_type = if matches!(target, Constant::FieldRef { .. }) {
        let ty = FieldType::parse_java_str(descriptor)?;
        owner
            .resolve_field(&name, Some(&ty))?
            .ok_or_else(|| crate::JavaError::NoSuchFieldError(name.into_owned()))?;
        get_field_type_class(thread, caller, Some(ty)).await?
    } else {
        let descriptor = descriptor.to_str_lossy();
        if matches!(target, Constant::InterfaceMethodRef { .. }) != owner.is_interface() {
            return Err(
                crate::JavaError::IncompatibleClassChangeError(owner.name().to_owned()).into(),
            );
        }
        let resolved = super::method_resolver::resolve_symbolic_method(
            &owner,
            &name,
            &descriptor,
            thread.vm()?.method_registry(),
        )?;
        if !check_method_access(
            &**thread,
            Some(caller),
            &resolved.declaring_class,
            *resolved.method.access_flags(),
            LookupModeFlags::PRIVATE,
        )
        .await?
        {
            return Err(crate::JavaError::IllegalAccessError(format!(
                "{}.{}{}",
                owner.name(),
                name,
                descriptor
            ))
            .into());
        }
        get_method_type(thread, caller, &descriptor).await?
    };
    // A bootstrap handle is a symbolic reference of the class containing the constant.
    // The JDK linker applies that class's lookup privileges and special-caller semantics.
    thread.try_invoke(
        "java/lang/invoke/MethodHandleNatives",
        "linkMethodHandleConstant(Ljava/lang/Class;ILjava/lang/Class;Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;",
        &[caller.to_object(thread).await?, Value::Int(i32::from(reference_kind.kind())),
          owner.to_object(thread).await?, member_name.to_object(thread).await?, member_type],
    ).await
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
    caller: &Arc<Class>,
    constant_pool: &ConstantPool<'_>,
    bootstrap_method: &BootstrapMethod,
    resolving: &[u16],
) -> Result<Vec<Value>> {
    let mut arguments = Vec::with_capacity(bootstrap_method.arguments.len());
    for argument in &bootstrap_method.arguments {
        let constant = constant_pool.try_get(*argument)?;
        match constant {
            Constant::Integer(value) => {
                arguments.push(value.to_object(thread).await?);
            }
            Constant::Float(value) => {
                arguments.push(value.to_object(thread).await?);
            }
            Constant::Long(value) => {
                arguments.push(value.to_object(thread).await?);
            }
            Constant::Double(value) => {
                arguments.push(value.to_object(thread).await?);
            }
            Constant::String(value) => {
                let string = constant_pool.try_get_utf8(*value)?;
                let value = thread
                    .vm()?
                    .string_pool()
                    .intern_java_str(thread, string)
                    .await?;
                arguments.push(value);
            }
            Constant::Class(class_index) => {
                let class_name = constant_pool.try_get_utf8(*class_index)?;
                // Per JVM spec §5.4.3.1, class resolution should NOT trigger initialization
                let class = resolve_bootstrap_class(thread, caller, class_name).await?;
                let class_object = class.to_object(thread).await?;
                arguments.push(class_object);
            }
            Constant::Dynamic { .. } => {
                let value = Box::pin(resolve_dynamic_constant(
                    thread, caller, *argument, resolving,
                ))
                .await?;
                // Apply the JDK bootstrap argument boxing rule, including its
                // cached small Integers. Other wrapper and reference identities survive.
                let argument = thread
                    .try_invoke(
                        "java/lang/invoke/BootstrapMethodInvoker",
                        "maybeReBox(Ljava/lang/Object;)Ljava/lang/Object;",
                        &[value],
                    )
                    .await?;
                arguments.push(argument);
            }
            Constant::MethodType(descriptor_index) => {
                let descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
                let s = descriptor.to_str_lossy();
                let method_type = get_method_type(thread, caller, &s).await?;
                arguments.push(method_type);
            }
            Constant::MethodHandle { .. } => {
                arguments.push(resolve_method_handle(thread, caller, *argument).await?);
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

pub(super) async fn resolve_method_handle(
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
    index: u16,
) -> Result<Value> {
    cached_constant(
        thread,
        caller,
        index,
        link_method_handle(thread, caller, index),
    )
    .await
}

pub(super) async fn resolve_dynamic_constant(
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
    index: u16,
    resolving: &[u16],
) -> Result<Value> {
    cached_constant(
        thread,
        caller,
        index,
        resolve_dynamic_constant_uncached(thread, caller, index, resolving),
    )
    .await
}

/// Publish a dynamic constant or method handle (including linkage failures) once per CP entry.
async fn cached_constant(
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
    index: u16,
    resolve: impl Future<Output = Result<Value>>,
) -> Result<Value> {
    let entries = thread.vm()?.resolved_constants.for_class(caller);
    let entry = entries
        .get(index)
        .ok_or(crate::Error::InvalidConstantIndex(index))?;
    if let Some(value) = entry.get() {
        return value.clone().map_err(crate::Error::Throwable);
    }
    let resolved = match resolve.await {
        Ok(value) => Ok(value),
        Err(error) => {
            let cause = super::convert_error_to_throwable(thread, error).await?;
            let linkage_error = thread.class("java/lang/LinkageError").await?;
            let class = cause.as_object_ref()?.class().clone();
            if !linkage_error
                .is_assignable_from(thread.as_ref(), &class)
                .await?
            {
                return Err(crate::Error::Throwable(cause));
            }
            Err(cause)
        }
    };
    entry
        .get_or_init(|| resolved)
        .clone()
        .map_err(crate::Error::Throwable)
}

async fn resolve_dynamic_constant_uncached(
    thread: &Arc<Thread>,
    caller: &Arc<Class>,
    index: u16,
    resolving: &[u16],
) -> Result<Value> {
    if resolving.contains(&index) {
        return Err(
            crate::JavaError::StackOverflowError("Circular dynamic constant".to_owned()).into(),
        );
    }
    let pool = caller.constant_pool();
    let Constant::Dynamic {
        bootstrap_method_attr_index,
        name_and_type_index,
    } = pool.try_get(index)?
    else {
        return Err(crate::Error::InvalidConstantIndex(index));
    };
    let bootstrap = get_bootstrap_method_attribute(caller, *bootstrap_method_attr_index)?;
    let handle = resolve_method_handle(thread, caller, bootstrap.bootstrap_method_ref).await?;
    let (name, descriptor) = pool.try_get_name_and_type(*name_and_type_index)?;
    let name = thread
        .vm()?
        .string_pool()
        .intern_java_str(thread, pool.try_get_utf8(*name)?)
        .await?;
    let ty = FieldType::parse_java_str(pool.try_get_utf8(*descriptor)?)?;
    let mirror = get_field_type_class(thread, caller, Some(ty)).await?;
    let mut path = resolving.to_vec();
    path.push(index);
    let values =
        resolve_static_bootstrap_arguments(thread, caller, pool, &bootstrap, &path).await?;
    let array = Reference::try_from((thread.class("[Ljava/lang/Object;").await?, values))?;
    let arguments = Value::new_object(thread.vm()?.garbage_collector(), array);
    thread.try_invoke("java/lang/invoke/MethodHandleNatives",
        "linkDynamicConstantImpl(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
        &[caller.to_object(thread).await?, handle, name, mirror, arguments]).await
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
    let key = CallSiteKey::new(
        frame.class().clone(),
        frame.method().clone(),
        frame.program_counter(),
    );
    thread
        .vm()?
        .call_site_cache()
        .resolve_with_cache(&thread, key, || {
            resolve_call_site_uncached(frame, method_index)
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
    let bootstrap_method_attr_descriptor = bootstrap_method_attr_descriptor.to_str_lossy();

    // Get the bootstrap method
    let bootstrap_method_attribute =
        get_bootstrap_method_attribute(current_class, bootstrap_method_attr_index)?;

    // 2.2 The bootstrap method entry contains:
    //     - bootstrap_method_ref (CONSTANT_MethodHandle_info index)
    //     - num_bootstrap_arguments (count of static arguments)
    //     - bootstrap_arguments[] (array of constant pool indices)
    let handle = resolve_method_handle(
        &thread,
        current_class,
        bootstrap_method_attribute.bootstrap_method_ref,
    )
    .await?;

    // 3.2 Prepare method name and method type for the invokedynamic call site
    let method_name = thread
        .vm()?
        .string_pool()
        .intern_java_str(&thread, bootstrap_method_attr_name)
        .await?;
    let method_type =
        get_method_type(&thread, current_class, &bootstrap_method_attr_descriptor).await?;

    // 3.4 Resolve static arguments
    let static_arguments = resolve_static_bootstrap_arguments(
        &thread,
        current_class,
        constant_pool,
        &bootstrap_method_attribute,
        &[],
    )
    .await?;

    let lookup_class = thread
        .class("java/lang/invoke/MethodHandles$Lookup")
        .await?;
    let caller_lookup = thread
        .try_invoke(
            lookup_class.name(),
            "in(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;",
            &[
                lookup_class.static_value("IMPL_LOOKUP")?,
                current_class.to_object(&thread).await?,
            ],
        )
        .await?;
    let mut arguments = vec![caller_lookup, method_name, method_type.clone()];
    arguments.extend(static_arguments);
    let array = Reference::try_from((thread.class("[Ljava/lang/Object;").await?, arguments))?;
    let arguments = Value::new_object(thread.vm()?.garbage_collector(), array);

    match invoke_call_site_bootstrap(&thread, handle, arguments, method_type).await {
        Ok(site) => Ok(site),
        Err(error) => {
            let cause = super::convert_error_to_throwable(&thread, error).await?;
            let class = cause.as_object_ref()?.class().clone();
            if thread
                .class("java/lang/Error")
                .await?
                .is_assignable_from(thread.as_ref(), &class)
                .await?
            {
                return Err(crate::Error::Throwable(cause));
            }
            let wrapped = thread
                .object(
                    "java/lang/BootstrapMethodError",
                    "Ljava/lang/String;Ljava/lang/Throwable;",
                    &[
                        "CallSite bootstrap method initialization exception"
                            .to_object(&thread)
                            .await?,
                        cause,
                    ],
                )
                .await?;
            Err(crate::Error::Throwable(wrapped))
        }
    }
}

/// Invoke the bootstrap and check the target before publishing the call site.
async fn invoke_call_site_bootstrap(
    thread: &Thread,
    handle: Value,
    arguments: Value,
    expected_type: Value,
) -> Result<Value> {
    let site = thread
        .try_invoke(
            "java/lang/invoke/MethodHandle",
            "invokeWithArguments([Ljava/lang/Object;)Ljava/lang/Object;",
            &[handle, arguments],
        )
        .await?;
    let call_site = thread.class("java/lang/invoke/CallSite").await?;
    let is_site = thread
        .try_invoke(
            "java/lang/Class",
            "isInstance(Ljava/lang/Object;)Z",
            &[call_site.to_object(thread).await?, site.clone()],
        )
        .await?;
    if !is_site.as_bool()? {
        let cause = thread
            .object(
                "java/lang/ClassCastException",
                "Ljava/lang/String;",
                &["CallSite bootstrap method failed to produce an instance of CallSite"],
            )
            .await?;
        return Err(crate::Error::Throwable(cause));
    }
    let target = thread
        .try_invoke(
            "java/lang/invoke/CallSite",
            "getTarget()Ljava/lang/invoke/MethodHandle;",
            std::slice::from_ref(&site),
        )
        .await?;
    let actual_type = thread
        .try_invoke(
            "java/lang/invoke/MethodHandle",
            "type()Ljava/lang/invoke/MethodType;",
            &[target],
        )
        .await?;
    let matches = thread
        .try_invoke(
            "java/lang/invoke/MethodType",
            "equals(Ljava/lang/Object;)Z",
            &[actual_type, expected_type],
        )
        .await?;
    if !matches.as_bool()? {
        return Err(crate::JavaError::WrongMethodTypeException(
            "CallSite target does not match the invokedynamic method type".into(),
        )
        .into());
    }
    Ok(site)
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
    let call_site = resolve_call_site(frame, method_index).await?;
    let target_method_handle = thread
        .try_invoke(
            "java/lang/invoke/CallSite",
            "getTarget()Ljava/lang/invoke/MethodHandle;",
            &[call_site],
        )
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
        // Method has a return value; push it onto the operand stack
        stack.push(result)?;
    }

    Ok(ExecutionResult::Continue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::{ClassFile, ReferenceKind};

    #[tokio::test]
    async fn cache_linkage_errors_from_bootstrap_handles_and_arguments() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let mut pool = ConstantPool::default();
        let this_class = pool.add_class("MissingBootstrapCaller")?;
        let super_class = pool.add_class("java/lang/Object")?;
        let method = pool.add_method_ref(super_class, "absentBootstrap", "()Ljava/lang/Object;")?;
        let missing_handle = pool.add_method_handle(ReferenceKind::InvokeStatic, method)?;
        let bootstraps = pool.add_class("java/lang/invoke/ConstantBootstraps")?;
        let cast = pool.add_method_ref(bootstraps, "explicitCast",
            "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;")?;
        let cast = pool.add_method_handle(ReferenceKind::InvokeStatic, cast)?;
        let name_index = pool.add_utf8("BootstrapMethods")?;
        let bad_bootstrap = pool.add_dynamic(0, "badBootstrap", "I")?;
        let bad_argument = pool.add_dynamic(1, "badArgument", "I")?;
        let caller = Class::from(
            None,
            ClassFile {
                this_class,
                super_class,
                constant_pool: pool,
                attributes: vec![Attribute::BootstrapMethods {
                    name_index,
                    methods: vec![
                        BootstrapMethod {
                            bootstrap_method_ref: missing_handle,
                            arguments: vec![],
                        },
                        BootstrapMethod {
                            bootstrap_method_ref: cast,
                            arguments: vec![missing_handle],
                        },
                    ],
                }],
                ..Default::default()
            },
        )?;
        thread.register_class(caller.clone()).await?;
        let entries = vm.resolved_constants.for_class(&caller);
        for index in [bad_bootstrap, bad_argument] {
            let error = resolve_dynamic_constant(&thread, &caller, index, &[])
                .await
                .unwrap_err();
            let crate::Error::Throwable(cause) = error else {
                panic!("unexpected error: {error}")
            };
            assert_eq!(
                cause.as_object_ref()?.class().name(),
                "java/lang/NoSuchMethodError"
            );
            assert!(matches!(
                entries.get(index).and_then(|entry| entry.get()),
                Some(Err(_))
            ));
        }
        Ok(())
    }
}

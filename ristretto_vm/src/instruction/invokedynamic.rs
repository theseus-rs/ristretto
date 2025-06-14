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
//! 1.3 Resolve the `CONSTANT_Dynamic_info` or `CONSTANT_InvokeDynamic_info` entry at that index
//!
//! 1.4 Validate that the entry contains: (e.g. `ClassFile.verify()`, also occurs at runtime)
//!   - bootstrap_method_attr_index (points to BootstrapMethods attribute)
//!   - name_and_type_index (method name and descriptor)
//!
//! ## Step 2: Bootstrap Method Resolution
//!
//! 2.1 Use bootstrap_method_attr_index to locate the specific bootstrap method entry
//!
//! 2.2 The bootstrap method entry contains:
//!   - bootstrap_method_ref (CONSTANT_MethodHandle_info index)
//!   - num_bootstrap_arguments (count of static arguments)
//!   - bootstrap_arguments[] (array of constant pool indices)
//!
//! 2.3 Extract method name from name_and_type_index:
//!   - Resolve CONSTANT_NameAndType_info entry
//!   - Extract UTF8 string for method name
//!
//! 2.4 Validate bootstrap method signature matches required pattern:
//!     (MethodHandles.Lookup, String, MethodType, ...additionalArgs) -> CallSite
//!
//! ## Step 3: Argument Preparation
//!
//! 3.1 Create MethodHandles.Lookup object:
//!   - Set lookup class to the class containing the invokedynamic instruction
//!   - Set access modes based on the calling class's access rights
//!   - Lookup modes include: MODULE, PACKAGE, PROTECTED, PUBLIC
//!
//! 3.2 Create MethodType from method descriptor:
//!   - Parse method descriptor string (e.g., "(Ljava/lang/String;I)V")
//!   - Resolve all parameter and return types
//!   - Construct MethodType object with resolved types
//!
//! 3.3 Resolve the bootstrap_method_ref to get the actual MethodHandle
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
//! 4.2 Invoke bootstrap method using MethodHandle.invoke():
//!   ```text
//!   try {
//!       CallSite result = (CallSite) bootstrapMethodHandle.invoke(args);
//!   } catch (Throwable t) {
//!       throw new BootstrapMethodError("Bootstrap method failed", t);
//!   }
//!   ```
//!
//! 4.3 Validate returned CallSite:
//!   - Must not be null
//!   - CallSite.type() must exactly match the expected MethodType
//!   - If validation fails, throw BootstrapMethodError
//!
//! ## Step 5: Call Site Linkage and Caching
//!
//! 5.1 Extract target MethodHandle from CallSite:
//!     MethodHandle target = callSite.getTarget();
//!
//! 5.2 Validate target MethodHandle:
//!   - Must not be null
//!   - target.type() must exactly match expected MethodType
//!   - If validation fails, throw BootstrapMethodError
//!
//! 5.3 Create call site cache entry:
//!   - Store CallSite object indexed by the invokedynamic instruction location
//!   - Store target MethodHandle for fast access
//!   - Mark call site as resolved
//!
//! 5.4 Set up call site invalidation handling:
//!   - If CallSite is MutableCallSite or VolatileCallSite:
//!     - Register for target change notifications
//!     - Set up invalidation callbacks for JIT compiler
//!
//! ## Step 6: Method Invocation Setup
//!
//! 6.1 Prepare for actual method invocation:
//!   - Stack frame contains the runtime arguments for the dynamic method
//!   - Target MethodHandle is now available for invocation
//!
//! 6.2 Configure JIT compilation hints:
//!   - Mark call site for potential inlining
//!   - If ConstantCallSite, mark target as stable for aggressive optimization
//!   - If mutable call site, set up guard conditions for speculative inlining
//!
//! 6.3 Execute the target method:
//!   - Use MethodHandle.invoke() or invokeExact()
//!   - Pass runtime arguments from the current stack frame
//!   - Handle return value according to method descriptor
//!
//! # References
//!
//! - [JVM Specification §6.5](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic)
//! - [JVM Specification §4.7.23](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.23)

use crate::JavaError::BootstrapMethodError;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{JavaObject, Result, VM};
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{Constant, ConstantPool, FieldType, MethodAccessFlags, ReferenceKind};
use ristretto_classloader::{Class, ConcurrentVec, Method, Reference, Value};
use std::sync::Arc;
use tracing::debug;

/// **Step 1.3** Resolve the `CONSTANT_Dynamic_info` or `CONSTANT_InvokeDynamic_info` entry at that
/// index
///
/// Extracts bootstrap method information from the constant pool for dynamic invocation.
///
/// This function retrieves the bootstrap method attribute index and name/type index from the
/// constant pool entry referenced by `method_index`. It handles both `Constant::Dynamic` and
/// `Constant::InvokeDynamic` constant types.
fn get_bootstrap_method_attribute_name_and_type(
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<(u16, u16)> {
    let (bootstrap_method_attr_index, name_and_type_index) =
        match constant_pool.try_get(method_index)? {
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Field descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2
                (*bootstrap_method_attr_index, *name_and_type_index)
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Method descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.3
                (*bootstrap_method_attr_index, *name_and_type_index)
            } // Method
            _ => {
                return Err(BootstrapMethodError(format!(
                    "Invalid constant pool index for invokedynamic: {method_index}"
                ))
                .into());
            }
        };
    Ok((bootstrap_method_attr_index, name_and_type_index))
}

/// **Step 2.1** Use bootstrap_method_attr_index to locate the specific bootstrap method entry
///
/// Get the bootstrap method definition for the specified class.
///
/// This function retrieves a specific `BootstrapMethod` from the class's bootstrap methods
/// attribute using the provided index. Bootstrap methods are used in the Java Virtual Machine
/// to support dynamic invocation through the `invokedynamic` instruction.
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
///   - bootstrap_method_ref (CONSTANT_MethodHandle_info index)
///   - num_bootstrap_arguments (count of static arguments)
///   - bootstrap_arguments[] (array of constant pool indices)
///
/// Resolves the bootstrap method reference and retrieves the method handle, class, and method name
/// and method descriptor for the invokedynamic instruction.
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
async fn resolve_method_handles_lookup(thread: &Thread) -> Result<Value> {
    let method_handles_class = thread.class("java.lang.invoke.MethodHandles").await?;
    let lookup_method = method_handles_class
        .try_get_method("lookup", "()Ljava/lang/invoke/MethodHandles$Lookup;")?;
    let lookup = thread
        .try_execute(&method_handles_class, &lookup_method, Vec::<Value>::new())
        .await?;
    Ok(lookup)
}

/// **For Step 3.2** Resolves a Java class object corresponding to a field type.
///
/// This function takes an optional `FieldType` and returns the Java class object (as a `Value`)
/// that represents that type in the JVM. For primitive types and arrays, this returns the
/// corresponding class objects (like `java.lang.Integer.TYPE` for `int`). For reference types, it
/// loads the class for the specified type.
async fn get_field_type_class(vm: &VM, field_type: Option<FieldType>) -> Result<Value> {
    let class_name = if let Some(field_type) = field_type {
        field_type.class_name()
    } else {
        "void".to_string()
    };
    let class = vm.class(class_name).await?;
    class.to_object(vm).await
}

/// **Step 3.2** Create MethodType from method descriptor:
///   - Parse method descriptor string (e.g., "(Ljava/lang/String;I)V")
///   - Resolve all parameter and return types
///   - Construct MethodType object with resolved types
///
/// Constructs a `java.lang.invoke.MethodType` object from a method descriptor string.
///
/// This function parses the provided method descriptor into its constituent parts (argument types
/// and return type), creates `Class` objects for each type, and then invokes the appropriate
/// `MethodType.methodType()` factory method to create a `MethodType` instance.
async fn get_method_type(vm: &VM, thread: &Thread, method_descriptor: &str) -> Result<Value> {
    let (argument_types, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
    let return_class = get_field_type_class(vm, return_type).await?;

    let method_type_class = thread.class("java.lang.invoke.MethodType").await?;
    if argument_types.is_empty() {
        let method = method_type_class.try_get_method(
            "methodType",
            "(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
        )?;
        return thread
            .try_execute(&method_type_class, &method, vec![return_class])
            .await;
    }

    let first_argument = get_field_type_class(vm, argument_types.first().cloned()).await?;
    let argument_classes = ConcurrentVec::from(Vec::with_capacity(argument_types.len() - 1));
    for argument_type in argument_types.iter().skip(1) {
        let argument_class = get_field_type_class(vm, Some(argument_type.clone())).await?;
        let argument_reference: Reference = argument_class.try_into()?;
        argument_classes.push(Some(argument_reference))?;
    }
    let class_array = vm.class("[Ljava/lang/Class;").await?;
    let arguments = Value::from(Reference::Array(class_array, argument_classes));

    let method = method_type_class.try_get_method(
        "methodType",
        "(Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
    )?;
    let arguments = vec![return_class, first_argument, arguments];
    thread
        .try_execute(&method_type_class, &method, arguments)
        .await
}

/// **Step 3.3** Resolve the bootstrap_method_ref to get the actual MethodHandle
///
/// Retrieves a `MethodHandle` object for the current execution context.
///
/// This function obtains a lookup object that has the access privileges of the caller's class. The
/// lookup object is used for finding and binding methods during dynamic method invocation.
async fn resolve_method_handle(vm: &VM, thread: &Thread, frame: &Frame) -> Result<Value> {
    let lookup = resolve_method_handles_lookup(thread).await?;

    // Get the caller MethodHandle
    let class = frame.class();
    let class_object = class.to_object(vm).await?;
    let method = frame.method();
    let method_name = method.name();
    let method_name_object = method_name.to_object(vm).await?;
    let method_descriptor = method.descriptor();
    let method_type = get_method_type(vm, thread, method_descriptor).await?;
    let mut arguments = vec![method_name_object, method_type];

    let lookup_class = thread
        .class("java.lang.invoke.MethodHandles$Lookup")
        .await?;
    let _find_method = if method_name == "<init>" {
        lookup_class.try_get_method(
            "findConstructor",
            "(Ljava/lang/Class;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    } else if method.access_flags().contains(MethodAccessFlags::STATIC) {
        arguments.insert(0, class_object);
        lookup_class.try_get_method(
            "findStatic",
            "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    } else {
        arguments.insert(0, class_object);
        lookup_class.try_get_method(
            "findVirtual",
            "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
        )?
    };

    arguments.insert(0, lookup);
    // TODO: fix error
    // Exception java/lang/InternalError: Method not found: java/lang/invoke/DirectMethodHandle$Holder.invokeStatic(DD)V
    //     at java/lang/Error.<init>(Error.java:67)
    //     at java/lang/VirtualMachineError.<init>(VirtualMachineError.java:54)
    //     at java/lang/InternalError.<init>(InternalError.java:51)
    //     at java/lang/invoke/MemberName$Factory.resolve(MemberName.java:962)
    //     at java/lang/invoke/MemberName$Factory.resolveOrNull(MemberName.java:1006)
    //     at java/lang/invoke/InvokerBytecodeGenerator.resolveFrom(InvokerBytecodeGenerator.java:647)
    //     at java/lang/invoke/InvokerBytecodeGenerator.lookupPregenerated(InvokerBytecodeGenerator.java:699)
    //     at java/lang/invoke/InvokerBytecodeGenerator.generateCustomizedCode(InvokerBytecodeGenerator.java:708)
    //     at java/lang/invoke/LambdaForm.compileToBytecode(LambdaForm.java:849)
    //     at java/lang/invoke/DirectMethodHandle.makePreparedLambdaForm(DirectMethodHandle.java:303)
    //     at java/lang/invoke/DirectMethodHandle.preparedLambdaForm(DirectMethodHandle.java:231)
    //     at java/lang/invoke/DirectMethodHandle.preparedLambdaForm(DirectMethodHandle.java:216)
    //     at java/lang/invoke/DirectMethodHandle.preparedLambdaForm(DirectMethodHandle.java:225)
    //     at java/lang/invoke/DirectMethodHandle.make(DirectMethodHandle.java:106)
    //     at java/lang/invoke/MethodHandles$Lookup.getDirectMethodCommon(MethodHandles.java:4109)
    //     at java/lang/invoke/MethodHandles$Lookup.getDirectMethod(MethodHandles.java:4053)
    //     at java/lang/invoke/MethodHandles$Lookup.findStatic(MethodHandles.java:2676)
    //     at HelloWorld.main(HelloWorld.java:4)
    // let method_handle = thread
    //     .try_execute(&lookup_class, &find_method, arguments)
    //     .await?;
    let method_handle = Value::Object(None);
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
async fn resolve_static_bootstrap_arguments(
    vm: &Arc<VM>,
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
                let value = string.to_object(vm).await?;
                arguments.push(value);
            }
            _ => {
                return Err(BootstrapMethodError(format!(
                    "Invalid bootstrap argument type: {argument}"
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
    let _bootstrap_method_attr_descriptor =
        constant_pool.try_get_utf8(*bootstrap_method_attr_descriptor_index)?;

    // Get the bootstrap method
    let bootstrap_method_attribute =
        get_bootstrap_method_attribute(frame, bootstrap_method_attr_index)?;

    // 2.2 The bootstrap method entry contains:
    //     - bootstrap_method_ref (CONSTANT_MethodHandle_info index)
    //     - num_bootstrap_arguments (count of static arguments)
    //     - bootstrap_arguments[] (array of constant pool indices)
    let (reference_kind, bootstrap_class, bootstrap_method_descriptor, bootstrap_method) =
        resolve_bootstrap_method(&thread, constant_pool, &bootstrap_method_attribute).await?;

    debug!(
        "invokedynamic: bootstrap method({reference_kind}): {}.{}{}",
        bootstrap_class.name(),
        bootstrap_method.name(),
        bootstrap_method.descriptor()
    );

    // Invoke the bootstrap method
    let vm = thread.vm()?;
    let method_handle = resolve_method_handle(&vm, &thread, frame).await?;
    let static_arguments =
        resolve_static_bootstrap_arguments(&vm, constant_pool, &bootstrap_method_attribute).await?;
    let method_name = bootstrap_method_attr_name.to_object(&vm).await?;
    let method_type = get_method_type(&vm, &thread, bootstrap_method_descriptor).await?;

    // 4.1 Construct argument array for bootstrap method:
    let mut arguments = vec![method_handle, method_name, method_type];
    arguments.extend(static_arguments);

    // 4.2 Invoke bootstrap method using MethodHandle.invoke():
    // let call_site = ...

    // 4.3 Validate returned CallSite:
    // validate_call_site(call_site);

    // Ok(call_site)
    todo!("invokedynamic get_call_site")
}

/// Executes the `invokedynamic` JVM instruction for dynamic method invocation.
///
/// The `invokedynamic` instruction is how of Java supports dynamic languages and lambda
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
/// # References
///
/// - [JVM Specification §6.5](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic)
#[inline]
pub(crate) async fn invokedynamic(
    frame: &Frame,
    _stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    // 1.1 JVM encounters invokedynamic instruction during bytecode execution
    let _call_site = resolve_call_site(frame, method_index).await?;
    todo!("invokedynamic")
}

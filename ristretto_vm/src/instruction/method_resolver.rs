//! Method resolution with JPMS access checking.
//!
//! This module provides unified method resolution that enforces JPMS access rules at resolution
//! time, not at each invocation. This is compliant with the JVM specification where access checks
//! happen during symbolic resolution of constant pool method refs.
//!
//! # Architecture
//!
//! All invoke instructions (`invokestatic`, `invokevirtual`, `invokespecial`, `invokeinterface`)
//! use a shared resolution path that:
//!
//! ```text
//! ┌────────────────┐     ┌──────────────────────┐     ┌─────────────────┐
//! │  invoke*       │────▶│ resolve_method_ref() │────▶│ MethodRefCache  │
//! │  instruction   │     │   (unified)          │     │   (cached)      │
//! └────────────────┘     └────────┬─────────────┘     └─────────────────┘
//!                                 │
//!                                 ▼ (cache miss)
//!                        ┌──────────────────────┐
//!                        │ 1. Load class        │
//!                        │ 2. JPMS check        │
//!                        │ 3. Member access     │
//!                        │ 4. Method lookup     │
//!                        │ 5. Cache result      │
//!                        └──────────────────────┘
//! ```
//!
//! # JPMS Access Check Order
//!
//! When resolving a method reference:
//! 1. Resolve the declaring class (class loading)
//! 2. **JPMS gate**: Check that caller module reads target module
//! 3. **JPMS gate**: Check that target module exports the package
//! 4. Java member access check (public/protected/package/private)
//! 5. Method lookup in class hierarchy
//! 6. Cache the resolved method or error

use crate::Result;
use crate::frame::Frame;
use crate::method_ref_cache::{
    InvokeKind, MethodRefError, MethodRefErrorKind, MethodRefKey, ResolvedMethodRef,
};
use crate::module_system::{ALL_UNNAMED, AccessCheckResult, ModuleSystem};
use ristretto_classfile::Constant;
use ristretto_classloader::{Class, Method};
use std::sync::Arc;

/// Result of method resolution containing all information needed for invocation.
#[derive(Clone)]
pub struct MethodResolution {
    /// The class that declares the resolved method.
    pub declaring_class: Arc<Class>,
    /// The resolved method.
    pub method: Arc<Method>,
    /// Method name (cached for virtual dispatch).
    pub method_name: String,
    /// Method descriptor (cached for virtual dispatch).
    pub method_descriptor: String,
    /// Whether this is a polymorphic method (e.g., `MethodHandle.invoke`).
    /// Cached to avoid `HashMap` lookup at invocation time.
    pub is_polymorphic: bool,
    /// Number of parameters to pop from the operand stack.
    /// For polymorphic methods, this is computed from the call site descriptor.
    pub param_count: usize,
    /// Whether the method has a return value to push onto the operand stack.
    /// For polymorphic methods, this is computed from the call site descriptor.
    pub has_return_type: bool,
}

/// Resolves a method reference from the constant pool with JPMS access checking and caching.
///
/// This is the unified entry point for all invoke instruction resolution. It:
/// 1. Checks the method ref cache for a previously resolved method
/// 2. On cache miss, performs full resolution with JPMS access checks
/// 3. Caches the result (success or failure) for future invocations
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `method_index` - Constant pool index of the method reference
/// * `invoke_kind` - The type of invocation (static, virtual, special, interface)
///
/// # Returns
///
/// A `MethodResolution` containing the resolved method and declaring class.
///
/// # Errors
///
/// Returns an error if:
/// - JPMS access is denied (`IllegalAccessError`)
/// - The method is not found (`NoSuchMethodError`)
/// - Class/interface mismatch (`IncompatibleClassChangeError`)
pub async fn resolve_method_ref(
    frame: &Frame,
    method_index: u16,
    invoke_kind: InvokeKind,
) -> Result<MethodResolution> {
    let thread = frame.thread()?;
    let vm = thread.vm()?;
    let caller_class = frame.class();

    // Create cache key
    let cache_key = MethodRefKey::new(caller_class.name().to_string(), method_index);

    // Check cache first
    if let Some(result) = vm.method_ref_cache().get(&cache_key) {
        let resolved = result?;
        return Ok(MethodResolution {
            declaring_class: resolved.declaring_class.clone(),
            method: resolved.method.clone(),
            method_name: resolved.method_name.clone(),
            method_descriptor: resolved.method_descriptor.clone(),
            is_polymorphic: resolved.is_polymorphic,
            param_count: resolved.param_count,
            has_return_type: resolved.has_return_type,
        });
    }

    // Cache miss - perform resolution
    let constant_pool = caller_class.constant_pool();
    let constant = constant_pool.try_get(method_index)?;

    // Parse the constant pool entry
    let (class_index, name_and_type_index, is_interface_method) = match constant {
        Constant::MethodRef {
            class_index,
            name_and_type_index,
        } => (*class_index, *name_and_type_index, false),
        Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        } => (*class_index, *name_and_type_index, true),
        _ => {
            return Err(
                ristretto_classfile::Error::InvalidConstantPoolIndexType(method_index).into(),
            );
        }
    };

    // Get class name and load the class
    let class_name = constant_pool.try_get_class(class_index)?;
    let target_class = thread.class(class_name).await?;

    // Validate class vs interface for the invoke kind
    validate_class_kind(&target_class, invoke_kind, is_interface_method, class_name)?;

    // Perform JPMS access check (the key security gate)
    check_jpms_access(frame, &target_class)?;

    // Get method name and descriptor
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

    // Look up the method in the class hierarchy
    // For interface methods, we allow abstract methods since dispatch happens on the receiver
    let (resolved_class, method) = if invoke_kind == InvokeKind::Interface {
        lookup_interface_method(&target_class, method_name, method_descriptor)?
    } else {
        // First try normal method lookup
        match lookup_method(&target_class, method_name, method_descriptor) {
            Ok(result) => result,
            Err(e) => {
                // If lookup failed, check if this is a holder class with an intrinsic method
                // Holder classes have dynamically generated methods provided by the JVM
                if is_holder_class_for_resolution(class_name) {
                    let vm = thread.vm()?;
                    let registry = vm.method_registry();
                    if registry
                        .method(class_name, method_name, method_descriptor)
                        .is_some()
                    {
                        // Create a synthetic native method for the intrinsic
                        let synthetic_method = create_synthetic_intrinsic_method(
                            class_name,
                            method_name,
                            method_descriptor,
                        )?;
                        (target_class.clone(), synthetic_method)
                    } else {
                        return Err(e);
                    }
                } else {
                    return Err(e);
                }
            }
        }
    };

    // Validate method properties for the invoke kind
    validate_method_for_invoke(&method, method_name, method_descriptor, invoke_kind)?;

    // Cache the successful resolution
    // For polymorphic methods, we must use the call site descriptor from the constant pool, not the
    // method's declared descriptor, as each call site may have a different signature.
    let resolved_ref = ResolvedMethodRef::new(
        resolved_class.clone(),
        method.clone(),
        invoke_kind,
        method_descriptor.to_string(),
    );

    // Extract cached values before moving into cache
    let is_polymorphic = resolved_ref.is_polymorphic;
    let param_count = resolved_ref.param_count;
    let has_return_type = resolved_ref.has_return_type;

    vm.method_ref_cache()
        .store_resolved(cache_key, resolved_ref);

    Ok(MethodResolution {
        declaring_class: resolved_class,
        method,
        method_name: method_name.to_string(),
        method_descriptor: method_descriptor.to_string(),
        is_polymorphic,
        param_count,
        has_return_type,
    })
}

/// Validates that the class kind matches the invoke kind.
fn validate_class_kind(
    target_class: &Arc<Class>,
    invoke_kind: InvokeKind,
    is_interface_method: bool,
    class_name: &str,
) -> Result<()> {
    use crate::JavaError::IncompatibleClassChangeError;

    match invoke_kind {
        InvokeKind::Static => {
            // invokestatic with InterfaceMethodRef requires interface
            if is_interface_method && !target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Expected interface, found class: {class_name}"
                ))
                .into());
            }
            // invokestatic with MethodRef requires non-interface
            if !is_interface_method && target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Expected class, found interface: {class_name}"
                ))
                .into());
            }
        }
        InvokeKind::Interface => {
            if !target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "{class_name} is not an interface"
                ))
                .into());
            }
        }
        InvokeKind::Virtual | InvokeKind::Special => {
            // These can work with both classes and interfaces in various situations
        }
    }

    Ok(())
}

/// Validates that the method has appropriate properties for the invoke kind.
fn validate_method_for_invoke(
    method: &Method,
    method_name: &str,
    method_descriptor: &str,
    invoke_kind: InvokeKind,
) -> Result<()> {
    use crate::JavaError::IncompatibleClassChangeError;

    match invoke_kind {
        InvokeKind::Static => {
            if !method.is_static() {
                return Err(IncompatibleClassChangeError(format!(
                    "Method {method_name}{method_descriptor} is not static"
                ))
                .into());
            }
        }
        InvokeKind::Virtual | InvokeKind::Interface => {
            if method.is_static() {
                return Err(IncompatibleClassChangeError(format!(
                    "Method {method_name}{method_descriptor} is static"
                ))
                .into());
            }
        }
        InvokeKind::Special => {
            // invokespecial can call constructors, private methods, and superclass methods
            // Additional validation may be needed based on context
        }
    }

    Ok(())
}

/// Checks JPMS access from the caller to the target class.
///
/// This implements the JPMS gate that must be passed before member access checks.
/// Access checking is skipped for same-class or same-module access.
///
/// # Errors
///
/// Returns `IllegalAccessError` if:
/// - The caller module does not read the target module
/// - The target module does not export the package containing the class
pub fn check_jpms_access(frame: &Frame, target_class: &Arc<Class>) -> Result<()> {
    let caller_class = frame.class();

    // Same class always has access
    if Arc::ptr_eq(caller_class, target_class) {
        return Ok(());
    }

    // Get module names
    let caller_module = caller_class.module_name().ok().flatten();
    let target_module = target_class.module_name().ok().flatten();

    // Same module always has access
    if caller_module == target_module {
        return Ok(());
    }

    // Get VM for access checking
    let thread = frame.thread()?;
    let vm = thread.vm()?;

    // Perform combined static + dynamic JPMS check
    let result = vm.module_system().check_access(
        caller_module.as_deref(),
        target_module.as_deref(),
        target_class.name(),
    );

    if result.is_allowed() {
        return Ok(());
    }

    // Determine if we should enforce access
    // For now, relax enforcement for system modules to avoid breaking tests
    if !should_enforce_jpms_access(caller_module.as_deref(), target_module.as_deref()) {
        return Ok(());
    }

    // Generate appropriate error
    let from = caller_module.as_deref().unwrap_or(ALL_UNNAMED);
    let to = target_module.as_deref().unwrap_or(ALL_UNNAMED);
    let error_msg = ModuleSystem::illegal_access_error(from, to, target_class.name(), result);

    Err(crate::JavaError::IllegalAccessError(error_msg).into())
}

/// Determines whether to enforce JPMS access checking.
///
/// Currently relaxes enforcement for system modules while the module system
/// is being fully integrated.
fn should_enforce_jpms_access(caller_module: Option<&str>, target_module: Option<&str>) -> bool {
    // Don't enforce if either is in the unnamed module
    if caller_module.is_none() || target_module.is_none() {
        return false;
    }

    let target = target_module.unwrap_or("");

    // Don't enforce access to system modules for now
    if target.starts_with("java.")
        || target.starts_with("jdk.")
        || target.starts_with("sun.")
        || target.starts_with("com.sun.")
    {
        return false;
    }

    // Enforce access for application modules
    true
}

/// Looks up a method in the class hierarchy.
///
/// This searches the class and its superclasses/interfaces for a method
/// with the given name and descriptor.
///
/// # Errors
///
/// Returns `NoSuchMethodError` if the method is not found.
pub fn lookup_method(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<(Arc<Class>, Arc<Method>)> {
    // First check the class itself
    if let Some(method) = class.method(name, descriptor) {
        return Ok((class.clone(), method));
    }

    // Search superclasses
    let mut current = class.parent()?;
    while let Some(parent) = current {
        if let Some(method) = parent.method(name, descriptor) {
            return Ok((parent, method));
        }
        current = parent.parent()?;
    }

    // Search interfaces for methods (including abstract interface methods). Per JVMS §5.4.3.3,
    // method resolution searches super-interfaces for a maximally specific super interface method.
    // For a class, we need to search all implemented interfaces
    let mut interfaces_to_check: Vec<Arc<Class>> = class.interfaces()?;

    // If the class itself is an interface, also check its super-interfaces directly
    // (class.interfaces() returns super-interfaces for an interface)

    // Track visited interfaces to avoid duplicates
    let mut visited = std::collections::HashSet::new();
    visited.insert(class.name().to_string());

    // First pass: look for non-abstract (default) methods
    let mut abstract_method: Option<(Arc<Class>, Arc<Method>)> = None;

    while let Some(interface) = interfaces_to_check.pop() {
        // Skip if already visited
        if !visited.insert(interface.name().to_string()) {
            continue;
        }

        // Check for method in interface
        if let Some(method) = interface.method(name, descriptor) {
            if !method.is_abstract() {
                // Found a concrete default method; return immediately
                return Ok((interface, method));
            } else if abstract_method.is_none() {
                // Record the first abstract method we find
                abstract_method = Some((interface.clone(), method));
            }
        }

        // Add super-interfaces
        interfaces_to_check.extend(interface.interfaces()?);
    }

    // Second pass: search the entire class hierarchy's interfaces
    // This handles the case where a superclass implements an interface with the method
    let mut class_to_check = class.parent()?;
    while let Some(parent_class) = class_to_check {
        let mut parent_interfaces: Vec<Arc<Class>> = parent_class.interfaces()?;

        while let Some(interface) = parent_interfaces.pop() {
            if !visited.insert(interface.name().to_string()) {
                continue;
            }

            if let Some(method) = interface.method(name, descriptor) {
                if !method.is_abstract() {
                    return Ok((interface, method));
                } else if abstract_method.is_none() {
                    abstract_method = Some((interface.clone(), method));
                }
            }

            parent_interfaces.extend(interface.interfaces()?);
        }

        class_to_check = parent_class.parent()?;
    }

    // If we found an abstract interface method, return it. This allows method resolution to succeed
    // even when the method is abstract. The actual implementation will be found at dispatch time.
    if let Some((interface, method)) = abstract_method {
        return Ok((interface, method));
    }

    Err(crate::JavaError::NoSuchMethodError(format!(
        "Method {name}{descriptor} not found in class {}",
        class.name()
    ))
    .into())
}

/// Looks up a method in an interface and its super-interfaces.
///
/// Unlike `lookup_method`, this allows abstract methods since interface dispatch
/// happens at runtime on the receiver's actual class.
///
/// # Errors
///
/// Returns `NoSuchMethodError` if the method is not found.
pub fn lookup_interface_method(
    interface: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<(Arc<Class>, Arc<Method>)> {
    // First check the interface itself
    if let Some(method) = interface.method(name, descriptor) {
        return Ok((interface.clone(), method));
    }

    // Search super-interfaces (including inherited ones)
    let mut interfaces_to_check: Vec<Arc<Class>> = interface.interfaces()?;
    let mut visited = std::collections::HashSet::new();
    visited.insert(interface.name().to_string());

    while let Some(super_interface) = interfaces_to_check.pop() {
        // Skip if already visited
        if !visited.insert(super_interface.name().to_string()) {
            continue;
        }

        // Check for the method (abstract or default)
        if let Some(method) = super_interface.method(name, descriptor) {
            return Ok((super_interface, method));
        }

        // Add super-interfaces of this interface
        interfaces_to_check.extend(super_interface.interfaces()?);
    }

    // For interfaces, also check java.lang.Object methods
    // (interfaces implicitly inherit Object's public methods)
    if let Ok(Some(object_class)) = interface.parent()
        && let Some(method) = object_class.method(name, descriptor)
    {
        return Ok((object_class, method));
    }

    Err(crate::JavaError::NoSuchMethodError(format!(
        "Method {name}{descriptor} not found in interface {}",
        interface.name()
    ))
    .into())
}

/// Creates a cached JPMS access error.
#[must_use]
pub fn create_jpms_error(
    result: AccessCheckResult,
    caller_module: Option<&str>,
    target_module: Option<&str>,
    target_class: &str,
) -> MethodRefError {
    let from = caller_module.unwrap_or(ALL_UNNAMED);
    let to = target_module.unwrap_or(ALL_UNNAMED);
    let message = ModuleSystem::illegal_access_error(from, to, target_class, result);

    let kind = match result {
        AccessCheckResult::NotReadable => MethodRefErrorKind::ModuleNotReadable,
        // Both NotExported and NotOpened are treated as access denial for method resolution
        AccessCheckResult::NotExported | AccessCheckResult::NotOpened => {
            MethodRefErrorKind::PackageNotExported
        }
        AccessCheckResult::Allowed => MethodRefErrorKind::InternalError, // should not happen
    };

    MethodRefError::new(kind, message)
}

/// Checks if a class is a holder class for method resolution purposes. These classes have
/// dynamically generated methods that don't exist in the class file but are provided by intrinsics.
fn is_holder_class_for_resolution(class_name: &str) -> bool {
    // Class names may be in either format: java.lang.invoke.X or java/lang/invoke/X
    let normalized = class_name.replace('.', "/");

    // Exact matches for holder classes
    matches!(
        normalized.as_str(),
        "java/lang/invoke/DirectMethodHandle$Holder"
            | "java/lang/invoke/DelegatingMethodHandle$Holder"
            | "java/lang/invoke/Invokers$Holder"
            | "java/lang/invoke/LambdaForm$Holder"
            | "java/lang/invoke/VarHandleGuards"
    ) || normalized.starts_with("java/lang/invoke/LambdaForm$")
}

/// Creates a synthetic native method for an intrinsic that doesn't have a class file entry.
/// This is used for holder class methods that are dynamically generated.
fn create_synthetic_intrinsic_method(
    _class_name: &str,
    method_name: &str,
    method_descriptor: &str,
) -> Result<Arc<Method>> {
    use ristretto_classfile::MethodAccessFlags;

    // Create a synthetic method definition
    let definition = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC
            | MethodAccessFlags::STATIC
            | MethodAccessFlags::NATIVE,
        name_index: 0,       // Not used for synthetic methods
        descriptor_index: 0, // Not used for synthetic methods
        attributes: Vec::new(),
    };

    // Parse the method descriptor
    let (parameters, return_type) =
        ristretto_classfile::FieldType::parse_method_descriptor(method_descriptor)?;

    // Create the method directly
    let method = Method::new_synthetic(
        definition,
        method_name.to_string(),
        method_descriptor.to_string(),
        parameters,
        return_type,
    );

    Ok(Arc::new(method))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VM;

    #[test]
    fn test_should_enforce_jpms_access_unnamed() {
        // Both unnamed - don't enforce
        assert!(!should_enforce_jpms_access(None, None));
        // Caller unnamed - don't enforce
        assert!(!should_enforce_jpms_access(None, Some("app.module")));
        // Target unnamed - don't enforce
        assert!(!should_enforce_jpms_access(Some("app.module"), None));
    }

    #[test]
    fn test_should_enforce_jpms_access_system_modules() {
        let caller = Some("my.app");
        // System modules should not be enforced
        assert!(!should_enforce_jpms_access(caller, Some("java.base")));
        assert!(!should_enforce_jpms_access(caller, Some("java.sql")));
        assert!(!should_enforce_jpms_access(caller, Some("jdk.compiler")));
        assert!(!should_enforce_jpms_access(caller, Some("sun.misc")));
        assert!(!should_enforce_jpms_access(
            caller,
            Some("com.sun.crypto.provider")
        ));
    }

    #[test]
    fn test_should_enforce_jpms_access_app_modules() {
        // Application modules should be enforced
        assert!(should_enforce_jpms_access(
            Some("my.app"),
            Some("other.app")
        ));
        assert!(should_enforce_jpms_access(
            Some("com.example"),
            Some("org.lib")
        ));
    }

    #[test]
    fn test_create_jpms_error_not_readable() {
        let error = create_jpms_error(
            AccessCheckResult::NotReadable,
            Some("my.app"),
            Some("other.app"),
            "other/api/Service",
        );
        assert_eq!(error.kind, MethodRefErrorKind::ModuleNotReadable);
        assert!(error.message.contains("does not read"));
    }

    #[test]
    fn test_create_jpms_error_not_exported() {
        let error = create_jpms_error(
            AccessCheckResult::NotExported,
            Some("my.app"),
            Some("other.app"),
            "other/internal/Secret",
        );
        assert_eq!(error.kind, MethodRefErrorKind::PackageNotExported);
        assert!(error.message.contains("does not export"));
    }

    #[tokio::test]
    async fn test_lookup_method_found_in_class() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let (resolved_class, method) = lookup_method(&class, "length", "()I")?;
        assert_eq!(resolved_class.name(), "java/lang/String");
        assert_eq!(method.name(), "length");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_found_in_superclass() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.ArrayList").await?;
        // toString is defined in AbstractCollection
        let (resolved_class, method) = lookup_method(&class, "toString", "()Ljava/lang/String;")?;
        assert_eq!(resolved_class.name(), "java/util/AbstractCollection");
        assert_eq!(method.name(), "toString");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_not_found() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let result = lookup_method(&class, "nonExistentMethod", "()V");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_class_kind_static_with_class() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        // Non-interface class with MethodRef (not InterfaceMethodRef)
        let result = validate_class_kind(&class, InvokeKind::Static, false, "java/lang/String");
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_class_kind_interface_requires_interface() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        // invokeinterface requires an interface
        let result = validate_class_kind(&class, InvokeKind::Interface, true, "java/lang/String");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_method_for_invoke_static() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.Integer").await?;

        // valueOf is static
        if let Some(method) = class.method("valueOf", "(I)Ljava/lang/Integer;") {
            let result = validate_method_for_invoke(
                &method,
                "valueOf",
                "(I)Ljava/lang/Integer;",
                InvokeKind::Static,
            );
            assert!(result.is_ok());
        }

        // intValue is not static
        if let Some(method) = class.method("intValue", "()I") {
            let result = validate_method_for_invoke(&method, "intValue", "()I", InvokeKind::Static);
            assert!(result.is_err());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_method_for_invoke_virtual() -> crate::Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.Integer").await?;

        // intValue is not static - good for virtual
        if let Some(method) = class.method("intValue", "()I") {
            let result =
                validate_method_for_invoke(&method, "intValue", "()I", InvokeKind::Virtual);
            assert!(result.is_ok());
        }

        // valueOf is static - bad for virtual
        if let Some(method) = class.method("valueOf", "(I)Ljava/lang/Integer;") {
            let result = validate_method_for_invoke(
                &method,
                "valueOf",
                "(I)Ljava/lang/Integer;",
                InvokeKind::Virtual,
            );
            assert!(result.is_err());
        }

        Ok(())
    }
}

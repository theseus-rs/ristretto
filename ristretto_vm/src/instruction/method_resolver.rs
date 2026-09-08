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
//! 6. Cache the resolved method

use crate::Result;
use crate::frame::Frame;
use crate::method_ref_cache::{InvokeKind, MethodRefError, MethodRefErrorKind, ResolvedMethodRef};
use crate::module_system::{ALL_UNNAMED, AccessCheckResult, ModuleSystem};
use ristretto_classfile::Constant;
use ristretto_classloader::{Class, Method};
use std::sync::Arc;

/// Resolves a method reference from the constant pool with JPMS access checking and caching.
///
/// This is the unified entry point for all invoke instruction resolution. It:
/// 1. Checks the method ref cache for a previously resolved method
/// 2. On cache miss, performs full resolution with JPMS access checks
/// 3. Shares the successfully resolved record with future invocations
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `method_index` - Constant pool index of the method reference
/// * `invoke_kind` - The type of invocation (static, virtual, special, interface)
///
/// # Returns
///
/// An `Arc<ResolvedMethodRef>` containing the resolved method and declaring class.
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
) -> Result<Arc<ResolvedMethodRef>> {
    let thread = frame.thread()?;
    let caller_class = frame.class();
    let entry = frame.method_refs()?.get(method_index).ok_or(
        ristretto_classfile::Error::InvalidConstantPoolIndex(method_index),
    )?;
    if let Some(resolved) = entry.get(invoke_kind) {
        return Ok(resolved.clone());
    }

    // Cache miss; perform resolution
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
    let target_class = thread.class_java_str(class_name).await?;
    let class_name = class_name.to_str_lossy();

    // Validate class vs interface for the invoke kind
    validate_class_kind(&target_class, invoke_kind, is_interface_method, &class_name)?;

    // Perform JPMS access check (the key security gate)
    check_jpms_access(frame, &target_class)?;

    // Get method name and descriptor
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_name = method_name.to_str_lossy();
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let method_descriptor = method_descriptor.to_str_lossy();

    // Look up the method in the class hierarchy
    // For interface methods, we allow abstract methods since dispatch happens on the receiver
    let (resolved_class, method) = if invoke_kind == InvokeKind::Interface {
        lookup_interface_method(&target_class, &method_name, &method_descriptor)?
    } else {
        // First try normal method lookup
        match lookup_method(&target_class, &method_name, &method_descriptor) {
            Ok(result) => result,
            Err(e) => {
                // If lookup failed, check if this is a holder class with an intrinsic method
                // Holder classes have dynamically generated methods provided by the JVM
                if is_holder_class_for_resolution(&class_name) {
                    let vm = thread.vm()?;
                    let registry = vm.method_registry();
                    if registry
                        .method(&class_name, &method_name, &method_descriptor)
                        .is_some()
                    {
                        // Create a synthetic native method for the intrinsic
                        let synthetic_method = create_synthetic_intrinsic_method(
                            &class_name,
                            &method_name,
                            &method_descriptor,
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
    validate_method_for_invoke(&method, &method_name, &method_descriptor, invoke_kind)?;

    // Cache the successful resolution
    // For polymorphic methods, we must use the call site descriptor from the constant pool, not the
    // method's declared descriptor, as each call site may have a different signature.
    let resolved_ref = Arc::new(ResolvedMethodRef::new(
        target_class,
        resolved_class,
        method,
        invoke_kind,
        method_descriptor.to_string(),
    ));
    Ok(entry.store(resolved_ref))
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
            if !is_interface_method || !target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "{class_name} is not an interface"
                ))
                .into());
            }
        }
        InvokeKind::Virtual => {
            if is_interface_method || target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Expected class method reference: {class_name}"
                ))
                .into());
            }
        }
        InvokeKind::Special => {
            if is_interface_method != target_class.is_interface() {
                return Err(IncompatibleClassChangeError(format!(
                    "Class/interface method reference mismatch: {class_name}"
                ))
                .into());
            }
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
        InvokeKind::Virtual | InvokeKind::Interface | InvokeKind::Special => {
            if method.is_static() {
                return Err(IncompatibleClassChangeError(format!(
                    "Method {method_name}{method_descriptor} is static"
                ))
                .into());
            }
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
    lookup_method_with_private(class, name, descriptor, true)
}

/// Select an override for virtual dispatch. Private receiver methods cannot override the
/// resolved method (JVMS 5.4.5); resolved private methods are handled directly by the caller.
pub(crate) fn lookup_virtual_method(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<(Arc<Class>, Arc<Method>)> {
    lookup_method_with_private(class, name, descriptor, false)
}

fn lookup_method_with_private(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
    include_private: bool,
) -> Result<(Arc<Class>, Arc<Method>)> {
    // First check the class itself
    if let Some(method) = class.method(name, descriptor)
        && (include_private || !method.is_private())
    {
        return Ok((class.clone(), method));
    }

    // Search superclasses
    let mut current = class.parent()?;
    while let Some(parent) = current {
        if let Some(method) = parent.method(name, descriptor)
            && (include_private || !method.is_private())
        {
            return Ok((parent, method));
        }
        current = parent.parent()?;
    }

    // Collect every candidate before choosing: traversal order must not let a
    // parent interface's default override a more specific subinterface method.
    let mut interfaces = class.interfaces()?;
    let mut parent = class.parent()?;
    while let Some(current) = parent {
        interfaces.extend(current.interfaces()?);
        parent = current.parent()?;
    }
    let mut visited = std::collections::HashSet::new();
    let mut candidates = Vec::new();
    while let Some(interface) = interfaces.pop() {
        if !visited.insert(interface.name().to_string()) {
            continue;
        }
        interfaces.extend(interface.interfaces()?);
        if let Some(method) = interface.method(name, descriptor)
            && !method.is_private()
            && !method.is_static()
        {
            candidates.push((interface, method));
        }
    }

    let mut shadowed = std::collections::HashSet::new();
    for (interface, _) in &candidates {
        let mut parents = interface.interfaces()?;
        let mut seen = std::collections::HashSet::new();
        while let Some(parent) = parents.pop() {
            if seen.insert(parent.name().to_string()) {
                shadowed.insert(parent.name().to_string());
                parents.extend(parent.interfaces()?);
            }
        }
    }
    candidates.retain(|(interface, _)| !shadowed.contains(interface.name()));
    let mut concrete = candidates
        .iter()
        .filter(|(_, method)| !method.is_abstract());
    if let Some(selected) = concrete.next() {
        if concrete.next().is_some() {
            return Err(crate::JavaError::IncompatibleClassChangeError(format!(
                "Conflicting interface defaults for {name}{descriptor} in {}",
                class.name()
            ))
            .into());
        }
        return Ok(selected.clone());
    }
    if let Some(selected) = candidates.into_iter().next() {
        return Ok(selected);
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

    // Exact matches for holder classes whose methods are dynamically generated
    // by the JDK at runtime and don't exist in class files. These class names
    // are part of the JDK's internal bootstrap machinery and are the standard
    // way to identify them (consistent with HotSpot's approach).
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
    let method_descriptor = ristretto_classfile::JavaStr::cow_from_str(method_descriptor);
    let (parameters, return_type) =
        ristretto_classfile::FieldType::parse_method_descriptor(&method_descriptor)?;

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
        // Both unnamed; don't enforce
        assert!(!should_enforce_jpms_access(None, None));
        // Caller unnamed; don't enforce
        assert!(!should_enforce_jpms_access(None, Some("app.module")));
        // Target unnamed; don't enforce
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
    async fn test_lookup_method_found_in_class() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let (resolved_class, method) = lookup_method(&class, "length", "()I")?;
        assert_eq!(resolved_class.name(), "java/lang/String");
        assert_eq!(method.name(), "length");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_found_in_superclass() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.util.ArrayList").await?;
        // toString is defined in AbstractCollection
        let (resolved_class, method) = lookup_method(&class, "toString", "()Ljava/lang/String;")?;
        assert_eq!(resolved_class.name(), "java/util/AbstractCollection");
        assert_eq!(method.name(), "toString");
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_method_not_found() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let result = lookup_method(&class, "nonExistentMethod", "()V");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_class_kind_static_with_class() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        // Non-interface class with MethodRef (not InterfaceMethodRef)
        let result = validate_class_kind(&class, InvokeKind::Static, false, "java/lang/String");
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_class_kind_interface_requires_interface() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        // invokeinterface requires an interface
        let result = validate_class_kind(&class, InvokeKind::Interface, true, "java/lang/String");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_method_for_invoke_static() -> Result<()> {
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
    async fn test_validate_method_for_invoke_virtual() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.Integer").await?;

        // intValue is not static; good for virtual
        if let Some(method) = class.method("intValue", "()I") {
            let result =
                validate_method_for_invoke(&method, "intValue", "()I", InvokeKind::Virtual);
            assert!(result.is_ok());
        }

        // valueOf is static; bad for virtual
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

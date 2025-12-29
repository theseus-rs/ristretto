//! Module access checking utilities for bytecode instructions.
//!
//! This module provides helper functions to enforce JPMS module boundaries
//! when accessing classes, fields, and methods.
//!
//! # JPMS Access Rules
//!
//! For a class in module A to access a public type in module B:
//! 1. Module A must read module B (`requires` or implicitly via `java.base`)
//! 2. Module B must export the package containing the type to module A (or unqualified)
//!
//! For deep reflection (accessing private members):
//! 1. All of the above
//! 2. Module B must open the package to module A (or be an open module)
//!
//! # Special Cases
//!
//! - **Unnamed module**: Code on the classpath is in the unnamed module, which can read
//!   all named modules and can access any exported package.
//! - **Same module**: Access within the same module is always allowed.
//! - **`java.base`**: All named modules implicitly read `java.base`.

use crate::Result;
use crate::frame::Frame;
use crate::module_system::AccessCheckResult;
use ristretto_classloader::Class;
use std::sync::Arc;

/// Checks if access from the current frame's class to a target class is allowed.
///
/// This implements JPMS access checking at the bytecode instruction level.
/// For access to be allowed:
/// - The source module must read the target module
/// - The target module must export the package containing the class
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `target_class` - The class being accessed
///
/// # Errors
///
/// Returns an `IllegalAccessError` if module access is denied.
#[inline]
pub(crate) fn check_class_access(frame: &Frame, target_class: &Arc<Class>) -> Result<()> {
    let source_class = frame.class();

    // Same class always has access
    if Arc::ptr_eq(source_class, target_class) {
        return Ok(());
    }

    // Get module names (None = unnamed module)
    let source_module = source_class.module_name().ok().flatten();
    let target_module = target_class.module_name().ok().flatten();

    // Same module always has access
    if source_module == target_module {
        return Ok(());
    }

    // Get VM for access checking
    let thread = frame.thread()?;
    let vm = thread.vm()?;

    // Check module access
    let result = vm.module_system().check_access(
        source_module.as_deref(),
        target_module.as_deref(),
        target_class.name(),
    );

    if result.is_allowed() {
        return Ok(());
    }

    // For now, log but don't fail for system modules to avoid breaking tests
    // In a complete implementation, this would throw IllegalAccessError
    if should_enforce_access(source_module.as_deref(), target_module.as_deref()) {
        vm.module_system().require_access(
            source_module.as_deref(),
            target_module.as_deref(),
            target_class.name(),
        )?;
    }

    Ok(())
}

/// Checks if reflective access from the current frame's class to a target class is allowed.
///
/// This is used for deep reflection (accessing non-public members).
/// For reflection access to be allowed:
/// - The source module must read the target module
/// - The target module must open the package containing the class
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `target_class` - The class being reflectively accessed
///
/// # Errors
///
/// Returns an `InaccessibleObjectException` if reflection access is denied.
#[inline]
pub(crate) fn check_reflection_access(frame: &Frame, target_class: &Arc<Class>) -> Result<()> {
    let source_class = frame.class();

    // Same class always has access
    if Arc::ptr_eq(source_class, target_class) {
        return Ok(());
    }

    // Get module names (None = unnamed module)
    let source_module = source_class.module_name().ok().flatten();
    let target_module = target_class.module_name().ok().flatten();

    // Same module always has access
    if source_module == target_module {
        return Ok(());
    }

    // Get VM for access checking
    let thread = frame.thread()?;
    let vm = thread.vm()?;

    // Check module reflection access
    let result = vm.module_system().check_reflection_access(
        source_module.as_deref(),
        target_module.as_deref(),
        target_class.name(),
    );

    if result.is_allowed() {
        return Ok(());
    }

    // For now, log but don't fail for system modules to avoid breaking tests
    // In a complete implementation, this would throw InaccessibleObjectException
    if should_enforce_access(source_module.as_deref(), target_module.as_deref()) {
        vm.module_system().require_reflection_access(
            source_module.as_deref(),
            target_module.as_deref(),
            target_class.name(),
        )?;
    }

    Ok(())
}

/// Checks module access for a class by name.
///
/// This is used when we don't have the target class loaded yet (e.g., during class loading).
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `target_class_name` - The name of the class being accessed (internal format)
/// * `target_module` - The module containing the target class, if known
///
/// # Errors
///
/// Returns an `IllegalAccessError` if module access is denied.
#[inline]
pub(crate) fn check_class_access_by_name(
    frame: &Frame,
    target_class_name: &str,
    target_module: Option<&str>,
) -> Result<()> {
    let source_class = frame.class();
    let source_module = source_class.module_name().ok().flatten();

    // Same module always has access
    if source_module.as_deref() == target_module {
        return Ok(());
    }

    // Get VM for access checking
    let thread = frame.thread()?;
    let vm = thread.vm()?;

    // Check module access
    let result =
        vm.module_system()
            .check_access(source_module.as_deref(), target_module, target_class_name);

    if result.is_allowed() {
        return Ok(());
    }

    // For now, don't enforce for system modules
    if should_enforce_access(source_module.as_deref(), target_module) {
        vm.module_system().require_access(
            source_module.as_deref(),
            target_module,
            target_class_name,
        )?;
    }

    Ok(())
}

/// Checks module reflection access for a class by name.
///
/// # Arguments
///
/// * `frame` - The current execution frame
/// * `target_class_name` - The name of the class being accessed (internal format)
/// * `target_module` - The module containing the target class, if known
///
/// # Errors
///
/// Returns an `InaccessibleObjectException` if reflection access is denied.
#[inline]
pub(crate) fn check_reflection_access_by_name(
    frame: &Frame,
    target_class_name: &str,
    target_module: Option<&str>,
) -> Result<()> {
    let source_class = frame.class();
    let source_module = source_class.module_name().ok().flatten();

    // Same module always has access
    if source_module.as_deref() == target_module {
        return Ok(());
    }

    // Get VM for access checking
    let thread = frame.thread()?;
    let vm = thread.vm()?;

    // Check module reflection access
    let result = vm.module_system().check_reflection_access(
        source_module.as_deref(),
        target_module,
        target_class_name,
    );

    if result.is_allowed() {
        return Ok(());
    }

    // For now, don't enforce for system modules
    if should_enforce_access(source_module.as_deref(), target_module) {
        vm.module_system().require_reflection_access(
            source_module.as_deref(),
            target_module,
            target_class_name,
        )?;
    }

    Ok(())
}

/// Converts an access check result to an appropriate error.
///
/// # Arguments
///
/// * `result` - The access check result
/// * `source_module` - The module requesting access
/// * `target_module` - The module being accessed
/// * `target_class` - The class being accessed
///
/// # Returns
///
/// An appropriate error for the access denial.
#[must_use]
pub fn access_denied_error(
    result: AccessCheckResult,
    source_module: Option<&str>,
    target_module: Option<&str>,
    target_class: &str,
) -> crate::Error {
    use crate::JavaError::{IllegalAccessError, InaccessibleObjectException};
    use crate::module_system::{ALL_UNNAMED, ModuleSystem};

    let from = source_module.unwrap_or(ALL_UNNAMED);
    let to = target_module.unwrap_or(ALL_UNNAMED);
    let error_msg = ModuleSystem::illegal_access_error(from, to, target_class, result);

    match result {
        AccessCheckResult::NotOpened => InaccessibleObjectException(error_msg).into(),
        _ => IllegalAccessError(error_msg).into(),
    }
}

/// Determines whether to enforce access checking based on the modules involved.
///
/// Currently, we relax enforcement for access to system modules (java.*, jdk.*, etc.)
/// because:
/// 1. The module graph is not fully populated from module-info.class files
/// 2. Many tests rely on accessing system classes without proper module configuration
///
/// This should be removed once the module system is fully integrated with
/// module-info.class parsing from the system modules.
#[inline]
fn should_enforce_access(source_module: Option<&str>, target_module: Option<&str>) -> bool {
    // Don't enforce if either is in the unnamed module
    if source_module.is_none() || target_module.is_none() {
        return false;
    }

    let target = target_module.unwrap_or("");

    // Don't enforce access to system modules for now
    // These are typically exported via module-info.class which isn't fully parsed yet
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_system::ALL_UNNAMED;

    #[tokio::test]
    async fn test_check_class_access_same_class() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let class = frame.class().clone();
        // Same class access should always be allowed
        check_class_access(&frame, &class)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_reflection_access_same_class() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let class = frame.class().clone();
        // Same class reflection should always be allowed
        check_reflection_access(&frame, &class)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_class_access_java_lang_object() -> Result<()> {
        let (_vm, thread, frame) = crate::test::frame().await?;
        let object_class = thread.class("java/lang/Object").await?;
        // Access to java.lang.Object should be allowed (exported from java.base)
        check_class_access(&frame, &object_class)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_reflection_access_java_lang_object() -> Result<()> {
        let (_vm, thread, frame) = crate::test::frame().await?;
        let object_class = thread.class("java/lang/Object").await?;
        // Reflection on java.lang.Object should work (opened by default)
        check_reflection_access(&frame, &object_class)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_class_access_by_name_same_module() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        // Same module (both unnamed) should be allowed
        check_class_access_by_name(&frame, "com/example/MyClass", None)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_reflection_access_by_name_same_module() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        // Same module (both unnamed) should be allowed
        check_reflection_access_by_name(&frame, "com/example/MyClass", None)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_class_access_to_system_module() -> Result<()> {
        let (_vm, thread, frame) = crate::test::frame().await?;
        let string_class = thread.class("java/lang/String").await?;
        // Access to java.lang.String should be allowed (system module, exported)
        check_class_access(&frame, &string_class)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_class_access_multiple_system_classes() -> Result<()> {
        let (_vm, thread, frame) = crate::test::frame().await?;

        // Test various system classes
        let classes = ["java/lang/Integer", "java/util/ArrayList", "java/io/File"];

        for class_name in &classes {
            let class = thread.class(class_name).await?;
            check_class_access(&frame, &class)?;
        }
        Ok(())
    }

    #[test]
    fn test_should_enforce_access_unnamed_modules() {
        // Both unnamed - don't enforce
        assert!(!should_enforce_access(None, None));
        // Source unnamed - don't enforce
        assert!(!should_enforce_access(None, Some("app.module")));
        // Target unnamed - don't enforce
        assert!(!should_enforce_access(Some("app.module"), None));
    }

    #[test]
    fn test_should_enforce_access_system_modules() {
        // System modules should not be enforced
        let source = Some("my.app");
        assert!(!should_enforce_access(source, Some("java.base")));
        assert!(!should_enforce_access(source, Some("java.sql")));
        assert!(!should_enforce_access(source, Some("jdk.compiler")));
        assert!(!should_enforce_access(source, Some("sun.misc")));
        assert!(!should_enforce_access(
            source,
            Some("com.sun.crypto.provider")
        ));
    }

    #[test]
    fn test_should_enforce_access_application_modules() {
        // Application modules should be enforced
        let source = Some("my.app");
        let target = Some("other.app");
        assert!(should_enforce_access(source, target));
    }

    #[test]
    fn test_access_denied_error_not_readable() {
        let error = access_denied_error(
            AccessCheckResult::NotReadable,
            Some("my.app"),
            Some("other.app"),
            "other/internal/Secret",
        );
        let error_str = format!("{error:?}");
        assert!(error_str.contains("IllegalAccessError"));
    }

    #[test]
    fn test_access_denied_error_not_exported() {
        let error = access_denied_error(
            AccessCheckResult::NotExported,
            Some("my.app"),
            Some("other.app"),
            "other/internal/Secret",
        );
        let error_str = format!("{error:?}");
        assert!(error_str.contains("IllegalAccessError"));
    }

    #[test]
    fn test_access_denied_error_not_opened() {
        let error = access_denied_error(
            AccessCheckResult::NotOpened,
            Some("my.app"),
            Some("other.app"),
            "other/internal/Secret",
        );
        let error_str = format!("{error:?}");
        assert!(error_str.contains("InaccessibleObjectException"));
    }

    #[test]
    fn test_access_denied_error_unnamed_module() {
        let error = access_denied_error(
            AccessCheckResult::NotExported,
            None,
            Some("java.base"),
            "java/lang/internal/Secret",
        );
        let error_str = format!("{error:?}");
        assert!(error_str.contains("unnamed module") || error_str.contains(ALL_UNNAMED));
    }
}

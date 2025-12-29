//! Module access checking.
//!
//! This module implements the JPMS access control rules as defined by the Java SE specification.
//! It enforces both compile-time style access (exports) and runtime reflection access (opens).
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
//! - **Automatic modules**: JARs on the module path without `module-info.class` become
//!   automatic modules, which export and open all packages.
//! - **`java.base`**: All named modules implicitly read `java.base`.
//! - **Same module**: Access within the same module is always allowed.

use crate::module::error::ModuleError;
use crate::module::resolution::ResolvedConfiguration;

/// Result of an access check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessCheckResult {
    /// Access is allowed.
    Allowed,
    /// Access is denied because the module is not readable.
    NotReadable,
    /// Access is denied because the package is not exported.
    NotExported,
    /// Access is denied because the package is not opened (for reflection).
    NotOpened,
}

impl AccessCheckResult {
    /// Returns true if access is allowed.
    #[must_use]
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns true if access is denied.
    #[must_use]
    pub fn is_denied(&self) -> bool {
        !self.is_allowed()
    }

    /// Returns the reason for denial as a string.
    #[must_use]
    pub fn denial_reason(&self) -> Option<&'static str> {
        match self {
            Self::Allowed => None,
            Self::NotReadable => Some("module does not read target module"),
            Self::NotExported => Some("package is not exported to this module"),
            Self::NotOpened => Some("package is not opened for deep reflection"),
        }
    }
}

impl std::fmt::Display for AccessCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allowed => write!(f, "access allowed"),
            Self::NotReadable => write!(f, "module not readable"),
            Self::NotExported => write!(f, "package not exported"),
            Self::NotOpened => write!(f, "package not opened for reflection"),
        }
    }
}

/// Constant representing the unnamed module (classpath code).
pub const UNNAMED_MODULE: &str = "ALL-UNNAMED";

/// Constant representing the `java.base` module.
pub const JAVA_BASE_MODULE: &str = "java.base";

/// Module access checker for enforcing encapsulation.
///
/// This struct provides methods to check whether module access is permitted
/// according to JPMS rules. It considers:
/// - Module readability graph
/// - Package exports (qualified and unqualified)
/// - Package opens (for reflection)
/// - Command-line overrides (`--add-reads`, `--add-exports`, `--add-opens`)
/// - Special modules (unnamed, automatic, open)
#[derive(Debug)]
pub struct AccessCheck<'a> {
    config: &'a ResolvedConfiguration,
}

impl<'a> AccessCheck<'a> {
    /// Creates a new access checker for the given configuration.
    #[must_use]
    pub fn new(config: &'a ResolvedConfiguration) -> Self {
        Self { config }
    }

    /// Checks if `from_module` can access a public type in `package` from `to_module`.
    ///
    /// This implements the JPMS access check:
    /// 1. `from_module` must read `to_module`
    /// 2. `to_module` must export `package` to `from_module`
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `UNNAMED_MODULE` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `package` - The package containing the target type (in internal format, e.g., "java/lang")
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating whether access is allowed or denied.
    #[must_use]
    pub fn check_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> AccessCheckResult {
        // Same module always allowed
        if from_module == to_module {
            return AccessCheckResult::Allowed;
        }

        // Handle unnamed module (classpath code)
        if from_module == UNNAMED_MODULE {
            return self.check_unnamed_module_access(to_module, package);
        }

        // Check readability first
        if !self.can_read(from_module, to_module) {
            return AccessCheckResult::NotReadable;
        }

        // Check export
        if !self.is_exported(to_module, package, from_module) {
            return AccessCheckResult::NotExported;
        }

        AccessCheckResult::Allowed
    }

    /// Checks if `from_module` can reflectively access `package` in `to_module`.
    ///
    /// This requires the package to be "opened" for deep reflection.
    /// Deep reflection allows access to non-public members of classes.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `UNNAMED_MODULE` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `package` - The package containing the target type (in internal format)
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating whether reflection access is allowed or denied.
    #[must_use]
    pub fn check_reflection_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> AccessCheckResult {
        // Same module always allowed
        if from_module == to_module {
            return AccessCheckResult::Allowed;
        }

        // Handle unnamed module
        if from_module == UNNAMED_MODULE {
            return self.check_unnamed_module_reflection(to_module, package);
        }

        // Check readability first
        if !self.can_read(from_module, to_module) {
            return AccessCheckResult::NotReadable;
        }

        // Check if package is opened
        if !self.is_opened(to_module, package, from_module) {
            return AccessCheckResult::NotOpened;
        }

        AccessCheckResult::Allowed
    }

    /// Checks if `from_module` can read `to_module`.
    ///
    /// A module can read another module if:
    /// - It has a `requires` directive for that module
    /// - It transitively requires a module that has `requires transitive`
    /// - `--add-reads` was specified on the command line
    /// - The target is `java.base` (implicitly readable)
    #[must_use]
    pub fn can_read(&self, from_module: &str, to_module: &str) -> bool {
        // Same module can always read itself
        if from_module == to_module {
            return true;
        }

        // java.base is implicitly readable by all named modules
        if to_module == JAVA_BASE_MODULE {
            return true;
        }

        // Check the resolved configuration
        self.config.reads(from_module, to_module)
    }

    /// Checks if a package is exported from `to_module` to `from_module`.
    ///
    /// This checks:
    /// - Unqualified exports (to all modules)
    /// - Qualified exports (to specific modules)
    /// - `--add-exports` overrides
    /// - Automatic modules (export everything)
    #[must_use]
    pub fn is_exported(&self, to_module: &str, package: &str, from_module: &str) -> bool {
        self.config.exports(to_module, package, from_module)
    }

    /// Checks if a package is opened from `to_module` to `from_module`.
    ///
    /// This checks:
    /// - Unqualified opens (to all modules)
    /// - Qualified opens (to specific modules)
    /// - `--add-opens` overrides
    /// - Open modules (all packages implicitly opened)
    /// - Automatic modules (open everything)
    #[must_use]
    pub fn is_opened(&self, to_module: &str, package: &str, from_module: &str) -> bool {
        self.config.opens(to_module, package, from_module)
    }

    /// Checks access for code in the unnamed module (classpath).
    fn check_unnamed_module_access(&self, to_module: &str, package: &str) -> AccessCheckResult {
        // Unnamed module can read all named modules, so just check exports

        // Check if package is exported (at least unqualified)
        if let Some(module) = self.config.get(to_module) {
            let descriptor = module.descriptor();

            // Automatic modules export all packages
            if module.reference().is_automatic() {
                if descriptor.packages.contains(package) {
                    return AccessCheckResult::Allowed;
                }
                return AccessCheckResult::NotExported;
            }

            // Check explicit exports
            if descriptor.exports_package(package, None) {
                return AccessCheckResult::Allowed;
            }

            // Check qualified export to ALL-UNNAMED
            if descriptor.exports_package(package, Some(UNNAMED_MODULE)) {
                return AccessCheckResult::Allowed;
            }

            // Check --add-exports for ALL-UNNAMED
            if let Some(module_exports) = self.config.add_exports().get(to_module)
                && let Some(targets) = module_exports.get(package)
                && targets.contains(UNNAMED_MODULE)
            {
                return AccessCheckResult::Allowed;
            }
        }

        AccessCheckResult::NotExported
    }

    /// Checks reflection access for code in the unnamed module (classpath).
    fn check_unnamed_module_reflection(&self, to_module: &str, package: &str) -> AccessCheckResult {
        // Unnamed module can read all named modules, so just check opens

        if let Some(module) = self.config.get(to_module) {
            let descriptor = module.descriptor();

            // Open modules open all packages
            if descriptor.is_open() && descriptor.packages.contains(package) {
                return AccessCheckResult::Allowed;
            }

            // Automatic modules open all packages
            if module.reference().is_automatic() {
                if descriptor.packages.contains(package) {
                    return AccessCheckResult::Allowed;
                }
                return AccessCheckResult::NotOpened;
            }

            // Check explicit opens
            if descriptor.opens_package(package, None) {
                return AccessCheckResult::Allowed;
            }

            // Check qualified opens to ALL-UNNAMED
            if descriptor.opens_package(package, Some(UNNAMED_MODULE)) {
                return AccessCheckResult::Allowed;
            }

            // Check --add-opens for ALL-UNNAMED
            if let Some(module_opens) = self.config.add_opens().get(to_module)
                && let Some(targets) = module_opens.get(package)
                && targets.contains(UNNAMED_MODULE)
            {
                return AccessCheckResult::Allowed;
            }
        }

        AccessCheckResult::NotOpened
    }

    /// Converts an access check failure to an appropriate error.
    ///
    /// This creates an error with a message matching `OpenJDK`'s style for
    /// `IllegalAccessError` and `InaccessibleObjectException`.
    #[must_use]
    pub fn to_error(
        &self,
        result: AccessCheckResult,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> ModuleError {
        match result {
            AccessCheckResult::Allowed => {
                ModuleError::InternalError("to_error called for allowed access".to_string())
            }
            AccessCheckResult::NotReadable | AccessCheckResult::NotExported => {
                ModuleError::AccessDenied {
                    from_module: from_module.to_string(),
                    to_module: to_module.to_string(),
                    package: package.to_string(),
                }
            }
            AccessCheckResult::NotOpened => ModuleError::ReflectionAccessDenied {
                from_module: from_module.to_string(),
                to_module: to_module.to_string(),
                package: package.to_string(),
            },
        }
    }

    /// Checks access and returns an error if denied.
    ///
    /// # Errors
    ///
    /// Returns a `ModuleError` if access is denied.
    pub fn require_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> Result<(), ModuleError> {
        let result = self.check_access(from_module, to_module, package);
        if result.is_allowed() {
            Ok(())
        } else {
            Err(self.to_error(result, from_module, to_module, package))
        }
    }

    /// Checks reflection access and returns an error if denied.
    ///
    /// # Errors
    ///
    /// Returns a `ModuleError` if reflection access is denied.
    pub fn require_reflection_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> Result<(), ModuleError> {
        let result = self.check_reflection_access(from_module, to_module, package);
        if result.is_allowed() {
            Ok(())
        } else {
            Err(self.to_error(result, from_module, to_module, package))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::descriptor::{Exports, ModuleDescriptor, Opens};
    use crate::module::reference::ModuleReference;
    use crate::module::resolution::{ResolvedConfiguration, ResolvedModule};
    use std::collections::{BTreeMap, HashMap, HashSet};

    /// Creates a test module descriptor.
    fn create_descriptor(name: &str) -> ModuleDescriptor {
        let mut descriptor = ModuleDescriptor::new(name.to_string());
        descriptor.packages.insert(format!("{name}/internal"));
        descriptor.packages.insert(format!("{name}/api"));
        descriptor
    }

    /// Creates a test module descriptor with exports.
    fn create_descriptor_with_exports(
        name: &str,
        exports: Vec<(&str, Option<Vec<&str>>)>,
    ) -> ModuleDescriptor {
        let mut descriptor = create_descriptor(name);
        for (package, targets) in exports {
            descriptor.packages.insert(package.to_string());
            descriptor.exports.push(Exports {
                package: package.to_string(),
                targets: targets.map(|t| t.iter().map(|s| (*s).to_string()).collect()),
            });
        }
        descriptor
    }

    /// Creates a test module descriptor with opens.
    fn create_descriptor_with_opens(
        name: &str,
        opens: Vec<(&str, Option<Vec<&str>>)>,
    ) -> ModuleDescriptor {
        let mut descriptor = create_descriptor(name);
        for (package, targets) in opens {
            descriptor.packages.insert(package.to_string());
            descriptor.opens.push(Opens {
                package: package.to_string(),
                targets: targets.map(|t| t.iter().map(|s| (*s).to_string()).collect()),
            });
        }
        descriptor
    }

    /// Creates a resolved module.
    fn create_resolved_module(descriptor: ModuleDescriptor, reads: Vec<&str>) -> ResolvedModule {
        let reference = ModuleReference::system(descriptor);
        let mut resolved = ResolvedModule::new(reference);
        for read in reads {
            resolved.add_read(read.to_string());
        }
        resolved
    }

    /// Creates a test configuration with the given modules.
    fn create_test_config(modules: Vec<ResolvedModule>) -> ResolvedConfiguration {
        let mut resolved = BTreeMap::new();
        let mut package_to_module = BTreeMap::new();

        for module in modules {
            let name = module.name().to_string();
            for package in &module.descriptor().packages {
                package_to_module.insert(package.clone(), name.clone());
            }
            resolved.insert(name, module);
        }

        ResolvedConfiguration::new(resolved, package_to_module, HashMap::new(), HashMap::new())
    }

    /// Creates a test configuration with add-exports.
    fn create_config_with_exports(
        modules: Vec<ResolvedModule>,
        add_exports: HashMap<String, HashMap<String, HashSet<String>>>,
    ) -> ResolvedConfiguration {
        let mut resolved = BTreeMap::new();
        let mut package_to_module = BTreeMap::new();

        for module in modules {
            let name = module.name().to_string();
            for package in &module.descriptor().packages {
                package_to_module.insert(package.clone(), name.clone());
            }
            resolved.insert(name, module);
        }

        ResolvedConfiguration::new(resolved, package_to_module, add_exports, HashMap::new())
    }

    /// Creates a test configuration with add-opens.
    fn create_config_with_opens(
        modules: Vec<ResolvedModule>,
        add_opens: HashMap<String, HashMap<String, HashSet<String>>>,
    ) -> ResolvedConfiguration {
        let mut resolved = BTreeMap::new();
        let mut package_to_module = BTreeMap::new();

        for module in modules {
            let name = module.name().to_string();
            for package in &module.descriptor().packages {
                package_to_module.insert(package.clone(), name.clone());
            }
            resolved.insert(name, module);
        }

        ResolvedConfiguration::new(resolved, package_to_module, HashMap::new(), add_opens)
    }

    // ==================== AccessCheckResult tests ====================

    #[test]
    fn test_access_check_result_allowed() {
        let result = AccessCheckResult::Allowed;
        assert!(result.is_allowed());
        assert!(!result.is_denied());
        assert!(result.denial_reason().is_none());
        assert_eq!(format!("{result}"), "access allowed");
    }

    #[test]
    fn test_access_check_result_not_readable() {
        let result = AccessCheckResult::NotReadable;
        assert!(!result.is_allowed());
        assert!(result.is_denied());
        assert_eq!(
            result.denial_reason(),
            Some("module does not read target module")
        );
        assert_eq!(format!("{result}"), "module not readable");
    }

    #[test]
    fn test_access_check_result_not_exported() {
        let result = AccessCheckResult::NotExported;
        assert!(!result.is_allowed());
        assert!(result.is_denied());
        assert_eq!(
            result.denial_reason(),
            Some("package is not exported to this module")
        );
        assert_eq!(format!("{result}"), "package not exported");
    }

    #[test]
    fn test_access_check_result_not_opened() {
        let result = AccessCheckResult::NotOpened;
        assert!(!result.is_allowed());
        assert!(result.is_denied());
        assert_eq!(
            result.denial_reason(),
            Some("package is not opened for deep reflection")
        );
        assert_eq!(format!("{result}"), "package not opened for reflection");
    }

    // ==================== Same module access tests ====================

    #[test]
    fn test_same_module_access_always_allowed() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]);
        let config = create_test_config(vec![module_a]);
        let checker = AccessCheck::new(&config);

        // Same module access is always allowed, even for non-exported packages
        assert_eq!(
            checker.check_access("module.a", "module.a", "module.a/internal"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_same_module_reflection_always_allowed() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]);
        let config = create_test_config(vec![module_a]);
        let checker = AccessCheck::new(&config);

        // Same module reflection is always allowed, even for non-opened packages
        assert_eq!(
            checker.check_reflection_access("module.a", "module.a", "module.a/internal"),
            AccessCheckResult::Allowed
        );
    }

    // ==================== Readability tests ====================

    #[test]
    fn test_readable_module_exported_package() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        // Module A reads B and B exports the package
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_not_readable_module_denied() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]); // Does not read B
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        // Module A doesn't read B
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::NotReadable
        );
    }

    #[test]
    fn test_can_read_checks_readability() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        assert!(checker.can_read("module.a", "module.b"));
        assert!(!checker.can_read("module.b", "module.a"));
    }

    #[test]
    fn test_can_read_same_module() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]);
        let config = create_test_config(vec![module_a]);
        let checker = AccessCheck::new(&config);

        assert!(checker.can_read("module.a", "module.a"));
    }

    #[test]
    fn test_java_base_implicitly_readable() {
        // java.base should be implicitly readable by all named modules
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]);
        let config = create_test_config(vec![module_a]);
        let checker = AccessCheck::new(&config);

        assert!(checker.can_read("module.a", JAVA_BASE_MODULE));
    }

    // ==================== Export tests ====================

    #[test]
    fn test_non_exported_package_denied() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]); // No exports
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        // Module A reads B but B doesn't export the package
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/internal"),
            AccessCheckResult::NotExported
        );
    }

    #[test]
    fn test_qualified_export_to_specific_module() {
        let descriptor_b = create_descriptor_with_exports(
            "module.b",
            vec![("module.b/internal", Some(vec!["module.a"]))],
        );
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let module_c = create_resolved_module(create_descriptor("module.c"), vec!["module.b"]);
        let config = create_test_config(vec![module_a, module_b, module_c]);
        let checker = AccessCheck::new(&config);

        // Module A can access (qualified export)
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/internal"),
            AccessCheckResult::Allowed
        );

        // Module C cannot access (not in qualified export list)
        assert_eq!(
            checker.check_access("module.c", "module.b", "module.b/internal"),
            AccessCheckResult::NotExported
        );
    }

    #[test]
    fn test_add_exports_override() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]); // No exports

        // Add --add-exports module.b/module.b/internal=module.a
        let mut add_exports: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
        let mut package_map = HashMap::new();
        let mut targets = HashSet::new();
        targets.insert("module.a".to_string());
        package_map.insert("module.b/internal".to_string(), targets);
        add_exports.insert("module.b".to_string(), package_map);

        let config = create_config_with_exports(vec![module_a, module_b], add_exports);
        let checker = AccessCheck::new(&config);

        // Now module A can access the internal package
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/internal"),
            AccessCheckResult::Allowed
        );
    }

    // ==================== Opens tests ====================

    #[test]
    fn test_exported_but_not_opened_package() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        // Regular access allowed
        assert_eq!(
            checker.check_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );

        // Reflection access denied (not opened)
        assert_eq!(
            checker.check_reflection_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::NotOpened
        );
    }

    #[test]
    fn test_opened_package_allows_reflection() {
        let mut descriptor_b =
            create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        descriptor_b.opens.push(Opens {
            package: "module.b/api".to_string(),
            targets: None,
        });

        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        assert_eq!(
            checker.check_reflection_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_qualified_opens() {
        let descriptor_b = create_descriptor_with_opens(
            "module.b",
            vec![("module.b/api", Some(vec!["module.a"]))],
        );
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let module_c = create_resolved_module(create_descriptor("module.c"), vec!["module.b"]);
        let config = create_test_config(vec![module_a, module_b, module_c]);
        let checker = AccessCheck::new(&config);

        // Module A can reflect (qualified opens)
        assert_eq!(
            checker.check_reflection_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );

        // Module C cannot reflect (not in qualified opens list)
        assert_eq!(
            checker.check_reflection_access("module.c", "module.b", "module.b/api"),
            AccessCheckResult::NotOpened
        );
    }

    #[test]
    fn test_add_opens_override() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(descriptor_b, vec![]);

        // Add --add-opens module.b/module.b/api=module.a
        let mut add_opens: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
        let mut package_map = HashMap::new();
        let mut targets = HashSet::new();
        targets.insert("module.a".to_string());
        package_map.insert("module.b/api".to_string(), targets);
        add_opens.insert("module.b".to_string(), package_map);

        let config = create_config_with_opens(vec![module_a, module_b], add_opens);
        let checker = AccessCheck::new(&config);

        // Now module A can reflect
        assert_eq!(
            checker.check_reflection_access("module.a", "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );
    }

    // ==================== Unnamed module tests ====================

    #[test]
    fn test_unnamed_module_access_exported_package() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_b]);
        let checker = AccessCheck::new(&config);

        // Unnamed module can access exported packages
        assert_eq!(
            checker.check_access(UNNAMED_MODULE, "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_unnamed_module_cannot_access_non_exported() {
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]);
        let config = create_test_config(vec![module_b]);
        let checker = AccessCheck::new(&config);

        // Unnamed module cannot access non-exported packages
        assert_eq!(
            checker.check_access(UNNAMED_MODULE, "module.b", "module.b/internal"),
            AccessCheckResult::NotExported
        );
    }

    #[test]
    fn test_unnamed_module_with_add_exports() {
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]);

        // Add --add-exports module.b/module.b/internal=ALL-UNNAMED
        let mut add_exports: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
        let mut package_map = HashMap::new();
        let mut targets = HashSet::new();
        targets.insert(UNNAMED_MODULE.to_string());
        package_map.insert("module.b/internal".to_string(), targets);
        add_exports.insert("module.b".to_string(), package_map);

        let config = create_config_with_exports(vec![module_b], add_exports);
        let checker = AccessCheck::new(&config);

        // Now unnamed module can access
        assert_eq!(
            checker.check_access(UNNAMED_MODULE, "module.b", "module.b/internal"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_unnamed_module_reflection_opened_package() {
        let descriptor_b = create_descriptor_with_opens("module.b", vec![("module.b/api", None)]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_b]);
        let checker = AccessCheck::new(&config);

        // Unnamed module can reflect on opened packages
        assert_eq!(
            checker.check_reflection_access(UNNAMED_MODULE, "module.b", "module.b/api"),
            AccessCheckResult::Allowed
        );
    }

    #[test]
    fn test_unnamed_module_cannot_reflect_non_opened() {
        let descriptor_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_b = create_resolved_module(descriptor_b, vec![]);
        let config = create_test_config(vec![module_b]);
        let checker = AccessCheck::new(&config);

        // Unnamed module cannot reflect on non-opened packages
        assert_eq!(
            checker.check_reflection_access(UNNAMED_MODULE, "module.b", "module.b/api"),
            AccessCheckResult::NotOpened
        );
    }

    // ==================== Error conversion tests ====================

    #[test]
    fn test_to_error_not_readable() {
        let config = create_test_config(vec![]);
        let checker = AccessCheck::new(&config);

        let error = checker.to_error(
            AccessCheckResult::NotReadable,
            "module.a",
            "module.b",
            "pkg",
        );
        assert!(matches!(error, ModuleError::AccessDenied { .. }));
    }

    #[test]
    fn test_to_error_not_exported() {
        let config = create_test_config(vec![]);
        let checker = AccessCheck::new(&config);

        let error = checker.to_error(
            AccessCheckResult::NotExported,
            "module.a",
            "module.b",
            "pkg",
        );
        assert!(matches!(error, ModuleError::AccessDenied { .. }));
    }

    #[test]
    fn test_to_error_not_opened() {
        let config = create_test_config(vec![]);
        let checker = AccessCheck::new(&config);

        let error = checker.to_error(AccessCheckResult::NotOpened, "module.a", "module.b", "pkg");
        assert!(matches!(error, ModuleError::ReflectionAccessDenied { .. }));
    }

    #[test]
    fn test_require_access_success() {
        let desc_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(desc_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        assert!(
            checker
                .require_access("module.a", "module.b", "module.b/api")
                .is_ok()
        );
    }

    #[test]
    fn test_require_access_failure() {
        let module_a = create_resolved_module(create_descriptor("module.a"), vec![]);
        let module_b = create_resolved_module(create_descriptor("module.b"), vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        let result = checker.require_access("module.a", "module.b", "module.b/internal");
        assert!(result.is_err());
    }

    #[test]
    fn test_require_reflection_access_success() {
        let desc_b = create_descriptor_with_opens("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(desc_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        assert!(
            checker
                .require_reflection_access("module.a", "module.b", "module.b/api")
                .is_ok()
        );
    }

    #[test]
    fn test_require_reflection_access_failure() {
        let desc_b = create_descriptor_with_exports("module.b", vec![("module.b/api", None)]);
        let module_a = create_resolved_module(create_descriptor("module.a"), vec!["module.b"]);
        let module_b = create_resolved_module(desc_b, vec![]);
        let config = create_test_config(vec![module_a, module_b]);
        let checker = AccessCheck::new(&config);

        let result = checker.require_reflection_access("module.a", "module.b", "module.b/api");
        assert!(result.is_err());
    }
}

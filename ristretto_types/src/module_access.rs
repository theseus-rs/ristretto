use crate::Result;
use ristretto_classloader::Value;

/// Information about a defined module.
#[derive(Clone, Debug)]
pub struct DefinedModule {
    /// Module name.
    pub name: String,
    /// Whether the module is open (all packages implicitly opened).
    pub is_open: bool,
    /// Module version (optional).
    pub version: Option<String>,
    /// Module location (optional, e.g., path to JAR).
    pub location: Option<String>,
    /// Packages contained in this module.
    pub packages: ahash::AHashSet<String>,
    /// The java.lang.Module object for this module (set during defineModule0).
    pub module_object: Option<Value>,
}

impl DefinedModule {
    /// Creates a new defined module.
    #[must_use]
    pub fn new(name: String, is_open: bool) -> Self {
        Self {
            name,
            is_open,
            version: None,
            location: None,
            packages: ahash::AHashSet::default(),
            module_object: None,
        }
    }
}

/// Special constant for "all unnamed modules" target.
pub const ALL_UNNAMED: &str = "ALL-UNNAMED";

/// Result of an access check.
pub use ristretto_classloader::module::AccessCheckResult;

/// Get the package name from a fully-qualified class name.
#[must_use]
pub fn package_from_class_name(class_name: &str) -> &str {
    let class_name = if let Some(pos) = class_name.find(';') {
        &class_name[..pos]
    } else {
        class_name
    };
    let class_name = class_name.trim_start_matches('[').trim_start_matches('L');
    if let Some(pos) = class_name.rfind('/') {
        &class_name[..pos]
    } else {
        ""
    }
}

/// Trait for module system access, used by intrinsic methods.
pub trait ModuleAccess: Send + Sync {
    /// Add an export from `source_module` of package to `target_module`.
    fn add_export(&self, source_module: &str, package: &str, target_module: Option<&str>);

    /// Add an export from `source_module` of package to all modules.
    fn add_export_to_all(&self, source_module: &str, package: &str);

    /// Add an export from `source_module` of package to all unnamed modules.
    fn add_export_to_all_unnamed(&self, source_module: &str, package: &str);

    /// Add a read edge from `source_module` to `target_module`.
    fn add_read(&self, source_module: &str, target_module: &str);

    /// Define a module.
    fn define_module(&self, module: DefinedModule);

    /// Check reflection access between modules.
    fn check_reflection_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> AccessCheckResult;

    /// Require reflection access, returning an error if denied.
    ///
    /// # Errors
    /// Returns an error if access is denied.
    fn require_reflection_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> Result<()>;

    /// Set the boot class loader's unnamed module.
    fn set_boot_unnamed_module(&self, module: Value);

    /// Get the boot class loader's unnamed module.
    fn boot_unnamed_module(&self) -> Option<Value>;

    /// Get the module object for a given package.
    fn get_module_for_package(&self, package: &str) -> Option<Value>;
}

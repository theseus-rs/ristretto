//! Runtime module system for tracking dynamic JPMS state.
//!
//! This module provides a complete module system implementation that combines:
//! - **Static configuration** (`ResolvedConfiguration`): Module graph resolved from
//!   module-info.class descriptors and command-line options at startup
//! - **Dynamic state**: Runtime modifications made via `Module.addExports0()`, etc.
//!
//! # Architecture
//!
//! The `ModuleSystem` struct encapsulates all JPMS state and provides:
//! - Module resolution from system modules (jimage), module path, and upgrade module path
//! - Command-line option processing (`--add-exports`, `--add-opens`, `--add-reads`)
//! - Access checking that combines static and dynamic state
//! - Runtime module modifications via JNI/intrinsic methods
//!
//! # Re-exports
//!
//! This module re-exports `AccessCheckResult` from `ristretto_classloader` for convenience,
//! ensuring a single canonical type is used throughout the codebase.

use crate::Configuration;
use crate::JavaError::{IllegalAccessError, InaccessibleObjectException};
use crate::Result;
use parking_lot::RwLock;
use ristretto_classloader::Value;
use ristretto_classloader::module::{
    AccessCheck, ModuleFinder, ModuleFinderChain, ModulePathFinder, ResolvedConfiguration,
    Resolver, SystemModuleFinder,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use tracing::debug;

// Re-export AccessCheckResult from classloader - this is the canonical location
pub use ristretto_classloader::module::AccessCheckResult;

/// Special constant for "all unnamed modules" target.
pub const ALL_UNNAMED: &str = "ALL-UNNAMED";

/// Special constant for exporting/opening to all modules.
pub const ALL_MODULES: &str = "ALL";

/// The `java.base` module name - implicitly readable by all modules.
pub const JAVA_BASE_MODULE: &str = "java.base";

/// Complete module system combining static configuration and dynamic runtime state.
///
/// This structure provides:
/// - **Static configuration** (`ResolvedConfiguration`): Module graph resolved at startup
/// - **Dynamic state**: Runtime modifications via `Module.addExports0()`, etc.
///
/// Access checking combines both sources - static configuration is checked first,
/// then dynamic modifications are checked if the static check fails.
#[derive(Debug)]
pub struct ModuleSystem {
    /// Static module configuration from resolution at startup.
    resolved_configuration: ResolvedConfiguration,

    /// Dynamic exports: `source_module` -> (package -> set of target modules).
    /// An entry means the package is exported from `source_module` to the targets.
    exports: RwLock<HashMap<String, HashMap<String, HashSet<String>>>>,

    /// Dynamic opens: `source_module` -> (package -> set of target modules).
    /// An entry means the package is opened from `source_module` to the targets.
    opens: RwLock<HashMap<String, HashMap<String, HashSet<String>>>>,

    /// Dynamic reads: `source_module` -> set of target modules.
    /// An entry means `source_module` can read the targets.
    reads: RwLock<HashMap<String, HashSet<String>>>,

    /// Defined modules: `module_name` -> module info.
    /// Tracks modules that have been defined via `Module.defineModule0`.
    modules: RwLock<HashMap<String, DefinedModule>>,

    /// The boot class loader's unnamed module.
    /// This is set by `BootLoader.setBootLoaderUnnamedModule0` during JVM initialization
    /// and used as the default module for classes that don't have an explicit module.
    boot_unnamed_module: RwLock<Option<Value>>,
}

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
    pub packages: HashSet<String>,
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
            packages: HashSet::new(),
        }
    }
}

impl ModuleSystem {
    /// Creates a new module system initialized with command-line options and resolved modules.
    ///
    /// This function performs full JPMS module resolution:
    /// 1. Loads system modules from the jimage file (for Java 9+)
    /// 2. Loads modules from the module path (if specified)
    /// 3. Resolves the module graph starting from root modules
    /// 4. Applies command-line overrides (--add-reads, --add-exports, --add-opens)
    ///
    /// # Errors
    ///
    /// Returns an error if module resolution fails.
    pub async fn new(
        configuration: &Configuration,
        java_home: &Path,
        java_major_version: u16,
    ) -> Result<Self> {
        let mut module_system = Self::empty();

        // Build the resolver with command line overrides
        let resolver = Self::build_resolver(configuration, &mut module_system);

        // For Java 8 and earlier, there's no module system; use configuration with overrides only
        if java_major_version <= 8 {
            debug!("Java 8 or earlier; module system disabled");
            module_system.resolved_configuration =
                Self::create_fallback_configuration(configuration);
            return Ok(module_system);
        }

        // Build the module finder chain
        let finder_chain = Self::build_finder_chain(configuration, java_home).await;

        // Check if we have a valid finder chain (needs system modules)
        if finder_chain.find("java.base").is_none() {
            debug!("No system modules found, using fallback configuration");
            module_system.resolved_configuration =
                Self::create_fallback_configuration(configuration);
            return Ok(module_system);
        }

        // Determine root modules for resolution
        let root_modules = Self::determine_root_modules(configuration, &finder_chain);

        debug!("Resolving modules with roots: {root_modules:?}");

        // Resolve the module graph
        module_system.resolved_configuration = match resolver.resolve(&root_modules, &finder_chain)
        {
            Ok(config) => {
                debug!(
                    "Resolved {} modules with {} packages",
                    config.len(),
                    config
                        .modules()
                        .map(|m| m.descriptor().packages.len())
                        .sum::<usize>()
                );
                config
            }
            Err(error) => {
                debug!("Module resolution failed: {error}, using fallback configuration");
                Self::create_fallback_configuration(configuration)
            }
        };

        Ok(module_system)
    }

    /// Creates a new empty module system with no static configuration.
    ///
    /// This is primarily useful for testing or when creating a module system
    /// that will only use dynamic modifications.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            resolved_configuration: ResolvedConfiguration::empty(),
            exports: RwLock::new(HashMap::new()),
            opens: RwLock::new(HashMap::new()),
            reads: RwLock::new(HashMap::new()),
            modules: RwLock::new(HashMap::new()),
            boot_unnamed_module: RwLock::new(None),
        }
    }

    /// Returns the resolved configuration.
    #[must_use]
    pub fn resolved_configuration(&self) -> &ResolvedConfiguration {
        &self.resolved_configuration
    }

    /// Builds the resolver with command-line overrides and populates initial dynamic state.
    fn build_resolver(configuration: &Configuration, module_system: &mut Self) -> Resolver {
        let mut resolver = Resolver::new();

        // Apply --add-reads
        for read in configuration.add_reads() {
            resolver.add_read(read.source.clone(), read.target.clone());
            module_system.add_read(&read.source, &read.target);
        }

        // Apply --add-exports
        for export in configuration.add_exports() {
            resolver.add_export(
                export.source.clone(),
                export.package.clone(),
                export.target.clone(),
            );
            module_system.add_export(&export.source, &export.package, Some(&export.target));
        }

        // Apply --add-opens
        for opens in configuration.add_opens() {
            resolver.add_opens(
                opens.source.clone(),
                opens.package.clone(),
                opens.target.clone(),
            );
            module_system.add_opens(&opens.source, &opens.package, Some(&opens.target));
        }

        // Apply --limit-modules
        if !configuration.limit_modules().is_empty() {
            resolver.set_limit_modules(configuration.limit_modules().clone());
        }

        resolver
    }

    /// Builds the module finder chain with precedence:
    /// 1. Upgrade module path (highest precedence)
    /// 2. System modules (from jimage)
    /// 3. Module path (lowest precedence for named modules)
    async fn build_finder_chain(
        configuration: &Configuration,
        java_home: &Path,
    ) -> ModuleFinderChain {
        let mut finder_chain = ModuleFinderChain::new();

        // Add upgrade module path finder if specified
        Self::add_upgrade_module_path(&mut finder_chain, configuration).await;

        // Add system module finder from jimage
        Self::add_system_modules(&mut finder_chain, java_home).await;

        // Add module path finder if specified
        Self::add_module_path(&mut finder_chain, configuration).await;

        finder_chain
    }

    /// Adds the upgrade module path finder to the chain if specified.
    async fn add_upgrade_module_path(
        finder_chain: &mut ModuleFinderChain,
        configuration: &Configuration,
    ) {
        if configuration.upgrade_module_path().is_empty() {
            return;
        }

        let upgrade_paths: Vec<PathBuf> = configuration
            .upgrade_module_path()
            .iter()
            .map(PathBuf::from)
            .collect();

        match ModulePathFinder::new(&upgrade_paths).await {
            Ok(upgrade_finder) => {
                debug!(
                    "Loaded {} modules from upgrade module path",
                    upgrade_finder.find_all().len()
                );
                finder_chain.add(Box::new(upgrade_finder));
            }
            Err(error) => {
                debug!("Failed to load upgrade module path: {error}");
            }
        }
    }

    /// Adds the system module finder from jimage to the chain.
    async fn add_system_modules(finder_chain: &mut ModuleFinderChain, java_home: &Path) {
        let jimage_path = java_home.join("lib").join("modules");

        if !jimage_path.exists() {
            debug!("No jimage found at {}", jimage_path.display());
            return;
        }

        match SystemModuleFinder::new(&jimage_path).await {
            Ok(system_finder) => {
                debug!(
                    "Loaded {} system modules from jimage",
                    system_finder.find_all().len()
                );
                finder_chain.add(Box::new(system_finder));
            }
            Err(error) => {
                debug!("Failed to load system modules from jimage: {error}");
            }
        }
    }

    /// Adds the module path finder to the chain if specified.
    async fn add_module_path(finder_chain: &mut ModuleFinderChain, configuration: &Configuration) {
        if configuration.module_path().is_empty() {
            return;
        }

        let module_paths: Vec<PathBuf> = configuration
            .module_path()
            .iter()
            .map(PathBuf::from)
            .collect();

        match ModulePathFinder::new(&module_paths).await {
            Ok(module_path_finder) => {
                debug!(
                    "Loaded {} modules from module path",
                    module_path_finder.find_all().len()
                );
                finder_chain.add(Box::new(module_path_finder));
            }
            Err(error) => {
                debug!("Failed to load module path: {error}");
            }
        }
    }

    /// Determines the root modules for resolution.
    fn determine_root_modules(
        configuration: &Configuration,
        finder_chain: &ModuleFinderChain,
    ) -> Vec<String> {
        let mut root_modules: Vec<String> = Vec::new();

        // Always include java.base as it's required
        root_modules.push("java.base".to_string());

        // Add the main module if running in module mode (-m/--module)
        if let Some(main_module) = configuration.main_module()
            && !root_modules.contains(&main_module.name)
        {
            root_modules.push(main_module.name.clone());
        }

        // Add modules from --add-modules
        Self::add_modules_from_config(&mut root_modules, configuration, finder_chain);

        // If no explicit roots beyond java.base, add commonly needed modules
        Self::add_default_modules(&mut root_modules, finder_chain);

        root_modules
    }

    /// Adds modules from --add-modules configuration.
    fn add_modules_from_config(
        root_modules: &mut Vec<String>,
        configuration: &Configuration,
        finder_chain: &ModuleFinderChain,
    ) {
        for module in configuration.add_modules() {
            if module == "ALL-SYSTEM" {
                // Add all system modules
                for reference in finder_chain.find_all() {
                    let name = reference.name().to_string();
                    if !root_modules.contains(&name) {
                        root_modules.push(name);
                    }
                }
            } else if module == "ALL-MODULE-PATH" || module == "ALL-DEFAULT" {
                // ALL-MODULE-PATH: Add all modules from module path (resolved as needed)
                // ALL-DEFAULT: Add default modules (those that export at least one package)
                // These are simplifications - OpenJDK has more complex rules
            } else if !root_modules.contains(module) {
                root_modules.push(module.clone());
            }
        }
    }

    /// Adds commonly needed default modules if no explicit roots beyond java.base.
    fn add_default_modules(root_modules: &mut Vec<String>, finder_chain: &ModuleFinderChain) {
        if root_modules.len() > 1 {
            return;
        }

        let default_modules = [
            "java.logging",
            "java.management",
            "java.naming",
            "java.security.sasl",
            "java.xml",
        ];

        for module in default_modules {
            if finder_chain.find(module).is_some() && !root_modules.contains(&module.to_string()) {
                root_modules.push(module.to_string());
            }
        }
    }

    /// Creates a fallback configuration with only command-line overrides.
    ///
    /// This is used when:
    /// - Java 8 or earlier (no module system)
    /// - No jimage file found
    /// - Module resolution fails
    fn create_fallback_configuration(configuration: &Configuration) -> ResolvedConfiguration {
        let add_exports: HashMap<String, HashMap<String, HashSet<String>>> = configuration
            .add_exports()
            .iter()
            .fold(HashMap::new(), |mut acc, export| {
                acc.entry(export.source.clone())
                    .or_default()
                    .entry(export.package.clone())
                    .or_default()
                    .insert(export.target.clone());
                acc
            });

        let add_opens: HashMap<String, HashMap<String, HashSet<String>>> = configuration
            .add_opens()
            .iter()
            .fold(HashMap::new(), |mut acc, opens| {
                acc.entry(opens.source.clone())
                    .or_default()
                    .entry(opens.package.clone())
                    .or_default()
                    .insert(opens.target.clone());
                acc
            });

        ResolvedConfiguration::new(BTreeMap::new(), BTreeMap::new(), add_exports, add_opens)
    }

    /// Checks if access from one module to a class in another module is allowed.
    ///
    /// This implements JPMS access checking by combining:
    /// 1. **Static configuration**: Module descriptors and command-line options
    /// 2. **Dynamic state**: Runtime modifications via `Module.addExports0()`, etc.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `ALL_UNNAMED` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `to_class_name` - The fully qualified class name (in internal format)
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating if access is allowed.
    #[must_use]
    pub fn check_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> AccessCheckResult {
        let from = from_module.unwrap_or(ALL_UNNAMED);
        let to = to_module.unwrap_or(ALL_UNNAMED);
        let package = Self::package_from_class_name(to_class_name);

        // First check static configuration
        let static_checker = AccessCheck::new(&self.resolved_configuration);
        let static_result = static_checker.check_access(from, to, package);
        if static_result.is_allowed() {
            return static_result;
        }

        // Check dynamic runtime state
        self.check_dynamic_access(from, to, package)
    }

    /// Checks if reflection access from one module to a class in another module is allowed.
    ///
    /// This implements JPMS reflection access checking by combining:
    /// 1. **Static configuration**: Module descriptors and command-line options
    /// 2. **Dynamic state**: Runtime modifications via `Module.addOpens0()`, etc.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `ALL_UNNAMED` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `to_class_name` - The fully qualified class name (in internal format)
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating if reflection access is allowed.
    #[must_use]
    pub fn check_reflection_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> AccessCheckResult {
        let from = from_module.unwrap_or(ALL_UNNAMED);
        let to = to_module.unwrap_or(ALL_UNNAMED);
        let package = Self::package_from_class_name(to_class_name);

        // First check static configuration
        let static_checker = AccessCheck::new(&self.resolved_configuration);
        let static_result = static_checker.check_reflection_access(from, to, package);
        if static_result.is_allowed() {
            return static_result;
        }

        // Check dynamic runtime state
        self.check_dynamic_reflection_access(from, to, package)
    }

    /// Adds an export from `source_module` of `package` to `target_module`.
    ///
    /// This is called by `Module.addExports0(Module, String, Module)`.
    /// If `target_module` is `None`, the package is exported to all modules.
    pub fn add_export(&self, source_module: &str, package: &str, target_module: Option<&str>) {
        let target = target_module.unwrap_or(ALL_UNNAMED);
        let mut exports = self.exports.write();
        exports
            .entry(source_module.to_string())
            .or_default()
            .entry(package.to_string())
            .or_default()
            .insert(target.to_string());
    }

    /// Adds an unqualified export from `source_module` of `package` to all modules.
    ///
    /// This is called by `Module.addExportsToAll0(Module, String)`.
    pub fn add_export_to_all(&self, source_module: &str, package: &str) {
        // Use a special marker for "all modules"
        self.add_export(source_module, package, Some("ALL"));
    }

    /// Adds an export from `source_module` of `package` to all unnamed modules.
    ///
    /// This is called by `Module.addExportsToAllUnnamed0(Module, String)`.
    pub fn add_export_to_all_unnamed(&self, source_module: &str, package: &str) {
        self.add_export(source_module, package, Some(ALL_UNNAMED));
    }

    /// Checks if `package` in `source_module` is exported to `target_module`.
    #[must_use]
    pub fn is_exported(&self, source_module: &str, package: &str, target_module: &str) -> bool {
        let exports = self.exports.read();
        if let Some(module_exports) = exports.get(source_module)
            && let Some(targets) = module_exports.get(package)
        {
            // Check for specific target, ALL-UNNAMED, or ALL (unqualified)
            return targets.contains(target_module)
                || targets.contains("ALL")
                || (target_module == ALL_UNNAMED && targets.contains(ALL_UNNAMED));
        }
        false
    }

    /// Adds an opens from `source_module` of `package` to `target_module`.
    ///
    /// This is called by `Module.addOpens0(Module, String, Module)`.
    /// If `target_module` is `None`, the package is opened to all modules.
    pub fn add_opens(&self, source_module: &str, package: &str, target_module: Option<&str>) {
        let target = target_module.unwrap_or(ALL_UNNAMED);
        let mut opens = self.opens.write();
        opens
            .entry(source_module.to_string())
            .or_default()
            .entry(package.to_string())
            .or_default()
            .insert(target.to_string());
    }

    /// Adds an unqualified opens from `source_module` of `package` to all modules.
    pub fn add_opens_to_all(&self, source_module: &str, package: &str) {
        self.add_opens(source_module, package, Some("ALL"));
    }

    /// Adds an opens from `source_module` of `package` to all unnamed modules.
    pub fn add_opens_to_all_unnamed(&self, source_module: &str, package: &str) {
        self.add_opens(source_module, package, Some(ALL_UNNAMED));
    }

    /// Checks if `package` in `source_module` is opened to `target_module`.
    #[must_use]
    pub fn is_opened(&self, source_module: &str, package: &str, target_module: &str) -> bool {
        // First check if the module is open (all packages implicitly opened)
        if self.is_module_open(source_module) {
            return true;
        }

        let opens = self.opens.read();
        if let Some(module_opens) = opens.get(source_module)
            && let Some(targets) = module_opens.get(package)
        {
            return targets.contains(target_module)
                || targets.contains("ALL")
                || (target_module == ALL_UNNAMED && targets.contains(ALL_UNNAMED));
        }
        false
    }

    /// Adds a read edge from `source_module` to `target_module`.
    ///
    /// This is called by `Module.addReads0(Module, Module)`.
    pub fn add_read(&self, source_module: &str, target_module: &str) {
        let mut reads = self.reads.write();
        reads
            .entry(source_module.to_string())
            .or_default()
            .insert(target_module.to_string());
    }

    /// Checks if `source_module` reads `target_module`.
    #[must_use]
    pub fn can_read(&self, source_module: &str, target_module: &str) -> bool {
        // Same module can always read itself
        if source_module == target_module {
            return true;
        }

        // java.base is implicitly readable by all modules
        if target_module == "java.base" {
            return true;
        }

        let reads = self.reads.read();
        reads
            .get(source_module)
            .is_some_and(|targets| targets.contains(target_module))
    }

    /// Defines a new module.
    ///
    /// This is called by `Module.defineModule0`.
    pub fn define_module(&self, module: DefinedModule) {
        let mut modules = self.modules.write();
        modules.insert(module.name.clone(), module);
    }

    /// Gets a defined module by name.
    #[must_use]
    pub fn get_module(&self, name: &str) -> Option<DefinedModule> {
        let modules = self.modules.read();
        modules.get(name).cloned()
    }

    /// Checks if a module is defined and is open.
    #[must_use]
    pub fn is_module_open(&self, name: &str) -> bool {
        let modules = self.modules.read();
        modules.get(name).is_some_and(|m| m.is_open)
    }

    /// Returns all dynamic exports for merging with static configuration.
    #[must_use]
    pub fn get_all_exports(&self) -> HashMap<String, HashMap<String, HashSet<String>>> {
        self.exports.read().clone()
    }

    /// Returns all dynamic opens for merging with static configuration.
    #[must_use]
    pub fn get_all_opens(&self) -> HashMap<String, HashMap<String, HashSet<String>>> {
        self.opens.read().clone()
    }

    /// Returns all dynamic reads for merging with static configuration.
    #[must_use]
    pub fn get_all_reads(&self) -> HashMap<String, HashSet<String>> {
        self.reads.read().clone()
    }

    /// Gets the boot class loader's unnamed module.
    ///
    /// Returns `None` if the unnamed module has not been set yet (i.e., before
    /// `BootLoader.setBootLoaderUnnamedModule0` is called during JVM initialization).
    #[must_use]
    pub fn boot_unnamed_module(&self) -> Option<Value> {
        let guard = self.boot_unnamed_module.read();
        guard.clone()
    }
    /// Sets the boot class loader's unnamed module.
    ///
    /// This is called by `BootLoader.setBootLoaderUnnamedModule0` during JVM initialization.
    /// The unnamed module is used as the default module for classes that don't have
    /// an explicit module assignment.
    pub fn set_boot_unnamed_module(&self, module: Value) {
        let mut guard = self.boot_unnamed_module.write();
        *guard = Some(module);
    }

    /// Checks if `from_module` can access `package` in `to_module` based on **dynamic state only**.
    ///
    /// This checks only runtime modifications (via `Module.addExports0()`, etc.).
    /// For complete JPMS access checking, the VM should also check the static
    /// `ResolvedConfiguration` from the classloader.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `ALL_UNNAMED` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `package` - The package containing the target type (in internal format, e.g., "java/lang")
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating whether access is allowed by dynamic state.
    #[must_use]
    pub fn check_dynamic_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> AccessCheckResult {
        // Same module always has access
        if from_module == to_module {
            return AccessCheckResult::Allowed;
        }

        // Unnamed module (classpath code) has special rules
        if from_module == ALL_UNNAMED {
            return self.check_unnamed_module_access(to_module, package);
        }

        // Check readability first
        if !self.can_read(from_module, to_module) {
            return AccessCheckResult::NotReadable;
        }

        // Check if the package is exported
        if !self.is_exported(to_module, package, from_module) {
            return AccessCheckResult::NotExported;
        }

        AccessCheckResult::Allowed
    }

    /// Checks if `from_module` can reflect on `package` in `to_module` based on **dynamic state only**.
    ///
    /// This checks only runtime modifications (via `Module.addOpens0()`, etc.).
    /// For complete JPMS reflection checking, the VM should also check the static
    /// `ResolvedConfiguration` from the classloader.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The module requesting access (use `ALL_UNNAMED` for classpath code)
    /// * `to_module` - The module containing the target type
    /// * `package` - The package containing the target type (in internal format)
    ///
    /// # Returns
    ///
    /// An `AccessCheckResult` indicating whether reflection access is allowed by dynamic state.
    #[must_use]
    pub fn check_dynamic_reflection_access(
        &self,
        from_module: &str,
        to_module: &str,
        package: &str,
    ) -> AccessCheckResult {
        // Same module always has access
        if from_module == to_module {
            return AccessCheckResult::Allowed;
        }

        // Unnamed module has special rules
        if from_module == ALL_UNNAMED {
            return self.check_unnamed_module_reflection(to_module, package);
        }

        // Check readability first
        if !self.can_read(from_module, to_module) {
            return AccessCheckResult::NotReadable;
        }

        // Check if the package is opened
        if !self.is_opened(to_module, package, from_module) {
            return AccessCheckResult::NotOpened;
        }

        AccessCheckResult::Allowed
    }

    /// Checks access for code in the unnamed module (classpath).
    ///
    /// The unnamed module can read all named modules, so we only check exports.
    fn check_unnamed_module_access(&self, to_module: &str, package: &str) -> AccessCheckResult {
        // Check if the package is exported (to ALL-UNNAMED or to ALL)
        if self.is_exported(to_module, package, ALL_UNNAMED) {
            return AccessCheckResult::Allowed;
        }

        // Check if the module has opened the package to all
        if self.is_opened(to_module, package, ALL_UNNAMED) {
            return AccessCheckResult::Allowed;
        }

        AccessCheckResult::NotExported
    }

    /// Checks reflection access for code in the unnamed module (classpath).
    fn check_unnamed_module_reflection(&self, to_module: &str, package: &str) -> AccessCheckResult {
        // Check if the package is opened (to ALL-UNNAMED or to ALL)
        if self.is_opened(to_module, package, ALL_UNNAMED) {
            return AccessCheckResult::Allowed;
        }

        AccessCheckResult::NotOpened
    }

    /// Extracts the package name from a fully qualified class name.
    ///
    /// # Examples
    ///
    /// - `java/lang/String` → `java/lang`
    /// - `com/example/MyClass` → `com/example`
    /// - `MyClass` → empty string (default package)
    #[must_use]
    pub fn package_from_class_name(class_name: &str) -> &str {
        if let Some(last_slash) = class_name.rfind('/') {
            &class_name[..last_slash]
        } else {
            "" // Default package
        }
    }

    /// Returns an `IllegalAccessError` message for a denied access check.
    #[must_use]
    pub fn illegal_access_error(
        from_module: &str,
        to_module: &str,
        class_name: &str,
        result: AccessCheckResult,
    ) -> String {
        let package = Self::package_from_class_name(class_name);
        let class_display = class_name.replace('/', ".");
        let from_display = if from_module == ALL_UNNAMED {
            "unnamed module".to_string()
        } else {
            format!("module {from_module}")
        };

        match result {
            AccessCheckResult::Allowed => {
                format!("access to {class_display} allowed") // Should not happen
            }
            AccessCheckResult::NotReadable => {
                format!(
                    "{from_display} cannot access class {class_display} \
                     (in module {to_module}) because {from_display} does not read module {to_module}"
                )
            }
            AccessCheckResult::NotExported => {
                format!(
                    "{from_display} cannot access class {class_display} \
                     (in module {to_module}) because module {to_module} does not export {package} to {from_display}"
                )
            }
            AccessCheckResult::NotOpened => {
                format!(
                    "{from_display} cannot reflectively access class {class_display} \
                     (in module {to_module}) because module {to_module} does not open {package} to {from_display}"
                )
            }
        }
    }

    /// Requires module access from one module to a class in another module.
    ///
    /// This is a convenience method that combines `check_access` with error generation.
    /// It throws `IllegalAccessError` if access is denied.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The name of the module requesting access (None for unnamed/classpath)
    /// * `to_module` - The name of the module containing the target class (None for unnamed/classpath)
    /// * `to_class_name` - The fully qualified name of the target class (in internal format)
    ///
    /// # Errors
    ///
    /// Returns an `IllegalAccessError` if access is denied.
    pub fn require_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> Result<()> {
        let result = self.check_access(from_module, to_module, to_class_name);
        if result.is_allowed() {
            return Ok(());
        }

        let from = from_module.unwrap_or(ALL_UNNAMED);
        let to = to_module.unwrap_or(ALL_UNNAMED);
        let error_msg = Self::illegal_access_error(from, to, to_class_name, result);
        Err(IllegalAccessError(error_msg).into())
    }

    /// Requires reflection access from one module to a class in another module.
    ///
    /// This is a convenience method that combines `check_reflection_access` with error generation.
    /// It throws `InaccessibleObjectException` if reflection access is denied.
    ///
    /// # Arguments
    ///
    /// * `from_module` - The name of the module requesting access (None for unnamed/classpath)
    /// * `to_module` - The name of the module containing the target class (None for unnamed/classpath)
    /// * `to_class_name` - The fully qualified name of the target class (in internal format)
    ///
    /// # Errors
    ///
    /// Returns an `InaccessibleObjectException` if reflection access is denied.
    pub fn require_reflection_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> Result<()> {
        let result = self.check_reflection_access(from_module, to_module, to_class_name);
        if result.is_allowed() {
            return Ok(());
        }

        let from = from_module.unwrap_or(ALL_UNNAMED);
        let to = to_module.unwrap_or(ALL_UNNAMED);
        let error_msg = Self::illegal_access_error(from, to, to_class_name, result);
        Err(InaccessibleObjectException(error_msg).into())
    }
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModuleRead;
    use crate::configuration::{ConfigurationBuilder, ModuleExport, ModuleOpens};

    #[test]
    fn test_new_module_system() {
        let module_system = ModuleSystem::empty();
        assert!(!module_system.is_exported("java.base", "java/lang", "my.module"));
        assert!(!module_system.is_opened("java.base", "java/lang", "my.module"));
        assert!(!module_system.can_read("my.module", "java.sql"));
    }

    #[test]
    fn test_add_export() {
        let module_system = ModuleSystem::empty();
        module_system.add_export("java.base", "java/lang", Some("my.module"));

        assert!(module_system.is_exported("java.base", "java/lang", "my.module"));
        assert!(!module_system.is_exported("java.base", "java/lang", "other.module"));
    }

    #[test]
    fn test_add_export_to_all() {
        let module_system = ModuleSystem::empty();
        module_system.add_export_to_all("java.base", "java/lang");

        // Should be exported to any module
        assert!(module_system.is_exported("java.base", "java/lang", "my.module"));
        assert!(module_system.is_exported("java.base", "java/lang", "other.module"));
    }

    #[test]
    fn test_add_export_to_all_unnamed() {
        let module_system = ModuleSystem::empty();
        module_system.add_export_to_all_unnamed("java.base", "java/lang");

        assert!(module_system.is_exported("java.base", "java/lang", ALL_UNNAMED));
        // Other modules should not have access
        assert!(!module_system.is_exported("java.base", "java/lang", "my.module"));
    }

    #[test]
    fn test_add_opens() {
        let module_system = ModuleSystem::empty();
        module_system.add_opens("java.base", "java/lang", Some("my.module"));

        assert!(module_system.is_opened("java.base", "java/lang", "my.module"));
        assert!(!module_system.is_opened("java.base", "java/lang", "other.module"));
    }

    #[test]
    fn test_add_opens_to_all() {
        let module_system = ModuleSystem::empty();
        module_system.add_opens_to_all("java.base", "java/lang");

        assert!(module_system.is_opened("java.base", "java/lang", "my.module"));
        assert!(module_system.is_opened("java.base", "java/lang", "other.module"));
    }

    #[test]
    fn test_add_opens_to_all_unnamed() {
        let module_system = ModuleSystem::empty();
        module_system.add_opens_to_all_unnamed("java.base", "java/lang");

        assert!(module_system.is_opened("java.base", "java/lang", ALL_UNNAMED));
        assert!(!module_system.is_opened("java.base", "java/lang", "my.module"));
    }

    #[test]
    fn test_open_module() {
        let module_system = ModuleSystem::empty();
        let mut open_module = DefinedModule::new("my.open.module".to_string(), true);
        open_module.packages.insert("my/pkg".to_string());
        module_system.define_module(open_module);

        // Open module should have all packages opened
        assert!(module_system.is_opened("my.open.module", "my/pkg", "any.module"));
        assert!(module_system.is_opened("my.open.module", "other/pkg", "any.module"));
    }

    #[test]
    fn test_add_read() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "java.sql");

        assert!(module_system.can_read("my.module", "java.sql"));
        assert!(!module_system.can_read("my.module", "java.xml"));
    }

    #[test]
    fn test_can_read_same_module() {
        let module_system = ModuleSystem::empty();
        // Same module can always read itself
        assert!(module_system.can_read("my.module", "my.module"));
    }

    #[test]
    fn test_can_read_java_base() {
        let module_system = ModuleSystem::empty();
        // java.base is implicitly readable by all
        assert!(module_system.can_read("my.module", "java.base"));
    }

    #[test]
    fn test_define_module() {
        let module_system = ModuleSystem::empty();
        let mut defined_module = DefinedModule::new("my.module".to_string(), false);
        defined_module.version = Some("1.0".to_string());
        defined_module.location = Some("/path/to/module.jar".to_string());
        defined_module.packages.insert("my/pkg".to_string());
        module_system.define_module(defined_module);

        let retrieved = module_system.get_module("my.module");
        assert!(retrieved.is_some());
        let retrieved_module = retrieved.unwrap();
        assert_eq!(retrieved_module.name, "my.module");
        assert!(!retrieved_module.is_open);
        assert_eq!(retrieved_module.version, Some("1.0".to_string()));
        assert!(retrieved_module.packages.contains("my/pkg"));
    }

    #[test]
    fn test_get_module_not_found() {
        let module_system = ModuleSystem::empty();
        assert!(module_system.get_module("nonexistent").is_none());
    }

    #[test]
    fn test_get_all_exports() {
        let module_system = ModuleSystem::empty();
        module_system.add_export("java.base", "java/lang", Some("my.module"));
        module_system.add_export("java.sql", "java/sql", Some("my.module"));

        let exports = module_system.get_all_exports();
        assert_eq!(exports.len(), 2);
        assert!(exports.contains_key("java.base"));
        assert!(exports.contains_key("java.sql"));
    }

    #[test]
    fn test_get_all_opens() {
        let module_system = ModuleSystem::empty();
        module_system.add_opens("java.base", "java/lang", Some("my.module"));

        let opens = module_system.get_all_opens();
        assert_eq!(opens.len(), 1);
    }

    #[test]
    fn test_get_all_reads() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "java.sql");
        module_system.add_read("my.module", "java.xml");

        let reads = module_system.get_all_reads();
        assert_eq!(reads.len(), 1);
        assert!(reads.get("my.module").unwrap().contains("java.sql"));
        assert!(reads.get("my.module").unwrap().contains("java.xml"));
    }

    #[test]
    fn test_default() {
        let module_system = ModuleSystem::default();
        assert!(module_system.get_all_exports().is_empty());
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
        let denial_message = result.denial_reason();
        assert!(denial_message.is_some());
        assert!(denial_message.unwrap().contains("does not read"));
        assert_eq!(format!("{result}"), "module not readable");
    }

    #[test]
    fn test_access_check_result_not_exported() {
        let result = AccessCheckResult::NotExported;
        assert!(!result.is_allowed());
        assert!(result.is_denied());
        let denial_message = result.denial_reason();
        assert!(denial_message.is_some());
        assert!(denial_message.unwrap().contains("not exported"));
        assert_eq!(format!("{result}"), "package not exported");
    }

    #[test]
    fn test_access_check_result_not_opened() {
        let result = AccessCheckResult::NotOpened;
        assert!(!result.is_allowed());
        assert!(result.is_denied());
        let denial_message = result.denial_reason();
        assert!(denial_message.is_some());
        assert!(denial_message.unwrap().contains("not opened"));
        assert_eq!(format!("{result}"), "package not opened for reflection");
    }

    // ==================== check_dynamic_access tests ====================

    #[test]
    fn test_check_dynamic_access_same_module() {
        let module_system = ModuleSystem::empty();
        // Same module access is always allowed
        let result = module_system.check_dynamic_access("my.module", "my.module", "my/pkg");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_access_not_readable() {
        let module_system = ModuleSystem::empty();
        // Module doesn't read the other module
        let result = module_system.check_dynamic_access("my.module", "other.module", "other/pkg");
        assert_eq!(result, AccessCheckResult::NotReadable);
    }

    #[test]
    fn test_check_dynamic_access_reads_but_not_exported() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        // Module reads but package is not exported
        let result =
            module_system.check_dynamic_access("my.module", "other.module", "other/internal");
        assert_eq!(result, AccessCheckResult::NotExported);
    }

    #[test]
    fn test_check_dynamic_access_allowed_with_export() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export("other.module", "other/api", Some("my.module"));
        // Module reads and package is exported
        let result = module_system.check_dynamic_access("my.module", "other.module", "other/api");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_access_java_base_always_readable() {
        let module_system = ModuleSystem::empty();
        module_system.add_export_to_all("java.base", "java/lang");
        // java.base is implicitly readable by all modules
        let result = module_system.check_dynamic_access("my.module", "java.base", "java/lang");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_access_unnamed_module_exported() {
        let module_system = ModuleSystem::empty();
        module_system.add_export("java.base", "java/lang", Some(ALL_UNNAMED));
        // Unnamed module can access if package is exported to ALL-UNNAMED
        let result = module_system.check_dynamic_access(ALL_UNNAMED, "java.base", "java/lang");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_access_unnamed_module_not_exported() {
        let module_system = ModuleSystem::empty();
        // Unnamed module cannot access non-exported packages
        let result =
            module_system.check_dynamic_access(ALL_UNNAMED, "java.base", "java/lang/internal");
        assert_eq!(result, AccessCheckResult::NotExported);
    }

    // ==================== check_dynamic_reflection_access tests ====================

    #[test]
    fn test_check_dynamic_reflection_access_same_module() {
        let module_system = ModuleSystem::empty();
        // Same module reflection is always allowed
        let result =
            module_system.check_dynamic_reflection_access("my.module", "my.module", "my/pkg");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_reflection_access_not_readable() {
        let module_system = ModuleSystem::empty();
        // Module doesn't read the other module
        let result =
            module_system.check_dynamic_reflection_access("my.module", "other.module", "other/pkg");
        assert_eq!(result, AccessCheckResult::NotReadable);
    }

    #[test]
    fn test_check_dynamic_reflection_access_not_opened() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export("other.module", "other/api", Some("my.module"));
        // Module reads and package is exported, but not opened
        let result =
            module_system.check_dynamic_reflection_access("my.module", "other.module", "other/api");
        assert_eq!(result, AccessCheckResult::NotOpened);
    }

    #[test]
    fn test_check_dynamic_reflection_access_allowed_with_opens() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_opens("other.module", "other/internal", Some("my.module"));
        // Module reads and package is opened
        let result = module_system.check_dynamic_reflection_access(
            "my.module",
            "other.module",
            "other/internal",
        );
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_dynamic_reflection_access_open_module() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "open.module");
        let open_module = DefinedModule::new("open.module".to_string(), true);
        module_system.define_module(open_module);
        // Open module has all packages opened
        let result =
            module_system.check_dynamic_reflection_access("my.module", "open.module", "any/pkg");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_reflection_unnamed_module_opened() {
        let module_system = ModuleSystem::empty();
        module_system.add_opens("java.base", "java/lang", Some(ALL_UNNAMED));
        // Unnamed module can reflect if package is opened to ALL-UNNAMED
        let result =
            module_system.check_dynamic_reflection_access(ALL_UNNAMED, "java.base", "java/lang");
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_check_reflection_unnamed_module_not_opened() {
        let module_system = ModuleSystem::empty();
        // Unnamed module cannot reflect on non-opened packages
        let result =
            module_system.check_dynamic_reflection_access(ALL_UNNAMED, "java.base", "java/lang");
        assert_eq!(result, AccessCheckResult::NotOpened);
    }

    // ==================== Utility function tests ====================

    #[test]
    fn test_package_from_class_name() {
        assert_eq!(
            ModuleSystem::package_from_class_name("java/lang/String"),
            "java/lang"
        );
        assert_eq!(
            ModuleSystem::package_from_class_name("com/example/MyClass"),
            "com/example"
        );
        assert_eq!(ModuleSystem::package_from_class_name("MyClass"), "");
        assert_eq!(ModuleSystem::package_from_class_name("a/b/c/D"), "a/b/c");
    }

    #[test]
    fn test_illegal_access_error_not_readable() {
        let error_message = ModuleSystem::illegal_access_error(
            "my.module",
            "other.module",
            "other/internal/Secret",
            AccessCheckResult::NotReadable,
        );
        assert!(error_message.contains("my.module"));
        assert!(error_message.contains("other.module"));
        assert!(error_message.contains("does not read"));
    }

    #[test]
    fn test_illegal_access_error_not_exported() {
        let error_message = ModuleSystem::illegal_access_error(
            "my.module",
            "other.module",
            "other/internal/Secret",
            AccessCheckResult::NotExported,
        );
        assert!(error_message.contains("my.module"));
        assert!(error_message.contains("other.module"));
        assert!(error_message.contains("does not export"));
    }

    #[test]
    fn test_illegal_access_error_not_opened() {
        let error_message = ModuleSystem::illegal_access_error(
            "my.module",
            "other.module",
            "other/internal/Secret",
            AccessCheckResult::NotOpened,
        );
        assert!(error_message.contains("reflectively"));
        assert!(error_message.contains("does not open"));
    }

    #[test]
    fn test_illegal_access_error_unnamed_module() {
        let error_message = ModuleSystem::illegal_access_error(
            ALL_UNNAMED,
            "java.base",
            "java/lang/internal/Secret",
            AccessCheckResult::NotExported,
        );
        assert!(error_message.contains("unnamed module"));
        assert!(error_message.contains("java.base"));
    }

    // ==================== ModuleSystem::new async tests ====================

    #[tokio::test]
    async fn test_module_system_new_with_add_exports() {
        let configuration = ConfigurationBuilder::new()
            .add_export(ModuleExport::new("java.base", "java.lang", "my.module"))
            .add_export(ModuleExport::new("java.sql", "java.sql", "ALL-UNNAMED"))
            .build()
            .unwrap();

        let module_system = ModuleSystem::new(&configuration, &std::path::PathBuf::new(), 21).await;
        assert!(module_system.is_ok());

        let module_system = module_system.unwrap();

        // Verify the resolved configuration contains the add_exports
        let add_exports = module_system.resolved_configuration().add_exports();
        assert!(add_exports.contains_key("java.base"));
        assert!(add_exports.contains_key("java.sql"));

        // Verify specific export targets
        let java_base_exports = add_exports.get("java.base").unwrap();
        assert!(
            java_base_exports
                .get("java.lang")
                .unwrap()
                .contains("my.module")
        );

        let java_sql_exports = add_exports.get("java.sql").unwrap();
        assert!(
            java_sql_exports
                .get("java.sql")
                .unwrap()
                .contains("ALL-UNNAMED")
        );
    }

    #[tokio::test]
    async fn test_module_system_new_with_add_opens() {
        let configuration = ConfigurationBuilder::new()
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "my.module",
            ))
            .add_opens(ModuleOpens::new("java.base", "sun.reflect", "ALL-UNNAMED"))
            .build()
            .unwrap();

        let module_system = ModuleSystem::new(&configuration, &std::path::PathBuf::new(), 21).await;
        assert!(module_system.is_ok());

        let module_system = module_system.unwrap();

        // Verify the resolved configuration contains the add_opens
        let add_opens = module_system.resolved_configuration().add_opens();
        assert!(add_opens.contains_key("java.base"));

        // Verify specific opens targets
        let java_base_opens = add_opens.get("java.base").unwrap();
        assert!(
            java_base_opens
                .get("java.lang.reflect")
                .unwrap()
                .contains("my.module")
        );
        assert!(
            java_base_opens
                .get("sun.reflect")
                .unwrap()
                .contains("ALL-UNNAMED")
        );
    }

    #[tokio::test]
    async fn test_module_system_new_java8_fallback() {
        let configuration = ConfigurationBuilder::new().build().unwrap();

        // Java 8 should use fallback configuration
        let module_system = ModuleSystem::new(&configuration, &std::path::PathBuf::new(), 8).await;
        assert!(module_system.is_ok());

        let module_system = module_system.unwrap();
        // Fallback configuration should be empty (no resolved modules)
        assert!(module_system.resolved_configuration().is_empty());
    }

    #[tokio::test]
    async fn test_module_system_check_access() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export("other.module", "other/api", Some("my.module"));

        // Should allow access when reads + exports are set
        let result = module_system.check_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/SomeClass",
        );
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[tokio::test]
    async fn test_module_system_check_reflection_access() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_opens("other.module", "other/internal", Some("my.module"));

        // Should allow reflection access when reads + opens are set
        let result = module_system.check_reflection_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/Secret",
        );
        assert_eq!(result, AccessCheckResult::Allowed);
    }

    #[test]
    fn test_resolved_configuration_getter() {
        let module_system = ModuleSystem::empty();
        let config = module_system.resolved_configuration();
        assert!(config.is_empty());
    }

    #[tokio::test]
    async fn test_create_module_system_with_add_reads() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .add_read(ModuleRead::new("my.module", "java.sql"))
            .build()?;

        let module_system = ModuleSystem::new(&configuration, &PathBuf::new(), 21).await?;
        assert!(module_system.can_read("my.module", "java.sql"));
        assert!(!module_system.can_read("my.module", "java.xml"));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_module_system_with_add_exports() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .add_export(ModuleExport::new("java.base", "java.lang", "my.module"))
            .build()?;

        let module_system = ModuleSystem::new(&configuration, &PathBuf::new(), 21).await?;
        assert!(module_system.is_exported("java.base", "java.lang", "my.module"));
        assert!(!module_system.is_exported("java.base", "java.lang", "other.module"));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_module_system_with_add_opens() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "my.module",
            ))
            .build()?;

        let module_system = ModuleSystem::new(&configuration, &PathBuf::new(), 21).await?;
        assert!(module_system.is_opened("java.base", "java.lang.reflect", "my.module"));
        assert!(!module_system.is_opened("java.base", "java.lang.reflect", "other.module"));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_module_system_combined() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .add_read(ModuleRead::new("my.module", "java.sql"))
            .add_read(ModuleRead::new("my.module", "java.xml"))
            .add_export(ModuleExport::new("java.base", "java.lang", "ALL-UNNAMED"))
            .add_opens(ModuleOpens::new(
                "java.base",
                "java.lang.reflect",
                "ALL-UNNAMED",
            ))
            .build()?;

        let module_system = ModuleSystem::new(&configuration, &PathBuf::new(), 21).await?;

        // Check reads
        assert!(module_system.can_read("my.module", "java.sql"));
        assert!(module_system.can_read("my.module", "java.xml"));

        // Check exports
        assert!(module_system.is_exported("java.base", "java.lang", "ALL-UNNAMED"));

        // Check opens
        assert!(module_system.is_opened("java.base", "java.lang.reflect", "ALL-UNNAMED"));

        Ok(())
    }

    // ==================== require_access tests ====================

    #[test]
    fn test_require_access_same_module_success() {
        let module_system = ModuleSystem::empty();
        // Same module access is always allowed
        let result =
            module_system.require_access(Some("my.module"), Some("my.module"), "my/pkg/MyClass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_with_export_success() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export("other.module", "other/api", Some("my.module"));

        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/SomeClass",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_not_readable_failure() {
        let module_system = ModuleSystem::empty();
        // Module doesn't read the other module - should fail
        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/pkg/SomeClass",
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("does not read"));
    }

    #[test]
    fn test_require_access_not_exported_failure() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        // Package is not exported - should fail
        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/Secret",
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("does not export"));
    }

    #[test]
    fn test_require_access_unnamed_to_named_module_success() {
        let module_system = ModuleSystem::empty();
        module_system.add_export_to_all_unnamed("java.base", "java/lang");

        // Unnamed module accessing exported package
        let result = module_system.require_access(None, Some("java.base"), "java/lang/String");
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_unnamed_to_named_module_failure() {
        let module_system = ModuleSystem::empty();
        // Package not exported to unnamed
        let result =
            module_system.require_access(None, Some("java.base"), "java/lang/internal/Secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_require_access_both_unnamed_success() {
        let module_system = ModuleSystem::empty();
        // Both modules are unnamed - should always succeed
        let result = module_system.require_access(None, None, "com/example/MyClass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_to_java_base_success() {
        let module_system = ModuleSystem::empty();
        module_system.add_export_to_all("java.base", "java/lang");

        // java.base is implicitly readable by all modules
        let result =
            module_system.require_access(Some("my.module"), Some("java.base"), "java/lang/String");
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_export_to_all_success() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export_to_all("other.module", "other/public");

        // Export to ALL should allow any module
        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/public/PublicClass",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_wrong_target_module_failure() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        // Export only to different.module, not my.module
        module_system.add_export("other.module", "other/api", Some("different.module"));

        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/SomeClass",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_require_access_multiple_packages_partial_export() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export("other.module", "other/api", Some("my.module"));
        // other/internal is NOT exported

        // Should succeed for exported package
        let result1 = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/PublicClass",
        );
        assert!(result1.is_ok());

        // Should fail for non-exported package
        let result2 = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/InternalClass",
        );
        assert!(result2.is_err());
    }

    #[test]
    fn test_require_access_default_package_class() {
        let module_system = ModuleSystem::empty();
        // Class in default package (no slash in name)
        let result = module_system.require_access(None, None, "DefaultPackageClass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_deeply_nested_package() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        module_system.add_export(
            "other.module",
            "com/example/deep/nested/api",
            Some("my.module"),
        );

        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "com/example/deep/nested/api/DeepClass",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_access_error_message_contains_module_names() {
        let module_system = ModuleSystem::empty();
        let result = module_system.require_access(
            Some("requesting.module"),
            Some("target.module"),
            "target/pkg/TargetClass",
        );
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("requesting.module"));
        assert!(error_msg.contains("target.module"));
    }

    #[test]
    fn test_require_access_error_message_contains_class_name() {
        let module_system = ModuleSystem::empty();
        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "com/example/MyClass",
        );
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Class name should be in dot format in error message
        assert!(error_msg.contains("com.example.MyClass"));
    }

    #[test]
    fn test_require_access_opened_package_requires_export_for_regular_access() {
        let module_system = ModuleSystem::empty();
        module_system.add_read("my.module", "other.module");
        // Only opened, not explicitly exported
        module_system.add_opens("other.module", "other/internal", Some("my.module"));

        // Opens does NOT imply export for regular (non-reflection) access
        // Regular access requires explicit export
        let result = module_system.require_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/InternalClass",
        );
        // This should fail because opened packages are only for reflection access
        assert!(result.is_err());
    }

    #[test]
    fn test_require_access_unnamed_module_can_access_opened_package() {
        let module_system = ModuleSystem::empty();
        // Open package to all unnamed
        module_system.add_opens_to_all_unnamed("java.base", "java/lang/internal");

        // Unnamed module accessing opened package - this works because
        // check_unnamed_module_access checks both exports AND opens
        let result =
            module_system.require_access(None, Some("java.base"), "java/lang/internal/Unsafe");
        assert!(result.is_ok());
    }
}

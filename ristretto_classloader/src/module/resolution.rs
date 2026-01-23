//! Module resolution implementation.

use crate::module::descriptor::ModuleDescriptor;
use crate::module::error::{ModuleError, Result};
use crate::module::finder::ModuleFinder;
use crate::module::reference::ModuleReference;
use ahash::{AHashMap, AHashSet};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// A resolved module in a configuration.
#[derive(Clone, Debug)]
pub struct ResolvedModule {
    /// The module reference.
    reference: ModuleReference,
    /// Modules that this module reads (direct readability).
    reads: BTreeSet<String>,
}

impl ResolvedModule {
    /// Creates a new resolved module.
    #[must_use]
    pub fn new(reference: ModuleReference) -> Self {
        Self {
            reference,
            reads: BTreeSet::new(),
        }
    }

    /// Returns the module reference.
    #[must_use]
    pub fn reference(&self) -> &ModuleReference {
        &self.reference
    }

    /// Returns the module name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.reference.name()
    }

    /// Returns the module descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ModuleDescriptor {
        self.reference.descriptor()
    }

    /// Returns the set of modules this module reads.
    #[must_use]
    pub fn reads(&self) -> &BTreeSet<String> {
        &self.reads
    }

    /// Returns true if this module reads the given module.
    #[must_use]
    pub fn reads_module(&self, module: &str) -> bool {
        self.reads.contains(module)
    }

    /// Adds a module to the reads set.
    pub fn add_read(&mut self, module: String) {
        self.reads.insert(module);
    }
}

/// Module resolver that builds a configuration from root modules.
#[derive(Debug)]
pub struct Resolver {
    /// Additional reads to add (from --add-reads).
    add_reads: AHashMap<String, AHashSet<String>>,
    /// Additional exports to add (from --add-exports).
    add_exports: AHashMap<String, AHashMap<String, AHashSet<String>>>,
    /// Additional opens to add (from --add-opens).
    add_opens: AHashMap<String, AHashMap<String, AHashSet<String>>>,
    /// Modules to limit (from --limit-modules).
    limit_modules: Option<AHashSet<String>>,
}

impl Resolver {
    /// Creates a new resolver with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            add_reads: AHashMap::default(),
            add_exports: AHashMap::default(),
            add_opens: AHashMap::default(),
            limit_modules: None,
        }
    }

    /// Adds a read edge (from --add-reads SOURCE=TARGET).
    pub fn add_read(&mut self, source: String, target: String) {
        self.add_reads.entry(source).or_default().insert(target);
    }

    /// Adds an export override (from --add-exports SOURCE/PACKAGE=TARGET).
    pub fn add_export(&mut self, source: String, package: String, target: String) {
        self.add_exports
            .entry(source)
            .or_default()
            .entry(package)
            .or_default()
            .insert(target);
    }

    /// Adds an opens override (from --add-opens SOURCE/PACKAGE=TARGET).
    pub fn add_opens(&mut self, source: String, package: String, target: String) {
        self.add_opens
            .entry(source)
            .or_default()
            .entry(package)
            .or_default()
            .insert(target);
    }

    /// Sets the limit-modules set.
    pub fn set_limit_modules(&mut self, modules: AHashSet<String>) {
        self.limit_modules = Some(modules);
    }

    /// Resolves a configuration from root modules using the given finder.
    ///
    /// # Errors
    ///
    /// Returns an error if resolution fails.
    pub fn resolve(
        &self,
        root_modules: &[String],
        finder: &dyn ModuleFinder,
    ) -> Result<ResolvedConfiguration> {
        let mut resolved: BTreeMap<String, ResolvedModule> = BTreeMap::new();
        let mut package_to_module: BTreeMap<String, String> = BTreeMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut visited: AHashSet<String> = AHashSet::default();

        // Seed with root modules
        for root in root_modules {
            if !visited.contains(root) {
                queue.push_back(root.clone());
                visited.insert(root.clone());
            }
        }

        // Always include java.base if not already included
        if !visited.contains("java.base") {
            queue.push_front("java.base".to_string());
            visited.insert("java.base".to_string());
        }

        // BFS resolution
        while let Some(module_name) = queue.pop_front() {
            // Check limit-modules
            if let Some(ref limit) = self.limit_modules
                && !limit.contains(&module_name)
            {
                continue;
            }

            let reference = finder
                .find(&module_name)
                .ok_or_else(|| ModuleError::ModuleNotFound(module_name.clone()))?;

            let descriptor = reference.descriptor().clone();
            let is_automatic = reference.is_automatic();

            // Check for split packages
            for package in &descriptor.packages {
                if let Some(existing_module) = package_to_module.get(package) {
                    if existing_module != &module_name {
                        return Err(ModuleError::SplitPackage {
                            package: package.clone(),
                            module1: existing_module.clone(),
                            module2: module_name.clone(),
                        });
                    }
                } else {
                    package_to_module.insert(package.clone(), module_name.clone());
                }
            }

            let mut resolved_module = ResolvedModule::new(reference);

            // Process requires
            for req in &descriptor.requires {
                // Add to reads
                resolved_module.add_read(req.name.clone());

                // Enqueue if not visited (unless static and not present)
                if !visited.contains(&req.name) {
                    if req.is_static() {
                        // Static requires are optional - only add if the module exists
                        if finder.find(&req.name).is_some() {
                            queue.push_back(req.name.clone());
                            visited.insert(req.name.clone());
                        }
                    } else {
                        queue.push_back(req.name.clone());
                        visited.insert(req.name.clone());
                    }
                }
            }

            // All modules implicitly read java.base
            if module_name != "java.base" {
                resolved_module.add_read("java.base".to_string());
            }

            // Automatic modules read all other modules
            if is_automatic {
                // Will be populated after resolution
            }

            resolved.insert(module_name, resolved_module);
        }

        // Second pass: compute transitive readability and automatic module reads
        Self::compute_transitive_reads(&mut resolved);

        // Apply --add-reads
        for (source, targets) in &self.add_reads {
            if let Some(module) = resolved.get_mut(source) {
                for target in targets {
                    if target == "ALL-UNNAMED" {
                        // Special handling for unnamed module
                        module.add_read("ALL-UNNAMED".to_string());
                    } else {
                        module.add_read(target.clone());
                    }
                }
            }
        }

        // Handle automatic modules - they read all resolved named modules
        let all_module_names: Vec<String> = resolved.keys().cloned().collect();
        for (name, module) in &mut resolved {
            if module.reference.is_automatic() {
                for other_name in &all_module_names {
                    if other_name != name {
                        module.add_read(other_name.clone());
                    }
                }
            }
        }

        Ok(ResolvedConfiguration {
            resolved,
            package_to_module,
            add_exports: self.add_exports.clone(),
            add_opens: self.add_opens.clone(),
        })
    }

    /// Computes transitive readability (requires transitive).
    fn compute_transitive_reads(resolved: &mut BTreeMap<String, ResolvedModule>) {
        // Build a map of which modules export transitively to which
        let mut transitive_exports: AHashMap<String, Vec<String>> = AHashMap::default();

        for (name, module) in resolved.iter() {
            for req in &module.descriptor().requires {
                if req.is_transitive() {
                    transitive_exports
                        .entry(name.clone())
                        .or_default()
                        .push(req.name.clone());
                }
            }
        }

        // For each module, compute the transitive closure of reads
        let module_names: Vec<String> = resolved.keys().cloned().collect();

        for name in &module_names {
            let reads_clone: Vec<String> = {
                let module = &resolved[name];
                module.reads.iter().cloned().collect()
            };

            let mut additional_reads = AHashSet::default();

            for read in &reads_clone {
                // If the read module transitively requires something, we also read it
                Self::collect_transitive_reads(
                    read,
                    &transitive_exports,
                    &mut additional_reads,
                    &mut AHashSet::default(),
                );
            }

            if let Some(module) = resolved.get_mut(name) {
                for read in additional_reads {
                    module.add_read(read);
                }
            }
        }
    }

    fn collect_transitive_reads(
        module_name: &str,
        transitive_exports: &AHashMap<String, Vec<String>>,
        result: &mut AHashSet<String>,
        visited: &mut AHashSet<String>,
    ) {
        if visited.contains(module_name) {
            return;
        }
        visited.insert(module_name.to_string());

        if let Some(transitives) = transitive_exports.get(module_name) {
            for transitive in transitives {
                result.insert(transitive.clone());
                Self::collect_transitive_reads(transitive, transitive_exports, result, visited);
            }
        }
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

/// A resolved module configuration.
#[derive(Clone, Debug)]
pub struct ResolvedConfiguration {
    /// Resolved modules by name.
    pub(crate) resolved: BTreeMap<String, ResolvedModule>,
    /// Package to module mapping.
    pub(crate) package_to_module: BTreeMap<String, String>,
    /// Additional exports (from --add-exports).
    pub(crate) add_exports: AHashMap<String, AHashMap<String, AHashSet<String>>>,
    /// Additional opens (from --add-opens).
    pub(crate) add_opens: AHashMap<String, AHashMap<String, AHashSet<String>>>,
}

impl ResolvedConfiguration {
    /// Creates an empty configuration.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            resolved: BTreeMap::new(),
            package_to_module: BTreeMap::new(),
            add_exports: AHashMap::default(),
            add_opens: AHashMap::default(),
        }
    }

    /// Creates a new configuration with the given resolved modules and maps.
    #[must_use]
    pub fn new(
        resolved: BTreeMap<String, ResolvedModule>,
        package_to_module: BTreeMap<String, String>,
        add_exports: AHashMap<String, AHashMap<String, AHashSet<String>>>,
        add_opens: AHashMap<String, AHashMap<String, AHashSet<String>>>,
    ) -> Self {
        Self {
            resolved,
            package_to_module,
            add_exports,
            add_opens,
        }
    }
}

impl ResolvedConfiguration {
    /// Returns the resolved module by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ResolvedModule> {
        self.resolved.get(name)
    }

    /// Returns all resolved modules.
    pub fn modules(&self) -> impl Iterator<Item = &ResolvedModule> {
        self.resolved.values()
    }

    /// Returns the number of resolved modules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resolved.len()
    }

    /// Returns true if there are no resolved modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resolved.is_empty()
    }

    /// Finds the module containing the given package.
    #[must_use]
    pub fn find_module_for_package(&self, package: &str) -> Option<&str> {
        self.package_to_module.get(package).map(String::as_str)
    }

    /// Returns true if module `from` reads module `to`.
    #[must_use]
    pub fn reads(&self, from: &str, to: &str) -> bool {
        self.resolved.get(from).is_some_and(|m| m.reads_module(to))
    }

    /// Returns true if module `to` exports `package` to module `from`.
    #[must_use]
    pub fn exports(&self, to: &str, package: &str, from: &str) -> bool {
        // Check --add-exports override
        if let Some(module_exports) = self.add_exports.get(to)
            && let Some(targets) = module_exports.get(package)
            && (targets.contains("ALL-UNNAMED") || targets.contains(from))
        {
            return true;
        }

        // Check module descriptor
        if let Some(module) = self.resolved.get(to) {
            let descriptor = module.descriptor();

            // Automatic modules export all packages
            if module.reference().is_automatic() {
                return descriptor.packages.contains(package);
            }

            return descriptor.exports_package(package, Some(from));
        }
        false
    }

    /// Returns true if module `to` opens `package` to module `from`.
    #[must_use]
    pub fn opens(&self, to: &str, package: &str, from: &str) -> bool {
        // Check --add-opens override
        if let Some(module_opens) = self.add_opens.get(to)
            && let Some(targets) = module_opens.get(package)
            && (targets.contains("ALL-UNNAMED") || targets.contains(from))
        {
            return true;
        }

        // Check module descriptor
        if let Some(module) = self.resolved.get(to) {
            let descriptor = module.descriptor();

            // Automatic and open modules open all packages
            if module.reference().is_automatic() || descriptor.is_open() {
                return descriptor.packages.contains(package);
            }

            return descriptor.opens_package(package, Some(from));
        }
        false
    }

    /// Returns the `add_exports` map for use by the access checker.
    #[must_use]
    pub fn add_exports(&self) -> &AHashMap<String, AHashMap<String, AHashSet<String>>> {
        &self.add_exports
    }

    /// Returns the `add_opens` map for use by the access checker.
    #[must_use]
    pub fn add_opens(&self) -> &AHashMap<String, AHashMap<String, AHashSet<String>>> {
        &self.add_opens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::reference::ModuleReference;

    fn create_test_descriptor(name: &str) -> ModuleDescriptor {
        let mut descriptor = ModuleDescriptor::new(name.to_string());
        descriptor
            .packages
            .insert(format!("{}/internal", name.replace('.', "/")));
        descriptor
    }

    #[test]
    fn test_resolved_module() {
        let descriptor = create_test_descriptor("test.module");
        let reference = ModuleReference::system(descriptor);
        let mut resolved = ResolvedModule::new(reference);

        resolved.add_read("java.base".to_string());
        assert!(resolved.reads_module("java.base"));
        assert!(!resolved.reads_module("java.sql"));
    }

    #[test]
    fn test_resolver_new() {
        let resolver = Resolver::new();
        assert!(resolver.add_reads.is_empty());
        assert!(resolver.add_exports.is_empty());
        assert!(resolver.add_opens.is_empty());
        assert!(resolver.limit_modules.is_none());
    }

    #[test]
    fn test_resolver_add_read() {
        let mut resolver = Resolver::new();
        resolver.add_read("my.module".to_string(), "java.sql".to_string());
        assert!(
            resolver
                .add_reads
                .get("my.module")
                .is_some_and(|targets| targets.contains("java.sql"))
        );
    }
}

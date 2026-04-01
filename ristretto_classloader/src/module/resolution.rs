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
            // Check limit-modules (java.base is always included per JEP 261 §2)
            if module_name != "java.base"
                && let Some(ref limit) = self.limit_modules
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
                        // Static requires are optional; only add if the module exists
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

        // Service binding: resolve modules that provide services used by resolved modules.
        // This is recursive; newly resolved modules may use services too.
        self.bind_services(&mut resolved, &mut package_to_module, &mut visited, finder)?;

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

        // Handle automatic modules; they read all resolved named modules
        let resolved_names: AHashSet<String> = resolved.keys().cloned().collect();
        for (name, module) in &mut resolved {
            if module.reference.is_automatic() {
                for other_name in &resolved_names {
                    if other_name != name {
                        module.add_read(other_name.clone());
                    }
                }
            }
        }

        // Prune read edges that reference modules not in the resolved set.
        // This can happen when --limit-modules filters out a required module after
        // the read edge was recorded during BFS traversal. ALL-UNNAMED is a virtual
        // target used by --add-reads and is always retained.
        for module in resolved.values_mut() {
            module
                .reads
                .retain(|read| read == "ALL-UNNAMED" || resolved_names.contains(read));
        }

        Ok(ResolvedConfiguration {
            resolved,
            package_to_module,
            add_exports: self.add_exports.clone(),
            add_opens: self.add_opens.clone(),
        })
    }

    /// Binds service provider modules into the resolution.
    ///
    /// For each resolved module that `uses` a service, finds all observable modules that
    /// `provides` that service and resolves them (along with their transitive dependencies).
    /// This is repeated until no new modules are added (fixpoint).
    fn bind_services(
        &self,
        resolved: &mut BTreeMap<String, ResolvedModule>,
        package_to_module: &mut BTreeMap<String, String>,
        visited: &mut AHashSet<String>,
        finder: &dyn ModuleFinder,
    ) -> Result<()> {
        // Guard against theoretical infinite loops with a maximum iteration count;
        // in practice the visited set ensures convergence but this is defensive.
        const MAX_ITERATIONS: usize = 1024;

        // Build a map of service -> provider modules from all observable modules
        let mut service_providers: AHashMap<String, Vec<String>> = AHashMap::default();
        for reference in finder.find_all() {
            let desc = reference.descriptor();
            for provides in &desc.provides {
                service_providers
                    .entry(provides.service.clone())
                    .or_default()
                    .push(desc.name.clone());
            }
        }

        // Iteratively resolve service providers until fixpoint.
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                return Err(ModuleError::ResolutionFailed(
                    "Service binding exceeded maximum iterations".to_string(),
                ));
            }
            // Collect all services used by currently resolved modules
            let mut needed_services: AHashSet<String> = AHashSet::default();
            for module in resolved.values() {
                for service in &module.descriptor().uses {
                    needed_services.insert(service.clone());
                }
            }

            // Find provider modules that aren't already resolved
            let mut new_modules: Vec<String> = Vec::new();
            for service in &needed_services {
                let Some(providers) = service_providers.get(service) else {
                    continue;
                };

                for provider in providers {
                    if !visited.contains(provider) && !resolved.contains_key(provider) {
                        // Check limit-modules
                        if let Some(ref limit) = self.limit_modules
                            && !limit.contains(provider)
                        {
                            continue;
                        }
                        if finder.find(provider).is_some() {
                            new_modules.push(provider.clone());
                            visited.insert(provider.clone());
                        }
                    }
                }
            }

            if new_modules.is_empty() {
                break;
            }

            // Resolve the new provider modules and their transitive dependencies
            self.resolve_service_providers(
                new_modules,
                resolved,
                package_to_module,
                visited,
                finder,
            )?;
        }

        Ok(())
    }

    /// Resolves service provider modules and their transitive dependencies via BFS.
    fn resolve_service_providers(
        &self,
        new_modules: Vec<String>,
        resolved: &mut BTreeMap<String, ResolvedModule>,
        package_to_module: &mut BTreeMap<String, String>,
        visited: &mut AHashSet<String>,
        finder: &dyn ModuleFinder,
    ) -> Result<()> {
        let mut queue: VecDeque<String> = new_modules.into_iter().collect();
        while let Some(module_name) = queue.pop_front() {
            if resolved.contains_key(&module_name) {
                continue;
            }

            // Check limit-modules for transitive dependencies (java.base always included)
            if module_name != "java.base"
                && let Some(ref limit) = self.limit_modules
                && !limit.contains(&module_name)
            {
                continue;
            }

            let Some(reference) = finder.find(&module_name) else {
                continue;
            };

            let descriptor = reference.descriptor().clone();

            // Check for split packages (same check as main resolution)
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
                if req.is_static() {
                    // Static dependencies are optional; only add read edge if found
                    if finder.find(&req.name).is_some() {
                        resolved_module.add_read(req.name.clone());
                        if !visited.contains(&req.name) {
                            queue.push_back(req.name.clone());
                            visited.insert(req.name.clone());
                        }
                    }
                } else if resolved.contains_key(&req.name) || visited.contains(&req.name) {
                    // Non-static dependency already resolved or queued
                    resolved_module.add_read(req.name.clone());
                } else {
                    // Non-static dependency must exist
                    finder
                        .find(&req.name)
                        .ok_or_else(|| ModuleError::ModuleNotFound(req.name.clone()))?;
                    resolved_module.add_read(req.name.clone());
                    queue.push_back(req.name.clone());
                    visited.insert(req.name.clone());
                }
            }

            if module_name != "java.base" {
                resolved_module.add_read("java.base".to_string());
            }

            resolved.insert(module_name, resolved_module);
        }

        Ok(())
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
            && targets.contains(from)
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
            && targets.contains(from)
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
    use crate::module::descriptor::{Exports, ModuleFlags, Provides, Requires, RequiresFlags};
    use crate::module::reference::ModuleReference;
    use std::collections::HashMap;

    /// In-memory module finder for testing resolution.
    struct TestFinder {
        modules: HashMap<String, ModuleReference>,
    }

    impl TestFinder {
        fn new() -> Self {
            Self {
                modules: HashMap::new(),
            }
        }

        fn add(&mut self, reference: ModuleReference) {
            self.modules.insert(reference.name().to_string(), reference);
        }
    }

    impl ModuleFinder for TestFinder {
        fn find(&self, name: &str) -> Option<ModuleReference> {
            self.modules.get(name).cloned()
        }

        fn find_all(&self) -> Vec<ModuleReference> {
            self.modules.values().cloned().collect()
        }
    }

    fn java_base_descriptor() -> ModuleDescriptor {
        let mut desc = ModuleDescriptor::new("java.base".to_string());
        desc.packages.insert("java/lang".to_string());
        desc.packages.insert("java/io".to_string());
        desc.packages.insert("java/util".to_string());
        desc.exports.push(Exports {
            package: "java/lang".to_string(),
            targets: None,
        });
        desc.exports.push(Exports {
            package: "java/io".to_string(),
            targets: None,
        });
        desc.exports.push(Exports {
            package: "java/util".to_string(),
            targets: None,
        });
        desc
    }

    fn create_test_descriptor(name: &str) -> ModuleDescriptor {
        let mut descriptor = ModuleDescriptor::new(name.to_string());
        descriptor
            .packages
            .insert(format!("{}/internal", name.replace('.', "/")));
        descriptor
    }

    fn create_finder_with_base() -> TestFinder {
        let mut finder = TestFinder::new();
        finder.add(ModuleReference::system(java_base_descriptor()));
        finder
    }

    // Basic tests

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

    // Breadth-first search resolution tests

    #[test]
    fn test_resolve_java_base_only() {
        let finder = create_finder_with_base();
        let resolver = Resolver::new();
        let config = resolver.resolve(&[], &finder).expect("resolution failed");

        assert_eq!(config.len(), 1);
        assert!(config.get("java.base").is_some());
    }

    #[test]
    fn test_resolve_root_module_requires_dependency() {
        let mut finder = create_finder_with_base();

        // java.sql requires java.base (implicit) and java.logging
        let mut sql_desc = ModuleDescriptor::new("java.sql".to_string());
        sql_desc.packages.insert("java/sql".to_string());
        sql_desc.requires.push(Requires {
            name: "java.logging".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(sql_desc));

        let mut logging_desc = ModuleDescriptor::new("java.logging".to_string());
        logging_desc
            .packages
            .insert("java/util/logging".to_string());
        finder.add(ModuleReference::system(logging_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["java.sql".to_string()], &finder)
            .expect("resolution failed");

        // Should resolve java.base, java.sql, and java.logging
        assert_eq!(config.len(), 3);
        assert!(config.get("java.base").is_some());
        assert!(config.get("java.sql").is_some());
        assert!(config.get("java.logging").is_some());

        // java.sql should read java.logging and java.base
        let sql = config.get("java.sql").expect("java.sql missing");
        assert!(sql.reads_module("java.logging"));
        assert!(sql.reads_module("java.base"));
    }

    #[test]
    fn test_resolve_implicit_java_base_read() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        finder.add(ModuleReference::system(app_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // Every non-java.base module implicitly reads java.base
        let app = config.get("my.app").expect("my.app missing");
        assert!(app.reads_module("java.base"));

        // java.base does NOT read itself
        let base = config.get("java.base").expect("java.base missing");
        assert!(!base.reads_module("java.base"));
    }

    #[test]
    fn test_resolve_module_not_found() {
        let finder = create_finder_with_base();
        let resolver = Resolver::new();
        let result = resolver.resolve(&["nonexistent.module".to_string()], &finder);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ModuleError::ModuleNotFound(ref name) if name == "nonexistent.module")
        );
    }

    // Split package tests

    #[test]
    fn test_resolve_split_package_error() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("shared/pkg".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut mod_b = ModuleDescriptor::new("mod.b".to_string());
        mod_b.packages.insert("shared/pkg".to_string());
        finder.add(ModuleReference::system(mod_b));

        let resolver = Resolver::new();
        let result = resolver.resolve(&["mod.a".to_string(), "mod.b".to_string()], &finder);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            ModuleError::SplitPackage {
                package,
                module1,
                module2,
            } => {
                assert_eq!(package, "shared/pkg");
                // The modules could be in either order depending on BFS visit order
                assert!(
                    (module1 == "mod.a" && module2 == "mod.b")
                        || (module1 == "mod.b" && module2 == "mod.a")
                );
            }
            _ => panic!("Expected SplitPackage error, got: {err:?}"),
        }
    }

    // Service binding tests

    #[test]
    fn test_resolve_service_binding_pulls_provider() {
        let mut finder = create_finder_with_base();

        // my.app uses service com.example.Service
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.uses.push("com.example.Service".to_string());
        finder.add(ModuleReference::system(app_desc));

        // my.provider provides com.example.Service
        let mut provider_desc = ModuleDescriptor::new("my.provider".to_string());
        provider_desc.packages.insert("my/provider".to_string());
        provider_desc.provides.push(Provides {
            service: "com.example.Service".to_string(),
            implementations: vec!["my.provider.ServiceImpl".to_string()],
        });
        finder.add(ModuleReference::system(provider_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // Service binding should pull in my.provider
        assert!(config.get("my.provider").is_some());
        assert_eq!(config.len(), 3); // java.base + my.app + my.provider
    }

    #[test]
    fn test_resolve_service_binding_transitive_providers() {
        let mut finder = create_finder_with_base();

        // my.app uses ServiceA
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.uses.push("com.example.ServiceA".to_string());
        finder.add(ModuleReference::system(app_desc));

        // my.provider1 provides ServiceA and uses ServiceB
        let mut p1_desc = ModuleDescriptor::new("my.provider1".to_string());
        p1_desc.packages.insert("my/provider1".to_string());
        p1_desc.provides.push(Provides {
            service: "com.example.ServiceA".to_string(),
            implementations: vec!["my.provider1.Impl".to_string()],
        });
        p1_desc.uses.push("com.example.ServiceB".to_string());
        finder.add(ModuleReference::system(p1_desc));

        // my.provider2 provides ServiceB
        let mut p2_desc = ModuleDescriptor::new("my.provider2".to_string());
        p2_desc.packages.insert("my/provider2".to_string());
        p2_desc.provides.push(Provides {
            service: "com.example.ServiceB".to_string(),
            implementations: vec!["my.provider2.Impl".to_string()],
        });
        finder.add(ModuleReference::system(p2_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // Transitive service binding: app -> provider1 -> provider2
        assert!(config.get("my.provider1").is_some());
        assert!(config.get("my.provider2").is_some());
        assert_eq!(config.len(), 4);
    }

    #[test]
    fn test_resolve_service_binding_respects_limit_modules() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.uses.push("com.example.Service".to_string());
        finder.add(ModuleReference::system(app_desc));

        let mut provider_desc = ModuleDescriptor::new("my.provider".to_string());
        provider_desc.packages.insert("my/provider".to_string());
        provider_desc.provides.push(Provides {
            service: "com.example.Service".to_string(),
            implementations: vec!["my.provider.Impl".to_string()],
        });
        finder.add(ModuleReference::system(provider_desc));

        let mut resolver = Resolver::new();
        let mut limit = AHashSet::default();
        limit.insert("java.base".to_string());
        limit.insert("my.app".to_string());
        // Deliberately exclude my.provider
        resolver.set_limit_modules(limit);

        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // my.provider should NOT be resolved due to limit-modules
        assert!(config.get("my.provider").is_none());
        assert_eq!(config.len(), 2); // java.base + my.app
    }

    #[test]
    fn test_resolve_service_binding_split_package_in_provider() {
        let mut finder = create_finder_with_base();

        // my.app uses a service
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("shared/pkg".to_string());
        app_desc.uses.push("com.example.Service".to_string());
        finder.add(ModuleReference::system(app_desc));

        // my.provider provides the service but has a split package with my.app
        let mut provider_desc = ModuleDescriptor::new("my.provider".to_string());
        provider_desc.packages.insert("shared/pkg".to_string());
        provider_desc.provides.push(Provides {
            service: "com.example.Service".to_string(),
            implementations: vec!["my.provider.Impl".to_string()],
        });
        finder.add(ModuleReference::system(provider_desc));

        let resolver = Resolver::new();
        let result = resolver.resolve(&["my.app".to_string()], &finder);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ModuleError::SplitPackage { .. }
        ));
    }

    // limit-modules tests

    #[test]
    fn test_resolve_limit_modules_filters_root() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        finder.add(ModuleReference::system(app_desc));

        let mut resolver = Resolver::new();
        let mut limit = AHashSet::default();
        limit.insert("java.base".to_string());
        // my.app is NOT in the limit set
        resolver.set_limit_modules(limit);

        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // my.app should be filtered out by limit-modules
        assert!(config.get("my.app").is_none());
        assert_eq!(config.len(), 1); // Only java.base
    }

    #[test]
    fn test_resolve_limit_modules_filters_dependency() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.requires.push(Requires {
            name: "my.lib".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(app_desc));

        let mut lib_desc = ModuleDescriptor::new("my.lib".to_string());
        lib_desc.packages.insert("my/lib".to_string());
        finder.add(ModuleReference::system(lib_desc));

        let mut resolver = Resolver::new();
        let mut limit = AHashSet::default();
        limit.insert("java.base".to_string());
        limit.insert("my.app".to_string());
        // my.lib NOT in limit set
        resolver.set_limit_modules(limit);

        // my.lib is required (non-static) but filtered by limit-modules.
        // The read edge should be pruned since my.lib is not in the resolved set.
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");
        assert!(config.get("my.lib").is_none());

        // Verify no dangling read edge to the filtered module
        let app = config.get("my.app").expect("my.app missing");
        assert!(
            !app.reads_module("my.lib"),
            "read edge to filtered module should be pruned"
        );
    }

    #[test]
    fn test_resolve_limit_modules_always_includes_java_base() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        finder.add(ModuleReference::system(app_desc));

        let mut resolver = Resolver::new();
        let mut limit = AHashSet::default();
        // Only include my.app; java.base is NOT in the limit set
        limit.insert("my.app".to_string());
        resolver.set_limit_modules(limit);

        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // java.base must always be included per JEP 261 §2
        assert!(config.get("java.base").is_some());
        assert!(config.get("my.app").is_some());
        assert_eq!(config.len(), 2);
    }

    #[test]
    fn test_resolve_service_binding_transitive_deps_respect_limit_modules() {
        let mut finder = create_finder_with_base();

        // my.app uses a service
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.uses.push("com.example.Service".to_string());
        finder.add(ModuleReference::system(app_desc));

        // my.provider provides the service and requires my.transitive
        let mut provider_desc = ModuleDescriptor::new("my.provider".to_string());
        provider_desc.packages.insert("my/provider".to_string());
        provider_desc.provides.push(Provides {
            service: "com.example.Service".to_string(),
            implementations: vec!["my.provider.Impl".to_string()],
        });
        provider_desc.requires.push(Requires {
            name: "my.transitive".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(provider_desc));

        // my.transitive is a transitive dependency of the provider
        let mut transitive_desc = ModuleDescriptor::new("my.transitive".to_string());
        transitive_desc.packages.insert("my/transitive".to_string());
        finder.add(ModuleReference::system(transitive_desc));

        let mut resolver = Resolver::new();
        let mut limit = AHashSet::default();
        limit.insert("java.base".to_string());
        limit.insert("my.app".to_string());
        limit.insert("my.provider".to_string());
        // Deliberately exclude my.transitive
        resolver.set_limit_modules(limit);

        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // my.transitive should NOT be resolved because it is excluded by limit-modules
        assert!(config.get("my.transitive").is_none());
        assert!(config.get("my.provider").is_some());

        // Verify no dangling read edge from my.provider to my.transitive
        let provider = config.get("my.provider").expect("my.provider missing");
        assert!(
            !provider.reads_module("my.transitive"),
            "read edge to filtered module should be pruned in service provider resolution"
        );
    }

    // Cyclic dependency tests

    #[test]
    fn test_resolve_cyclic_requires_between_two_modules() {
        let mut finder = create_finder_with_base();

        // mod.a requires mod.b, mod.b requires mod.a (mutual dependency)
        let mut a_desc = ModuleDescriptor::new("mod.a".to_string());
        a_desc.packages.insert("mod/a".to_string());
        a_desc.requires.push(Requires {
            name: "mod.b".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(a_desc));

        let mut b_desc = ModuleDescriptor::new("mod.b".to_string());
        b_desc.packages.insert("mod/b".to_string());
        b_desc.requires.push(Requires {
            name: "mod.a".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(b_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("cyclic requires should not fail per JPMS spec");

        // Both modules should be resolved
        assert!(config.get("mod.a").is_some());
        assert!(config.get("mod.b").is_some());

        // Each reads the other
        let a = config.get("mod.a").expect("mod.a missing");
        assert!(a.reads_module("mod.b"));
        let b = config.get("mod.b").expect("mod.b missing");
        assert!(b.reads_module("mod.a"));
    }

    #[test]
    fn test_resolve_cyclic_requires_three_modules() {
        let mut finder = create_finder_with_base();

        // mod.a -> mod.b -> mod.c -> mod.a (cycle of length 3)
        let mut a_desc = ModuleDescriptor::new("mod.a".to_string());
        a_desc.packages.insert("mod/a".to_string());
        a_desc.requires.push(Requires {
            name: "mod.b".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(a_desc));

        let mut b_desc = ModuleDescriptor::new("mod.b".to_string());
        b_desc.packages.insert("mod/b".to_string());
        b_desc.requires.push(Requires {
            name: "mod.c".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(b_desc));

        let mut c_desc = ModuleDescriptor::new("mod.c".to_string());
        c_desc.packages.insert("mod/c".to_string());
        c_desc.requires.push(Requires {
            name: "mod.a".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(c_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("cyclic requires should not fail per JPMS spec");

        assert_eq!(config.len(), 4); // java.base + mod.a + mod.b + mod.c
    }

    // Split package in transitive dependency tests

    #[test]
    fn test_resolve_split_package_in_transitive_dependency() {
        let mut finder = create_finder_with_base();

        // mod.a requires mod.b; mod.b and mod.c share a package
        let mut a_desc = ModuleDescriptor::new("mod.a".to_string());
        a_desc.packages.insert("mod/a".to_string());
        a_desc.requires.push(Requires {
            name: "mod.b".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        a_desc.requires.push(Requires {
            name: "mod.c".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(a_desc));

        let mut b_desc = ModuleDescriptor::new("mod.b".to_string());
        b_desc.packages.insert("shared/pkg".to_string());
        finder.add(ModuleReference::system(b_desc));

        let mut c_desc = ModuleDescriptor::new("mod.c".to_string());
        c_desc.packages.insert("shared/pkg".to_string());
        finder.add(ModuleReference::system(c_desc));

        let resolver = Resolver::new();
        let result = resolver.resolve(&["mod.a".to_string()], &finder);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ModuleError::SplitPackage { .. }
        ));
    }

    // Static requires tests

    #[test]
    fn test_resolve_static_requires_present() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.requires.push(Requires {
            name: "my.optional".to_string(),
            flags: RequiresFlags::STATIC_PHASE,
            version: None,
        });
        finder.add(ModuleReference::system(app_desc));

        let mut opt_desc = ModuleDescriptor::new("my.optional".to_string());
        opt_desc.packages.insert("my/optional".to_string());
        finder.add(ModuleReference::system(opt_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // Static dependency is present, so it should be resolved
        assert!(config.get("my.optional").is_some());
        let app = config.get("my.app").expect("my.app missing");
        assert!(app.reads_module("my.optional"));
    }

    #[test]
    fn test_resolve_static_requires_absent() {
        let mut finder = create_finder_with_base();

        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.requires.push(Requires {
            name: "my.optional".to_string(),
            flags: RequiresFlags::STATIC_PHASE,
            version: None,
        });
        finder.add(ModuleReference::system(app_desc));
        // my.optional is NOT in the finder

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // Static dependency is absent - should NOT cause an error
        assert!(config.get("my.optional").is_none());
        assert_eq!(config.len(), 2); // java.base + my.app
    }

    // Transitive readability tests

    #[test]
    fn test_resolve_transitive_readability() {
        let mut finder = create_finder_with_base();

        // my.app requires java.sql
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.requires.push(Requires {
            name: "java.sql".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(app_desc));

        // java.sql requires transitive java.logging
        let mut sql_desc = ModuleDescriptor::new("java.sql".to_string());
        sql_desc.packages.insert("java/sql".to_string());
        sql_desc.requires.push(Requires {
            name: "java.logging".to_string(),
            flags: RequiresFlags::TRANSITIVE,
            version: None,
        });
        finder.add(ModuleReference::system(sql_desc));

        let mut logging_desc = ModuleDescriptor::new("java.logging".to_string());
        logging_desc
            .packages
            .insert("java/util/logging".to_string());
        finder.add(ModuleReference::system(logging_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        // my.app should transitively read java.logging through java.sql
        let app = config.get("my.app").expect("my.app missing");
        assert!(app.reads_module("java.sql"));
        assert!(
            app.reads_module("java.logging"),
            "my.app should transitively read java.logging via java.sql"
        );
    }

    #[test]
    fn test_resolve_deep_transitive_readability() {
        let mut finder = create_finder_with_base();

        // Chain: my.app -> A --(transitive)--> B --(transitive)--> C
        let mut app_desc = ModuleDescriptor::new("my.app".to_string());
        app_desc.packages.insert("my/app".to_string());
        app_desc.requires.push(Requires {
            name: "mod.a".to_string(),
            flags: RequiresFlags::empty(),
            version: None,
        });
        finder.add(ModuleReference::system(app_desc));

        let mut a_desc = ModuleDescriptor::new("mod.a".to_string());
        a_desc.packages.insert("mod/a".to_string());
        a_desc.requires.push(Requires {
            name: "mod.b".to_string(),
            flags: RequiresFlags::TRANSITIVE,
            version: None,
        });
        finder.add(ModuleReference::system(a_desc));

        let mut b_desc = ModuleDescriptor::new("mod.b".to_string());
        b_desc.packages.insert("mod/b".to_string());
        b_desc.requires.push(Requires {
            name: "mod.c".to_string(),
            flags: RequiresFlags::TRANSITIVE,
            version: None,
        });
        finder.add(ModuleReference::system(b_desc));

        let mut c_desc = ModuleDescriptor::new("mod.c".to_string());
        c_desc.packages.insert("mod/c".to_string());
        finder.add(ModuleReference::system(c_desc));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["my.app".to_string()], &finder)
            .expect("resolution failed");

        let app = config.get("my.app").expect("my.app missing");
        assert!(app.reads_module("mod.a"));
        assert!(app.reads_module("mod.b"), "should transitively read mod.b");
        assert!(
            app.reads_module("mod.c"),
            "should transitively read mod.c through chain"
        );
    }

    // Automatic module tests

    #[test]
    fn test_resolve_automatic_module_reads_all() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut auto_desc = ModuleDescriptor::new("auto.lib".to_string());
        auto_desc.packages.insert("auto/lib".to_string());
        finder.add(ModuleReference::automatic(
            auto_desc,
            std::path::PathBuf::from("auto-lib.jar"),
        ));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string(), "auto.lib".to_string()], &finder)
            .expect("resolution failed");

        // Automatic modules read all resolved named modules
        let auto = config.get("auto.lib").expect("auto.lib missing");
        assert!(
            auto.reads_module("java.base"),
            "automatic module should read java.base"
        );
        assert!(
            auto.reads_module("mod.a"),
            "automatic module should read all other resolved modules"
        );
    }

    // --add-reads tests

    #[test]
    fn test_resolve_add_reads_applied() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut mod_b = ModuleDescriptor::new("mod.b".to_string());
        mod_b.packages.insert("mod/b".to_string());
        finder.add(ModuleReference::system(mod_b));

        let mut resolver = Resolver::new();
        resolver.add_read("mod.a".to_string(), "mod.b".to_string());

        let config = resolver
            .resolve(&["mod.a".to_string(), "mod.b".to_string()], &finder)
            .expect("resolution failed");

        let a = config.get("mod.a").expect("mod.a missing");
        assert!(a.reads_module("mod.b"), "--add-reads should add read edge");
    }

    #[test]
    fn test_resolve_add_reads_all_unnamed() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut resolver = Resolver::new();
        resolver.add_read("mod.a".to_string(), "ALL-UNNAMED".to_string());

        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        let a = config.get("mod.a").expect("mod.a missing");
        assert!(
            a.reads_module("ALL-UNNAMED"),
            "ALL-UNNAMED should be in reads"
        );
    }

    // --add-exports / --add-opens tests

    #[test]
    fn test_resolve_add_exports_in_config() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a/internal".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut resolver = Resolver::new();
        resolver.add_export(
            "mod.a".to_string(),
            "mod/a/internal".to_string(),
            "mod.b".to_string(),
        );

        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        assert!(
            config.exports("mod.a", "mod/a/internal", "mod.b"),
            "--add-exports should make package visible"
        );
    }

    #[test]
    fn test_resolve_add_opens_in_config() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a/internal".to_string());
        finder.add(ModuleReference::system(mod_a));

        let mut resolver = Resolver::new();
        resolver.add_opens(
            "mod.a".to_string(),
            "mod/a/internal".to_string(),
            "mod.b".to_string(),
        );

        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        assert!(
            config.opens("mod.a", "mod/a/internal", "mod.b"),
            "--add-opens should make package reflectively accessible"
        );
    }

    // ResolvedConfiguration query tests

    #[test]
    fn test_resolved_configuration_find_module_for_package() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("com/example/a".to_string());
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        assert_eq!(
            config.find_module_for_package("com/example/a"),
            Some("mod.a")
        );
        assert_eq!(
            config.find_module_for_package("java/lang"),
            Some("java.base")
        );
        assert_eq!(config.find_module_for_package("nonexistent/pkg"), None);
    }

    #[test]
    fn test_resolved_configuration_reads() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a".to_string());
        mod_a.requires.push(Requires {
            name: "java.base".to_string(),
            flags: RequiresFlags::MANDATED,
            version: None,
        });
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        assert!(config.reads("mod.a", "java.base"));
        assert!(!config.reads("java.base", "mod.a"));
        assert!(!config.reads("nonexistent", "java.base"));
    }

    #[test]
    fn test_resolved_configuration_exports_unqualified() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a/api".to_string());
        mod_a.exports.push(Exports {
            package: "mod/a/api".to_string(),
            targets: None, // unqualified
        });
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        // Unqualified export is visible to any module
        assert!(config.exports("mod.a", "mod/a/api", "any.module"));
    }

    #[test]
    fn test_resolved_configuration_exports_qualified() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a/internal".to_string());
        mod_a.exports.push(Exports {
            package: "mod/a/internal".to_string(),
            targets: Some(vec!["mod.b".to_string()]),
        });
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        assert!(config.exports("mod.a", "mod/a/internal", "mod.b"));
        assert!(!config.exports("mod.a", "mod/a/internal", "mod.c"));
    }

    #[test]
    fn test_resolved_configuration_open_module_opens_all() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.flags = ModuleFlags::OPEN;
        mod_a.packages.insert("mod/a/internal".to_string());
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string()], &finder)
            .expect("resolution failed");

        // Open modules open all packages to all modules
        assert!(config.opens("mod.a", "mod/a/internal", "any.module"));
    }

    #[test]
    fn test_resolved_configuration_empty() {
        let config = ResolvedConfiguration::empty();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
        assert!(config.get("java.base").is_none());
    }

    // Duplicate root module handling

    #[test]
    fn test_resolve_duplicate_roots_deduped() {
        let mut finder = create_finder_with_base();

        let mut mod_a = ModuleDescriptor::new("mod.a".to_string());
        mod_a.packages.insert("mod/a".to_string());
        finder.add(ModuleReference::system(mod_a));

        let resolver = Resolver::new();
        let config = resolver
            .resolve(&["mod.a".to_string(), "mod.a".to_string()], &finder)
            .expect("resolution failed");

        // Should not fail or duplicate
        assert_eq!(config.len(), 2); // java.base + mod.a
    }
}

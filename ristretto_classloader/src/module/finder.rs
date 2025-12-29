//! Module finder implementations.

use crate::module::descriptor::ModuleDescriptor;
use crate::module::error::{ModuleError, Result};
use crate::module::reference::{ModuleReference, ModuleSource};
use ristretto_classfile::ClassFile;
use ristretto_jimage::Image as JImage;
use std::collections::{BTreeSet, HashMap};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// A finder for modules in a specific location or set of locations.
pub trait ModuleFinder: Send + Sync {
    /// Finds a module by name.
    fn find(&self, name: &str) -> Option<ModuleReference>;

    /// Returns all modules found by this finder.
    fn find_all(&self) -> Vec<ModuleReference>;
}

/// A finder that chains multiple finders together.
pub struct ModuleFinderChain {
    finders: Vec<Box<dyn ModuleFinder>>,
}

impl std::fmt::Debug for ModuleFinderChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleFinderChain")
            .field("finders_count", &self.finders.len())
            .finish()
    }
}

impl ModuleFinderChain {
    /// Creates a new empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            finders: Vec::new(),
        }
    }

    /// Adds a finder to the chain.
    pub fn add(&mut self, finder: Box<dyn ModuleFinder>) {
        self.finders.push(finder);
    }

    /// Creates a chain with the given finders.
    #[must_use]
    pub fn with_finders(finders: Vec<Box<dyn ModuleFinder>>) -> Self {
        Self { finders }
    }
}

impl Default for ModuleFinderChain {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleFinder for ModuleFinderChain {
    fn find(&self, name: &str) -> Option<ModuleReference> {
        for finder in &self.finders {
            if let Some(reference) = finder.find(name) {
                return Some(reference);
            }
        }
        None
    }

    fn find_all(&self) -> Vec<ModuleReference> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for finder in &self.finders {
            for reference in finder.find_all() {
                if seen.insert(reference.name().to_string()) {
                    result.push(reference);
                }
            }
        }
        result
    }
}

/// A finder for system modules from a jimage file.
#[derive(Debug)]
pub struct SystemModuleFinder {
    modules: HashMap<String, ModuleReference>,
}

impl SystemModuleFinder {
    /// Creates a new system module finder from a jimage file.
    ///
    /// # Errors
    ///
    /// Returns an error if the jimage cannot be read.
    pub async fn new(jimage_path: &Path) -> Result<Self> {
        #[cfg(not(target_family = "wasm"))]
        let modules = {
            let path = jimage_path.to_path_buf();
            tokio::task::spawn_blocking(move || Self::load_from_jimage(&path))
                .await
                .map_err(|e| ModuleError::IoError(e.to_string()))??
        };

        #[cfg(target_family = "wasm")]
        let modules = Self::load_from_jimage(jimage_path)?;

        Ok(Self { modules })
    }

    /// Loads modules from a jimage file (sync implementation).
    fn load_from_jimage(jimage_path: &Path) -> Result<HashMap<String, ModuleReference>> {
        let image =
            JImage::from_file(jimage_path).map_err(|e| ModuleError::IoError(e.to_string()))?;

        let mut modules: HashMap<String, ModuleReference> = HashMap::new();
        let mut module_packages: HashMap<String, BTreeSet<String>> = HashMap::new();

        // First pass: collect all packages per module
        for resource in &image {
            let resource = resource.map_err(|e| ModuleError::IoError(e.to_string()))?;
            let module_name = resource.module().to_string();
            if module_name.is_empty() {
                continue;
            }

            // Collect packages from class files
            if resource.extension() == "class" && resource.base() != "module-info" {
                let package = resource.parent().to_string();
                module_packages
                    .entry(module_name)
                    .or_default()
                    .insert(package);
            }
        }

        // Second pass: parse module-info.class for each module
        for resource in &image {
            let resource = resource.map_err(|e| ModuleError::IoError(e.to_string()))?;
            if resource.base() != "module-info" || resource.extension() != "class" {
                continue;
            }

            let module_name = resource.module().to_string();
            if module_name.is_empty() {
                continue;
            }

            let mut cursor = Cursor::new(resource.data().to_vec());
            let class_file = ClassFile::from_bytes(&mut cursor)
                .map_err(|e| ModuleError::DescriptorParseError(e.to_string()))?;

            let mut descriptor = ModuleDescriptor::from_class_file(&class_file)?;

            // Add packages discovered from class files
            if let Some(packages) = module_packages.get(&module_name) {
                for pkg in packages {
                    descriptor.packages.insert(pkg.clone());
                }
            }

            let reference = ModuleReference::new(descriptor, ModuleSource::System, None);
            modules.insert(module_name, reference);
        }

        Ok(modules)
    }

    /// Creates a finder with pre-built module references.
    #[must_use]
    pub fn with_modules(modules: HashMap<String, ModuleReference>) -> Self {
        Self { modules }
    }
}

impl ModuleFinder for SystemModuleFinder {
    fn find(&self, name: &str) -> Option<ModuleReference> {
        self.modules.get(name).cloned()
    }

    fn find_all(&self) -> Vec<ModuleReference> {
        self.modules.values().cloned().collect()
    }
}

/// A finder for modules on the module path (modular JARs and exploded modules).
#[derive(Debug)]
pub struct ModulePathFinder {
    modules: HashMap<String, ModuleReference>,
}

impl ModulePathFinder {
    /// Creates a new module path finder.
    ///
    /// # Errors
    ///
    /// Returns an error if any module cannot be read.
    pub async fn new(paths: &[PathBuf]) -> Result<Self> {
        #[cfg(not(target_family = "wasm"))]
        let modules = {
            let paths = paths.to_vec();
            tokio::task::spawn_blocking(move || Self::load_from_paths(&paths))
                .await
                .map_err(|e| ModuleError::IoError(e.to_string()))??
        };

        #[cfg(target_family = "wasm")]
        let modules = Self::load_from_paths(paths)?;

        Ok(Self { modules })
    }

    /// Loads modules from paths (sync implementation).
    fn load_from_paths(paths: &[PathBuf]) -> Result<HashMap<String, ModuleReference>> {
        let mut modules = HashMap::new();

        for path in paths {
            if path.is_file()
                && path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("jar"))
            {
                // Try to read as modular JAR
                if let Some(reference) = Self::read_modular_jar(path)? {
                    modules.insert(reference.name().to_string(), reference);
                }
            } else if path.is_dir() {
                // Check for exploded module (has module-info.class)
                let module_info_path = path.join("module-info.class");
                if module_info_path.exists() {
                    if let Some(reference) = Self::read_exploded_module(path)? {
                        modules.insert(reference.name().to_string(), reference);
                    }
                } else {
                    // Directory containing modules
                    if let Ok(entries) = std::fs::read_dir(path) {
                        for entry in entries.flatten() {
                            let entry_path = entry.path();
                            if entry_path.is_file()
                                && entry_path
                                    .extension()
                                    .is_some_and(|e| e.eq_ignore_ascii_case("jar"))
                                && let Some(reference) = Self::read_modular_jar(&entry_path)?
                            {
                                modules.insert(reference.name().to_string(), reference);
                            } else if entry_path.is_dir() {
                                let module_info = entry_path.join("module-info.class");
                                if module_info.exists()
                                    && let Some(reference) =
                                        Self::read_exploded_module(&entry_path)?
                                {
                                    modules.insert(reference.name().to_string(), reference);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(modules)
    }

    fn read_modular_jar(path: &Path) -> Result<Option<ModuleReference>> {
        let file = std::fs::File::open(path).map_err(|e| {
            ModuleError::IoError(format!("Failed to open {}: {}", path.display(), e))
        })?;

        let mut archive = ZipArchive::new(file).map_err(|e| {
            ModuleError::IoError(format!("Failed to read {}: {}", path.display(), e))
        })?;

        // Check for Automatic-Module-Name in manifest
        let manifest_module_name = Self::read_manifest_module_name(&mut archive);

        // Collect all packages
        let mut packages = BTreeSet::new();
        for entry_index in 0..archive.len() {
            let file = archive
                .by_index(entry_index)
                .map_err(|error| ModuleError::IoError(error.to_string()))?;
            let name = file.name();
            if name.to_ascii_lowercase().ends_with(".class")
                && !name.starts_with("META-INF/")
                && let Some(last_slash) = name.rfind('/')
            {
                let package = &name[..last_slash];
                if !package.is_empty() {
                    packages.insert(package.to_string());
                }
            }
        }

        // Try to read module-info.class
        let descriptor = if let Ok(mut module_info) = archive.by_name("module-info.class") {
            let mut data = Vec::new();
            std::io::Read::read_to_end(&mut module_info, &mut data)
                .map_err(|e| ModuleError::IoError(e.to_string()))?;
            let mut cursor = Cursor::new(data);
            let class_file = ClassFile::from_bytes(&mut cursor)
                .map_err(|e| ModuleError::DescriptorParseError(e.to_string()))?;
            let mut descriptor = ModuleDescriptor::from_class_file(&class_file)?;
            // Add discovered packages
            for pkg in packages {
                descriptor.packages.insert(pkg);
            }
            Some((descriptor, false))
        } else {
            // Create automatic module
            let jar_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let descriptor = ModuleDescriptor::automatic_from_jar_name(
                jar_name,
                manifest_module_name.as_deref(),
                packages,
            )?;
            Some((descriptor, true))
        };

        if let Some((desc, is_automatic)) = descriptor {
            let source = if is_automatic {
                ModuleSource::Automatic
            } else {
                ModuleSource::ModulePath
            };
            Ok(Some(ModuleReference::new(
                desc,
                source,
                Some(path.to_path_buf()),
            )))
        } else {
            Ok(None)
        }
    }

    fn read_manifest_module_name<R: std::io::Read + std::io::Seek>(
        archive: &mut ZipArchive<R>,
    ) -> Option<String> {
        let mut manifest = archive.by_name("META-INF/MANIFEST.MF").ok()?;
        let mut content = String::new();
        std::io::Read::read_to_string(&mut manifest, &mut content).ok()?;

        for line in content.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("Automatic-Module-Name:") {
                return Some(value.trim().to_string());
            }
        }
        None
    }

    fn read_exploded_module(path: &Path) -> Result<Option<ModuleReference>> {
        let module_info_path = path.join("module-info.class");
        if !module_info_path.exists() {
            return Ok(None);
        }

        let data =
            std::fs::read(&module_info_path).map_err(|e| ModuleError::IoError(e.to_string()))?;
        let mut cursor = Cursor::new(data);
        let class_file = ClassFile::from_bytes(&mut cursor)
            .map_err(|e| ModuleError::DescriptorParseError(e.to_string()))?;
        let mut descriptor = ModuleDescriptor::from_class_file(&class_file)?;

        // Discover packages from directory structure
        Self::discover_packages_in_dir(path, "", &mut descriptor.packages)?;

        Ok(Some(ModuleReference::new(
            descriptor,
            ModuleSource::ModulePath,
            Some(path.to_path_buf()),
        )))
    }

    fn discover_packages_in_dir(
        base: &Path,
        prefix: &str,
        packages: &mut BTreeSet<String>,
    ) -> Result<()> {
        let current = if prefix.is_empty() {
            base.to_path_buf()
        } else {
            base.join(prefix.replace('/', std::path::MAIN_SEPARATOR_STR))
        };

        if !current.is_dir() {
            return Ok(());
        }

        let entries =
            std::fs::read_dir(&current).map_err(|e| ModuleError::IoError(e.to_string()))?;

        let mut has_classes = false;
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if entry_path.is_file()
                && name_str.ends_with(".class")
                && name_str != "module-info.class"
            {
                has_classes = true;
            } else if entry_path.is_dir() && !name_str.starts_with('.') {
                let new_prefix = if prefix.is_empty() {
                    name_str.to_string()
                } else {
                    format!("{prefix}/{name_str}")
                };
                Self::discover_packages_in_dir(base, &new_prefix, packages)?;
            }
        }

        if has_classes && !prefix.is_empty() {
            packages.insert(prefix.to_string());
        }

        Ok(())
    }
}

impl ModuleFinder for ModulePathFinder {
    fn find(&self, name: &str) -> Option<ModuleReference> {
        self.modules.get(name).cloned()
    }

    fn find_all(&self) -> Vec<ModuleReference> {
        self.modules.values().cloned().collect()
    }
}

/// An empty module finder.
pub struct EmptyModuleFinder;

impl ModuleFinder for EmptyModuleFinder {
    fn find(&self, _name: &str) -> Option<ModuleReference> {
        None
    }

    fn find_all(&self) -> Vec<ModuleReference> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_finder() {
        let finder = EmptyModuleFinder;
        assert!(finder.find("java.base").is_none());
        assert!(finder.find_all().is_empty());
    }

    #[test]
    fn test_finder_chain() {
        let chain = ModuleFinderChain::new();
        assert!(chain.find("java.base").is_none());
        assert!(chain.find_all().is_empty());
    }
}

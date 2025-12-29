//! Module layer implementation.

use crate::module::resolution::ResolvedConfiguration;
use std::sync::Arc;

/// A module layer represents a set of resolved modules.
///
/// The boot layer is created during VM startup and contains the system modules
/// plus application modules.
#[derive(Clone, Debug)]
pub struct ModuleLayer {
    /// The configuration for this layer.
    configuration: Arc<ResolvedConfiguration>,
    /// Parent layers (if any).
    parents: Vec<Arc<ModuleLayer>>,
    /// Name of this layer (for diagnostics).
    name: String,
}

impl ModuleLayer {
    /// Creates a new module layer.
    #[must_use]
    pub fn new(
        name: String,
        configuration: ResolvedConfiguration,
        parents: Vec<Arc<ModuleLayer>>,
    ) -> Self {
        Self {
            configuration: Arc::new(configuration),
            parents,
            name,
        }
    }

    /// Creates the boot layer.
    #[must_use]
    pub fn boot(configuration: ResolvedConfiguration) -> Self {
        Self::new("boot".to_string(), configuration, Vec::new())
    }

    /// Returns the layer name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the configuration for this layer.
    #[must_use]
    pub fn configuration(&self) -> &ResolvedConfiguration {
        &self.configuration
    }

    /// Returns the parent layers.
    #[must_use]
    pub fn parents(&self) -> &[Arc<ModuleLayer>] {
        &self.parents
    }

    /// Finds a module by name in this layer or any parent.
    #[must_use]
    pub fn find_module(&self, name: &str) -> Option<&crate::module::resolution::ResolvedModule> {
        if let Some(module) = self.configuration.get(name) {
            return Some(module);
        }
        for parent in &self.parents {
            if let Some(module) = parent.find_module(name) {
                return Some(module);
            }
        }
        None
    }

    /// Finds the module containing the given package.
    #[must_use]
    pub fn find_module_for_package(&self, package: &str) -> Option<&str> {
        if let Some(module) = self.configuration.find_module_for_package(package) {
            return Some(module);
        }
        for parent in &self.parents {
            if let Some(module) = parent.find_module_for_package(package) {
                return Some(module);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_layer_boot() {
        let config = ResolvedConfiguration::empty();
        let layer = ModuleLayer::boot(config);
        assert_eq!(layer.name(), "boot");
    }
}

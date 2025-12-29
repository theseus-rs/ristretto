//! Module reference representing a module and its location.

use crate::module::descriptor::ModuleDescriptor;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

/// The source type of a module.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ModuleSource {
    /// System module from jimage.
    System,
    /// Module from upgrade module path.
    Upgrade,
    /// Module from module path (modular JAR or exploded).
    ModulePath,
    /// Automatic module from module path.
    Automatic,
    /// Synthetic module (e.g., unnamed module).
    Synthetic,
}

/// A reference to a module, including its descriptor and location.
#[derive(Clone, Debug)]
pub struct ModuleReference {
    /// The module descriptor.
    descriptor: Arc<ModuleDescriptor>,
    /// The source type.
    source: ModuleSource,
    /// Location of the module (JAR path, jmod path, or system image).
    location: Option<PathBuf>,
    /// Whether this is an automatic module.
    is_automatic: bool,
}

impl ModuleReference {
    /// Creates a new module reference.
    #[must_use]
    pub fn new(
        descriptor: ModuleDescriptor,
        source: ModuleSource,
        location: Option<PathBuf>,
    ) -> Self {
        let is_automatic = matches!(source, ModuleSource::Automatic);
        Self {
            descriptor: Arc::new(descriptor),
            source,
            location,
            is_automatic,
        }
    }

    /// Creates a system module reference.
    #[must_use]
    pub fn system(descriptor: ModuleDescriptor) -> Self {
        Self::new(descriptor, ModuleSource::System, None)
    }

    /// Creates an automatic module reference.
    #[must_use]
    pub fn automatic(descriptor: ModuleDescriptor, location: PathBuf) -> Self {
        Self::new(descriptor, ModuleSource::Automatic, Some(location))
    }

    /// Returns the module descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ModuleDescriptor {
        &self.descriptor
    }

    /// Returns the module name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.descriptor.name
    }

    /// Returns the module source type.
    #[must_use]
    pub fn source(&self) -> &ModuleSource {
        &self.source
    }

    /// Returns the module location.
    #[must_use]
    pub fn location(&self) -> Option<&PathBuf> {
        self.location.as_ref()
    }

    /// Returns true if this is an automatic module.
    #[must_use]
    pub fn is_automatic(&self) -> bool {
        self.is_automatic
    }

    /// Returns true if this is a system module.
    #[must_use]
    pub fn is_system(&self) -> bool {
        matches!(self.source, ModuleSource::System)
    }
}

impl fmt::Display for ModuleReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.descriptor)?;
        if let Some(ref loc) = self.location {
            write!(f, " ({})", loc.display())?;
        }
        Ok(())
    }
}

impl PartialEq for ModuleReference {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor.name == other.descriptor.name && self.source == other.source
    }
}

impl Eq for ModuleReference {}

impl std::hash::Hash for ModuleReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.descriptor.name.hash(state);
        self.source.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_reference() {
        let descriptor = ModuleDescriptor::new("test.module".to_string());
        let reference = ModuleReference::system(descriptor);
        assert_eq!(reference.name(), "test.module");
        assert!(reference.is_system());
        assert!(!reference.is_automatic());
    }

    #[test]
    fn test_automatic_module_reference() {
        let descriptor = ModuleDescriptor::new("automatic.module".to_string());
        let reference = ModuleReference::automatic(descriptor, PathBuf::from("test.jar"));
        assert_eq!(reference.name(), "automatic.module");
        assert!(!reference.is_system());
        assert!(reference.is_automatic());
    }
}

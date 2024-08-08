use crate::{Class, ClassPath, Result};
use dashmap::DashMap;
use std::sync::Arc;

/// Implementation of a Java class loader.
#[derive(Debug)]
pub struct ClassLoader {
    name: String,
    class_path: ClassPath,
    parent: Option<Arc<ClassLoader>>,
    classes: DashMap<String, Arc<Class>>,
}

impl ClassLoader {
    /// Create a new class loader with the given name and parent.
    pub fn new<S: AsRef<str>>(name: S, class_path: ClassPath) -> Self {
        Self {
            name: name.as_ref().to_string(),
            class_path,
            parent: None,
            classes: DashMap::new(),
        }
    }

    /// Get the name of the class loader.
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the class path.
    #[must_use]
    pub fn get_class_path(&self) -> &ClassPath {
        &self.class_path
    }

    /// Get the parent class loader.
    pub fn get_parent(&self) -> Option<Arc<ClassLoader>> {
        self.parent.as_ref().map(Arc::clone)
    }

    /// Set the parent class loader.
    pub fn set_parent(&mut self, parent: Option<Arc<ClassLoader>>) {
        self.parent = parent;
    }

    /// Load a class by name.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub async fn load_class<S: AsRef<str>>(loader: &Arc<Self>, name: S) -> Result<Arc<Class>> {
        let name = name.as_ref();
        if let Some(class) = loader.classes.get(name) {
            return Ok(Arc::clone(&class));
        }

        let class_file = loader.class_path.read_class(name).await?;
        let class = Arc::new(Class::new(Arc::clone(loader), class_file));
        loader.classes.insert(name.to_string(), Arc::clone(&class));
        Ok(class)
    }
}

/// Default implementation of a class loader.
impl Default for ClassLoader {
    /// Create a new bootstrap class loader with no parent.
    fn default() -> Self {
        Self::new("bootstrap", ClassPath::default())
    }
}

/// Implement equality for class loaders.
impl PartialEq for ClassLoader {
    /// Compare class loaders by name.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.get_parent() == other.get_parent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let name = "test";
        let class_loader = ClassLoader::new(name, ClassPath::default());
        assert_eq!(name, class_loader.get_name());
        assert_eq!(&ClassPath::default(), class_loader.get_class_path());
        assert!(class_loader.get_parent().is_none());
    }

    #[test]
    fn test_default() {
        let class_loader = ClassLoader::default();
        assert_eq!("bootstrap", class_loader.get_name());
        assert!(class_loader.get_parent().is_none());
    }

    #[test]
    fn test_equality() {
        let class_loader1 = ClassLoader::new("test", ClassPath::default());
        let class_loader2 = ClassLoader::new("test", ClassPath::default());
        assert_eq!(class_loader1, class_loader2);
    }

    #[test]
    fn test_inequality() {
        let class_loader1 = ClassLoader::new("test1", ClassPath::default());
        let class_loader2 = ClassLoader::new("test2", ClassPath::default());
        assert_ne!(class_loader1, class_loader2);
    }

    #[test]
    fn test_set_parent() {
        let mut class_loader1 = ClassLoader::new("test1", ClassPath::default());
        let class_loader2 = ClassLoader::new("test2", ClassPath::default());
        class_loader1.set_parent(Some(Arc::new(class_loader2)));
        assert_eq!(
            "test2",
            class_loader1.get_parent().expect("parent").get_name()
        );
    }

    #[tokio::test]
    async fn test_load_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = class_path_entries.join(":");
        let class_loader = Arc::new(ClassLoader::new("test", ClassPath::from(&class_path)));
        let class = ClassLoader::load_class(&class_loader, "HelloWorld").await?;
        let class_file = class.get_class_file();
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }
}

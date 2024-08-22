use crate::Error::ClassNotFound;
use crate::{Class, ClassPath, Result};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Implementation of a Java class loader.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-5.html>
#[derive(Debug)]
pub struct ClassLoader {
    name: String,
    class_path: ClassPath,
    parent: Option<Rc<ClassLoader>>,
    classes: Arc<RwLock<HashMap<String, Class>>>,
}

impl ClassLoader {
    /// Create a new class loader with the given name and parent.
    // TODO: Fix cyclic dependency between Attribute and Record in ClassFile
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new<S: AsRef<str>>(name: S, class_path: ClassPath) -> Self {
        Self {
            name: name.as_ref().to_string(),
            class_path,
            parent: None,
            classes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the name of the class loader.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the class path.
    #[must_use]
    pub fn class_path(&self) -> &ClassPath {
        &self.class_path
    }

    /// Get the parent class loader.
    #[must_use]
    pub fn parent(&self) -> Option<&ClassLoader> {
        if let Some(parent) = self.parent.as_ref() {
            return Some(parent);
        }
        None
    }

    /// Set the parent class loader.
    pub fn set_parent(&mut self, parent: Option<ClassLoader>) {
        if let Some(parent) = parent {
            self.parent = Some(Rc::new(parent));
        } else {
            self.parent = None;
        }
    }

    /// Load a class by name.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub async fn load<S: AsRef<str>>(&self, name: S) -> Result<Class> {
        let name = name.as_ref();

        {
            let classes = self.classes.read().await;
            if let Some(class) = classes.get(name) {
                return Ok(class.clone());
            }
        }

        // Convert hierarchy of class loaders to a flat list.
        let mut class_loader = self;
        let mut class_loaders = vec![class_loader];
        while let Some(parent) = class_loader.parent() {
            class_loader = parent;
            class_loaders.push(parent);
        }

        // Iterate over class loaders in reverse order.
        for class_loader in class_loaders.into_iter().rev() {
            let class_path = class_loader.class_path();
            if let Ok(class_file) = class_path.read_class(name).await {
                let class = Class::new(class_loader.clone(), class_file);
                let mut classes = class_loader.classes.write().await;
                classes.insert(name.to_string(), class.clone());
                return Ok(class);
            }
        }

        Err(ClassNotFound(name.to_string()))
    }
}

impl Clone for ClassLoader {
    /// Clone the class loader.
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            class_path: self.class_path.clone(),
            parent: self.parent.as_ref().map(Clone::clone),
            classes: Arc::clone(&self.classes),
        }
    }
}

/// Implement equality for class loaders.
impl PartialEq for ClassLoader {
    /// Compare class loaders by name.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test_log::test]
    fn test_new() {
        let name = "test";
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new(name, class_path);
        assert_eq!(name, class_loader.name());
        assert_eq!(&ClassPath::from("."), class_loader.class_path());
        assert!(class_loader.parent().is_none());
    }

    #[test_log::test]
    fn test_equality() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test", class_path2);
        assert_eq!(class_loader1, class_loader2);
    }

    #[test_log::test]
    fn test_inequality() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        assert_ne!(class_loader1, class_loader2);
    }

    #[test_log::test]
    fn test_set_parent() {
        let class_path1 = ClassPath::from(".");
        let mut class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader1.set_parent(Some(class_loader2));
        assert_eq!("test2", class_loader1.parent().expect("parent").name());
    }

    #[test_log::test(tokio::test)]
    async fn test_load_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "HelloWorld";
        let class = class_loader.load(class_name).await?;
        let class_file = class.class_file();
        assert_eq!(class_name, class_file.class_name()?);

        // Load the same class again to test caching
        let class = class_loader.load(class_name).await?;
        let class_file = class.class_file();
        assert_eq!(class_name, class_file.class_name()?);
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_load_class_parent() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];
        let class_path = ClassPath::from(class_path_entries.join(":"));
        let boot_class_loader = ClassLoader::new("test", class_path);
        let foo_class_path = ClassPath::from("foo");
        let mut class_loader = ClassLoader::new("test", foo_class_path);
        class_loader.set_parent(Some(boot_class_loader));

        let class = class_loader.load("HelloWorld").await?;
        let class_file = class.class_file();
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_load_class_not_found() {
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new("test", class_path);
        let result = class_loader.load("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
    }
}

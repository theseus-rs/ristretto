use crate::Error::{ClassNotFound, InternalError};
use crate::{Class, ClassPath, Result, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;

/// Implementation of a Java class loader.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html>
#[derive(Debug)]
pub struct ClassLoader {
    this: Weak<ClassLoader>,
    name: String,
    class_path: ClassPath,
    parent: Arc<RwLock<Option<Arc<ClassLoader>>>>,
    classes: Arc<RwLock<HashMap<String, Arc<Class>>>>,
    object: Arc<RwLock<Option<Value>>>,
}

impl ClassLoader {
    /// Create a new class loader with the given name and parent.
    pub fn new<S: AsRef<str>>(name: S, class_path: ClassPath) -> Arc<Self> {
        Arc::new_cyclic(|weak_self| ClassLoader {
            this: weak_self.clone(),
            name: name.as_ref().to_string(),
            class_path,
            parent: Arc::new(RwLock::new(None)),
            classes: Arc::new(RwLock::new(HashMap::new())),
            object: Arc::new(RwLock::new(None)),
        })
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
    pub async fn parent(&self) -> Option<Arc<ClassLoader>> {
        let parent_guard = self.parent.read().await;
        if let Some(parent) = parent_guard.as_ref() {
            return Some(parent.clone());
        }
        None
    }

    /// Set the parent class loader.
    pub async fn set_parent(&self, parent: Option<Arc<ClassLoader>>) {
        let mut parent_guard = self.parent.write().await;
        *parent_guard = parent;
    }

    /// Load a class by name.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub async fn load<S: AsRef<str>>(&self, name: S) -> Result<Arc<Class>> {
        self.load_with_status(name).await.map(|(class, _)| class)
    }

    /// Load a class by name with a boolean status indicating if the class was loaded previously.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub async fn load_with_status<S: AsRef<str>>(
        &self,
        class_name: S,
    ) -> Result<(Arc<Class>, bool)> {
        let class_name = class_name.as_ref().to_string().replace('.', "/");
        let class_name = class_name.as_str();
        {
            let classes = self.classes.read().await;
            if let Some(class) = classes.get(class_name) {
                return Ok((Arc::clone(class), true));
            }
        }

        // Convert hierarchy of class loaders to a flat list so that we can iterate over them from
        // the boot class loader to the current class loader.
        let mut class_loader = self
            .this
            .upgrade()
            .ok_or(InternalError("Unable to upgrade class loader".to_string()))?;
        let mut class_loaders = vec![class_loader.clone()];
        while let Some(parent) = class_loader.parent().await {
            class_loaders.push(parent.clone());
            class_loader = parent;
        }

        for class_loader in class_loaders.into_iter().rev() {
            let class_path = class_loader.class_path();
            if let Ok(class_file) = class_path.read_class(class_name).await {
                let mut classes = self.classes.write().await;
                // Check if the class was loaded while waiting for the lock.
                if let Some(class) = classes.get(class_name) {
                    return Ok((class.clone(), true));
                }
                let class_loader = Arc::downgrade(&class_loader);
                let class = Class::from(Some(class_loader), class_file)?;
                classes.insert(class_name.to_string(), class.clone());
                return Ok((class, false));
            }
        }

        Err(ClassNotFound(class_name.to_string()))
    }

    /// Register a class with the class loader.
    ///
    /// # Errors
    ///
    /// if the class cannot be registered.
    pub async fn register(&self, class: Arc<Class>) -> Result<()> {
        let mut classes = self.classes.write().await;
        let class_name = class.name().to_string();
        classes.insert(class_name, class);
        Ok(())
    }

    /// Get the object for the class loader.
    pub async fn object(&self) -> Option<Value> {
        let object_guard = self.object.read().await;
        object_guard.as_ref().cloned()
    }

    /// Set the object for the class loader.
    ///
    /// # Errors
    ///
    /// if the object cannot be set due to a poisoned lock.
    pub async fn set_object(&self, object: Option<Value>) {
        let mut object_guard = self.object.write().await;
        *object_guard = object;
    }
}

impl Clone for ClassLoader {
    /// Clone the class loader.
    fn clone(&self) -> Self {
        Self {
            this: self.this.clone(),
            name: self.name.clone(),
            class_path: self.class_path.clone(),
            parent: Arc::clone(&self.parent),
            classes: Arc::clone(&self.classes),
            object: Arc::clone(&self.object),
        }
    }
}

impl Display for ClassLoader {
    /// Display the class loader.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.class_path)?;
        Ok(())
    }
}

/// Implement equality for class loaders.
impl PartialEq for ClassLoader {
    /// Compare class loaders by name.
    fn eq(&self, other: &Self) -> bool {
        match (self.this.upgrade(), other.this.upgrade()) {
            (Some(self_loader), Some(other_loader)) => Arc::ptr_eq(&self_loader, &other_loader),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_new() {
        let name = "test";
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new(name, class_path);
        assert_eq!(name, class_loader.name());
        assert_eq!(&ClassPath::from("."), class_loader.class_path());
        assert!(class_loader.parent().await.is_none());
    }

    #[test]
    fn test_equality() {
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new("test", class_path);
        let class_loader2 = class_loader.clone();
        assert_eq!(class_loader, class_loader2);
    }

    #[test]
    fn test_inequality() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        assert_ne!(class_loader1, class_loader2);
    }

    #[tokio::test]
    async fn test_set_parent() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader2.set_parent(Some(class_loader1.clone())).await;
        assert_eq!(
            "test1",
            class_loader2.parent().await.expect("parent").name()
        );
        class_loader2.set_parent(None).await;
        assert_eq!(None, class_loader2.parent().await);
    }

    #[tokio::test]
    async fn test_load_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "HelloWorld";
        let class = class_loader.load(class_name).await?;
        assert_eq!(class_name, class.name());

        // Load the same class again to test caching
        let class = class_loader.load(class_name).await?;
        assert_eq!(class_name, class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_load_class_more_than_once() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "Simple";

        // Set a static value on the class to test class caching
        let expected_value = Value::Int(21);
        let class = &mut class_loader.load(class_name).await?;
        class.set_static_value("ANSWER", expected_value.clone())?;

        // Load the same class again and verify that the static value is still set
        let class = class_loader.load(class_name).await?;
        let value = class.static_value("ANSWER")?;
        assert_eq!(expected_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_class_parent() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];
        let class_path = ClassPath::from(class_path_entries.join(":"));
        let boot_class_loader = ClassLoader::new("test", class_path);
        let foo_class_path = ClassPath::from("foo");
        let class_loader = ClassLoader::new("test", foo_class_path);
        class_loader.set_parent(Some(boot_class_loader)).await;

        let class = class_loader.load("HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_load_class_not_found() {
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new("test", class_path);
        let result = class_loader.load("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
    }

    #[test]
    fn test_to_string() {
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new("test", class_path);
        assert_eq!("test=.", class_loader.to_string());
    }

    #[tokio::test]
    async fn test_to_string_parent() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader2.set_parent(Some(class_loader1)).await;
        // Note: Display implementation no longer shows parent chain due to async limitations
        assert_eq!("test2=.", class_loader2.to_string());
    }

    #[tokio::test]
    async fn test_clone() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar");
        let class_path = ClassPath::from(classes_jar.to_string_lossy());
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "HelloWorld";
        let _class = class_loader.load(class_name).await?;
        let clone = class_loader.clone();
        assert_eq!(class_loader, clone);
        assert_eq!(class_loader.class_path(), clone.class_path());
        let original_parent = class_loader.parent().await;
        let clone_parent = clone.parent().await;
        assert_eq!(original_parent.is_some(), clone_parent.is_some());
        assert_eq!(
            class_loader.classes.read().await.len(),
            clone.classes.read().await.len()
        );
        Ok(())
    }
}

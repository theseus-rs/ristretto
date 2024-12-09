use crate::Error::ClassNotFound;
use crate::{Class, ClassPath, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Implementation of a Java class loader.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html>
#[derive(Debug)]
pub struct ClassLoader {
    name: String,
    class_path: ClassPath,
    parent: Arc<Option<ClassLoader>>,
    classes: Arc<RwLock<HashMap<String, Arc<Class>>>>,
}

impl ClassLoader {
    /// Create a new class loader with the given name and parent.
    pub fn new<S: AsRef<str>>(name: S, class_path: ClassPath) -> Self {
        Self {
            name: name.as_ref().to_string(),
            class_path,
            parent: Arc::new(None),
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
        self.parent = Arc::new(parent);
    }

    /// Load a class by name.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub async fn load<S: AsRef<str>>(&self, name: S) -> Result<Arc<Class>> {
        self.load_with_status(name).await.map(|(class, _)| class)
    }

    /// Load a class by name with a boolean status indicating if the class was loaded previously.
    ///
    /// # Errors
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
        let mut class_loader = self;
        let mut class_loaders = vec![class_loader];
        while let Some(parent) = class_loader.parent() {
            class_loader = parent;
            class_loaders.push(parent);
        }

        for class_loader in class_loaders.into_iter().rev() {
            let class_path = class_loader.class_path();
            if let Ok(class_file) = class_path.read_class(class_name).await {
                let mut classes = self.classes.write().await;
                // Check if the class was loaded while waiting for the lock.
                if let Some(class) = classes.get(class_name) {
                    return Ok((class.clone(), true));
                }
                let class = Arc::new(Class::from(class_file)?);
                classes.insert(class_name.to_string(), class.clone());
                return Ok((class, false));
            }
        }

        Err(ClassNotFound(class_name.to_string()))
    }

    /// Register a class with the class loader.
    ///
    /// # Errors
    /// if the class cannot be registered.
    pub async fn register(&self, class: Arc<Class>) -> Result<()> {
        let mut classes = self.classes.write().await;
        let class_name = class.name().to_string();
        classes.insert(class_name, class);
        Ok(())
    }
}

impl Clone for ClassLoader {
    /// Clone the class loader.
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            class_path: self.class_path.clone(),
            parent: Arc::clone(&self.parent),
            classes: Arc::clone(&self.classes),
        }
    }
}

impl Display for ClassLoader {
    /// Display the class loader.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.class_path)?;

        let mut class_loader = self;
        while let Some(parent) = class_loader.parent() {
            class_loader = parent;
            write!(f, "; {}={}", class_loader.name, class_loader.class_path)?;
        }
        Ok(())
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
    use crate::Value;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let name = "test";
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new(name, class_path);
        assert_eq!(name, class_loader.name());
        assert_eq!(&ClassPath::from("."), class_loader.class_path());
        assert!(class_loader.parent().is_none());
    }

    #[test]
    fn test_equality() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test", class_path2);
        assert_eq!(class_loader1, class_loader2);
    }

    #[test]
    fn test_inequality() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        assert_ne!(class_loader1, class_loader2);
    }

    #[test]
    fn test_set_parent() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let mut class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader2.set_parent(Some(class_loader1));
        assert_eq!("test1", class_loader2.parent().expect("parent").name());
        class_loader2.set_parent(None);
        assert_eq!(None, class_loader2.parent());
    }

    #[tokio::test]
    async fn test_load_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
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
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "Simple";

        // Set a static value on the class to test class caching
        let expected_value = Value::Int(21);
        let class = &mut class_loader.load(class_name).await?;
        let answer_field = class.static_field("ANSWER")?;
        answer_field.set_value(expected_value.clone())?;

        // Load the same class again and verify that the static value is still set
        let class = class_loader.load(class_name).await?;
        let answer_field = class.static_field("ANSWER")?;
        let value = answer_field.value()?;
        assert_eq!(expected_value, value);
        Ok(())
    }

    #[tokio::test]
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

    #[test]
    fn test_to_string_parent() {
        let class_path1 = ClassPath::from(".");
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let mut class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader2.set_parent(Some(class_loader1));
        assert_eq!("test2=.; test1=.", class_loader2.to_string());
    }

    #[tokio::test]
    async fn test_clone() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let class_path = ClassPath::from(classes_jar.to_string_lossy());
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "HelloWorld";
        let _class = class_loader.load(class_name).await?;
        let clone = class_loader.clone();
        assert_eq!(class_loader, clone);
        assert_eq!(class_loader.class_path(), clone.class_path());
        assert_eq!(class_loader.parent(), clone.parent());
        assert_eq!(
            class_loader.classes.read().await.len(),
            clone.classes.read().await.len()
        );
        Ok(())
    }
}

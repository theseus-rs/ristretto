use crate::Error::{ClassNotFound, PoisonedLock};
use crate::{Class, ClassPath, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

/// Implementation of a Java class loader.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html>
#[derive(Debug)]
pub struct ClassLoader {
    name: String,
    class_path: ClassPath,
    parent: Option<Arc<ClassLoader>>,
    classes: Arc<RwLock<HashMap<String, Arc<Class>>>>,
}

impl ClassLoader {
    /// Create a new class loader with the given name and parent.
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
            self.parent = Some(Arc::new(parent));
        } else {
            self.parent = None;
        }
    }

    /// Load a class by name.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub fn load<S: AsRef<str>>(&self, name: S) -> Result<Arc<Class>> {
        self.load_with_status(name).map(|(class, _)| class)
    }

    /// Load a class by name with a boolean status indicating if the class was loaded previously.
    ///
    /// # Errors
    /// if the class file cannot be read.
    pub fn load_with_status<S: AsRef<str>>(&self, name: S) -> Result<(Arc<Class>, bool)> {
        let name = name.as_ref();
        {
            let classes = self
                .classes
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if let Some(class) = classes.get(name) {
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
            if let Ok(class_file) = class_path.read_class(name) {
                let mut classes = self
                    .classes
                    .write()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                // Check if the class was loaded while waiting for the lock.
                if let Some(class) = classes.get(name) {
                    return Ok((class.clone(), true));
                }
                let class = Arc::new(Class::from(class_file)?);
                classes.insert(name.to_string(), class.clone());
                return Ok((class, false));
            }
        }

        Err(ClassNotFound(name.to_string()))
    }

    /// Register a class with the class loader.
    ///
    /// # Errors
    /// if the class cannot be registered.
    pub fn register(&mut self, class: Arc<Class>) -> Result<()> {
        let mut classes = self
            .classes
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
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
            parent: self.parent.as_ref().map(Clone::clone),
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
        let mut class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(".");
        let class_loader2 = ClassLoader::new("test2", class_path2);
        class_loader1.set_parent(Some(class_loader2));
        assert_eq!("test2", class_loader1.parent().expect("parent").name());
    }

    #[test]
    fn test_load_class() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "HelloWorld";
        let class = class_loader.load(class_name)?;
        let class_file = class.class_file();
        assert_eq!(class_name, class_file.class_name()?);

        // Load the same class again to test caching
        let class = class_loader.load(class_name)?;
        let class_file = class.class_file();
        assert_eq!(class_name, class_file.class_name()?);
        Ok(())
    }

    #[test]
    fn test_load_class_more_than_once() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];

        let class_path = ClassPath::from(class_path_entries.join(":"));
        let class_loader = ClassLoader::new("test", class_path);
        let class_name = "Simple";

        // Set a static value on the class to test class caching
        let expected_value = Value::Int(21);
        let class = &mut class_loader.load(class_name)?;
        let answer_field = class.static_field("ANSWER")?;
        answer_field.set_value(expected_value.clone())?;

        // Load the same class again and verify that the static value is still set
        let class = class_loader.load(class_name)?;
        let answer_field = class.static_field("ANSWER")?;
        let value = answer_field.value()?;
        assert_eq!(expected_value, value);
        Ok(())
    }

    #[test]
    fn test_load_class_parent() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entries = [classes_directory.to_string_lossy().to_string()];
        let class_path = ClassPath::from(class_path_entries.join(":"));
        let boot_class_loader = ClassLoader::new("test", class_path);
        let foo_class_path = ClassPath::from("foo");
        let mut class_loader = ClassLoader::new("test", foo_class_path);
        class_loader.set_parent(Some(boot_class_loader));

        let class = class_loader.load("HelloWorld")?;
        let class_file = class.class_file();
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    #[test]
    fn test_load_class_not_found() {
        let class_path = ClassPath::from(".");
        let class_loader = ClassLoader::new("test", class_path);
        let result = class_loader.load("Foo");
        assert!(matches!(result, Err(ClassNotFound(_))));
    }
}

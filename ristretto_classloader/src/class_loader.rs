use crate::Error::ClassNotFound;
use crate::module::ResolvedConfiguration;
use crate::{Class, ClassPath, Result, Value};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, JAVA_1_0_2};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;

/// Implementation of a Java class loader.
///
/// # References
///
/// - [JVMS §5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html)
#[derive(Debug)]
pub struct ClassLoader {
    this: Weak<ClassLoader>,
    name: String,
    class_path: ClassPath,
    parent: Arc<RwLock<Option<Arc<ClassLoader>>>>,
    classes: Arc<RwLock<HashMap<String, Arc<Class>>>>,
    object: Arc<RwLock<Option<Value>>>,
    /// Module configuration for JPMS support.
    /// When set, the class loader will determine module names from packages during class loading.
    module_configuration: Arc<RwLock<Option<Arc<ResolvedConfiguration>>>>,
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
            module_configuration: Arc::new(RwLock::new(None)),
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
    pub async fn load(&self, name: &str) -> Result<Arc<Class>> {
        self.load_with_status(name).await.map(|(class, _)| class)
    }

    /// Load a class by name with a boolean status indicating if the class was loaded previously.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub async fn load_with_status(&self, class_name: &str) -> Result<(Arc<Class>, bool)> {
        let class_name_internal = class_name.replace('.', "/");
        let class_name_internal = class_name_internal.as_str();

        // Check if the class is already loaded in this class loader.
        {
            let classes = self.classes.read().await;
            if let Some(class) = classes.get(class_name_internal) {
                return Ok((Arc::clone(class), true));
            }
        }

        // Attempt to load the class from the parent class loader first.
        if let Some(parent) = self.parent().await
            && let Ok((class, loaded)) = Box::pin(parent.load_with_status(class_name)).await
        {
            return Ok((class, loaded));
        }

        if class_name_internal.starts_with('[') {
            if let Ok(class) = self.create_array_class(class_name_internal) {
                return Ok(self.cache_class(class_name_internal, class).await);
            }
        } else {
            let class_path = self.class_path();
            if let Ok(class_file) = class_path.read_class(class_name_internal).await {
                let class = Class::from(Some(self.this.clone()), class_file)?;
                self.set_class_module_name(&class, class_name_internal)
                    .await?;
                return Ok(self.cache_class(class_name_internal, class).await);
            }
        }

        Err(ClassNotFound(class_name_internal.to_string()))
    }

    /// Determines and sets the module name for a class based on its package.
    ///
    /// This uses the module configuration's package-to-module mapping to determine which module
    /// (if any) the class belongs to.
    async fn set_class_module_name(&self, class: &Arc<Class>, class_name: &str) -> Result<()> {
        let config_guard = self.module_configuration.read().await;
        if let Some(ref config) = *config_guard {
            // Extract package from class name (e.g., "java/lang/String" -> "java/lang")
            let package = Self::package_from_class_name(class_name);
            if let Some(module_name) = config.find_module_for_package(package) {
                class.set_module_name(Some(module_name.to_string()))?;
            }
        }
        Ok(())
    }

    /// Extracts the package name from a fully qualified class name.
    ///
    /// # Examples
    ///
    /// - `java/lang/String` → `java/lang`
    /// - `com/example/MyClass` → `com/example`
    /// - `MyClass` → empty string (default package)
    #[must_use]
    pub fn package_from_class_name(class_name: &str) -> &str {
        if let Some(last_slash) = class_name.rfind('/') {
            &class_name[..last_slash]
        } else {
            // Default package
            ""
        }
    }

    /// Cache a class in the class loader.
    async fn cache_class(&self, class_name: &str, class: Arc<Class>) -> (Arc<Class>, bool) {
        let mut classes = self.classes.write().await;
        // Check if the class was loaded while waiting for the lock.
        if let Some(class) = classes.get(class_name) {
            return (class.clone(), true);
        }
        classes.insert(class_name.to_string(), class.clone());
        (class, false)
    }

    /// Create an array class.
    ///
    /// Array classes are created by the JVM, not loaded from class files. The array class is
    /// created in the same class loader as its component type.
    ///
    /// # Reference
    ///
    /// - [JVMS §5.3.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.3.3)
    fn create_array_class(&self, class_name: &str) -> Result<Arc<Class>> {
        // Validate the array descriptor before creating the class
        // Valid array descriptors:
        // - [B, [C, [D, [F, [I, [J, [S, [Z for primitive arrays
        // - [Lclassname; for object arrays
        // - [[... for multi-dimensional arrays
        let component_type = class_name
            .strip_prefix('[')
            .ok_or_else(|| ClassNotFound(class_name.to_string()))?;

        // Validate component type is a valid descriptor
        let is_valid = match component_type.chars().next() {
            Some('B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z') if component_type.len() == 1 => {
                true
            }
            Some('[') => true, // multi-dimensional array
            Some('L') => component_type.ends_with(';') && component_type.len() > 2,
            _ => false,
        };

        if !is_valid {
            return Err(ClassNotFound(class_name.to_string()));
        }

        let mut constant_pool = ConstantPool::new();
        let this_class_index = constant_pool.add_class(class_name)?;
        let super_class_index = constant_pool.add_class("java/lang/Object")?;
        let cloneable_index = constant_pool.add_class("java/lang/Cloneable")?;
        let serializable_index = constant_pool.add_class("java/io/Serializable")?;
        let class_file = ClassFile {
            version: JAVA_1_0_2,
            constant_pool,
            access_flags: ClassAccessFlags::PUBLIC
                | ClassAccessFlags::FINAL
                | ClassAccessFlags::ABSTRACT,
            this_class: this_class_index,
            super_class: super_class_index,
            interfaces: vec![cloneable_index, serializable_index],
            ..Default::default()
        };

        Class::from(Some(self.this.clone()), class_file)
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

    /// Register all classes with the class loader.
    ///
    /// # Errors
    ///
    /// if the class cannot be registered.
    pub async fn register_all<I>(&self, class: I) -> Result<()>
    where
        I: IntoIterator<Item = Arc<Class>>,
    {
        let mut classes = self.classes.write().await;
        for class in class {
            let class_name = class.name().to_string();
            classes.entry(class_name).or_insert_with(|| class);
        }
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

    /// Get the module configuration for JPMS support.
    pub async fn module_configuration(&self) -> Option<Arc<ResolvedConfiguration>> {
        let config_guard = self.module_configuration.read().await;
        config_guard.clone()
    }

    /// Set the module configuration for JPMS support.
    ///
    /// When set, the class loader will determine module names for loaded classes
    /// based on the package-to-module mapping in the configuration.
    pub async fn set_module_configuration(&self, config: Option<Arc<ResolvedConfiguration>>) {
        let mut config_guard = self.module_configuration.write().await;
        *config_guard = config;
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
            module_configuration: Arc::clone(&self.module_configuration),
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
        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new(name, class_path);
        assert_eq!(name, class_loader.name());
        assert_eq!(&ClassPath::from(&["."]), class_loader.class_path());
        assert!(class_loader.parent().await.is_none());
    }

    #[test]
    fn test_equality() {
        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);
        let class_loader2 = class_loader.clone();
        assert_eq!(class_loader, class_loader2);
    }

    #[test]
    fn test_inequality() {
        let class_path1 = ClassPath::from(&["."]);
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(&["."]);
        let class_loader2 = ClassLoader::new("test2", class_path2);
        assert_ne!(class_loader1, class_loader2);
    }

    #[tokio::test]
    async fn test_set_parent() {
        let class_path1 = ClassPath::from(&["."]);
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(&["."]);
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
        let class_path = ClassPath::from(&[classes_directory]);
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
        let class_path = ClassPath::from(&[classes_directory]);
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
        let class_path = ClassPath::from(&[classes_directory]);
        let boot_class_loader = ClassLoader::new("test", class_path);
        let foo_class_path = ClassPath::from(&["foo"]);
        let class_loader = ClassLoader::new("test", foo_class_path);
        class_loader.set_parent(Some(boot_class_loader)).await;

        let class = class_loader.load("HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_load_order() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("..").join("classes");
        let class_path = ClassPath::from(&[classes_directory]);
        let parent_loader = ClassLoader::new("parent", class_path.clone());
        let child_loader = ClassLoader::new("child", class_path.clone());
        child_loader.set_parent(Some(parent_loader.clone())).await;

        // Load the class in the parent loader
        let parent_class = parent_loader.load("HelloWorld").await?;

        // Register a fake array class in the child loader with a distinct name
        // (using proper array notation to satisfy validation)
        let fake_class_name = "[LHelloWorld;";
        let fake_class = child_loader.create_array_class(fake_class_name)?;
        child_loader.register(fake_class.clone()).await?;

        // Load the class from the child loader - should delegate to parent for HelloWorld
        // but return the registered class for [LHelloWorld;
        let loaded_class = child_loader.load("[LHelloWorld;").await?;

        // Verify that the loaded class is the one from the child loader, not the parent
        assert_eq!(fake_class, loaded_class);
        assert_ne!(parent_class, loaded_class);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_class_not_found() {
        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);
        let result = class_loader.load("Foo").await;
        assert!(matches!(result, Err(ClassNotFound(_))));
    }

    #[test]
    fn test_to_string() {
        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);
        assert_eq!("test=.", class_loader.to_string());
    }

    #[tokio::test]
    async fn test_to_string_parent() {
        let class_path1 = ClassPath::from(&["."]);
        let class_loader1 = ClassLoader::new("test1", class_path1);
        let class_path2 = ClassPath::from(&["."]);
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
        let class_path = ClassPath::from(&[classes_jar]);
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

    #[tokio::test]
    async fn test_module_configuration() -> Result<()> {
        use crate::module::ResolvedConfiguration;
        use std::collections::{BTreeMap, HashMap};

        // Create a module configuration with a package-to-module mapping
        let mut package_to_module = BTreeMap::new();
        package_to_module.insert("test/pkg".to_string(), "test.module".to_string());

        let config = ResolvedConfiguration::new(
            BTreeMap::new(),
            package_to_module,
            HashMap::new(),
            HashMap::new(),
        );

        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);

        // Verify no module configuration initially
        assert!(class_loader.module_configuration().await.is_none());

        // Set module configuration
        class_loader
            .set_module_configuration(Some(Arc::new(config)))
            .await;

        // Verify configuration is set
        assert!(class_loader.module_configuration().await.is_some());

        Ok(())
    }

    #[test]
    fn test_package_from_class_name() {
        assert_eq!(
            ClassLoader::package_from_class_name("java/lang/String"),
            "java/lang"
        );
        assert_eq!(
            ClassLoader::package_from_class_name("com/example/MyClass"),
            "com/example"
        );
        assert_eq!(ClassLoader::package_from_class_name("MyClass"), "");
        assert_eq!(ClassLoader::package_from_class_name("a/b/c/D"), "a/b/c");
    }
}

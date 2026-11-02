use crate::Error::{ClassNotFound, InternalError};
use crate::module::{ModuleDescriptor, ModuleDescriptorCache, ResolvedConfiguration};
use crate::{Class, ClassLoaderType, ClassPath, Result, Value};
use ahash::AHashMap;
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, Constant, ConstantPool, JAVA_1_0_2, JavaStr, JavaString,
};
use std::borrow::Cow;
use std::fmt::Display;
use std::path::Path;
use std::sync::{Arc, LazyLock, Weak};
use tokio::sync::RwLock;

/// Synchronous lock for module configuration to avoid async overhead on every class load.
type ModuleConfigLock = ristretto_gc::sync::RwLock<Option<Arc<ResolvedConfiguration>>>;

// All arrays share these constants. Cloning this template allocates one constant
// pool vector and borrows the fixed UTF-8 names instead of allocating each name.
static ARRAY_CLASS_TEMPLATE: LazyLock<Result<ClassFile<'static>>> = LazyLock::new(|| {
    fn add_class(pool: &mut ConstantPool<'static>, name: &'static str) -> Result<u16> {
        let index = pool.add(Constant::Utf8(Cow::Borrowed(JavaStr::try_from_str(name)?)))?;
        Ok(pool.add(Constant::Class(index))?)
    }
    let mut constant_pool = ConstantPool::new();
    let this_class = add_class(&mut constant_pool, "")?; // Array name replaces UTF-8 entry 1.
    let super_class = add_class(&mut constant_pool, "java/lang/Object")?;
    let cloneable = add_class(&mut constant_pool, "java/lang/Cloneable")?;
    let serializable = add_class(&mut constant_pool, "java/io/Serializable")?;
    Ok(ClassFile {
        version: JAVA_1_0_2,
        constant_pool,
        access_flags: ClassAccessFlags::PUBLIC
            | ClassAccessFlags::FINAL
            | ClassAccessFlags::ABSTRACT,
        this_class,
        super_class,
        interfaces: vec![cloneable, serializable],
        ..Default::default()
    })
});

/// Implementation of a Java class loader.
///
/// # References
///
/// - [JVMS §5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html)
#[derive(Debug)]
pub struct ClassLoader {
    this: Weak<ClassLoader>,
    name: String,
    loader_type: Option<ClassLoaderType>,
    class_path: ClassPath,
    parent: Arc<RwLock<Option<Arc<ClassLoader>>>>,
    classes: Arc<RwLock<AHashMap<JavaString, Arc<Class>>>>,
    object: Arc<RwLock<Option<Value>>>,
    /// Module configuration for JPMS support.
    /// When set, the class loader will determine module names from packages during class loading.
    /// Uses a synchronous lock to avoid async overhead on every class load.
    module_configuration: Arc<ModuleConfigLock>,
    /// System module descriptors cached for the lifetime of this class loader.
    module_descriptor_cache: Arc<ModuleDescriptorCache>,
}

impl ClassLoader {
    /// Create a custom class loader with the given name and class path.
    pub fn new<S: AsRef<str>>(name: S, class_path: ClassPath) -> Arc<Self> {
        Self::new_inner(name.as_ref(), None, class_path)
    }

    /// Create a built-in class loader with the name defined by the Java runtime.
    #[must_use]
    pub fn new_builtin(loader_type: ClassLoaderType, class_path: ClassPath) -> Arc<Self> {
        Self::new_inner(loader_type.name(), Some(loader_type), class_path)
    }

    fn new_inner(
        name: &str,
        loader_type: Option<ClassLoaderType>,
        class_path: ClassPath,
    ) -> Arc<Self> {
        Arc::new_cyclic(|weak_self| ClassLoader {
            this: weak_self.clone(),
            name: name.to_string(),
            loader_type,
            class_path,
            parent: Arc::new(RwLock::new(None)),
            classes: Arc::new(RwLock::new(AHashMap::default())),
            object: Arc::new(RwLock::new(None)),
            module_configuration: Arc::new(ModuleConfigLock::new(None)),
            module_descriptor_cache: Arc::new(ModuleDescriptorCache::default()),
        })
    }

    /// Get the name of the class loader.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the role of this built-in class loader, or `None` for a custom class loader.
    #[must_use]
    pub fn loader_type(&self) -> Option<ClassLoaderType> {
        self.loader_type
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
    pub async fn load(&self, name: &JavaStr) -> Result<Arc<Class>> {
        self.load_with_status(name).await.map(|(class, _)| class)
    }

    /// Load a class by name with a boolean status indicating if the class was loaded previously.
    ///
    /// # Errors
    ///
    /// if the class file cannot be read.
    pub async fn load_with_status(&self, class_name: &JavaStr) -> Result<(Arc<Class>, bool)> {
        let class_name_internal: Cow<'_, JavaStr> =
            if memchr::memchr(b'.', class_name.as_bytes()).is_none() {
                Cow::Borrowed(class_name)
            } else {
                // '.' and '/' are both single-byte ASCII; replacing one with
                // the other preserves MUTF-8 validity.
                let mut bytes = class_name.as_bytes().to_vec();
                for b in &mut bytes {
                    if *b == b'.' {
                        *b = b'/';
                    }
                }
                // SAFETY: class_name is already valid MUTF-8 and replacing ASCII '.'
                // with ASCII '/' preserves MUTF-8 validity.
                #[expect(unsafe_code)]
                Cow::Owned(unsafe { JavaString::from_mutf8_unchecked(bytes) })
            };

        // Check if the class is already loaded in this class loader.
        {
            let classes = self.classes.read().await;
            if let Some(class) = classes.get(&*class_name_internal) {
                return Ok((Arc::clone(class), true));
            }
        }

        // Ordinary classes delegate before entering the large I/O future. Do
        // not cache parent definitions here: legacy anonymous-class registration
        // can replace them, and an initiating alias would retain stale metadata.
        if !class_name_internal.as_bytes().starts_with(b"[")
            && let Some(parent) = self.parent().await
            && let Ok(class) = Box::pin(parent.load_with_status(&class_name_internal)).await
        {
            return Ok(class);
        }
        // An empty path cannot supply an ordinary definition. Avoid allocating
        // the I/O future (and a second not-found error) for these common misses.
        if !class_name_internal.as_bytes().starts_with(b"[")
            && self.class_path.iter().next().is_none()
        {
            return Err(ClassNotFound(class_name_internal.to_rust_string()));
        }
        // Keep the large I/O/synthesis future off the cached lookup path.
        Box::pin(self.load_uncached(&class_name_internal)).await
    }

    async fn load_uncached(&self, class_name: &JavaStr) -> Result<(Arc<Class>, bool)> {
        let class_name_str = class_name.to_str_lossy();
        if class_name_str.starts_with('[') {
            return self.load_array_class(class_name, &class_name_str).await;
        }

        let include_code_source = self.parent().await.is_some();
        if let Ok(class_file) = self
            .class_path
            .read_class_with_code_source(class_name_str.as_ref(), include_code_source)
            .await
        {
            let class = Class::from_shared(Some(self.this.clone()), class_file)?;
            self.set_class_module_name(&class, &class_name_str)?;
            return Ok(self.cache_class(class_name.to_owned(), class).await);
        }
        Err(ClassNotFound(class_name.to_rust_string()))
    }

    /// Determines and sets the module name for a class based on its package.
    ///
    /// This uses the module configuration's package-to-module mapping to determine which module
    /// (if any) the class belongs to. Uses a synchronous lock to avoid async overhead.
    fn set_class_module_name(&self, class: &Arc<Class>, class_name: &str) -> Result<()> {
        let config_guard = self.module_configuration.read();
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
    /// - `java/lang/String` -> `java/lang`
    /// - `com/example/MyClass` -> `com/example`
    /// - `MyClass` -> empty string (default package)
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
    async fn cache_class(&self, class_name: JavaString, class: Arc<Class>) -> (Arc<Class>, bool) {
        let mut classes = self.classes.write().await;
        // Check if the class was loaded while waiting for the lock.
        if let Some(class) = classes.get(&*class_name) {
            return (class.clone(), true);
        }
        classes.insert(class_name, class.clone());
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
    async fn load_array_class(
        &self,
        name: &JavaStr,
        class_name: &str,
    ) -> Result<(Arc<Class>, bool)> {
        let base_descriptor = class_name.trim_start_matches('[');
        let dimensions = class_name.len() - base_descriptor.len();
        let base_name = base_descriptor
            .strip_prefix('L')
            .and_then(|name| name.strip_suffix(';'));
        let valid_base = if let Some(base_name) = base_name {
            !base_name.is_empty() && !base_name.contains(['[', ';'])
        } else {
            matches!(
                base_descriptor,
                "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z"
            )
        };
        if !(1..=255).contains(&dimensions) || !valid_base {
            return Err(ClassNotFound(class_name.to_string()));
        }

        // Resolve once through the initiating loader, then use the component's
        // defining cache. Probing every parent's array factory repeats failed
        // component lookups for classes defined only in a child.
        let array_base_class = match base_name {
            Some(base_name) => Some(self.load(&JavaStr::cow_from_str(base_name)).await?),
            None => None,
        };
        let defining_loader = if let Some(component) = &array_base_class {
            component.class_loader()?
        } else {
            let mut bootstrap = self.parent().await;
            while let Some(loader) = &bootstrap {
                let Some(parent) = loader.parent().await else {
                    break;
                };
                bootstrap = Some(parent);
            }
            bootstrap
        };

        let defining_loader = defining_loader.as_deref().unwrap_or(self);
        let is_defining_loader = std::ptr::eq(self, defining_loader);

        if !is_defining_loader {
            let cached = defining_loader.classes.read().await.get(name).cloned();
            if let Some(class) = cached {
                let (class, _) = self.cache_class(name.to_owned(), class).await;
                return Ok((class, true));
            }
        }

        let mut class_file = ARRAY_CLASS_TEMPLATE
            .as_ref()
            .map_err(|error| InternalError(error.to_string()))?
            .clone();
        class_file
            .constant_pool
            .set(1, Constant::Utf8(Cow::Owned(name.to_owned())))?;
        let class = Class::from_array(
            Some(defining_loader.this.clone()),
            class_file,
            array_base_class,
        )?;
        let (class, loaded) = defining_loader.cache_class(name.to_owned(), class).await;
        if is_defining_loader {
            return Ok((class, loaded));
        }
        let (class, raced) = self.cache_class(name.to_owned(), class).await;
        Ok((class, loaded || raced))
    }

    /// Register a class with the class loader.
    ///
    /// # Errors
    ///
    /// if the class cannot be registered.
    pub async fn register(&self, class: Arc<Class>) -> Result<()> {
        let mut classes = self.classes.write().await;
        let class_name = JavaString::from(class.java_name());
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
            let class_name = JavaString::from(class.java_name());
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
    #[must_use]
    pub fn module_configuration(&self) -> Option<Arc<ResolvedConfiguration>> {
        let config_guard = self.module_configuration.read();
        config_guard.clone()
    }

    /// Set the module configuration for JPMS support.
    ///
    /// When set, the class loader will determine module names for loaded classes
    /// based on the package-to-module mapping in the configuration.
    pub fn set_module_configuration(&self, config: Option<Arc<ResolvedConfiguration>>) {
        let mut config_guard = self.module_configuration.write();
        *config_guard = config;
    }

    /// Reads and caches a system module descriptor for this class loader.
    ///
    /// # Errors
    ///
    /// Returns an error if the jimage or module descriptor cannot be read.
    pub async fn read_module_descriptor(
        &self,
        jimage_path: &Path,
        module_name: &str,
    ) -> crate::module::Result<ModuleDescriptor> {
        Arc::clone(&self.module_descriptor_cache)
            .read_descriptor(jimage_path, module_name)
            .await
    }

    /// Get all loaded classes.
    pub async fn loaded_classes(&self) -> Vec<Arc<Class>> {
        let classes = self.classes.read().await;
        classes.values().cloned().collect()
    }

    /// Check whether a class with the given name has already been loaded (cached) by this
    /// classloader or any of its parents.
    ///
    /// Unlike [`load`](Self::load), this method never reads from disk or creates new classes.
    /// It only checks the in-memory caches.
    ///
    /// Per [JVMS §5.3.4](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.3.4),
    /// `ClassLoader.findLoadedClass` must return a previously loaded class without triggering
    /// new class loading.
    pub async fn find_loaded(&self, class_name: &JavaStr) -> Option<Arc<Class>> {
        let class_name_internal: Cow<'_, JavaStr> =
            if memchr::memchr(b'.', class_name.as_bytes()).is_none() {
                Cow::Borrowed(class_name)
            } else {
                let mut bytes = class_name.as_bytes().to_vec();
                for b in &mut bytes {
                    if *b == b'.' {
                        *b = b'/';
                    }
                }
                #[expect(unsafe_code)]
                // SAFETY: class_name is already valid MUTF-8 and replacing ASCII '.' with ASCII '/'
                // preserves MUTF-8 validity.
                Cow::Owned(unsafe { JavaString::from_mutf8_unchecked(bytes) })
            };

        // Walk the classloader hierarchy iteratively
        let mut current = self.this.upgrade()?;
        loop {
            {
                let classes = current.classes.read().await;
                if let Some(class) = classes.get(&*class_name_internal) {
                    return Some(Arc::clone(class));
                }
            }

            match current.parent().await {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    /// Traverse the classloader hierarchy to find the bootstrap (root) classloader.
    ///
    /// # Errors
    ///
    /// Returns an error if the classloader has already been dropped.
    pub async fn bootstrap(&self) -> Result<Arc<ClassLoader>> {
        let mut current = self.this.upgrade().ok_or_else(|| {
            InternalError("classloader self reference has already been dropped".to_string())
        })?;
        loop {
            match current.parent().await {
                Some(parent) => current = parent,
                None => return Ok(current),
            }
        }
    }
}

impl Clone for ClassLoader {
    /// Clone the class loader.
    fn clone(&self) -> Self {
        Self {
            this: self.this.clone(),
            name: self.name.clone(),
            loader_type: self.loader_type,
            class_path: self.class_path.clone(),
            parent: Arc::clone(&self.parent),
            classes: Arc::clone(&self.classes),
            object: Arc::clone(&self.object),
            module_configuration: Arc::clone(&self.module_configuration),
            module_descriptor_cache: Arc::clone(&self.module_descriptor_cache),
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
        assert_eq!(None, class_loader.loader_type());
        assert_eq!(&ClassPath::from(&["."]), class_loader.class_path());
        assert!(class_loader.parent().await.is_none());
    }

    #[test]
    fn test_new_builtin() {
        let class_path = ClassPath::from(&["."]);
        for (loader_type, name) in [
            (ClassLoaderType::Bootstrap, "bootstrap"),
            (ClassLoaderType::Platform, "platform"),
            (ClassLoaderType::System, "app"),
        ] {
            let class_loader = ClassLoader::new_builtin(loader_type, class_path.clone());
            assert_eq!(name, class_loader.name());
            assert_eq!(Some(loader_type), class_loader.loader_type());
        }
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
        let class = class_loader
            .load(JavaStr::try_from_str(class_name)?)
            .await?;
        assert_eq!(class_name, class.name());
        assert_eq!(None, class.class_file().code_source_url);

        // Load the same class again to test caching
        let class = class_loader
            .load(JavaStr::try_from_str(class_name)?)
            .await?;
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
        let class = &mut class_loader
            .load(JavaStr::try_from_str(class_name)?)
            .await?;
        class.set_static_value("ANSWER", expected_value.clone())?;

        // Load the same class again and verify that the static value is still set
        let class = class_loader
            .load(JavaStr::try_from_str(class_name)?)
            .await?;
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

        let class = class_loader
            .load(JavaStr::try_from_str("HelloWorld")?)
            .await?;
        assert_eq!("HelloWorld", class.name());
        assert_eq!(None, class.class_file().code_source_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_child_class_code_source() -> Result<()> {
        let classes_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("classes");
        let classes_jar = classes_directory.join("classes.jar");

        for source in [classes_directory, classes_jar] {
            let class_path = ClassPath::from(&[source]);
            let expected_code_source = class_path.read_class("HelloWorld").await?.code_source_url;
            assert!(expected_code_source.is_some());

            let parent =
                ClassLoader::new_builtin(ClassLoaderType::Bootstrap, ClassPath::new(Vec::new()));
            let child = ClassLoader::new_builtin(ClassLoaderType::System, class_path);
            child.set_parent(Some(parent)).await;

            let class = child.load(JavaStr::try_from_str("HelloWorld")?).await?;
            assert_eq!(expected_code_source, class.class_file().code_source_url);
        }
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
        let parent_class = parent_loader
            .load(JavaStr::try_from_str("HelloWorld")?)
            .await?;

        // Register a distinct array definition in the child under the same name.
        let fake_class_name = "[LHelloWorld;";
        let parent_array = parent_loader
            .load(JavaStr::try_from_str(fake_class_name)?)
            .await?;
        let fake_class = Class::from_array(
            Some(Arc::downgrade(&child_loader)),
            parent_array.class_file().clone(),
            Some(parent_class),
        )?;
        child_loader.register(fake_class.clone()).await?;

        // Load the class from the child loader; should delegate to parent for HelloWorld
        // but return the registered class for [LHelloWorld;
        let loaded_class = child_loader
            .load(JavaStr::try_from_str("[LHelloWorld;")?)
            .await?;

        // Verify that the loaded class is the one from the child loader, not the parent
        assert!(Arc::ptr_eq(&fake_class, &loaded_class));
        assert!(!Arc::ptr_eq(&parent_array, &loaded_class));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_class_not_found() {
        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);
        let result = class_loader
            .load(JavaStr::try_from_str("Foo").expect("valid class name"))
            .await;
        assert!(matches!(result, Err(ClassNotFound(_))));
    }

    #[tokio::test]
    async fn test_hidden_array_names_and_identity() -> Result<()> {
        let parent = ClassLoader::new("parent", ClassPath::new(Vec::new()));
        let loader = ClassLoader::new("child", ClassPath::new(Vec::new()));
        loader.set_parent(Some(parent.clone())).await;

        for suffix in [42, 43] {
            let mut constant_pool = ConstantPool::new();
            let this_class = constant_pool.add_class("example/Generated+0xbar")?;
            let class_file = ClassFile {
                constant_pool,
                this_class,
                ..Default::default()
            };
            let hidden = Class::from_hidden(Some(Arc::downgrade(&loader)), class_file, suffix)?;
            loader.register(hidden.clone()).await?;

            for dimensions in [1, 2] {
                let prefix = "[".repeat(dimensions);
                let internal = [prefix.as_str(), "L", hidden.name(), ";"].concat();
                let public = [prefix.as_str(), "L", &hidden.public_name(), ";"].concat();
                let array = loader.load(&JavaStr::cow_from_str(&internal)).await?;
                let cached = loader.load(array.java_name()).await?;
                assert!(Arc::ptr_eq(&array, &cached));
                assert_eq!(array.name(), internal);
                assert_eq!(array.public_name(), public);
                assert!(!array.is_hidden());
                assert!(array.verify_cached().is_ok());
                assert_eq!(array.class_loader()?, Some(loader.clone()));
                // Public hidden names belong to mirrors, not symbolic lookup.
                assert!(loader.load(&JavaStr::cow_from_str(&public)).await.is_err());
                assert!(
                    parent
                        .find_loaded(&JavaStr::cow_from_str(&internal))
                        .await
                        .is_none()
                );
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_parent_registration_replacement_remains_visible() -> Result<()> {
        let parent = ClassLoader::new("parent", ClassPath::new(Vec::new()));
        let child = ClassLoader::new("child", ClassPath::new(Vec::new()));
        child.set_parent(Some(parent.clone())).await;
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("Generated")?;
        let file = ClassFile {
            constant_pool: pool,
            this_class,
            ..Default::default()
        };
        let first = Class::from(Some(Arc::downgrade(&parent)), file.clone())?;
        parent.register(first.clone()).await?;
        assert!(Arc::ptr_eq(&first, &child.load(first.java_name()).await?));
        // Legacy anonymous method-handle classes may reuse a registration name.
        let replacement = Class::from(Some(Arc::downgrade(&parent)), file)?;
        parent.register(replacement.clone()).await?;
        assert!(Arc::ptr_eq(
            &replacement,
            &child.load(replacement.java_name()).await?
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_array_descriptors() -> Result<()> {
        let loader = ClassLoader::new("test", ClassPath::new(Vec::new()));
        for descriptor in ["[", "[[V", "[[X", "[[I;", "[[L;", "[[", "[Lfoo;;"] {
            assert!(matches!(
                loader.load(&JavaStr::cow_from_str(descriptor)).await,
                Err(ClassNotFound(_))
            ));
        }
        let too_deep = ["[".repeat(256).as_str(), "I"].concat();
        assert!(
            loader
                .load(&JavaStr::cow_from_str(&too_deep))
                .await
                .is_err()
        );
        let deepest = ["[".repeat(255).as_str(), "I"].concat();
        assert!(
            loader
                .load(&JavaStr::cow_from_str(&deepest))
                .await?
                .is_array()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_hidden_array_uses_defining_loader_cache() -> Result<()> {
        let bootstrap =
            ClassLoader::new_builtin(ClassLoaderType::Bootstrap, ClassPath::new(Vec::new()));
        let app = ClassLoader::new_builtin(ClassLoaderType::System, ClassPath::new(Vec::new()));
        app.set_parent(Some(bootstrap.clone())).await;
        let sibling = ClassLoader::new("sibling", ClassPath::new(Vec::new()));
        sibling.set_parent(Some(bootstrap.clone())).await;

        let mut constant_pool = ConstantPool::new();
        let this_class = constant_pool.add_class("example/BootstrapHidden")?;
        let hidden = Class::from_hidden(
            Some(Arc::downgrade(&bootstrap)),
            ClassFile {
                constant_pool,
                this_class,
                ..Default::default()
            },
            42,
        )?;
        // Native hidden-class definition registers with the VM's top-level loader,
        // while the class retains its bootstrap defining-loader metadata.
        app.register(hidden.clone()).await?;
        sibling.register(hidden.clone()).await?;
        assert!(bootstrap.find_loaded(hidden.java_name()).await.is_none());

        for dimensions in [1, 2] {
            let prefix = "[".repeat(dimensions);
            let descriptor = [prefix.as_str(), "L", hidden.name(), ";"].concat();
            let name = JavaStr::cow_from_str(&descriptor);
            let array = app.load(&name).await?;
            assert_eq!(array.class_loader()?, Some(bootstrap.clone()));
            assert!(
                bootstrap
                    .find_loaded(&name)
                    .await
                    .is_some_and(|cached| Arc::ptr_eq(&array, &cached))
            );
            assert!(Arc::ptr_eq(&array, &app.load(&name).await?));
            assert!(Arc::ptr_eq(&array, &sibling.load(&name).await?));
            assert_eq!(
                array.public_name(),
                [prefix.as_str(), "L", &hidden.public_name(), ";"].concat()
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_ordinary_hidden_like_array_names() -> Result<()> {
        let loader = ClassLoader::new("test", ClassPath::new(Vec::new()));
        for name in [
            "example/Generated+0xbar",
            "example/Generated+0x000000000000002a",
            "ordinary/0x000000000000002a",
        ] {
            let mut constant_pool = ConstantPool::new();
            let this_class = constant_pool.add_class(name)?;
            let class_file = ClassFile {
                constant_pool,
                this_class,
                ..Default::default()
            };
            let class = Class::from(Some(Arc::downgrade(&loader)), class_file)?;
            assert!(!class.is_hidden());
            assert_eq!(class.public_name(), name.replace('/', "."));
            loader.register(class).await?;

            for dimensions in [1, 2] {
                let descriptor = ["[".repeat(dimensions).as_str(), "L", name, ";"].concat();
                let array = loader.load(&JavaStr::cow_from_str(&descriptor)).await?;
                assert!(!array.is_hidden());
                assert_eq!(array.public_name(), descriptor.replace('/', "."));
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_ordinary_internal_name_is_not_a_hidden_alias() -> Result<()> {
        let loader = ClassLoader::new("test", ClassPath::new(Vec::new()));
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("ordinary")?;
        let hidden = Class::from_hidden(
            Some(Arc::downgrade(&loader)),
            ClassFile {
                constant_pool: pool,
                this_class,
                ..Default::default()
            },
            42,
        )?;
        loader.register(hidden.clone()).await?;

        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("ordinary/0x000000000000002a")?;
        let ordinary = Class::from(
            Some(Arc::downgrade(&loader)),
            ClassFile {
                constant_pool: pool,
                this_class,
                ..Default::default()
            },
        )?;
        loader.register(ordinary.clone()).await?;
        assert_eq!(ordinary.name(), hidden.public_name());

        for class in [&ordinary, &hidden] {
            let resolved = loader.load(class.java_name()).await?;
            assert!(Arc::ptr_eq(class, &resolved));
            let descriptor = ["[L", class.name(), ";"].concat();
            let array = loader.load(&JavaStr::cow_from_str(&descriptor)).await?;
            assert_eq!(
                array.public_name(),
                ["[L", &class.public_name(), ";"].concat()
            );
            let component = loader
                .load(&JavaStr::cow_from_str(array.array_base_element_type()))
                .await?;
            assert!(Arc::ptr_eq(class, &component));
        }
        Ok(())
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
        let _class = class_loader
            .load(JavaStr::try_from_str(class_name)?)
            .await?;
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
        use ahash::AHashMap;
        use std::collections::BTreeMap;

        // Create a module configuration with a package-to-module mapping
        let mut package_to_module = AHashMap::default();
        package_to_module.insert("test/pkg".to_string(), "test.module".to_string());

        let config = ResolvedConfiguration::new(
            BTreeMap::new(),
            package_to_module,
            AHashMap::default(),
            AHashMap::default(),
        );

        let class_path = ClassPath::from(&["."]);
        let class_loader = ClassLoader::new("test", class_path);

        // Verify no module configuration initially
        assert!(class_loader.module_configuration().is_none());

        // Set module configuration
        class_loader.set_module_configuration(Some(Arc::new(config)));

        // Verify configuration is set
        assert!(class_loader.module_configuration().is_some());

        Ok(())
    }

    #[test]
    fn test_module_descriptor_cache_lifecycle() {
        let class_loader = ClassLoader::new("test", ClassPath::from(&["."]));
        let descriptor_cache = Arc::downgrade(&class_loader.module_descriptor_cache);

        assert!(descriptor_cache.upgrade().is_some());
        drop(class_loader);
        assert!(descriptor_cache.upgrade().is_none());
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

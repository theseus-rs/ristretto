use crate::java_object::JavaObject;
use crate::rust_value::RustValue;
use crate::thread::Thread;
use crate::Error::{InternalError, UnsupportedClassFileVersion};
use crate::{Configuration, ConfigurationBuilder, Result};
use ristretto_classfile::Version;
use ristretto_classloader::manifest::MAIN_CLASS;
use ristretto_classloader::{
    runtime, Class, ClassLoader, ClassPath, ClassPathEntry, ConcurrentVec, Method, Object,
    Reference, Value,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tracing::debug;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Java Virtual Machine
#[derive(Debug)]
pub struct VM {
    vm: Weak<VM>,
    configuration: Configuration,
    class_loader: Arc<RwLock<ClassLoader>>,
    main_class: Option<String>,
    java_home: PathBuf,
    java_version: String,
    java_class_file_version: Version,
}

/// VM
impl VM {
    /// The offset to add to the major version to get the class file version.  Java 1.0 has a class
    /// file major version of 45, so the class file major version is the Java version (1) + the
    /// class file offset version (44) = the Java 1 class file version (45).
    const CLASS_FILE_MAJOR_VERSION_OFFSET: u16 = 44;

    /// Create a new VM
    ///
    /// # Errors
    /// if the VM cannot be created
    pub async fn new(configuration: Configuration) -> Result<Arc<Self>> {
        let (java_home, java_version, bootstrap_class_loader) =
            if let Some(java_version) = configuration.java_version() {
                let (java_home, java_version, boostrap_class_loader) =
                    runtime::version_class_loader(java_version).await?;
                (java_home, java_version, boostrap_class_loader)
            } else if let Some(java_home) = configuration.java_home() {
                let (java_home, java_version, boostrap_class_loader) =
                    runtime::home_class_loader(java_home).await?;
                (java_home, java_version, boostrap_class_loader)
            } else {
                return Err(InternalError(
                    "Java version or Java home must be specified".to_string(),
                ));
            };

        debug!(
            "Java home: {}; version: {java_version}",
            java_home.to_string_lossy()
        );
        let major_version: u16 = java_version.split('.').next().unwrap_or("0").parse()?;
        let java_class_file_version =
            Version::from(major_version + Self::CLASS_FILE_MAJOR_VERSION_OFFSET, 0)?;
        debug!("Java class file version {java_class_file_version}");

        // TODO: implement extension class loader
        // <JAVA_HOME>/jre/lib/ext directory or any other directory specified by the java.ext.dirs
        // system property

        let class_path = configuration.class_path().clone();
        let mut system_class_loader = ClassLoader::new("system", class_path);
        system_class_loader.set_parent(Some(bootstrap_class_loader.clone()));
        let mut main_class_name = configuration.main_class().cloned();

        let class_loader = if let Some(jar) = configuration.jar() {
            let path = jar.to_string_lossy();
            let jar_class_path = ClassPath::from(path);
            let mut jar_class_loader = ClassLoader::new("jar", jar_class_path);
            jar_class_loader.set_parent(Some(system_class_loader.clone()));

            // If the main class is not specified, try to get it from the jar manifest file
            if main_class_name.is_none() {
                for class_path_entry in jar_class_loader.class_path().iter() {
                    if let ClassPathEntry::Jar(jar) = class_path_entry {
                        let manifest = jar.manifest().await?;
                        if let Some(jar_main_class) = manifest.attribute(MAIN_CLASS) {
                            main_class_name = Some(jar_main_class.to_string());
                            break;
                        };
                    }
                }
            }

            jar_class_loader
        } else {
            system_class_loader.clone()
        };
        debug!("classloader: {class_loader}");

        let main_class = if let Some(main_class_name) = main_class_name {
            debug!("main class: {main_class_name}");
            Some(main_class_name)
        } else {
            None
        };

        let vm = Arc::new_cyclic(|vm| VM {
            vm: vm.clone(),
            configuration,
            class_loader: Arc::new(RwLock::new(class_loader)),
            main_class,
            java_home,
            java_version,
            java_class_file_version,
        });
        vm.initialize().await?;
        Ok(vm)
    }

    /// Create a new VM with the default configuration
    ///
    /// # Errors
    /// if the VM cannot be created
    pub async fn default() -> Result<Arc<VM>> {
        let configuration = ConfigurationBuilder::default().build()?;
        VM::new(configuration).await
    }

    /// Get the configuration
    #[must_use]
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    /// Get the main class
    #[must_use]
    pub fn main_class(&self) -> Option<&String> {
        self.main_class.as_ref()
    }

    /// Get the Java home
    #[must_use]
    pub fn java_home(&self) -> &PathBuf {
        &self.java_home
    }

    /// Get the java version
    #[must_use]
    pub fn java_version(&self) -> &str {
        &self.java_version
    }

    /// Get the Java class file version
    #[must_use]
    pub fn java_class_file_version(&self) -> &Version {
        &self.java_class_file_version
    }

    /// Get the system properties
    #[must_use]
    pub fn system_properties(&self) -> &HashMap<String, String> {
        self.configuration().system_properties()
    }

    /// Initialize the VM
    ///
    /// # Errors
    /// if the VM cannot be initialized
    async fn initialize(&self) -> Result<()> {
        let system_class = self.class("java.lang.System").await?;

        if self.java_class_file_version <= JAVA_8 {
            let initialize_system_class_method =
                system_class.try_get_method("initializeSystemClass", "()V")?;
            self.invoke(&system_class, &initialize_system_class_method, vec![])
                .await?;
        } else {
            let init_phase1_method = system_class.try_get_method("initPhase1", "()V")?;
            self.invoke(&system_class, &init_phase1_method, vec![])
                .await?;

            // TODO: Implement System::initPhase2()
            // let init_phase2_method = system_class.try_get_method("initPhase2", "(ZZ)I")?;
            // let phase2_result = self
            //     .invoke(
            //         &system_class,
            //         &init_phase2_method,
            //         vec![Value::Int(1), Value::Int(1)],
            //     )
            //     .await?;
            // let Some(Value::Int(result)) = phase2_result else {
            //     return Err(InternalError(format!(
            //         "System::initPhase2() call failed: {phase2_result:?}"
            //     )));
            // };
            // if result != 0 {
            //     return Err(InternalError(format!(
            //         "System::initPhase2() call failed: {result}"
            //     )));
            // }

            // TODO: Implement System::initPhase3()
            // let init_phase3_method = system_class.try_get_method("initPhase3", "()V")?;
            // self.invoke(&system_class, &init_phase3_method, vec![])
            //     .await?;
        }

        Ok(())
    }

    /// Load a class (e.g. "java.lang.Object").
    ///
    /// # Errors
    /// if the class cannot be loaded
    pub async fn class(&self, class_name: &str) -> Result<Arc<Class>> {
        let class_name = class_name.replace('.', "/");
        let thread = &Thread::new(&self.vm);
        self.load_class(thread, &class_name).await
    }

    /// Get a class.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jls/se23/html/jls-12.html#jls-12.4.1>
    ///
    /// # Errors
    /// if the class cannot be loaded
    pub(crate) async fn load_class<S: AsRef<str>>(
        &self,
        thread: &Thread,
        class_name: S,
    ) -> Result<Arc<Class>> {
        let class_name = class_name.as_ref();
        let class_load_result = {
            let class_loader = self.class_loader.read().await;
            class_loader.load_with_status(class_name).await
        };

        let class = match class_load_result {
            Ok((class, previously_loaded)) => {
                // Determine if the class has already been loaded.  If the class has already been loaded,
                // return the class. Otherwise, the class must be initialized.
                if previously_loaded {
                    return Ok(class);
                }
                class
            }
            Err(error) => {
                if class_name.starts_with('[')
                    || [
                        "boolean", "byte", "char", "double", "float", "int", "long", "short",
                        "void",
                    ]
                    .contains(&class_name)
                {
                    let array_class = Arc::new(Class::new_named(class_name)?);
                    // Register the array class so that it will be available for future lookups.
                    self.register_class(array_class.clone()).await?;
                    array_class
                } else {
                    return Err(error.into());
                }
            }
        };

        let classes = self.prepare_class_initialization(&class).await?;
        for current_class in classes {
            if let Some(class_initializer) = current_class.class_initializer() {
                // Execute the class initializer on the current thread.
                thread
                    .execute(&current_class, &class_initializer, vec![], true)
                    .await?;
            }
        }
        Ok(class)
    }

    /// Prepare class initialization.
    ///
    /// # Errors
    /// if the class cannot be resolved
    async fn prepare_class_initialization(&self, class: &Arc<Class>) -> Result<Vec<Arc<Class>>> {
        let class_loader = self.class_loader.write().await;
        let mut classes = Vec::new();
        let mut index = 0;
        classes.push(class.clone());

        while index < classes.len() {
            let Some(current_class) = classes.get(index) else {
                break;
            };
            let current_class = current_class.clone();

            if current_class.class_file().version > self.java_class_file_version {
                return Err(UnsupportedClassFileVersion(
                    current_class.class_file().version.major(),
                ));
            }

            let mut interfaces = Vec::new();
            for interface_index in &current_class.class_file().interfaces {
                let interface_name = current_class
                    .constant_pool()
                    .try_get_class(*interface_index)?;
                let (interface_class, previously_loaded) =
                    class_loader.load_with_status(interface_name).await?;
                interfaces.push(interface_class.clone());
                if !previously_loaded && !classes.contains(&interface_class) {
                    classes.push(interface_class);
                }
            }
            current_class.set_interfaces(interfaces)?;

            // If the current class is java.lang.Object, skip the parent class logic since Object is
            // the root class.
            if current_class.name() == "java/lang/Object" {
                index += 1;
                continue;
            }

            let super_class_index = current_class.class_file().super_class;
            let super_class_name = if super_class_index == 0 {
                "java/lang/Object"
            } else {
                let constant_pool = current_class.constant_pool();
                constant_pool.try_get_class(super_class_index)?
            };

            let (super_class, previously_loaded) =
                class_loader.load_with_status(super_class_name).await?;
            current_class.set_parent(Some(super_class.clone()))?;
            if !previously_loaded && !classes.contains(&super_class) {
                classes.push(super_class);
            }

            index += 1;
        }

        // Classes are discovered from the top of the hierarchy to the bottom.  However, the class
        // initialization order is from the bottom to the top.  Reverse the classes so that the
        // classes are initialized from the bottom to the top.
        classes.reverse();
        Ok(classes)
    }

    /// Register a class.
    ///
    /// # Errors
    /// if the class cannot be registered
    pub(crate) async fn register_class(&self, class: Arc<Class>) -> Result<()> {
        debug!("register class: {class}");
        let class_loader = self.class_loader.write().await;
        class_loader.register(class).await?;
        Ok(())
    }

    /// Invoke the main method of the main class associated with the VM. The main method must have
    /// the signature `public static void main(String[] args)`.
    ///
    /// # Errors
    /// * if the main class is not specified
    /// * if the main class does not specify a main method
    /// * if the main method cannot be invoked
    pub async fn invoke_main<S: AsRef<str>>(&self, arguments: Vec<S>) -> Result<Option<Value>> {
        let Some(main_class_name) = &self.main_class else {
            return Err(InternalError("No main class specified".into()));
        };
        let main_class = self.class(main_class_name).await?;
        let Some(main_method) = main_class.main_method() else {
            return Err(InternalError(format!(
                "No main method found for {main_class_name}"
            )));
        };

        let mut string_arguments = Vec::new();
        for argument in arguments {
            let argument = argument.as_ref();
            let Value::Object(value) = argument.to_object(self).await? else {
                return Err(InternalError(format!(
                    "Failed to create string for argument {argument}"
                )));
            };
            string_arguments.push(value);
        }

        let string_array_class = self.class("[Ljava/lang/String;").await?;
        let string_arguments = Value::Object(Some(Reference::Array(
            string_array_class,
            ConcurrentVec::from(string_arguments),
        )));

        self.invoke(&main_class, &main_method, vec![string_arguments])
            .await
    }

    /// Invoke a method.  To invoke a method on an object reference, the object reference must be
    /// the first argument in the arguments vector.
    ///
    /// # Errors
    /// if the method cannot be invoked
    pub async fn invoke(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        arguments: Vec<Value>,
    ) -> Result<Option<Value>> {
        let thread = Thread::new(&self.vm);
        thread.execute(class, method, arguments, true).await
    }

    /// Invoke a method.  To invoke a method on an object reference, the object reference must be
    /// the first argument in the arguments vector.
    ///
    /// # Errors
    /// if the method cannot be invoked
    pub async fn try_invoke(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        let Some(value) = self.invoke(class, method, arguments).await? else {
            return Err(InternalError("No return value".into()));
        };
        Ok(value)
    }

    /// Create a new VM Object by invoking the constructor of the specified class.
    ///
    /// # Errors
    /// if the object cannot be created
    pub async fn new_object<S: AsRef<str>>(
        &self,
        class_name: S,
        descriptor: S,
        arguments: Vec<impl RustValue>,
    ) -> Result<Value> {
        let class_name = class_name.as_ref();
        let descriptor = &format!("({})V", descriptor.as_ref());
        let class = self.class(class_name).await?;
        let Some(constructor) = class.method("<init>", descriptor) else {
            return Err(InternalError(format!(
                "No constructor found: {class_name}.<init>({descriptor})"
            )));
        };

        let mut constructor_arguments = Vec::with_capacity(arguments.len() + 1);
        let object = Object::new(class.clone())?;
        constructor_arguments.insert(0, Value::from(object));
        for argument in arguments {
            let value = argument.to_value();
            constructor_arguments.push(value);
        }

        let object = {
            let thread = Thread::new(&self.vm);
            thread
                .execute(&class, &constructor, constructor_arguments, false)
                .await?;
            let frames = thread.frames().await?;
            let frame = frames
                .first()
                .ok_or(InternalError("No frame".to_string()))?;
            let locals = frame.locals();
            locals.get(0)?
        };
        Arc::try_unwrap(object)
            .map_err(|_| InternalError("Failed to create new object".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::ConfigurationBuilder;
    use ristretto_classloader::{ClassPath, DEFAULT_JAVA_VERSION};
    use std::path::PathBuf;

    fn classes_jar_path() -> PathBuf {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_manifest.join("../classes/classes.jar")
    }

    fn classes_jar_class_path() -> ClassPath {
        let classes_jar_path = classes_jar_path();
        ClassPath::from(classes_jar_path.to_string_lossy())
    }

    async fn test_vm() -> Result<Arc<VM>> {
        let class_path = classes_jar_class_path();
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .build()?;
        VM::new(configuration).await
    }

    #[tokio::test]
    async fn test_vm_new() -> Result<()> {
        let vm = test_vm().await?;
        assert!(vm
            .configuration
            .class_path()
            .to_string()
            .contains("classes.jar"));
        assert_eq!(DEFAULT_JAVA_VERSION, vm.java_version());
        assert!(vm.main_class().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_new_java_home() -> Result<()> {
        let vm = test_vm().await?;
        let configuration = ConfigurationBuilder::new()
            .java_home(vm.java_home().clone())
            .build()?;
        let java_home_vm = VM::new(configuration).await?;
        assert_eq!(vm.java_home(), java_home_vm.java_home());
        assert_eq!(vm.java_version(), java_home_vm.java_version());
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_set_main_class() -> Result<()> {
        let class_path = classes_jar_class_path();
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .main_class("HelloWorld")
            .build()?;
        let vm = VM::new(configuration).await?;
        let main_class = vm.main_class().expect("main class");
        assert_eq!("HelloWorld", main_class);
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_set_jar_with_main_class() -> Result<()> {
        let classes_jar_path = classes_jar_path();
        let configuration = ConfigurationBuilder::new().jar(classes_jar_path).build()?;
        let vm = VM::new(configuration).await?;
        let main_class = vm.main_class().expect("main class");
        assert_eq!("HelloWorld", main_class);
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_load_java_lang_object() -> Result<()> {
        let vm = test_vm().await?;
        let class = vm.class("java.lang.Object").await?;
        assert_eq!("java/lang/Object", class.name());
        Ok(())
    }

    async fn test_load_primitive_class(class_name: &str) -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class(class_name).await?;
        assert_eq!(class_name, class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_load_boolean() -> Result<()> {
        test_load_primitive_class("boolean").await
    }

    #[tokio::test]
    async fn test_load_byte() -> Result<()> {
        test_load_primitive_class("byte").await
    }

    #[tokio::test]
    async fn test_load_char() -> Result<()> {
        test_load_primitive_class("char").await
    }

    #[tokio::test]
    async fn test_load_double() -> Result<()> {
        test_load_primitive_class("double").await
    }

    #[tokio::test]
    async fn test_load_float() -> Result<()> {
        test_load_primitive_class("float").await
    }

    #[tokio::test]
    async fn test_load_int() -> Result<()> {
        test_load_primitive_class("int").await
    }

    #[tokio::test]
    async fn test_load_long() -> Result<()> {
        test_load_primitive_class("long").await
    }

    #[tokio::test]
    async fn test_load_short() -> Result<()> {
        test_load_primitive_class("short").await
    }

    #[tokio::test]
    async fn test_load_void() -> Result<()> {
        test_load_primitive_class("void").await
    }

    #[tokio::test]
    async fn test_hello_world_class() -> Result<()> {
        let vm = test_vm().await?;
        let thread = Thread::new(&vm.vm);
        let class = vm.load_class(&thread, "HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_constants_class() -> Result<()> {
        let vm = test_vm().await?;
        let thread = Thread::new(&vm.vm);
        let class = vm.load_class(&thread, "Constants").await?;
        assert_eq!("Constants", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_inheritance() -> Result<()> {
        let vm = test_vm().await?;
        let thread = Thread::new(&vm.vm);
        let hash_map = vm.load_class(&thread, "java/util/HashMap").await?;
        assert_eq!("java/util/HashMap", hash_map.name());

        let abstract_map = hash_map.parent()?.expect("HashMap parent");
        assert_eq!("java/util/AbstractMap", abstract_map.name());

        let object = abstract_map.parent()?.expect("AbstractMap parent");
        assert_eq!("java/lang/Object", object.name());
        assert!(object.parent()?.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object() -> Result<()> {
        let vm = test_vm().await?;
        let object = vm.new_object("java.lang.Integer", "I", vec![42]).await?;
        let value: i32 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }
}

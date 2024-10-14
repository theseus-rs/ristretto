use crate::call_stack::CallStack;
use crate::Error::UnsupportedClassFileVersion;
use crate::{Configuration, Result};
use ristretto_classfile::{mutf8, Version};
use ristretto_classloader::manifest::MAIN_CLASS;
use ristretto_classloader::Error::ParseError;
use ristretto_classloader::Reference::{ByteArray, CharArray};
use ristretto_classloader::{
    runtime, Class, ClassLoader, ClassPath, ClassPathEntry, ConcurrentVec, Method, Object,
    Reference, Value,
};
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tracing::debug;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Java Virtual Machine
#[derive(Debug)]
pub struct VM {
    vm: Weak<VM>,
    configuration: Configuration,
    class_loader: RwLock<ClassLoader>,
    main_class: Option<String>,
    runtime_version: String,
    java_version: Version,
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
        let runtime_version = configuration.runtime_version();
        debug!("runtime_version {runtime_version}");

        let major_version: u16 = runtime_version.split('.').next().unwrap_or("0").parse()?;
        let java_version = Version::from(major_version + Self::CLASS_FILE_MAJOR_VERSION_OFFSET, 0)?;
        debug!("Java version {java_version}");

        let (runtime_version, boostrap_class_loader) =
            runtime::class_loader(runtime_version).await?;
        let boostrap_class_loader = boostrap_class_loader;

        // TODO: implement extension class loader
        // <JAVA_HOME>/jre/lib/ext directory or any other directory specified by the java.ext.dirs
        // system property

        let class_path = configuration.class_path().clone();
        let mut system_class_loader = ClassLoader::new("system", class_path);
        system_class_loader.set_parent(Some(boostrap_class_loader.clone()));
        let mut main_class_name = configuration.main_class();

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
            class_loader: RwLock::new(class_loader),
            main_class,
            runtime_version,
            java_version,
        });
        vm.initialize().await?;
        Ok(vm)
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

    /// Get the version
    #[must_use]
    pub fn runtime_version(&self) -> &str {
        &self.runtime_version
    }

    /// Get the Java version
    #[must_use]
    pub fn java_version(&self) -> &Version {
        &self.java_version
    }

    /// Initialize the VM
    ///
    /// # Errors
    /// if the VM cannot be initialized
    async fn initialize(&self) -> Result<()> {
        let system_class = self.load("java.lang.System").await?;

        if self.java_version <= JAVA_8 {
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
            // let phase2_result = self.invoke(
            //     &system_class,
            //     &init_phase2_method,
            //     vec![Value::Int(1), Value::Int(1)],
            // )?;
            // let Some(Value::Int(result)) = phase2_result else {
            //     return Err(RuntimeError(format!(
            //         "System::initPhase2() call failed: {phase2_result:?}"
            //     )));
            // };
            // if result != 0 {
            //     return Err(RuntimeError(format!(
            //         "System::initPhase2() call failed: {result}"
            //     )));
            // }

            // TODO: Implement System::initPhase3()
            // let init_phase3_method = system_class.try_get_method("initPhase3", "()V")?;
            // self.invoke(&system_class, &init_phase3_method, vec![])?;
        }

        Ok(())
    }

    /// Load a class (e.g. "java.lang.Object").
    ///
    /// # Errors
    /// if the class cannot be loaded
    pub async fn load(&self, class_name: &str) -> Result<Arc<Class>> {
        let class_name = class_name.replace('.', "/");
        let call_stack = &CallStack::new(&self.vm);
        self.class(call_stack, &class_name).await
    }

    /// Get a class.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jls/se23/html/jls-12.html#jls-12.4.1>
    ///
    /// # Errors
    /// if the class cannot be loaded
    pub(crate) async fn class(
        &self,
        call_stack: &CallStack,
        class_name: &str,
    ) -> Result<Arc<Class>> {
        let class_load_result = {
            let class_loader = self.class_loader.read().await;
            class_loader.load_with_status(class_name).await
        };

        let class = match class_load_result {
            Ok((class, previously_loaded)) => {
                if previously_loaded {
                    return Ok(class);
                }
                class
            }
            Err(error) => {
                if class_name.starts_with('[') {
                    let array_class = Arc::new(Class::new_array(class_name)?);
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
                call_stack
                    .execute(&current_class, &class_initializer, vec![])
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

            if current_class.class_file().version > self.java_version {
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
        let mut class_loader = self.class_loader.write().await;
        class_loader.register(class).await?;
        Ok(())
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
        let call_stack = CallStack::new(&self.vm);
        call_stack.execute(class, method, arguments).await
    }

    /// Create a new java.lang.Class object as a VM Value.
    ///
    /// # Errors
    /// if the class object cannot be created
    pub(crate) async fn to_class_value(
        &self,
        call_stack: &Arc<CallStack>,
        class_name: &str,
    ) -> Result<Value> {
        let object_class_name = "java/lang/Class";
        let class = self.class(call_stack, object_class_name).await?;
        let object = Object::new(class)?;
        let name = self.to_string_value(call_stack, class_name).await?;
        let name_field = object.field("name")?;
        name_field.set_value(name)?;
        // TODO: a "null" class loader indicates a system class loader; this should be re-evaluated
        // to support custom class loaders
        let class_loader_field = object.field("classLoader")?;
        class_loader_field.unsafe_set_value(Value::Object(None))?;
        let reference = Reference::Object(object);
        let value = Value::Object(Some(reference));
        Ok(value)
    }

    /// Create a new string object.
    ///
    /// # Errors
    /// if the string object cannot be created
    pub async fn string<S: AsRef<str>>(&self, value: S) -> Result<Value> {
        let call_stack = CallStack::new(&self.vm);
        let value = value.as_ref();
        self.to_string_value(&call_stack, value).await
    }

    /// Create a new java.lang.String object as a VM Value.
    ///
    /// # Errors
    /// if the string object cannot be created
    pub(crate) async fn to_string_value(
        &self,
        call_stack: &Arc<CallStack>,
        value: &str,
    ) -> Result<Value> {
        let class_name = "java/lang/String";
        let class = self.class(call_stack, class_name).await?;
        let object = Object::new(class)?;

        // The String implementation changed in Java 9.
        // In Java 8 and earlier, the value field is a char array.
        // In Java 9 and later, the value field is a byte array.
        let array = if self.java_version <= JAVA_8 {
            let bytes = mutf8::to_bytes(value)?;
            let utf8_string =
                String::from_utf8(bytes).map_err(|error| ParseError(error.to_string()))?;
            let ucs2_chars: Vec<u16> = utf8_string.encode_utf16().collect();
            let chars = ConcurrentVec::from(ucs2_chars);
            CharArray(chars)
        } else {
            let coder_field = object.field("coder")?;
            coder_field.set_value(Value::Int(0))?; // LATIN1

            let bytes = mutf8::to_bytes(value)?;
            #[expect(clippy::cast_possible_wrap)]
            let bytes = bytes.iter().map(|&b| b as i8).collect();
            let bytes = ConcurrentVec::from(bytes);
            ByteArray(bytes)
        };

        let value_field = object.field("value")?;
        value_field.set_value(Value::Object(Some(array)))?;

        let hash_field = object.field("hash")?;
        hash_field.set_value(Value::Int(0))?;

        let reference = Reference::Object(object);
        let value = Value::Object(Some(reference));
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{ConfigurationBuilder, DEFAULT_RUNTIME_VERSION};
    use ristretto_classloader::ClassPath;
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
            .build();
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
        assert_eq!(DEFAULT_RUNTIME_VERSION, vm.runtime_version());
        assert!(vm.main_class().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_set_main_class() -> Result<()> {
        let class_path = classes_jar_class_path();
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .main_class("HelloWorld")
            .build();
        let vm = VM::new(configuration).await?;
        let main_class = vm.main_class().expect("main class");
        assert_eq!("HelloWorld", main_class);
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_set_jar_with_main_class() -> Result<()> {
        let classes_jar_path = classes_jar_path();
        let configuration = ConfigurationBuilder::new().jar(classes_jar_path).build();
        let vm = VM::new(configuration).await?;
        let main_class = vm.main_class().expect("main class");
        assert_eq!("HelloWorld", main_class);
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_load_java_lang_object() -> Result<()> {
        let vm = test_vm().await?;
        let class = vm.load("java.lang.Object").await?;
        assert_eq!("java/lang/Object", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_hello_world_class() -> Result<()> {
        let vm = test_vm().await?;
        let call_stack = CallStack::new(&vm.vm);
        let class = vm.class(&call_stack, "HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_constants_class() -> Result<()> {
        let vm = test_vm().await?;
        let call_stack = CallStack::new(&vm.vm);
        let class = vm.class(&call_stack, "Constants").await?;
        assert_eq!("Constants", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_inheritance() -> Result<()> {
        let vm = test_vm().await?;
        let call_stack = CallStack::new(&vm.vm);
        let hash_map = vm.class(&call_stack, "java/util/HashMap").await?;
        assert_eq!("java/util/HashMap", hash_map.name());

        let abstract_map = hash_map.parent()?.expect("HashMap parent");
        assert_eq!("java/util/AbstractMap", abstract_map.name());

        let object = abstract_map.parent()?.expect("AbstractMap parent");
        assert_eq!("java/lang/Object", object.name());
        assert!(object.parent()?.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_interfaces() -> Result<()> {
        let vm = test_vm().await?;
        let call_stack = CallStack::new(&vm.vm);

        let interface = vm.class(&call_stack, "java/util/NavigableMap").await?;
        let method = interface.try_get_virtual_method("size", "()I");
        assert!(method.is_ok());

        Ok(())
    }
}

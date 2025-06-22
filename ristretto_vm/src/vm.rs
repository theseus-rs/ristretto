use crate::Error::InternalError;
use crate::handles::Handles;
use crate::intrinsic_methods::MethodRegistry;
use crate::java_object::JavaObject;
use crate::rust_value::RustValue;
use crate::thread::Thread;
use crate::{Configuration, ConfigurationBuilder, Result};
use dashmap::DashMap;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_PREVIEW_MINOR_VERSION, Version};
use ristretto_classloader::manifest::MAIN_CLASS;
use ristretto_classloader::{
    Class, ClassLoader, ClassPath, ClassPathEntry, Object, Reference, Value, runtime,
};
use ristretto_jit::Compiler;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tracing::debug;

/// The offset to add to the major version to get the class file version.  Java 1.0 has a class
/// file major version of 45, so the class file major version is the Java version (1) + the
/// class file offset version (44) = the Java 1 class file version (45).
pub(crate) const CLASS_FILE_MAJOR_VERSION_OFFSET: u16 = 44;

/// Java Virtual Machine
#[derive(Debug)]
pub struct VM {
    vm: Weak<VM>,
    configuration: Configuration,
    class_loader: Arc<RwLock<ClassLoader>>,
    main_class: Option<String>,
    java_home: PathBuf,
    java_version: String,
    java_major_version: u16,
    java_class_file_version: Version,
    method_registry: MethodRegistry,
    next_thread_id: AtomicU64,
    threads: DashMap<u64, Arc<Thread>>,
    compiler: Option<Compiler>,
    handles: Handles,
}

/// VM
impl VM {
    /// Create a new VM
    ///
    /// # Errors
    ///
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
        let java_major_version: u16 = java_version.split('.').next().unwrap_or("0").parse()?;
        let class_file_minor_version = if configuration.preview_features() {
            JAVA_PREVIEW_MINOR_VERSION
        } else {
            0
        };
        let java_class_file_version = Version::from(
            java_major_version + CLASS_FILE_MAJOR_VERSION_OFFSET,
            class_file_minor_version,
        )?;
        debug!("Class file version {java_class_file_version}");

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
                        }
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

        let method_registry = MethodRegistry::new(&java_class_file_version);

        let compiler = match Compiler::new() {
            Ok(compiler) => Some(compiler),
            Err(error) => {
                debug!("JIT compiler not available: {error:?}");
                None
            }
        };

        let vm = Arc::new_cyclic(|vm| VM {
            vm: vm.clone(),
            configuration,
            class_loader: Arc::new(RwLock::new(class_loader)),
            main_class,
            java_home,
            java_version,
            java_major_version,
            java_class_file_version,
            method_registry,
            next_thread_id: AtomicU64::new(1),
            threads: DashMap::new(),
            compiler,
            handles: Handles::new(),
        });
        vm.initialize().await?;
        Ok(vm)
    }

    /// Create a new VM with the default configuration
    ///
    /// # Errors
    ///
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

    /// Get the class loader
    pub(crate) fn class_loader(&self) -> Arc<RwLock<ClassLoader>> {
        self.class_loader.clone()
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

    /// Get the java major version
    #[must_use]
    pub fn java_major_version(&self) -> u16 {
        self.java_major_version
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

    /// Get the method registry
    pub fn method_registry(&self) -> &MethodRegistry {
        &self.method_registry
    }

    /// Get the next thread ID
    ///
    /// # Errors
    ///
    /// if the thread identifier overflows
    pub(crate) fn next_thread_id(&self) -> Result<u64> {
        let id = self.next_thread_id.fetch_add(1, Ordering::SeqCst);
        if id == 0 {
            return Err(InternalError("Thread identifier overflow".to_string()));
        }
        Ok(id)
    }

    /// Get a thread by its identifier
    pub fn thread(&self, thread_id: u64) -> Option<Arc<Thread>> {
        self.threads.get(&thread_id).map(|thread| thread.clone())
    }

    /// Get the VM threads
    #[must_use]
    pub fn threads(&self) -> Vec<Arc<Thread>> {
        self.threads
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get the JIT Compiler
    pub(crate) fn compiler(&self) -> Option<&Compiler> {
        self.compiler.as_ref()
    }

    /// Get the handles
    pub(crate) fn handles(&self) -> &Handles {
        &self.handles
    }

    /// Create a new thread
    ///
    /// # Errors
    ///
    /// if the thread cannot be created
    pub(crate) fn new_thread(&self) -> Result<Arc<Thread>> {
        let thread = Thread::new(&self.vm)?;
        self.threads.insert(thread.id(), thread.clone());
        Ok(thread)
    }

    /// Initialize the VM
    ///
    /// # Errors
    ///
    /// if the VM cannot be initialized
    async fn initialize(&self) -> Result<()> {
        self.initialize_primordial_thread().await?;
        if self.java_class_file_version <= JAVA_8 {
            self.invoke(
                "java.lang.System",
                "initializeSystemClass",
                "()V",
                &[] as &[Value],
            )
            .await?;
        } else {
            self.invoke("java.lang.System", "initPhase1", "()V", &[] as &[Value])
                .await?;

            let phase2_result = self
                .invoke(
                    "java.lang.System",
                    "initPhase2",
                    "(ZZ)I",
                    &[Value::Int(1), Value::Int(1)],
                )
                .await?;
            let Some(Value::Int(result)) = phase2_result else {
                return Err(InternalError(format!(
                    "System::initPhase2() call failed: {phase2_result:?}"
                )));
            };
            if result != 0 {
                return Err(InternalError(format!(
                    "System::initPhase2() call failed: {result}"
                )));
            }

            self.invoke("java.lang.System", "initPhase3", "()V", &[] as &[Value])
                .await?;
        }

        Ok(())
    }

    /// Initialize the primordial thread
    ///
    /// # Errors
    ///
    /// if the primordial thread cannot be initialized
    async fn initialize_primordial_thread(&self) -> Result<()> {
        let thread = self.new_thread()?;
        let thread_id = i64::try_from(thread.id())?;
        let thread_group = thread
            .object("java.lang.ThreadGroup", "", &[] as &[Value])
            .await?;

        let java_version = self.java_class_file_version();

        // The internal structure of Thread changed in Java 19
        let new_thread = if java_version <= &JAVA_17 {
            let thread_class = self.class("java.lang.Thread").await?;
            let new_thread = Object::new(thread_class)?;
            new_thread.set_value("daemon", Value::Int(0))?;
            new_thread.set_value("eetop", Value::Long(0))?;
            new_thread.set_value("group", thread_group.clone())?;
            new_thread.set_value("priority", Value::Int(5))?;
            new_thread.set_value("stackSize", Value::Long(0))?;
            new_thread.set_value("threadStatus", Value::Int(4))?; // Runnable
            new_thread.set_value("tid", Value::Long(thread_id))?;
            Value::from(new_thread)
        } else {
            let field_holder_class = self.class("java.lang.Thread$FieldHolder").await?;
            let field_holder = Object::new(field_holder_class)?;
            field_holder.set_value("daemon", Value::Int(0))?;
            field_holder.set_value("group", thread_group.clone())?;
            field_holder.set_value("priority", Value::Int(5))?;
            field_holder.set_value("stackSize", Value::Long(0))?;
            field_holder.set_value("threadStatus", Value::Int(4))?; // Runnable
            let reference = Reference::from(field_holder);
            let field_holder = Value::Object(Some(reference));

            let thread_class = self.class("java.lang.Thread").await?;
            let new_thread = Object::new(thread_class)?;
            new_thread.set_value("eetop", Value::Long(0))?;
            new_thread.set_value("holder", field_holder)?;
            new_thread.set_value("interrupted", Value::Int(0))?;
            new_thread.set_value("tid", Value::Long(thread_id))?;
            Value::from(new_thread)
        };
        thread.set_java_object(new_thread).await;

        Ok(())
    }

    /// Get the primordial thread
    ///
    /// # Errors
    ///
    /// if the primordial thread cannot be found
    fn primordial_thread(&self) -> Result<Arc<Thread>> {
        let thread = self.threads.get(&1).map(|entry| entry.value().clone());
        let Some(thread) = thread else {
            return Err(InternalError("Primordial thread not found".into()));
        };
        Ok(thread)
    }

    /// Load a class (e.g. "java.lang.Object").
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded
    pub async fn class<S: AsRef<str>>(&self, class_name: S) -> Result<Arc<Class>> {
        let thread = self.primordial_thread()?;
        thread.class(class_name).await
    }

    /// Invoke the main method of the main class associated with the VM. The main method must have
    /// the signature `public static void main(String[] args)`.
    ///
    /// # Errors
    ///
    /// - if the main class is not specified
    /// - if the main class does not specify a main method
    /// - if the main method cannot be invoked
    pub async fn invoke_main<S: AsRef<str>>(&self, parameters: &[S]) -> Result<Option<Value>> {
        let Some(main_class_name) = &self.main_class else {
            return Err(InternalError("No main class specified".into()));
        };
        let main_class = self.class(main_class_name).await?;
        let Some(main_method) = main_class.main_method() else {
            return Err(InternalError(format!(
                "No main method found for {main_class_name}"
            )));
        };

        let mut string_parameters = Vec::with_capacity(parameters.len());
        for parameter in parameters {
            let parameter = parameter.as_ref();
            let value = parameter.to_object(self).await?;
            string_parameters.push(value);
        }

        let string_array_class = self.class("[Ljava/lang/String;").await?;
        let string_parameter = Value::try_from((string_array_class, string_parameters))?;

        self.invoke(
            main_class_name,
            main_method.name(),
            main_method.descriptor(),
            &[string_parameter],
        )
        .await
    }

    /// Invoke a method.  To invoke a method on an object reference, the object reference must be
    /// the first parameter in the parameters vector.
    ///
    /// # Errors
    ///
    /// if the method cannot be invoked
    pub async fn invoke<C, M, D>(
        &self,
        class: C,
        method: M,
        descriptor: D,
        parameters: &[impl RustValue],
    ) -> Result<Option<Value>>
    where
        C: AsRef<str>,
        M: AsRef<str>,
        D: AsRef<str>,
    {
        let class = self.class(class).await?;
        let method = class.try_get_method(method, descriptor)?;
        let thread = self.primordial_thread()?;
        thread.execute(&class, &method, parameters).await
    }

    /// Invoke a method.  To invoke a method on an object reference, the object reference must be
    /// the first parameter in the parameters vector.
    ///
    /// # Errors
    ///
    /// if the method cannot be invoked
    pub async fn try_invoke<C, M, D>(
        &self,
        class: C,
        method: M,
        descriptor: D,
        parameters: &[impl RustValue],
    ) -> Result<Value>
    where
        C: AsRef<str>,
        M: AsRef<str>,
        D: AsRef<str>,
    {
        let Some(value) = self.invoke(class, method, descriptor, parameters).await? else {
            return Err(InternalError("No return value".into()));
        };
        Ok(value)
    }

    /// Create a new VM Object by invoking the constructor of the specified class.
    ///
    /// # Errors
    ///
    /// if the object cannot be created
    pub async fn object<C, M>(
        &self,
        class_name: C,
        descriptor: M,
        parameters: &[impl RustValue],
    ) -> Result<Value>
    where
        C: AsRef<str>,
        M: AsRef<str>,
    {
        let thread = self.primordial_thread()?;
        thread.object(class_name, descriptor, parameters).await
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
        cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar")
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
        assert!(
            vm.configuration
                .class_path()
                .to_string()
                .contains("classes.jar")
        );
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
    async fn test_new_object_integer() -> Result<()> {
        let vm = test_vm().await?;
        let object = vm.object("java.lang.Integer", "I", &[42]).await?;
        let value: i32 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object_integer_from_string() -> Result<()> {
        let vm = test_vm().await?;
        let object = vm
            .object("java.lang.Integer", "Ljava/lang/String;", &["42"])
            .await?;
        let value: i32 = object.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object_string() -> Result<()> {
        let vm = test_vm().await?;
        let characters = "foo".chars().collect::<Vec<char>>();
        let object = vm.object("java.lang.String", "[C", &[characters]).await?;
        let value: String = object.try_into()?;
        assert_eq!("foo", value);
        Ok(())
    }
}

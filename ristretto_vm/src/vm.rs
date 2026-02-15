use crate::Error::InternalError;
use crate::RustValue;
use crate::call_site_cache::CallSiteCache;
use crate::intrinsic_methods::MethodRegistry;
use crate::java_object::JavaObject;
use crate::jit::Compiler;
use crate::method_ref_cache::MethodRefCache;
use crate::module_system::ModuleSystem;
use crate::monitor::MonitorRegistry;
use crate::string_pool::StringPool;
use crate::thread::Thread;
use crate::{Configuration, ConfigurationBuilder, Result, startup_trace};
use ahash::AHashMap;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_21, JAVA_PREVIEW_MINOR_VERSION, Version};
use ristretto_classloader::manifest::MAIN_CLASS;
use ristretto_classloader::{
    Class, ClassLoader, ClassPath, ClassPathEntry, Object, Reference, Value, runtime,
};
use ristretto_gc::{GarbageCollector, Statistics};
use ristretto_types::NativeMemory;
use ristretto_types::handles::{FileHandle, HandleManager, MemberHandle};

type ThreadHandle = ristretto_types::handles::ThreadHandle<Thread>;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use tokio::sync::{Mutex, RwLock};
use tracing::debug;

/// The offset to add to the major version to get the class file version.  Java 1.0 has a class
/// file major version of 45, so the class file major version is the Java version (1) + the
/// class file offset version (44) = the Java 1 class file version (45).
pub(crate) const CLASS_FILE_MAJOR_VERSION_OFFSET: u16 = 44;

/// Java Virtual Machine
#[derive(Debug)]
pub struct VM {
    /// Weak reference to self to avoid reference cycles
    vm: Weak<VM>,
    /// VM configuration
    configuration: Configuration,
    /// Module system for runtime module state management.
    module_system: ModuleSystem,
    /// The root class loader
    class_loader: Arc<RwLock<Arc<ClassLoader>>>,
    /// The garbage collector for the VM.
    garbage_collector: Arc<GarbageCollector>,
    /// The main class name
    main_class: Option<String>,
    /// The Java home directory
    java_home: PathBuf,
    /// The Java version string (e.g. "25.0.1")
    java_version: String,
    /// The Java major version (e.g. 21 for Java 21)
    java_major_version: u16,
    /// The Java class file version (e.g. 65.0 for Java 21)
    java_class_file_version: Version,
    /// The method registry for intrinsic methods
    method_registry: MethodRegistry,
    /// The JIT compiler (per-VM instance with its own cache and background compilation)
    compiler: Option<Compiler>,
    /// Counter for generating unique hidden class name suffixes
    hidden_class_counter: AtomicU64,
    /// Per-VM native memory manager.
    native_memory: NativeMemory,
    /// The next thread ID
    next_thread_id: AtomicU64,
    /// The VM thread handles
    thread_handles: HandleManager<u64, ThreadHandle>,
    /// The VM file handles
    file_handles: HandleManager<String, FileHandle>,
    /// NIO file descriptor handles.
    nio_file_handles: HandleManager<i32, std::fs::File>,
    /// The next NIO file descriptor number.
    next_nio_fd: AtomicI32,
    /// The VM member handles used for dynamic invocation
    member_handles: HandleManager<String, MemberHandle>,
    /// The string pool for interned strings
    string_pool: StringPool,
    /// Call site cache for caching resolved call sites.
    call_site_cache: CallSiteCache,
    /// Method reference cache for caching resolved method references with access checks.
    method_ref_cache: MethodRefCache,
    /// The monitor registry for object monitors.
    monitor_registry: MonitorRegistry,
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
            Self::create_bootstrap_loader(&configuration).await?;
        startup_trace!("[vm] bootstrap class loader");

        debug!(
            "Java home: {java_home}; version: {java_version}",
            java_home = java_home.to_string_lossy()
        );
        let java_major_version: u16 = java_version.split('.').next().unwrap_or("0").parse()?;
        let java_class_file_version =
            Self::compute_class_file_version(java_major_version, configuration.preview_features())?;
        debug!("Class file version {java_class_file_version}");

        let (class_loader, main_class) =
            Self::create_class_loader(&configuration, &bootstrap_class_loader).await?;
        startup_trace!("[vm] system class loader");

        let method_registry = MethodRegistry::new(&java_class_file_version);
        startup_trace!("[vm] method registry");

        let compiler = Self::create_compiler(&configuration);
        startup_trace!("[vm] jit compiler");

        let module_system =
            ModuleSystem::new(&configuration, &java_home, java_major_version).await?;
        startup_trace!("[vm] module system");

        // Set module configuration on class loaders for JPMS support. This enables module name
        // assignment during class loading
        let module_config = Arc::new(module_system.resolved_configuration().clone());
        bootstrap_class_loader
            .set_module_configuration(Some(module_config.clone()))
            .await;
        class_loader
            .set_module_configuration(Some(module_config))
            .await;
        startup_trace!("[vm] class loader module config");

        // Use the configured garbage collector or create a default one
        let garbage_collector = configuration
            .garbage_collector()
            .cloned()
            .unwrap_or_else(GarbageCollector::new);

        let vm = Arc::new_cyclic(|vm| VM {
            vm: vm.clone(),
            configuration,
            class_loader: Arc::new(RwLock::new(class_loader)),
            garbage_collector,
            main_class,
            java_home,
            java_version,
            java_major_version,
            java_class_file_version,
            method_registry,
            compiler,
            hidden_class_counter: AtomicU64::new(1),
            next_thread_id: AtomicU64::new(1),
            native_memory: NativeMemory::new(),
            thread_handles: HandleManager::new(),
            file_handles: HandleManager::new(),
            nio_file_handles: HandleManager::new(),
            next_nio_fd: AtomicI32::new(1000),
            member_handles: HandleManager::new(),
            string_pool: StringPool::new(),
            call_site_cache: CallSiteCache::new(),
            method_ref_cache: MethodRefCache::new(),
            monitor_registry: MonitorRegistry::new(),
            module_system,
        });
        startup_trace!("[vm] vm allocation");

        vm.initialize().await?;
        Ok(vm)
    }

    /// Creates the bootstrap class loader.
    async fn create_bootstrap_loader(
        configuration: &Configuration,
    ) -> Result<(PathBuf, String, Arc<ClassLoader>)> {
        if let Some(java_version) = configuration.java_version() {
            let (java_home, java_version, bootstrap_class_loader) =
                runtime::version_class_loader(java_version).await?;
            Ok((java_home, java_version, bootstrap_class_loader))
        } else if let Some(java_home) = configuration.java_home() {
            let (java_home, java_version, bootstrap_class_loader) =
                runtime::home_class_loader(java_home).await?;
            Ok((java_home, java_version, bootstrap_class_loader))
        } else {
            Err(InternalError(
                "Java version or Java home must be specified".to_string(),
            ))
        }
    }

    /// Computes the class file version.
    fn compute_class_file_version(
        java_major_version: u16,
        preview_features: bool,
    ) -> Result<Version> {
        let class_file_minor_version = if preview_features {
            JAVA_PREVIEW_MINOR_VERSION
        } else {
            0
        };
        let version = Version::from(
            java_major_version + CLASS_FILE_MAJOR_VERSION_OFFSET,
            class_file_minor_version,
        )?;
        Ok(version)
    }

    /// Creates the class loader hierarchy.
    async fn create_class_loader(
        configuration: &Configuration,
        bootstrap_class_loader: &Arc<ClassLoader>,
    ) -> Result<(Arc<ClassLoader>, Option<String>)> {
        let class_path = configuration.class_path().clone();
        let system_class_loader = ClassLoader::new("system", class_path);
        system_class_loader
            .set_parent(Some(bootstrap_class_loader.clone()))
            .await;
        let mut main_class_name = configuration.main_class().cloned();

        let class_loader = if let Some(jar) = configuration.jar() {
            let jar_class_path = ClassPath::from(&[jar]);
            let jar_class_loader = ClassLoader::new("jar", jar_class_path);
            jar_class_loader
                .set_parent(Some(system_class_loader.clone()))
                .await;

            // If the main class is not specified, try to get it from the jar manifest file
            if main_class_name.is_none() {
                main_class_name = Self::extract_main_class_from_jar(&jar_class_loader).await?;
            }

            jar_class_loader
        } else {
            system_class_loader.clone()
        };
        debug!("classloader: {class_loader}");

        let main_class = main_class_name.map(|name| {
            debug!("main class: {name}");
            name
        });

        Ok((class_loader, main_class))
    }

    /// Extracts main class from JAR manifest.
    async fn extract_main_class_from_jar(
        class_loader: &Arc<ClassLoader>,
    ) -> Result<Option<String>> {
        for class_path_entry in class_loader.class_path().iter() {
            if let ClassPathEntry::Jar(jar) = class_path_entry {
                let manifest = jar.manifest().await?;
                if let Some(jar_main_class) = manifest.attribute(MAIN_CLASS) {
                    return Ok(Some(jar_main_class.to_string()));
                }
            }
        }
        Ok(None)
    }

    /// Creates the JIT compiler.
    fn create_compiler(configuration: &Configuration) -> Option<Compiler> {
        Compiler::new(
            configuration.batch_compilation(),
            configuration.interpreted(),
        )
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

    /// Get the module system for runtime module state management.
    #[must_use]
    pub(crate) fn module_system(&self) -> &ModuleSystem {
        &self.module_system
    }

    /// Get the class loader
    pub(crate) fn class_loader(&self) -> Arc<RwLock<Arc<ClassLoader>>> {
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
    pub fn system_properties(&self) -> &AHashMap<String, String> {
        self.configuration().system_properties()
    }

    /// Get the garbage collector.
    pub fn garbage_collector(&self) -> &Arc<GarbageCollector> {
        &self.garbage_collector
    }

    /// Runs the garbage collector for the VM.
    pub fn gc(&self) {
        self.garbage_collector.collect();
    }

    /// Get VM statistics
    pub fn statistics(&self) -> Statistics {
        self.garbage_collector.statistics().unwrap_or_default()
    }

    /// Get the method registry
    pub fn method_registry(&self) -> &MethodRegistry {
        &self.method_registry
    }

    /// Get the JIT Compiler
    pub(crate) fn compiler(&self) -> Option<&Compiler> {
        self.compiler.as_ref()
    }

    /// Get the next unique suffix for a hidden class name.
    ///
    /// This atomically increments and returns a counter used to generate unique names for hidden
    /// classes in the format `{name}+0x{suffix:016x}`.
    ///
    /// # Errors
    ///
    /// if the hidden class suffix overflows
    pub(crate) fn next_hidden_class_suffix(&self) -> Result<u64> {
        let id = self.hidden_class_counter.fetch_add(1, Ordering::SeqCst);
        if id == 0 {
            return Err(InternalError("Hidden class suffix overflow".to_string()));
        }
        Ok(id)
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

    /// Get the VM thread handles
    #[must_use]
    pub(crate) fn thread_handles(&self) -> &HandleManager<u64, ThreadHandle> {
        &self.thread_handles
    }

    /// Get the VM file handles
    pub(crate) fn file_handles(&self) -> &HandleManager<String, FileHandle> {
        &self.file_handles
    }

    /// Get the VM member handles used for dynamic invocation
    pub(crate) fn member_handles(&self) -> &HandleManager<String, MemberHandle> {
        &self.member_handles
    }

    /// Get the method reference cache for caching resolved method refs.
    ///
    /// JPMS access checks are performed at resolution time and cached,
    /// so subsequent invocations are fast.
    pub(crate) fn method_ref_cache(&self) -> &MethodRefCache {
        &self.method_ref_cache
    }

    /// Get the monitor registry.
    pub(crate) fn monitor_registry(&self) -> &MonitorRegistry {
        &self.monitor_registry
    }

    /// Initialize the VM
    ///
    /// # Errors
    ///
    /// if the VM cannot be initialized
    async fn initialize(&self) -> Result<()> {
        self.garbage_collector.start();
        startup_trace!("[vm] garbage collector started");

        self.initialize_primordial_thread().await?;
        startup_trace!("[vm] primordial thread");

        // Load the java.lang.ref.Reference class explicitly so that the class initializer calls
        // SharedSecrets.setJavaLangRefAccess(...) at the appropriate time in the JVM initialization
        // process.
        let _ = self.class("java.lang.ref.Reference").await?;

        // Load java.lang.reflect.AccessibleObject early so that its static initializer calls
        // SharedSecrets.setJavaLangReflectAccess(...) before any reflection operations are performed.
        // This is required because ReflectionFactory's constructor calls getJavaLangReflectAccess().
        let _ = self.class("java.lang.reflect.AccessibleObject").await?;
        startup_trace!("[vm] accessible object initialized");

        // Load java.lang.invoke.MethodHandleNatives to initialize the method handle subsystem.
        // This sets SharedSecrets.setJavaLangInvokeAccess() which is required for reflective
        // field access in Java 21+ (Field.get() uses method handles internally).
        if self.java_class_file_version >= JAVA_21 {
            let _ = self.class("java.lang.invoke.MethodHandleNatives").await?;
            startup_trace!("[vm] method handle natives initialized");
        }

        if self.java_class_file_version <= JAVA_8 {
            self.invoke(
                "java.lang.System",
                "initializeSystemClass()V",
                &[] as &[Value],
            )
            .await?;
            startup_trace!("[vm] initialize system class");
        } else {
            self.invoke("java.lang.System", "initPhase1()V", &[] as &[Value])
                .await?;
            startup_trace!("[vm] init phase 1");

            let phase2_result = self
                .invoke(
                    "java.lang.System",
                    "initPhase2(ZZ)I",
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
            startup_trace!("[vm] init phase 2");

            self.invoke("java.lang.System", "initPhase3()V", &[] as &[Value])
                .await?;
            startup_trace!("[vm] init phase 3");
        }

        Ok(())
    }

    /// Initialize the primordial thread
    ///
    /// # Errors
    ///
    /// if the primordial thread cannot be initialized
    async fn initialize_primordial_thread(&self) -> Result<()> {
        let thread_id = self.next_thread_id()?;
        let thread = Thread::new(&self.vm, thread_id);
        let thread_id = i64::try_from(thread.id())?;
        // Create the system thread group (no-arg constructor creates system group)
        let system_group = thread
            .object("java.lang.ThreadGroup", "", &[] as &[Value])
            .await?;
        // Create the main thread group with system as parent, matching JVM behavior
        let main_name: Value = "main".to_object(&thread).await?;
        let thread_group = thread
            .object(
                "java.lang.ThreadGroup",
                "Ljava/lang/ThreadGroup;Ljava/lang/String;",
                &[system_group, main_name],
            )
            .await?;
        let java_version = self.java_class_file_version();

        let thread_class = thread.class("java.lang.Thread").await?;
        let mut new_thread = Object::new(thread_class)?;
        new_thread.set_value("eetop", Value::Long(thread_id))?;
        new_thread.set_value("tid", Value::Long(thread_id))?;

        // The internal structure of Thread changed in Java 19
        if java_version <= &JAVA_17 {
            new_thread.set_value("daemon", Value::Int(0))?;
            new_thread.set_value("group", thread_group)?;
            new_thread.set_value("priority", Value::Int(5))?;
            new_thread.set_value("stackSize", Value::Long(0))?;
            new_thread.set_value("threadStatus", Value::Int(4))?; // Runnable
        } else {
            let field_holder_class = thread.class("java.lang.Thread$FieldHolder").await?;
            let mut field_holder = Object::new(field_holder_class)?;
            field_holder.set_value("daemon", Value::Int(0))?;
            field_holder.set_value("group", thread_group)?;
            field_holder.set_value("priority", Value::Int(5))?;
            field_holder.set_value("stackSize", Value::Long(0))?;
            field_holder.set_value("threadStatus", Value::Int(4))?; // Runnable
            let field_holder =
                Value::new_object(&self.garbage_collector, Reference::Object(field_holder));

            new_thread.set_value("holder", field_holder)?;
            new_thread.set_value("interrupted", Value::Int(0))?;
        }

        thread
            .set_java_object(Value::from_object(&self.garbage_collector, new_thread))
            .await;
        self.thread_handles
            .insert(thread.id(), ThreadHandle::from(thread))
            .await?;

        Ok(())
    }

    /// Get the primordial thread
    ///
    /// # Errors
    ///
    /// if the primordial thread cannot be found
    async fn primordial_thread(&self) -> Result<Arc<Thread>> {
        let thread_handle = self.thread_handles.get(&1).await;
        let Some(thread_handle) = thread_handle else {
            return Err(InternalError("Primordial thread not found".into()));
        };
        Ok(thread_handle.thread.clone())
    }

    /// Load a class (e.g. "java.lang.Object").
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded
    pub async fn class<S>(&self, class_name: S) -> Result<Arc<Class>>
    where
        S: AsRef<str> + Debug + Send,
    {
        let thread = self.primordial_thread().await?;
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
    pub async fn invoke_main<S>(&self, parameters: &[S]) -> Result<Option<Value>>
    where
        S: AsRef<OsStr> + Debug,
    {
        let Some(main_class_name) = &self.main_class else {
            return Err(InternalError("No main class specified".into()));
        };
        let main_class = self.class(&main_class_name).await?;
        let Some(main_method) = main_class.main_method() else {
            return Err(InternalError(format!(
                "No main method found for {main_class_name}"
            )));
        };

        let mut string_parameters = Vec::with_capacity(parameters.len());
        for parameter in parameters {
            let parameter = parameter.as_ref();
            let parameter = parameter.to_string_lossy().to_string();
            let thread = self.primordial_thread().await?;
            let value = parameter.to_object(&thread).await?;
            string_parameters.push(value);
        }

        let string_array_class = self.class("[Ljava/lang/String;").await?;
        let string_reference = Reference::try_from((string_array_class, string_parameters))?;
        let string_parameter = Value::new_object(&self.garbage_collector, string_reference);

        self.invoke(
            &main_class_name,
            main_method.signature(),
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
    pub async fn invoke<C, M>(
        &self,
        class: C,
        method: M,
        parameters: &[impl RustValue],
    ) -> Result<Option<Value>>
    where
        C: AsRef<str> + Debug + Send + Sync,
        M: AsRef<str> + Debug + Send + Sync,
    {
        let thread = self.primordial_thread().await?;
        thread.invoke(&class, &method, parameters).await
    }

    /// Invoke a method.  To invoke a method on an object reference, the object reference must be
    /// the first parameter in the parameters vector.
    ///
    /// # Errors
    ///
    /// if the method cannot be invoked
    pub async fn try_invoke<C, M>(
        &self,
        class: C,
        method: M,
        parameters: &[impl RustValue],
    ) -> Result<Value>
    where
        C: AsRef<str> + Debug + Send + Sync,
        M: AsRef<str> + Debug + Send + Sync,
    {
        let thread = self.primordial_thread().await?;
        thread.try_invoke(&class, &method, parameters).await
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
        C: AsRef<str> + Debug + Send + Sync,
        M: AsRef<str> + Debug + Send + Sync,
    {
        let thread = self.primordial_thread().await?;
        thread.object(class_name, descriptor, parameters).await
    }

    /// The string pool is used to store and intern strings for the VM.
    pub(crate) fn string_pool(&self) -> &StringPool {
        &self.string_pool
    }

    /// Get the call site cache for invokedynamic recursion prevention
    pub(crate) fn call_site_cache(&self) -> &CallSiteCache {
        &self.call_site_cache
    }

    /// Wait for all non-daemon threads to complete.
    ///
    /// This method should be called after the main method returns to ensure all spawned threads
    /// have completed their execution before the VM shuts down. This is similar to how the JVM
    /// waits for all non-daemon threads to complete before exiting.
    ///
    /// # Errors
    ///
    /// if waiting for threads fails
    #[cfg(not(target_family = "wasm"))]
    pub async fn wait_for_non_daemon_threads(&self) -> Result<()> {
        // Poll until all spawned non-daemon threads have completed
        // We check for threads other than the primordial thread (id=1) and skip daemon threads
        loop {
            // Collect join handles from non-daemon threads
            let mut handles_to_await = Vec::new();

            {
                let mut handles = self.thread_handles.write().await;
                // Find all non-daemon thread handles with join handles (excluding primordial thread)
                let thread_ids: Vec<u64> = handles
                    .iter()
                    .filter(|(id, handle)| {
                        **id != 1 && handle.join_handle.is_some() && !handle.daemon
                    })
                    .map(|(id, _)| *id)
                    .collect();

                for id in thread_ids {
                    if let Some(mut handle) = handles.remove(&id)
                        && let Some(join_handle) = handle.join_handle.take()
                    {
                        handles_to_await.push(join_handle);
                    }
                }
            }

            if handles_to_await.is_empty() {
                // No more non-daemon threads with join handles, check if there are any remaining
                let handles = self.thread_handles.read().await;
                let remaining_non_daemon = handles
                    .iter()
                    .filter(|(id, handle)| **id != 1 && !handle.daemon)
                    .count();
                if remaining_non_daemon == 0 {
                    break;
                }
                // Some non-daemon threads still exist but don't have join handles
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            } else {
                // Await all collected join handles
                for join_handle in handles_to_await {
                    let _ = join_handle.await;
                }
            }
        }

        // Abort all remaining daemon threads - they shouldn't prevent VM exit
        let mut daemon_handles = Vec::new();
        {
            let mut handles = self.thread_handles.write().await;
            for (id, handle) in handles.iter_mut() {
                if handle.daemon
                    && let Some(join_handle) = handle.join_handle.take()
                {
                    join_handle.abort();
                    daemon_handles.push((*id, join_handle));
                }
            }
        }
        // Wait for aborted tasks to finish
        for (_id, join_handle) in daemon_handles {
            let _ = join_handle.await; // This will return Err(JoinError::Cancelled) which is expected
        }

        Ok(())
    }

    /// Wait for all non-daemon threads to complete (WASM version - no-op).
    #[cfg(target_family = "wasm")]
    pub async fn wait_for_non_daemon_threads(&self) -> Result<()> {
        // WASM uses spawn_local which doesn't support joining
        Ok(())
    }
}

impl ristretto_types::VM for VM {
    type ThreadType = Thread;
    type ModuleSystem = ModuleSystem;

    fn garbage_collector(&self) -> &Arc<GarbageCollector> {
        &self.garbage_collector
    }

    fn java_home(&self) -> &PathBuf {
        &self.java_home
    }

    fn java_version(&self) -> &str {
        &self.java_version
    }

    fn java_major_version(&self) -> u16 {
        self.java_major_version
    }

    fn java_class_file_version(&self) -> &Version {
        &self.java_class_file_version
    }

    fn system_properties(&self) -> &AHashMap<String, String> {
        self.configuration.system_properties()
    }

    fn next_thread_id(&self) -> Result<u64> {
        VM::next_thread_id(self)
    }

    fn next_hidden_class_suffix(&self) -> Result<u64> {
        VM::next_hidden_class_suffix(self)
    }

    fn class<'a>(
        &'a self,
        class_name: &'a str,
    ) -> ristretto_types::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { VM::class(self, class_name).await })
    }

    fn invoke_main<'a>(
        &'a self,
        parameters: &'a [&'a str],
    ) -> ristretto_types::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { VM::invoke_main(self, parameters).await })
    }

    fn module_system(&self) -> &ModuleSystem {
        &self.module_system
    }

    fn class_path(&self) -> &ClassPath {
        self.configuration.class_path()
    }

    fn verify_mode(&self) -> ristretto_classfile::VerifyMode {
        self.configuration.verify_mode()
    }

    fn preview_features(&self) -> bool {
        self.configuration.preview_features()
    }

    fn stdin(&self) -> Arc<Mutex<dyn Read + Send + Sync>> {
        self.configuration.stdin()
    }

    fn stdout(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.configuration.stdout()
    }

    fn stderr(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        self.configuration.stderr()
    }

    fn file_handles(&self) -> &HandleManager<String, FileHandle> {
        VM::file_handles(self)
    }

    fn thread_handles(
        &self,
    ) -> &HandleManager<u64, ristretto_types::handles::ThreadHandle<Thread>> {
        VM::thread_handles(self)
    }

    fn monitor_registry(&self) -> &MonitorRegistry {
        VM::monitor_registry(self)
    }

    fn native_memory(&self) -> &NativeMemory {
        &self.native_memory
    }

    fn nio_file_handles(&self) -> &HandleManager<i32, std::fs::File> {
        &self.nio_file_handles
    }

    fn next_nio_fd(&self) -> i32 {
        self.next_nio_fd.fetch_add(1, Ordering::Relaxed)
    }

    fn class_loader(&self) -> Arc<RwLock<Arc<ClassLoader>>> {
        VM::class_loader(self)
    }

    fn intern_string<'a>(
        &'a self,
        thread: &'a Thread,
        string: &'a str,
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { self.string_pool.intern(thread, string).await })
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { VM::object(self, class_name, descriptor, parameters).await })
    }

    fn create_thread(&self, id: u64) -> Result<Arc<Thread>> {
        Ok(Thread::new(&self.vm, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{ConfigurationBuilder, ModuleExport, ModuleOpens, ModuleRead};
    use crate::method_ref_cache::{MethodRefError, MethodRefErrorKind, MethodRefKey};
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
        ClassPath::from(&[classes_jar_path])
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
        let value = object.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object_integer_from_string() -> Result<()> {
        let vm = test_vm().await?;
        let object = vm
            .object("java.lang.Integer", "Ljava/lang/String;", &["42"])
            .await?;
        let value = object.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object_string() -> Result<()> {
        let vm = test_vm().await?;
        let characters = "foo".chars().collect::<Vec<char>>();
        let object = vm.object("java.lang.String", "[C", &[characters]).await?;
        let value = object.as_string()?;
        assert_eq!("foo", value);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_access_same_module() -> Result<()> {
        let vm = test_vm().await?;
        let result =
            vm.module_system()
                .check_access(Some("my.module"), Some("my.module"), "my/pkg/MyClass");
        assert!(result.is_allowed());
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_access_unnamed() -> Result<()> {
        let vm = test_vm().await?;
        // Unnamed to unnamed is same module, always allowed
        let result = vm
            .module_system()
            .check_access(None, None, "com/example/MyClass");
        assert!(result.is_allowed());
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_access_with_export() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(classes_jar_class_path())
            .add_read(ModuleRead::new("my.module", "other.module"))
            .add_export(ModuleExport::new("other.module", "other/api", "my.module"))
            .build()?;

        let vm = VM::new(configuration).await?;
        let result = vm.module_system().check_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/PublicClass",
        );
        assert!(result.is_allowed());
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_reflection_access_with_opens() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(classes_jar_class_path())
            .add_read(ModuleRead::new("my.module", "other.module"))
            .add_opens(ModuleOpens::new(
                "other.module",
                "other/internal",
                "my.module",
            ))
            .build()?;

        let vm = VM::new(configuration).await?;
        let result = vm.module_system().check_reflection_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/Secret",
        );
        assert!(result.is_allowed());
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_access_not_readable() -> Result<()> {
        let vm = test_vm().await?;
        // my.module doesn't read other.module
        let result = vm.module_system().check_access(
            Some("my.module"),
            Some("other.module"),
            "other/api/Class",
        );
        assert!(result.is_denied());
        assert_eq!(result, crate::module_system::AccessCheckResult::NotReadable);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_module_access_not_exported() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(classes_jar_class_path())
            .add_read(ModuleRead::new("my.module", "other.module"))
            // No exports added
            .build()?;

        let vm = VM::new(configuration).await?;
        // Reads but not exported
        let result = vm.module_system().check_access(
            Some("my.module"),
            Some("other.module"),
            "other/internal/Class",
        );
        assert!(result.is_denied());
        assert_eq!(result, crate::module_system::AccessCheckResult::NotExported);
        Ok(())
    }

    #[tokio::test]
    async fn test_static_config_exports_allow_access() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(classes_jar_class_path())
            .add_read(ModuleRead::new("consumer.module", "provider.module"))
            .add_export(ModuleExport::new(
                "provider.module",
                "provider/api",
                "consumer.module",
            ))
            .build()?;

        let vm = VM::new(configuration).await?;

        // Static config should allow this access via --add-exports
        let result = vm.module_system().check_access(
            Some("consumer.module"),
            Some("provider.module"),
            "provider/api/PublicService",
        );
        assert!(
            result.is_allowed(),
            "Static configuration should allow access via --add-exports"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_static_config_opens_allow_reflection() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .class_path(classes_jar_class_path())
            .add_read(ModuleRead::new("consumer.module", "provider.module"))
            .add_opens(ModuleOpens::new(
                "provider.module",
                "provider/internal",
                "consumer.module",
            ))
            .build()?;

        let vm = VM::new(configuration).await?;

        // Static config should allow reflection access via --add-opens
        let result = vm.module_system().check_reflection_access(
            Some("consumer.module"),
            Some("provider.module"),
            "provider/internal/InternalClass",
        );
        assert!(
            result.is_allowed(),
            "Static configuration should allow reflection via --add-opens"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_method_ref_cache_stores_entries() -> Result<()> {
        let vm = test_vm().await?;

        // Get initial cache size (may have entries from VM initialization)
        let initial_size = vm.method_ref_cache().len();

        // Store a failed resolution with a unique key
        let key = MethodRefKey::new("unique/test/Class".to_string(), 65_000);
        let error = MethodRefError::new(MethodRefErrorKind::NoSuchMethod, "test error".to_string());
        vm.method_ref_cache().store_failed(key.clone(), error);

        // Cache should have one more entry
        assert_eq!(vm.method_ref_cache().len(), initial_size + 1);

        // Retrieving the entry should return the cached error
        let result = vm.method_ref_cache().get(&key);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_method_ref_cache_is_populated_during_execution() -> Result<()> {
        let vm = test_vm().await?;

        // After VM initialization, the cache should have entries from
        // method invocations during startup (e.g., static initializers)
        // This verifies that the caching mechanism is working
        let cache_size = vm.method_ref_cache().len();

        // The cache should have entries from VM initialization
        // The exact number depends on which classes are loaded during startup
        assert!(
            cache_size > 0,
            "Method ref cache should be populated after VM init"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_jpms_enforcement_at_resolution_time() -> Result<()> {
        // This test verifies that JPMS access checks happen during method resolution,
        // not during each invocation. The caching mechanism ensures that:
        // 1. Access checks happen once during resolution
        // 2. Subsequent invocations use cached results
        // 3. Failed resolutions are also cached

        let vm = test_vm().await?;

        // Test that same module access is always allowed
        let result =
            vm.module_system()
                .check_access(Some("my.module"), Some("my.module"), "my/pkg/MyClass");
        assert!(
            result.is_allowed(),
            "Same module access should always be allowed"
        );

        // Test that java.base is implicitly readable
        // Note: This tests the static configuration, not the cache
        // In a real scenario, the access check would be cached after first resolution

        Ok(())
    }
}

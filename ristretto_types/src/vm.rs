use crate::Result;
use crate::handles::{FileHandle, HandleManager, ThreadHandle};
use crate::module_access::ModuleAccess;
use crate::monitor::MonitorRegistry;
use ahash::AHashMap;
use ristretto_classfile::{VerifyMode, Version};
use ristretto_classloader::{Class, ClassLoader, ClassPath, Value};
use ristretto_gc::GarbageCollector;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Trait representing the virtual machine.
pub trait VM: Send + Sync {
    /// The concrete thread type for this VM.
    type ThreadType: crate::Thread;

    /// The concrete module system type.
    type ModuleSystem: ModuleAccess;

    /// Get the garbage collector.
    fn garbage_collector(&self) -> &Arc<GarbageCollector>;

    /// Get the Java home directory.
    fn java_home(&self) -> &PathBuf;

    /// Get the Java version string.
    fn java_version(&self) -> &str;

    /// Get the Java major version number.
    fn java_major_version(&self) -> u16;

    /// Get the Java class file version.
    fn java_class_file_version(&self) -> &Version;

    /// Get the system properties.
    fn system_properties(&self) -> &AHashMap<String, String>;

    /// Get the next thread ID.
    ///
    /// # Errors
    /// Returns an error if the ID cannot be generated.
    fn next_thread_id(&self) -> Result<u64>;

    /// Get the next hidden class suffix.
    ///
    /// # Errors
    /// Returns an error if the suffix cannot be generated.
    fn next_hidden_class_suffix(&self) -> Result<u64>;

    /// Load a class by name.
    ///
    /// # Errors
    /// Returns an error if the class cannot be loaded.
    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>>;

    /// Invoke the main method of a class.
    ///
    /// # Errors
    /// Returns an error if the main method cannot be invoked.
    fn invoke_main<'a>(
        &'a self,
        parameters: &'a [&'a str],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>>;

    /// Get the module system.
    fn module_system(&self) -> &Self::ModuleSystem;

    /// Get the class path.
    fn class_path(&self) -> &ClassPath;

    /// Get the verification mode.
    fn verify_mode(&self) -> VerifyMode;

    /// Get whether preview features are enabled.
    fn preview_features(&self) -> bool;

    /// Get the standard input stream.
    fn stdin(&self) -> Arc<Mutex<dyn Read + Send + Sync>>;

    /// Get the standard output stream.
    fn stdout(&self) -> Arc<Mutex<dyn Write + Send + Sync>>;

    /// Get the standard error stream.
    fn stderr(&self) -> Arc<Mutex<dyn Write + Send + Sync>>;

    /// Get the file handles manager.
    fn file_handles(&self) -> &HandleManager<String, FileHandle>;

    /// Get the thread handles manager.
    fn thread_handles(&self) -> &HandleManager<u64, ThreadHandle<Self::ThreadType>>;

    /// Get the monitor registry.
    fn monitor_registry(&self) -> &MonitorRegistry;

    /// Get the class loader.
    fn class_loader(&self) -> Arc<RwLock<Arc<ClassLoader>>>;

    /// Intern a string, returning the associated Java String value.
    ///
    /// # Errors
    /// Returns an error if the string cannot be interned.
    fn intern_string<'a>(
        &'a self,
        thread: &'a Self::ThreadType,
        string: &'a str,
    ) -> crate::BoxFuture<'a, Result<Value>>;

    /// Create a new VM object by invoking a constructor.
    ///
    /// # Errors
    /// Returns an error if the object cannot be created.
    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>>;

    /// Create a new thread with the given ID.
    ///
    /// # Errors
    /// Returns an error if the thread cannot be created.
    fn create_thread(&self, id: u64) -> Result<Arc<Self::ThreadType>>;
}

/// Blanket implementation of VM for `Arc<V>` where `V: VM`.
impl<V: VM> VM for Arc<V> {
    type ThreadType = V::ThreadType;
    type ModuleSystem = V::ModuleSystem;

    fn garbage_collector(&self) -> &Arc<GarbageCollector> {
        (**self).garbage_collector()
    }

    fn java_home(&self) -> &PathBuf {
        (**self).java_home()
    }

    fn java_version(&self) -> &str {
        (**self).java_version()
    }

    fn java_major_version(&self) -> u16 {
        (**self).java_major_version()
    }

    fn java_class_file_version(&self) -> &Version {
        (**self).java_class_file_version()
    }

    fn system_properties(&self) -> &AHashMap<String, String> {
        (**self).system_properties()
    }

    fn next_thread_id(&self) -> Result<u64> {
        (**self).next_thread_id()
    }

    fn next_hidden_class_suffix(&self) -> Result<u64> {
        (**self).next_hidden_class_suffix()
    }

    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        (**self).class(class_name)
    }

    fn invoke_main<'a>(
        &'a self,
        parameters: &'a [&'a str],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        (**self).invoke_main(parameters)
    }

    fn module_system(&self) -> &Self::ModuleSystem {
        (**self).module_system()
    }

    fn class_path(&self) -> &ClassPath {
        (**self).class_path()
    }

    fn verify_mode(&self) -> VerifyMode {
        (**self).verify_mode()
    }

    fn preview_features(&self) -> bool {
        (**self).preview_features()
    }

    fn stdin(&self) -> Arc<Mutex<dyn Read + Send + Sync>> {
        (**self).stdin()
    }

    fn stdout(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        (**self).stdout()
    }

    fn stderr(&self) -> Arc<Mutex<dyn Write + Send + Sync>> {
        (**self).stderr()
    }

    fn file_handles(&self) -> &HandleManager<String, FileHandle> {
        (**self).file_handles()
    }

    fn thread_handles(&self) -> &HandleManager<u64, ThreadHandle<Self::ThreadType>> {
        (**self).thread_handles()
    }

    fn monitor_registry(&self) -> &MonitorRegistry {
        (**self).monitor_registry()
    }

    fn class_loader(&self) -> Arc<RwLock<Arc<ClassLoader>>> {
        (**self).class_loader()
    }

    fn intern_string<'a>(
        &'a self,
        thread: &'a Self::ThreadType,
        string: &'a str,
    ) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).intern_string(thread, string)
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        (**self).object(class_name, descriptor, parameters)
    }

    fn create_thread(&self, id: u64) -> Result<Arc<Self::ThreadType>> {
        (**self).create_thread(id)
    }
}

use crate::Result;
#[cfg(not(target_family = "wasm"))]
use crate::handles::SocketHandle;
use crate::handles::{FileHandle, HandleManager, ThreadHandle};
use crate::module_access::ModuleAccess;
use crate::monitor::MonitorRegistry;
use crate::native_memory::NativeMemory;
use crate::resource_manager::ResourceManager;
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

    /// Get the next NIO file descriptor.
    fn next_nio_fd(&self) -> i32;

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

    /// Get the native memory manager.
    fn native_memory(&self) -> &NativeMemory;

    /// Get the resource manager for type-erased per-VM resource storage.
    fn resource_manager(&self) -> &ResourceManager;

    /// Get the file handles manager.
    fn file_handles(&self) -> &HandleManager<i64, FileHandle>;

    /// Get the socket handles manager.
    #[cfg(not(target_family = "wasm"))]
    fn socket_handles(&self) -> &HandleManager<i32, SocketHandle>;

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

    fn next_nio_fd(&self) -> i32 {
        (**self).next_nio_fd()
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

    fn native_memory(&self) -> &NativeMemory {
        (**self).native_memory()
    }

    fn resource_manager(&self) -> &ResourceManager {
        (**self).resource_manager()
    }

    fn thread_handles(&self) -> &HandleManager<u64, ThreadHandle<Self::ThreadType>> {
        (**self).thread_handles()
    }

    fn file_handles(&self) -> &HandleManager<i64, FileHandle> {
        (**self).file_handles()
    }

    #[cfg(not(target_family = "wasm"))]
    fn socket_handles(&self) -> &HandleManager<i32, SocketHandle> {
        (**self).socket_handles()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Thread;
    use crate::test_utils;
    use ristretto_classfile::JAVA_17;

    #[tokio::test]
    async fn test_arc_vm_delegates_to_inner_vm() -> Result<()> {
        let vm: Arc<test_utils::MockVm> = test_utils::MockVm::new(JAVA_17);
        let vm_ref: &dyn VM<ThreadType = test_utils::MockThread, ModuleSystem = test_utils::MockModuleSystem> =
            &vm;
        let thread = test_utils::MockThread::new(vm.clone());

        assert!(Arc::strong_count(vm_ref.garbage_collector()) >= 1);
        assert_eq!(vm_ref.java_home(), &PathBuf::from("/mock/java/home"));
        assert_eq!(vm_ref.java_version(), "mock-java");
        assert_eq!(vm_ref.java_major_version(), JAVA_17.major());
        assert_eq!(vm_ref.java_class_file_version(), &JAVA_17);
        assert!(vm_ref.system_properties().is_empty());
        assert_eq!(vm_ref.next_thread_id()?, 1);
        assert_eq!(vm_ref.next_hidden_class_suffix()?, 1);
        assert_eq!(vm_ref.next_nio_fd(), 3);
        assert_eq!(
            vm_ref.class("java/lang/Object").await?.name(),
            "java/lang/Object"
        );
        assert_eq!(
            vm_ref.invoke_main(&["arg"]).await?.expect("main result"),
            Value::Int(0)
        );
        assert!(!vm_ref.module_system().is_lightweight_mode());
        assert!(vm_ref.class_path().iter().next().is_none());
        assert_eq!(vm_ref.verify_mode(), VerifyMode::Remote);
        assert!(!vm_ref.preview_features());
        assert!(vm_ref.stdin().try_lock().is_ok());
        assert!(vm_ref.stdout().try_lock().is_ok());
        assert!(vm_ref.stderr().try_lock().is_ok());
        assert!(!vm_ref.native_memory().contains(0));
        assert!(format!("{:?}", vm_ref.resource_manager()).contains("ResourceManager"));
        assert!(vm_ref.file_handles().read().await.is_empty());
        #[cfg(not(target_family = "wasm"))]
        assert!(vm_ref.socket_handles().read().await.is_empty());
        assert!(vm_ref.thread_handles().read().await.is_empty());
        assert!(Arc::ptr_eq(
            &vm_ref.monitor_registry().monitor(1),
            &vm_ref.monitor_registry().monitor(1)
        ));
        assert_eq!(vm_ref.class_loader().read().await.name(), "bootstrap");
        assert!(vm_ref.intern_string(&thread, "intern").await?.is_object());
        assert!(
            vm_ref
                .object("java/lang/Object", "()V", &[])
                .await?
                .is_object()
        );
        assert_eq!(vm_ref.create_thread(42)?.id(), 42);
        Ok(())
    }
}

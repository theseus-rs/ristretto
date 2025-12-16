use crate::Error::{InternalError, UnsupportedClassFileVersion};
use crate::JavaError::RuntimeException;
use crate::parameters::Parameters;
use crate::rust_value::{RustValue, process_values};
use crate::{Frame, Result, VM, jit};
use async_recursion::async_recursion;
use byte_unit::{Byte, UnitType};
use ristretto_classloader::Error::MethodNotFound;
use ristretto_classloader::{Class, Method, Object, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tokio::sync::{Notify, RwLock};
use tokio::time::{Instant, timeout_at};
use tracing::{Level, debug, event_enabled};

/// A state that is used to park a thread.  The thread will be parked until it is unparked by
/// another thread or interrupted.
#[derive(Debug)]
struct ParkState {
    permit: AtomicBool,
    interrupted: AtomicBool,
    notify: Notify,
}

impl ParkState {
    /// Create a new `ParkState`.
    fn new() -> Self {
        Self {
            permit: AtomicBool::new(false),
            interrupted: AtomicBool::new(false),
            notify: Notify::new(),
        }
    }
}

/// A thread is a single sequential flow of control within a program. It has its own call stack
/// and program counter.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5.2>
#[expect(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Thread {
    id: u64,
    vm: Weak<VM>,
    thread: Weak<Thread>,
    name: Arc<RwLock<String>>,
    java_object: Arc<RwLock<Value>>,
    frames: Arc<RwLock<Vec<Arc<Frame>>>>,
    park_state: ParkState,
}

impl Thread {
    /// Create a new thread.
    pub fn new(vm: &Weak<VM>, id: u64) -> Arc<Self> {
        let vm_ref = vm.clone();
        let name = format!("Thread-{id}");
        let java_object = Value::Object(None);
        Arc::new_cyclic(|thread| Thread {
            id,
            vm: vm_ref,
            thread: thread.clone(),
            name: Arc::new(RwLock::new(name)),
            java_object: Arc::new(RwLock::new(java_object)),
            frames: Arc::new(RwLock::new(Vec::new())),
            park_state: ParkState::new(),
        })
    }

    /// Get the identifier of the thread.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the virtual machine that owns the thread.
    ///
    /// # Errors
    ///
    /// if the virtual machine cannot be accessed.
    pub fn vm(&self) -> Result<Arc<VM>> {
        match self.vm.upgrade() {
            Some(vm) => Ok(vm),
            None => Err(InternalError("VM is not available".to_string())),
        }
    }

    /// Get the name of the thread.
    pub async fn name(&self) -> String {
        let name = self.name.read().await;
        name.clone()
    }

    /// Set the name of the thread.
    pub async fn set_name<S: AsRef<str>>(&self, name: S) {
        let new_name = name.as_ref();
        let mut name = self.name.write().await;
        *name = new_name.to_string();
    }

    /// Get the Java object for this thread.
    pub async fn java_object(&self) -> Value {
        let object = self.java_object.read().await;
        object.clone()
    }

    /// Set the Java thread object for this thread.
    pub async fn set_java_object(&self, new_java_object: Value) {
        let mut java_object = self.java_object.write().await;
        *java_object = new_java_object;
    }

    /// Get the frames in the thread.
    ///
    /// # Errors
    ///
    /// if the frames cannot be accessed.
    pub async fn frames(&self) -> Result<Vec<Arc<Frame>>> {
        let frames = self.frames.read().await;
        Ok(frames.clone())
    }

    /// Get the current frame in the thread.
    ///
    /// # Errors
    ///
    /// if the current frame cannot be accessed.
    pub async fn current_frame(&self) -> Result<Arc<Frame>> {
        let frames = self.frames.read().await;
        let frame = frames.last().ok_or(InternalError("No frame".to_string()))?;
        Ok(frame.clone())
    }

    /// Set the thread as interrupted.
    pub fn interrupt(&self) {
        self.park_state.interrupted.store(true, Ordering::SeqCst);
        self.unpark();
    }

    /// Check if the thread is interrupted and clear the interrupt if specified.
    pub fn is_interrupted(&self, clear_interrupt: bool) -> bool {
        if clear_interrupt {
            self.park_state.interrupted.swap(false, Ordering::SeqCst)
        } else {
            self.park_state.interrupted.load(Ordering::SeqCst)
        }
    }

    /// Park the thread.  If the permit is available, it will be consumed and the thread will return
    /// immediately. If the permit is not available, the thread will be parked until it is unparked
    /// or the specified time has elapsed.
    ///
    /// # Arguments
    ///
    /// * `is_absolute` - If true, the `time` parameter is treated as an absolute timestamp
    ///   (milliseconds since epoch).
    /// * `time` - The time to park the thread. If `is_absolute` is true, this is the absolute
    ///   timestamp in milliseconds since epoch. If `is_absolute` is false, this is the relative
    ///   duration in nanoseconds.
    ///
    /// # Errors
    ///
    /// If the parking operation fails, an error will be returned.
    pub async fn park(&self, is_absolute: bool, time: u64) -> Result<()> {
        if self.is_interrupted(false) {
            return Ok(());
        }

        // Fast-path: if permit is available, consume it and return
        if self.park_state.permit.swap(false, Ordering::Acquire) {
            return Ok(());
        }

        // Calculate target time or duration
        if time == 0 {
            // Double-check if we have been unparked before sleeping
            while self.park_state.permit.swap(false, Ordering::Acquire) {
                self.park_state.notify.notified().await;
            }
        } else if is_absolute {
            // Absolute timestamp (milliseconds since epoch)
            let now = u64::try_from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| RuntimeException(format!("Time went backwards: {error}")))?
                    .as_millis(),
            )?;
            let duration = if time > now {
                time.saturating_sub(now)
            } else {
                0
            };
            let deadline = Instant::now() + Duration::from_millis(duration);

            // Wait until permit or deadline
            let notified = self.park_state.notify.notified();
            let _ = timeout_at(deadline, notified).await;
            // Also check if unpark happened during sleep
            self.park_state.permit.swap(false, Ordering::Acquire);
        } else {
            // Relative duration in nanoseconds
            let duration = Duration::from_nanos(time);
            let deadline = Instant::now() + duration;

            let notified = self.park_state.notify.notified();
            let _ = timeout_at(deadline, notified).await;
            self.park_state.permit.swap(false, Ordering::Acquire);
        }
        Ok(())
    }

    /// Unpark the thread if it is parked.
    pub fn unpark(&self) {
        self.park_state.permit.store(true, Ordering::Release);
        self.park_state.notify.notify_one();
    }

    /// Get a class.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.1>
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded
    #[expect(clippy::multiple_bound_locations)]
    #[async_recursion(?Send)]
    pub(crate) async fn class<S: AsRef<str>>(&self, class_name: S) -> Result<Arc<Class>> {
        let class_name = class_name.as_ref();
        let (class, previously_loaded) = {
            let vm = self.vm()?;
            let class_loader_lock = vm.class_loader();
            let class_loader = class_loader_lock.read().await;
            class_loader.load_with_status(class_name).await?
        };

        // Determine if the class has already been loaded.  If the class has already been loaded,
        // return the class. Otherwise, the class must be initialized.
        if previously_loaded {
            return Ok(class);
        }

        let classes = self.prepare_class_initialization(&class).await?;
        for current_class in classes {
            if let Some(class_initializer) = current_class.class_initializer() {
                // Execute the class initializer on the current thread.
                self.execute(&current_class, &class_initializer, &[] as &[Value])
                    .await?;
            }
        }
        Ok(class)
    }

    /// Prepare class initialization.
    ///
    /// # Errors
    ///
    /// if the class cannot be resolved
    async fn prepare_class_initialization(&self, class: &Arc<Class>) -> Result<Vec<Arc<Class>>> {
        let vm = self.vm()?;
        let class_loader_lock = vm.class_loader();
        let class_loader = class_loader_lock.read().await;
        let mut classes = Vec::new();
        let mut index = 0;
        classes.push(class.clone());

        while index < classes.len() {
            let Some(current_class) = classes.get(index) else {
                break;
            };
            let current_class = current_class.clone();

            if current_class.class_file().version > *vm.java_class_file_version() {
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
    ///
    /// if the class cannot be registered
    pub(crate) async fn register_class(&self, class: Arc<Class>) -> Result<()> {
        debug!("register class: {class}");
        let vm = self.vm()?;
        let class_loader_lock = vm.class_loader();
        let class_loader = class_loader_lock.read().await;
        class_loader.register(class).await?;
        Ok(())
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
        C: AsRef<str>,
        M: AsRef<str>,
    {
        let class = self.class(class).await?;
        let method = method.as_ref();
        let index = method.find('(').unwrap_or_default();
        let name = &method[..index];
        let descriptor = &method[index..];
        let method = class.try_get_method(name, descriptor)?;
        self.execute(&class, &method, parameters).await
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
        C: AsRef<str>,
        M: AsRef<str>,
    {
        let Some(value) = self.invoke(class, method, parameters).await? else {
            return Err(InternalError("No return value".into()));
        };
        Ok(value)
    }

    /// Add a new frame to the thread and invoke the method. To invoke a method on an object
    /// reference, the object reference must be the first parameter in the parameters vector.
    ///
    /// # Errors
    ///
    /// if the method cannot be invoked.
    pub async fn execute(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        parameters: &[impl RustValue],
    ) -> Result<Option<Value>> {
        let class_name = class.name();
        let method_name = method.name();
        let method_descriptor = method.descriptor();
        let vm = self.vm()?;
        let parameters = process_values(self, parameters).await?;
        let method_registry = vm.method_registry();
        let rust_method = method_registry.method(class_name, method_name, method_descriptor);
        // If the method is not found in the registry, try to JIT compile it.
        let jit_method = if rust_method.is_none() {
            jit::compile(&vm, class, method).await?
        } else {
            None
        };

        if event_enabled!(Level::DEBUG) {
            self.debug_execute(
                class_name,
                method_name,
                method_descriptor,
                method,
                rust_method.is_some(),
                jit_method.is_some(),
            );
        }

        let (result, frame_added) = if let Some(rust_method) = rust_method {
            let Some(thread) = self.thread.upgrade() else {
                return Err(InternalError("Call stack is not available".to_string()));
            };
            let parameters = Parameters::new(parameters);
            let result = rust_method(thread, parameters).await;
            (result, false)
        } else if let Some(jit_method) = jit_method {
            let result = jit::execute(&jit_method, method, parameters);
            (result, false)
        } else if method.is_native() {
            return Err(MethodNotFound {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                method_descriptor: method_descriptor.to_string(),
            }
            .into());
        } else {
            let frame = Arc::new(Frame::new(&self.thread, class, method));

            // Limit the scope of the write lock to just adding the frame to the thread. This
            // is necessary because java.lang.Thread (e.g. countStackFrames) needs to be able to
            // access the thread's frames without causing a deadlock.
            {
                let mut frames = self.frames.write().await;
                frames.push(frame.clone());
            }
            let result = frame.execute(parameters).await;
            (result, true)
        };

        if event_enabled!(Level::DEBUG) {
            let result = match &result {
                Ok(Some(value)) => {
                    let value = value.to_string();
                    if value.len() > 100 {
                        format!("{}...", &value.as_str()[..97])
                    } else {
                        value
                    }
                }
                Ok(None) => "void".to_string(),
                Err(error) => {
                    format!("[ERROR] {error}")
                }
            };
            debug!("result: {class_name}.{method_name}{method_descriptor}: {result}");
        }

        if frame_added {
            let mut frames = self.frames.write().await;
            frames.pop();
        }

        result
    }

    /// Debug the execution of a method.
    #[expect(clippy::unused_self)]
    fn debug_execute(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: &Arc<Method>,
        is_rust: bool,
        is_jit: bool,
    ) {
        let execution_type = if is_rust {
            "rust"
        } else if is_jit {
            "jit"
        } else {
            "int"
        };
        let access_flags = method.access_flags();
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_memory()),
        );

        let pid = std::process::id() as usize;
        let memory = if let Some(process) = system.process(Pid::from(pid)) {
            let memory = process.memory();
            let memory = Byte::from_u64(memory).get_appropriate_unit(UnitType::Decimal);
            format!(" ({execution_type}; {memory:#.3})")
        } else {
            format!("({execution_type})")
        };
        debug!("execute{memory}: {class_name}.{method_name}{method_descriptor} {access_flags}");
    }

    /// Add a new frame to the thread and invoke the method. To invoke a method on an object
    /// reference, the object reference must be the first parameter in the parameters vector.
    ///
    /// # Errors
    ///
    /// if the method cannot be invoked.
    pub async fn try_execute(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        parameters: &[impl RustValue],
    ) -> Result<Value> {
        let result = Box::pin(self.execute(class, method, parameters)).await?;
        match result {
            Some(value) => Ok(value),
            None => Err(InternalError("No return value".to_string())),
        }
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
        let class_name = class_name.as_ref();
        let descriptor = &format!("({})V", descriptor.as_ref());
        let class = self.class(class_name).await?;
        let Some(constructor) = class.method("<init>", descriptor) else {
            return Err(InternalError(format!(
                "No constructor found: {class_name}.<init>{descriptor}"
            )));
        };

        let mut constructor_parameters = Vec::with_capacity(parameters.len() + 1);
        let object = Value::from(Object::new(class.clone())?);
        constructor_parameters.insert(0, object.clone());
        for parameter in parameters {
            let value = parameter.to_value();
            constructor_parameters.push(value);
        }
        let parameters = process_values(self, &constructor_parameters).await?;
        Box::pin(self.execute(&class, &constructor, &parameters)).await?;
        Ok(object)
    }

    /// Print the stack trace. Used for debugging.
    pub(crate) async fn print_stack_trace(&self) {
        let name = self.name().await;
        eprintln!("Thread: {name}");
        let frames = self.frames.read().await;
        for frame in frames.iter().rev() {
            let class = frame.class();
            let class_name = class.name();
            let mut source = class.source_file().unwrap_or_default().to_string();
            let method = frame.method();
            let method_name = method.name();
            let program_counter = frame.program_counter();
            let line_number = method.line_number(program_counter);
            if line_number > 0 {
                if source.is_empty() {
                    source = format!("{line_number}");
                } else {
                    source = format!("{source}:{line_number}");
                }
            }
            if source.is_empty() {
                eprintln!("    at {class_name}.{method_name}");
            } else {
                eprintln!("    at {class_name}.{method_name}({source})");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConfigurationBuilder;
    use ristretto_classloader::ClassPath;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_interrupt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        assert!(!thread.is_interrupted(false));
        thread.interrupt();
        assert!(thread.is_interrupted(false));

        // Clear the interrupt flag
        assert!(thread.is_interrupted(true));
        assert!(!thread.is_interrupted(false));
        Ok(())
    }

    #[tokio::test]
    async fn test_park() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let start_time = std::time::Instant::now();
        thread.park(false, 100_000_000).await?;
        let elapsed_time = start_time.elapsed();
        assert!(elapsed_time >= Duration::from_nanos(100_000_000));
        Ok(())
    }

    #[tokio::test]
    async fn test_park_interrupted() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        thread.interrupt();
        let start_time = std::time::Instant::now();
        thread.park(false, 100_000_000).await?;
        let elapsed_time = start_time.elapsed();
        // Thread should return immediately when interrupted
        assert!(elapsed_time < Duration::from_nanos(1_000_000));
        Ok(())
    }

    #[tokio::test]
    async fn test_unpark() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        thread.unpark();
        Ok(())
    }

    fn classes_jar_path() -> PathBuf {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_manifest
            .join("../")
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
    async fn test_hello_world_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_primitive_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("int").await?;
        assert_eq!("int", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_inheritance() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let hash_map = thread.class("java/util/HashMap").await?;
        assert_eq!("java/util/HashMap", hash_map.name());

        let abstract_map = hash_map.parent()?.expect("HashMap parent");
        assert_eq!("java/util/AbstractMap", abstract_map.name());

        let object = abstract_map.parent()?.expect("AbstractMap parent");
        assert_eq!("java/lang/Object", object.name());
        assert!(object.parent()?.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_new_object_integer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let object = thread.object("java/lang/Integer", "I", &[42]).await?;
        let value = object.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_print_stack_trace() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        thread.print_stack_trace().await;
        Ok(())
    }
}

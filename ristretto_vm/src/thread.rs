use crate::Error::{InternalError, UnsupportedClassFileVersion};
use crate::JavaError::{RuntimeException, StackOverflowError, VerifyError};
use crate::Parameters;
use crate::RustValue;
use crate::configuration::VerifyMode;
use crate::rust_value::process_values;
use crate::{Frame, Result, VM, jit};
use byte_unit::{Byte, UnitType};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{FieldAccessFlags, FieldType};
use ristretto_classloader::Error::MethodNotFound;
use ristretto_classloader::{Class, Method, Object, Reference, Value};
use ristretto_macros::async_method;
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
/// # References
/// - [JVMS §2.5.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5.2)
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
    #[must_use]
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

    /// Sleep the thread for the specified duration.  The sleep is interruptible - if another
    /// thread calls `interrupt()`, this method will return `true` to indicate the thread was
    /// interrupted, clearing the interrupt flag.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration to sleep.
    ///
    /// # Returns
    ///
    /// Returns `true` if the sleep was interrupted, `false` if it completed normally.
    pub async fn sleep(&self, duration: Duration) -> bool {
        // Check if already interrupted - return immediately if so
        if self.is_interrupted(true) {
            return true;
        }

        if duration.is_zero() {
            return false;
        }

        // Register for notification before sleeping
        let notified = self.park_state.notify.notified();

        tokio::select! {
            biased;  // Prefer checking sleep completion first

            () = tokio::time::sleep(duration) => {
                // Sleep completed normally
                false
            }
            () = notified => {
                // We were notified - check if it was an interrupt
                self.is_interrupted(true)
            }
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

    /// Get a class and ensure it is initialized.
    ///
    /// This implements the class initialization procedure as specified in
    /// [JLS §12.4.2](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.2):
    ///
    /// 1. If the class is already initialized, return immediately
    /// 2. If the class is in an erroneous state, throw `NoClassDefFoundError`
    /// 3. If the class is being initialized by the current thread, return (recursive initialization)
    /// 4. If the class is being initialized by another thread, wait and recheck
    /// 5. Mark the class as being initialized by the current thread
    /// 6. Initialize the direct superclass first (recursive)
    /// 7. Execute `<clinit>` for this class
    /// 8. If `<clinit>` throws, mark as Erroneous and throw `ExceptionInInitializerError`
    /// 9. Mark the class as Initialized
    ///
    /// Note: This implementation does NOT initialize interfaces as part of class initialization
    /// unless explicitly triggered per
    /// [JLS §12.4.1](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.1).
    ///
    /// # References
    ///
    /// - [JVMS §5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
    /// - [JLS §12.4.2](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.2)
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded or initialized
    #[expect(clippy::multiple_bound_locations)]
    #[async_method]
    pub async fn class<S: AsRef<str> + Send>(&self, class_name: S) -> Result<Arc<Class>> {
        let class_name = class_name.as_ref();

        // Load the class; the class tracks its own initialization state
        let class = self.load_and_link_class(class_name).await?;

        // Perform lazy, recursive initialization
        self.initialize_class(&class).await?;

        Ok(class)
    }

    /// Load and link a class without initializing it.
    ///
    /// This loads the class and resolves its superclass and interfaces (linking), but does not
    /// trigger initialization.
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded or linked
    #[async_method]
    pub(crate) async fn load_and_link_class(&self, class_name: &str) -> Result<Arc<Class>> {
        let vm = self.vm()?;
        let class = {
            let class_loader_lock = vm.class_loader();
            let class_loader = class_loader_lock.read().await;
            class_loader.load(class_name).await?
        };

        // Check class version compatibility
        if class.class_file().version > *vm.java_class_file_version() {
            return Err(UnsupportedClassFileVersion(
                class.class_file().version.major(),
            ));
        }

        // Verify class file according to the configured verify mode
        let verify_mode = vm.configuration().verify_mode();
        let should_verify = match verify_mode {
            VerifyMode::All => true,
            VerifyMode::Remote => {
                // Check if the class is from a trusted source (bootstrap class loader)
                // Classes from bootstrap loader are considered trusted
                let is_trusted = class
                    .class_loader()
                    .ok()
                    .flatten()
                    .is_some_and(|class_loader| class_loader.name() == "bootstrap");
                !is_trusted
            }
            VerifyMode::None => false,
        };

        if should_verify && let Err(error) = class.class_file().verify() {
            return Err(VerifyError(format!(
                "Verification failed for class {class_name}: {error}"
            ))
            .into());
        }

        // Link: resolve interfaces and recursively link them
        // Only link if:
        // 1. There are interfaces declared in the class file
        // 2. The interfaces haven't been linked yet (the interfaces vector is empty)
        let has_declared_interfaces = !class.class_file().interfaces.is_empty();
        let interfaces_not_linked = class.interfaces()?.is_empty();

        if has_declared_interfaces && interfaces_not_linked {
            let interface_names: Vec<String> = {
                let mut names = Vec::new();
                for interface_index in &class.class_file().interfaces {
                    let interface_name = class.constant_pool().try_get_class(*interface_index)?;
                    names.push(interface_name.to_string());
                }
                names
            };

            let mut interfaces = Vec::new();
            for interface_name in interface_names {
                // Recursively link each interface (this ensures interface inheritance is linked)
                let interface_class = self.load_and_link_class(&interface_name).await?;
                interfaces.push(interface_class);
            }
            class.set_interfaces(interfaces)?;
        }

        // Link: resolve superclass and recursively link the entire superclass chain
        // This ensures that all parent classes have their own parents resolved
        if class.parent()?.is_none() && class.name() != "java/lang/Object" {
            let super_class_name = {
                let super_class_index = class.class_file().super_class;
                if super_class_index == 0 {
                    "java/lang/Object".to_string()
                } else {
                    class
                        .constant_pool()
                        .try_get_class(super_class_index)?
                        .to_string()
                }
            };

            // Recursively link the superclass (this ensures the entire chain is linked)
            let super_class = self.load_and_link_class(&super_class_name).await?;
            class.set_parent(Some(super_class))?;
        }

        Ok(class)
    }

    /// Initialize a class following
    /// [JLS §12.4.2](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.2)
    /// state machine.
    ///
    /// # Static Field Initialization (JLS §12.4, JVMS §5.5)
    ///
    /// This implements the class initialization procedure where static fields are initialized:
    ///
    /// ## Initialization Order
    ///
    /// 1. **Superclass first**: The direct superclass is initialized before this class
    /// 2. **`<clinit>` execution**: Static field initializers and static blocks execute in textual order
    ///
    /// ## Compile-Time Constants (JLS §15.28)
    ///
    /// Fields with `ConstantValue` attribute (e.g., `static final int X = 42`) are initialized
    /// during the **preparation phase** (class loading), NOT here. Accessing such constants
    /// does NOT trigger class initialization.
    ///
    /// ## Key Behaviors
    ///
    /// - Uses lazy, recursive initialization
    /// - Handles circularity detection (same thread re-enters = OK, different thread = wait)
    /// - Initializes superclass before the class itself
    /// - Does NOT eagerly initialize interfaces per [JLS §12.4.1](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.4.1)
    /// - Caches initialization errors permanently
    ///
    /// ## Failure Semantics
    ///
    /// If `<clinit>` throws an exception:
    /// - Static fields may be **partially initialized** (no rollback occurs)
    /// - Class is marked as **Erroneous** (Failed state)
    /// - All future accesses throw `NoClassDefFoundError`
    ///
    /// ## Instance Fields NOT Affected
    ///
    /// Instance fields are NOT initialized here. They are:
    /// - Zeroed during object allocation (`Object::new`)
    /// - Initialized by constructor (`<init>`) during object construction
    ///
    /// # Errors
    ///
    /// if the class initialization fails
    #[async_method]
    async fn initialize_class(&self, class: &Arc<Class>) -> Result<()> {
        use crate::JavaError::{ExceptionInInitializerError, NoClassDefFoundError};
        use ristretto_classloader::InitializationAction;

        loop {
            let action = class.begin_initialization(self.id)?;

            match action {
                // Step 1 & 3: Already initialized or being initialized by current thread
                // Per JLS §12.4.2, circularity by same thread is allowed - return immediately
                InitializationAction::AlreadyInitialized
                | InitializationAction::AlreadyInitializing => {
                    return Ok(());
                }
                InitializationAction::Failed { error } => {
                    // Step 2: Previously failed, throw NoClassDefFoundError
                    return Err(NoClassDefFoundError(error).into());
                }
                InitializationAction::WaitForInitialization => {
                    // Step 4: Another thread is initializing, wait and recheck
                    // Use a timeout to handle race conditions where the notification was sent
                    // before we started waiting.
                    let _ = tokio::time::timeout(
                        Duration::from_millis(10),
                        class.wait_for_initialization(),
                    )
                    .await;
                    // Loop will continue to recheck the state
                }
                InitializationAction::ShouldInitialize => {
                    // Step 5: We are now the initializing thread
                    // Step 6: Initialize superclass first (recursive descent)
                    if let Some(superclass) = class.parent()?
                        && let Err(error) = self.initialize_class(&superclass).await
                    {
                        // Superclass initialization failed
                        let error_msg = format!("{error}");
                        class.fail_initialization(error_msg)?;
                        return Err(error);
                    }

                    // Step 6.5: Initialize String constants from ConstantValue attributes
                    // This happens during the preparation phase before <clinit> runs
                    if let Err(error) = self.initialize_string_constants(class).await {
                        let error_msg = format!("{error}");
                        class.fail_initialization(error_msg)?;
                        return Err(error);
                    }

                    // Step 7: Execute <clinit> for this class
                    if let Some(class_initializer) = class.class_initializer() {
                        match self
                            .execute(class, &class_initializer, &[] as &[Value])
                            .await
                        {
                            Ok(_) => {
                                // Step 9: Mark as initialized
                                class.complete_initialization()?;
                            }
                            Err(error) => {
                                // Step 8: <clinit> threw, mark as Erroneous
                                let error_msg = format!("{error}");
                                class.fail_initialization(error_msg.clone())?;
                                // Wrap in ExceptionInInitializerError (only first time)
                                return Err(ExceptionInInitializerError(error_msg).into());
                            }
                        }
                    } else {
                        // No <clinit>, just mark as initialized
                        class.complete_initialization()?;
                    }

                    return Ok(());
                }
            }
        }
    }

    /// Initialize String constants that have a `ConstantValue` attribute.
    ///
    /// Per JVM specification, static final fields with `ConstantValue` attributes should be
    /// initialized during the preparation phase, before `<clinit>` runs. For String constants,
    /// this means creating Java String objects from the constant pool values.
    ///
    /// # Errors
    ///
    /// if the String object cannot be created
    #[async_method]
    async fn initialize_string_constants(&self, class: &Arc<Class>) -> Result<()> {
        let constant_pool = class.constant_pool();

        for field in class.static_fields() {
            // Only process static final fields
            if !field
                .access_flags()
                .contains(FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
            {
                continue;
            }

            // Only process String fields
            let FieldType::Object(class_name) = field.field_type() else {
                continue;
            };
            if class_name != "java/lang/String" {
                continue;
            }

            // Check if the field has a ConstantValue attribute
            let constant_value_index = field.attributes().iter().find_map(|attr| {
                if let Attribute::ConstantValue {
                    constant_value_index,
                    ..
                } = attr
                {
                    Some(*constant_value_index)
                } else {
                    None
                }
            });

            let Some(constant_value_index) = constant_value_index else {
                continue;
            };

            // Get the string value from the constant pool
            let Ok(string_value) = constant_pool.try_get_string(constant_value_index) else {
                continue;
            };

            // Create a Java String object using the string pool for interning
            let vm = self.vm()?;
            let string_object = vm.string_pool().intern(self, string_value).await?;

            // Set the static field value
            class.set_static_value_unchecked(field.name(), string_object)?;
        }

        Ok(())
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
        C: AsRef<str> + Send + Sync,
        M: AsRef<str> + Send + Sync,
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
        C: AsRef<str> + Send + Sync,
        M: AsRef<str> + Send + Sync,
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
            if let Some(compiler) = vm.compiler() {
                compiler.compile(class, method).await?
            } else {
                None
            }
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
            // Check for native stack overflow before creating a new frame
            if let Some(remaining) = stacker::remaining_stack()
                && remaining < 512 * 1024
            {
                return Err(StackOverflowError(format!(
                    "{class_name}.{method_name}{method_descriptor}"
                ))
                .into());
            }
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
        C: AsRef<str> + Send + Sync,
        M: AsRef<str> + Send + Sync,
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
        let object = Value::new_object(
            self.vm()?.garbage_collector(),
            Reference::Object(Object::new(class.clone())?),
        );
        constructor_parameters.insert(0, object.clone());
        for parameter in parameters {
            let value = parameter.to_value(self.vm()?.garbage_collector());
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

impl ristretto_types::Thread for Thread {
    type Vm = VM;
    type Frame = crate::Frame;

    fn id(&self) -> u64 {
        self.id
    }

    fn vm(&self) -> Result<Arc<VM>> {
        Thread::vm(self)
    }

    fn name(&self) -> ristretto_types::BoxFuture<'_, String> {
        Box::pin(async move { Thread::name(self).await })
    }

    fn set_name<'a>(&'a self, name: &'a str) -> ristretto_types::BoxFuture<'a, ()> {
        Box::pin(async move { Thread::set_name(self, name).await })
    }

    fn java_object(&self) -> ristretto_types::BoxFuture<'_, Value> {
        Box::pin(async move { Thread::java_object(self).await })
    }

    fn set_java_object(&self, value: Value) -> ristretto_types::BoxFuture<'_, ()> {
        Box::pin(async move { Thread::set_java_object(self, value).await })
    }

    fn frames(&self) -> ristretto_types::BoxFuture<'_, Result<Vec<Arc<crate::Frame>>>> {
        Box::pin(async move { Thread::frames(self).await })
    }

    fn interrupt(&self) {
        Thread::interrupt(self);
    }

    fn is_interrupted(&self, clear_interrupt: bool) -> bool {
        Thread::is_interrupted(self, clear_interrupt)
    }

    fn sleep(&self, duration: Duration) -> ristretto_types::BoxFuture<'_, bool> {
        Box::pin(async move { Thread::sleep(self, duration).await })
    }

    fn park(&self, is_absolute: bool, time: u64) -> ristretto_types::BoxFuture<'_, Result<()>> {
        Box::pin(async move { Thread::park(self, is_absolute, time).await })
    }

    fn unpark(&self) {
        Thread::unpark(self);
    }

    fn class<'a>(
        &'a self,
        class_name: &'a str,
    ) -> ristretto_types::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { Thread::class(self, class_name).await })
    }

    fn load_and_link_class<'a>(
        &'a self,
        class_name: &'a str,
    ) -> ristretto_types::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { Thread::load_and_link_class(self, class_name).await })
    }

    fn register_class(&self, class: Arc<Class>) -> ristretto_types::BoxFuture<'_, Result<()>> {
        Box::pin(async move { Thread::register_class(self, class).await })
    }

    fn invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { Thread::invoke(self, class, method, parameters).await })
    }

    fn try_invoke<'a>(
        &'a self,
        class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { Thread::try_invoke(self, class, method, parameters).await })
    }

    fn execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { Thread::execute(self, class, method, parameters).await })
    }

    fn try_execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { Thread::try_execute(self, class, method, parameters).await })
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        descriptor: &'a str,
        parameters: &'a [Value],
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { Thread::object(self, class_name, descriptor, parameters).await })
    }

    fn intern_string<'a>(
        &'a self,
        string: &'a str,
    ) -> ristretto_types::BoxFuture<'a, Result<Value>> {
        Box::pin(async move {
            let vm = Thread::vm(self)?;
            vm.string_pool().intern(self, string).await
        })
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

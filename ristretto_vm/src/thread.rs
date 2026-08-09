use crate::Error::{InternalError, UnsupportedClassFileVersion};
use crate::JavaError::{RuntimeException, StackOverflowError, UnsatisfiedLinkError, VerifyError};
use crate::Parameters;
use crate::RustValue;
use crate::configuration::{DEFAULT_MAX_JAVA_STACK_SIZE, JAVA_STACK_SLOT_SIZE, VerifyMode};
use crate::frame::{ExecutionResult, MethodCall};
use crate::java_object::JavaObject;
use crate::rust_value::process_values;
use crate::{Frame, Result, VM, jit};

use parking_lot::RwLock as ParkingRwLock;
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{FieldAccessFlags, FieldType, JavaStr, MethodAccessFlags};
use ristretto_classloader::{Class, ClassLoaderType, Method, Object, Reference, Value};
use ristretto_intrinsics::get_monitor_id;
use ristretto_macros::async_method;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;
use tokio::sync::{Notify, RwLock};
use tokio::time::{Instant, timeout_at};
use tracing::{Level, debug, event_enabled};

#[cfg(all(not(target_family = "wasm"), not(target_os = "solaris")))]
// Leave room for another interpreter transition without rejecting legitimate bootstrap work on
// Tokio's smaller worker-thread stacks.
const NATIVE_STACK_RED_ZONE: usize = 128 * 1024;

const STACK_OVERFLOW_RESERVE_SLOTS: usize = 1_024;

/// Number of synchronous instructions to execute before yielding to the Tokio runtime.
const INSTRUCTION_YIELD_COUNT: u32 = 4096;

#[derive(Debug)]
struct MonitorGuard {
    monitor: Option<Arc<ristretto_types::monitor::Monitor>>,
    thread_id: u64,
}

impl MonitorGuard {
    fn new(monitor: Option<Arc<ristretto_types::monitor::Monitor>>, thread_id: u64) -> Self {
        Self { monitor, thread_id }
    }
}

impl Drop for MonitorGuard {
    fn drop(&mut self) {
        if let Some(monitor) = self.monitor.take() {
            let _ = monitor.release(self.thread_id);
        }
    }
}

#[derive(Debug)]
struct StackEntry {
    frame: Arc<Frame>,
    slots: usize,
    has_return_type: bool,
    _monitor: MonitorGuard,
}

#[derive(Debug)]
struct JavaStack {
    entries: Vec<StackEntry>,
    used_slots: usize,
    max_slots: usize,
    overflow_reserve_depth: usize,
}

impl JavaStack {
    fn new(max_slots: usize) -> Self {
        let entry_capacity = max_slots.saturating_add(STACK_OVERFLOW_RESERVE_SLOTS);
        Self {
            entries: Vec::with_capacity(entry_capacity),
            used_slots: 0,
            max_slots,
            overflow_reserve_depth: 0,
        }
    }

    fn frames(&self) -> Vec<Arc<Frame>> {
        self.entries
            .iter()
            .map(|entry| entry.frame.clone())
            .collect()
    }

    fn push(
        &mut self,
        frame: Arc<Frame>,
        has_return_type: bool,
        monitor: MonitorGuard,
    ) -> Result<()> {
        let slots = frame.stack_slots()?;
        let used_slots = self.checked_used_slots(slots, frame.class(), frame.method())?;
        self.used_slots = used_slots;
        self.entries.push(StackEntry {
            frame,
            slots,
            has_return_type,
            _monitor: monitor,
        });
        Ok(())
    }

    fn check_capacity(&self, slots: usize, class: &Class, method: &Method) -> Result<()> {
        self.checked_used_slots(slots, class, method).map(drop)
    }

    fn checked_used_slots(&self, slots: usize, class: &Class, method: &Method) -> Result<usize> {
        let reserve = if self.overflow_reserve_depth == 0 {
            0
        } else {
            STACK_OVERFLOW_RESERVE_SLOTS
        };
        let limit = self
            .max_slots
            .checked_add(reserve)
            .ok_or_else(|| InternalError("Java stack limit overflow".to_string()))?;
        let used_slots = self
            .used_slots
            .checked_add(slots)
            .ok_or_else(|| InternalError("Java stack slot count overflow".to_string()))?;
        if used_slots > limit {
            if self.overflow_reserve_depth == 0 {
                return Err(StackOverflowError(format!(
                    "{}.{}{}",
                    class.name(),
                    method.name(),
                    method.descriptor()
                ))
                .into());
            }
            return Err(InternalError(
                "StackOverflowError emergency reserve exhausted".to_string(),
            ));
        }
        Ok(used_slots)
    }

    fn push_jit(&mut self, frame: Arc<Frame>, has_return_type: bool, monitor: MonitorGuard) {
        self.entries.push(StackEntry {
            frame,
            slots: 0,
            has_return_type,
            _monitor: monitor,
        });
    }

    fn pop(&mut self) -> Option<StackEntry> {
        let entry = self.entries.pop()?;
        self.used_slots = self.used_slots.saturating_sub(entry.slots);
        Some(entry)
    }

    fn truncate(&mut self, depth: usize) {
        while self.entries.len() > depth {
            let _ = self.pop();
        }
    }
}

enum DispatchResult {
    FramePushed,
    Completed(Option<Value>),
}

struct ExecutionBoundary<'a> {
    thread: &'a Thread,
    depth: usize,
}

impl Drop for ExecutionBoundary<'_> {
    fn drop(&mut self) {
        self.thread.stack.write().truncate(self.depth);
    }
}

pub(crate) struct StackOverflowReserve<'a> {
    thread: &'a Thread,
}

impl Drop for StackOverflowReserve<'_> {
    fn drop(&mut self) {
        let mut stack = self.thread.stack.write();
        stack.overflow_reserve_depth = stack.overflow_reserve_depth.saturating_sub(1);
    }
}

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
    stack: ParkingRwLock<JavaStack>,
    instruction_yield_count: AtomicU32,
    /// Tracks class names currently being loaded via a Java classloader on this
    /// thread, preventing infinite recursion when `loadClass()` internally
    /// triggers further class resolution.
    /// Uses `std::sync::Mutex` (not tokio) so it can be accessed from `Drop`.
    java_cl_loading: Mutex<HashSet<String>>,
    park_state: ParkState,
}

impl Thread {
    /// Create a new thread.
    #[must_use]
    pub fn new(vm: &Weak<VM>, id: u64) -> Arc<Self> {
        let vm_ref = vm.clone();
        let max_stack_size = vm.upgrade().map_or(DEFAULT_MAX_JAVA_STACK_SIZE, |vm| {
            vm.configuration().max_java_stack_size()
        });
        let max_stack_slots = max_stack_size / JAVA_STACK_SLOT_SIZE;
        let name = format!("Thread-{id}");
        let java_object = Value::Object(None);
        Arc::new_cyclic(|thread| Thread {
            id,
            vm: vm_ref,
            thread: thread.clone(),
            name: Arc::new(RwLock::new(name)),
            java_object: Arc::new(RwLock::new(java_object)),
            stack: ParkingRwLock::new(JavaStack::new(max_stack_slots)),
            instruction_yield_count: AtomicU32::new(0),
            java_cl_loading: Mutex::new(HashSet::new()),
            park_state: ParkState::new(),
        })
    }

    /// Get the identifier of the thread.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Record a synchronous instruction and return whether the executor should yield.
    pub(crate) fn record_synchronous_instruction(&self) -> bool {
        let count = self
            .instruction_yield_count
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_add(1);
        count.is_multiple_of(INSTRUCTION_YIELD_COUNT)
    }

    /// Reset the yield counter when an instruction already yields through asynchronous work.
    pub(crate) fn reset_instruction_yield_count(&self) {
        self.instruction_yield_count.store(0, Ordering::Relaxed);
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
    pub fn frames(&self) -> Result<Vec<Arc<Frame>>> {
        Ok(self.stack.read().frames())
    }

    /// Get the current frame in the thread.
    ///
    /// # Errors
    ///
    /// if the current frame cannot be accessed.
    pub fn current_frame(&self) -> Result<Arc<Frame>> {
        let stack = self.stack.read();
        let entry = stack
            .entries
            .last()
            .ok_or(InternalError("No frame".to_string()))?;
        Ok(entry.frame.clone())
    }

    pub(crate) fn stack_overflow_reserve(&self) -> StackOverflowReserve<'_> {
        self.stack.write().overflow_reserve_depth += 1;
        StackOverflowReserve { thread: self }
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

    /// Sleep the thread for the specified duration.  The sleep is interruptible; if another
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
        // Check if already interrupted; return immediately if so
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
                // We were notified; check if it was an interrupt
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
            // Infinite park: wait until unparked
            loop {
                self.park_state.notify.notified().await;
                if self.park_state.permit.swap(false, Ordering::Acquire) {
                    break;
                }
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
        let java_str = JavaStr::cow_from_str(class_name);
        self.class_java_str(&java_str).await
    }

    /// Load, link, and initialize a class from a `JavaStr` reference.
    ///
    /// This is the primary class loading entry point. It accepts `&JavaStr` directly
    /// (e.g., from the constant pool) and pushes it through the entire loading chain,
    /// avoiding unnecessary string conversions.
    ///
    /// # Errors
    ///
    /// if the class cannot be loaded or initialized
    #[async_method]
    pub async fn class_java_str(&self, class_name: &JavaStr) -> Result<Arc<Class>> {
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
    pub(crate) async fn load_and_link_class(&self, class_name: &JavaStr) -> Result<Arc<Class>> {
        let vm = self.vm()?;
        let class = {
            let class_loader_lock = vm.class_loader();
            let class_loader = class_loader_lock.read().await;
            match class_loader.load(class_name).await {
                Ok(class) => class,
                Err(ristretto_classloader::Error::ClassNotFound(_)) => {
                    // Per JVM spec §5.3, when a class D references class C, the JVM must
                    // use D's defining classloader to load C.  If D was loaded by a
                    // user-defined (Java) classloader, the JVM invokes loadClass() on it.
                    drop(class_loader);
                    match self.load_class_via_java_classloader(&vm, class_name).await {
                        Ok(c) => c,
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e.into()),
            }
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
                    .is_some_and(|class_loader| {
                        class_loader.loader_type() == Some(ClassLoaderType::Bootstrap)
                    });
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
            // Clone the interface indices to avoid holding a borrow across await points
            let interface_indices: Vec<u16> = class.class_file().interfaces.clone();
            let mut interfaces = Vec::with_capacity(interface_indices.len());
            for interface_index in interface_indices {
                let interface_name = class.constant_pool().try_get_class(interface_index)?;
                // Pass &JavaStr directly from constant pool; no String allocation
                let interface_class = self.load_and_link_class(interface_name).await?;
                interfaces.push(interface_class);
            }
            class.set_interfaces(interfaces)?;
        }

        // Link: resolve superclass and recursively link the entire superclass chain
        // This ensures that all parent classes have their own parents resolved
        if class.parent()?.is_none() && class.name() != "java/lang/Object" {
            let super_class_index = class.class_file().super_class;
            if super_class_index == 0 {
                // Default to java/lang/Object; zero-copy via try_from_str on static ASCII
                let object_name = JavaStr::try_from_str("java/lang/Object")?;
                let super_class = self.load_and_link_class(object_name).await?;
                class.set_parent(Some(super_class))?;
            } else {
                let super_class_name = class.constant_pool().try_get_class(super_class_index)?;
                // Pass &JavaStr directly from constant pool; no String allocation
                let super_class = self.load_and_link_class(super_class_name).await?;
                class.set_parent(Some(super_class))?;
            }
        }

        Ok(class)
    }

    /// Attempt to load a class by invoking `ClassLoader.loadClass()` on the Java classloader
    /// associated with a class on the current call stack.
    ///
    /// Per [JVMS §5.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.3),
    /// when class D references class C, the JVM uses D's defining classloader to load C.  If D
    /// was loaded by a user-defined classloader, the JVM must invoke that classloader's
    /// `loadClass()` method so the classloader can locate, define, and register the class.
    ///
    /// This method walks the call stack from newest to oldest frame looking for a class whose
    /// Java mirror object carries a non-null `classLoader` field.  When found it invokes
    /// `ClassLoader.loadClass(className)` on that object.  The Java `loadClass` implementation
    /// ultimately calls the native `defineClass()`, which registers the class with the VM's
    /// Rust classloader so that subsequent lookups succeed.
    async fn load_class_via_java_classloader(
        &self,
        vm: &Arc<VM>,
        class_name: &JavaStr,
    ) -> Result<Arc<Class>> {
        // RAII guard that removes the class name from the loading set on drop,
        // ensuring cleanup even if a panic occurs during loadClass invocation.
        struct LoadGuard<'a> {
            loading: &'a Mutex<HashSet<String>>,
            name: Option<String>,
        }
        impl Drop for LoadGuard<'_> {
            fn drop(&mut self) {
                if let Some(name) = self.name.take()
                    && let Ok(mut set) = self.loading.lock()
                {
                    set.remove(&name);
                }
            }
        }

        let class_name_str = class_name.to_rust_string();

        // Re-entrance guard: if this class is already being loaded via a Java
        // classloader on this thread, bail out to prevent infinite recursion
        // (e.g. loadClass -> findLoadedClass -> load_and_link -> loadClass …).
        {
            let mut loading = self
                .java_cl_loading
                .lock()
                .map_err(|e| InternalError(e.to_string()))?;
            if !loading.insert(class_name_str.clone()) {
                return Err(ristretto_classloader::Error::ClassNotFound(class_name_str).into());
            }
        }

        let mut load_guard = LoadGuard {
            loading: &self.java_cl_loading,
            name: Some(class_name_str.clone()),
        };

        // Walk the call stack to find a Java classloader from the referencing class.
        // We must drop the frames lock before invoking any Java methods to avoid
        // deadlocks (execute() modifies frames).
        let java_classloader = {
            let stack = self.stack.read();
            let mut found = None;
            for entry in stack.entries.iter().rev() {
                let frame = &entry.frame;
                let class = frame.class();
                if let Ok(Some(mirror)) = class.object()
                    && let Ok(obj) = mirror.as_object_ref()
                    && let Ok(cl) = obj.value("classLoader")
                    && !cl.is_null()
                {
                    found = Some(cl);
                    break;
                }
            }
            found
        };

        let Some(java_classloader) = java_classloader else {
            return Err(ristretto_classloader::Error::ClassNotFound(class_name_str).into());
        };

        // Convert from internal format (slashes) to Java format (dots) for loadClass().
        let dot_name = class_name_str.replace('/', ".");

        let load_result = async {
            let cl_class = self.class("java/lang/ClassLoader").await?;
            let load_class_method =
                cl_class.try_get_method("loadClass", "(Ljava/lang/String;)Ljava/lang/Class;")?;
            let name_value: Value = dot_name.to_object(self).await?;
            self.execute(
                &cl_class,
                &load_class_method,
                &[java_classloader, name_value],
            )
            .await
        }
        .await;

        // Explicitly remove the guard before proceeding; disarm the drop.
        load_guard.name.take();
        if let Ok(mut set) = self.java_cl_loading.lock() {
            set.remove(&class_name_str);
        }

        let result = load_result?;
        if let Some(ref value) = result
            && !value.is_null()
        {
            // loadClass succeeded.  The class should now be registered in the Rust
            // classloader via the defineClass native call that loadClass ultimately made.
            let class_loader_lock = vm.class_loader();
            let class_loader = class_loader_lock.read().await;
            let loaded_class: Arc<Class> = class_loader.load(class_name).await?;
            return Ok(loaded_class);
        }

        Err(ristretto_classloader::Error::ClassNotFound(class_name_str).into())
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
                // Per JLS §12.4.2, circularity by same thread is allowed; return immediately
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
                                let error_msg = format!("{error:#}");
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
            let string_object = vm.string_pool().intern_java_str(self, string_value).await?;

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

    /// Acquire the monitor for an `ACC_SYNCHRONIZED` method. For instance methods, locks on `this`
    /// (first parameter). For static methods, locks on the class object.
    async fn acquire_sync_monitor(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        parameters: &[Value],
    ) -> Result<Option<Arc<ristretto_types::monitor::Monitor>>> {
        if !method
            .access_flags()
            .contains(MethodAccessFlags::SYNCHRONIZED)
        {
            return Ok(None);
        }
        let monitor_id = if method.is_static() {
            if let Ok(Some(Value::Object(Some(ref reference)))) = class.object() {
                get_monitor_id(&reference.read())
            } else {
                None
            }
        } else if let Some(Value::Object(Some(reference))) = parameters.first() {
            get_monitor_id(&reference.read())
        } else {
            None
        };
        if let Some(id) = monitor_id {
            let vm = self.vm()?;
            let monitor = vm.monitor_registry().monitor(id);
            monitor.acquire(self.id).await?;
            Ok(Some(monitor))
        } else {
            Ok(None)
        }
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
        let parameters = process_values(self, parameters).await?;
        let base_depth = self.stack.read().entries.len();
        let _boundary = ExecutionBoundary {
            thread: self,
            depth: base_depth,
        };
        let call = MethodCall {
            class: class.clone(),
            method: method.clone(),
            parameters,
            has_return_type: false,
        };

        match self.dispatch_method(call).await? {
            DispatchResult::FramePushed => self.run_interpreter(base_depth).await,
            DispatchResult::Completed(value) => Ok(value),
        }
    }

    async fn dispatch_method(&self, call: MethodCall) -> Result<DispatchResult> {
        let MethodCall {
            class,
            method,
            parameters,
            has_return_type,
        } = call;
        let class_name = class.name();
        let method_name = method.name();
        let method_descriptor = method.descriptor();
        let vm = self.vm()?;

        let sync_monitor = self
            .acquire_sync_monitor(&class, &method, &parameters)
            .await?;
        let monitor_guard = MonitorGuard::new(sync_monitor, self.id);

        let method_registry = vm.method_registry();
        let rust_method = method_registry.method(class_name, method_name, method_descriptor);
        let jit_method = if rust_method.is_none() {
            if let Some(compiler) = vm.compiler() {
                compiler.compile(&class, &method).await?
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
                &method,
                rust_method.is_some(),
                jit_method.is_some(),
            );
        }

        if let Some(rust_method) = rust_method {
            let Some(thread) = self.thread.upgrade() else {
                return Err(InternalError("Call stack is not available".to_string()));
            };
            let parameters = Parameters::new(parameters);
            let result = rust_method(thread, parameters).await;
            drop(monitor_guard);
            Self::debug_result(&class, &method, &result);
            return result.map(DispatchResult::Completed);
        } else if let Some(jit_method) = jit_method {
            let gc = vm.garbage_collector();
            let Some(thread) = self.thread.upgrade() else {
                return Err(InternalError("Call stack is not available".to_string()));
            };
            let frame = Arc::new(Frame::new(&self.thread, &class, &method));
            self.stack
                .write()
                .push_jit(frame, has_return_type, monitor_guard);
            let result = jit::execute(&jit_method, &parameters, gc, &vm, &thread, &class);
            let _ = self.stack.write().pop();
            Self::debug_result(&class, &method, &result);
            return result.map(DispatchResult::Completed);
        } else if method.is_native() {
            return Err(UnsatisfiedLinkError(format!(
                "'{class_name}.{method_name}{method_descriptor}'"
            ))
            .into());
        }

        let frame_slots = Frame::stack_slots_for(&method)?;
        self.stack
            .read()
            .check_capacity(frame_slots, &class, &method)?;
        let frame = Arc::new(Frame::with_parameters(
            &self.thread,
            &class,
            &method,
            parameters,
        )?);
        self.stack
            .write()
            .push(frame, has_return_type, monitor_guard)?;
        Ok(DispatchResult::FramePushed)
    }

    async fn run_interpreter(&self, base_depth: usize) -> Result<Option<Value>> {
        loop {
            let frame = {
                let stack = self.stack.read();
                let entry = stack.entries.last().ok_or_else(|| {
                    InternalError("Interpreter stack unexpectedly empty".to_string())
                })?;
                entry.frame.clone()
            };

            match frame.execute_instruction(self).await {
                Ok(ExecutionResult::Continue) => {}
                Ok(ExecutionResult::Call(call)) => {
                    let has_return_type = call.has_return_type;
                    match self.dispatch_method(call).await {
                        Ok(DispatchResult::FramePushed) => {}
                        Ok(DispatchResult::Completed(value)) => {
                            if let Err(error) = frame.complete_call(value, has_return_type).await {
                                self.propagate_error(base_depth, error, false).await?;
                            }
                        }
                        Err(error) => {
                            self.propagate_error(base_depth, error, false).await?;
                        }
                    }
                }
                Ok(ExecutionResult::Return(value)) => {
                    let entry = self.stack.write().pop().ok_or_else(|| {
                        InternalError("Interpreter stack unexpectedly empty".to_string())
                    })?;
                    let has_return_type = entry.has_return_type;
                    Self::debug_result(
                        entry.frame.class(),
                        entry.frame.method(),
                        &Ok(value.clone()),
                    );
                    drop(entry);

                    if self.stack.read().entries.len() == base_depth {
                        return Ok(value);
                    }

                    let caller = self.current_frame()?;
                    if let Err(error) = caller.complete_call(value, has_return_type).await {
                        self.propagate_error(base_depth, error, false).await?;
                    }
                }
                Ok(ExecutionResult::ContinueAtPosition(_)) => {
                    return Err(InternalError(
                        "Frame returned an unprocessed branch result".to_string(),
                    ));
                }
                Err(error) => {
                    self.propagate_error(base_depth, error, true).await?;
                }
            }
        }
    }

    async fn propagate_error(
        &self,
        base_depth: usize,
        mut error: crate::Error,
        skip_current: bool,
    ) -> Result<()> {
        if skip_current {
            let entry =
                self.stack.write().pop().ok_or_else(|| {
                    InternalError("Interpreter stack unexpectedly empty".to_string())
                })?;
            Self::debug_error(entry.frame.class(), entry.frame.method(), &error);
            drop(entry);
        }

        loop {
            if self.stack.read().entries.len() == base_depth {
                return Err(error);
            }

            let frame = self.current_frame()?;
            match frame.handle_error(error).await {
                Ok(()) => return Ok(()),
                Err(next_error) => {
                    let entry = self.stack.write().pop().ok_or_else(|| {
                        InternalError("Interpreter stack unexpectedly empty".to_string())
                    })?;
                    Self::debug_error(entry.frame.class(), entry.frame.method(), &next_error);
                    drop(entry);
                    error = next_error;
                }
            }
        }
    }

    fn debug_result(class: &Arc<Class>, method: &Arc<Method>, result: &Result<Option<Value>>) {
        if !event_enabled!(Level::DEBUG) {
            return;
        }
        let result_str = match result {
            Ok(Some(value)) => {
                let value = value.to_string();
                if value.len() > 100 {
                    format!("{}...", &value[..97])
                } else {
                    value
                }
            }
            Ok(None) => "void".to_string(),
            Err(error) => format!("[ERROR] {error}"),
        };
        debug!(
            "result: {}.{}{}: {result_str}",
            class.name(),
            method.name(),
            method.descriptor()
        );
    }

    fn debug_error(class: &Arc<Class>, method: &Arc<Method>, error: &crate::Error) {
        if event_enabled!(Level::DEBUG) {
            debug!(
                "result: {}.{}{}: [ERROR] {error}",
                class.name(),
                method.name(),
                method.descriptor()
            );
        }
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
        debug!(
            "execute({execution_type}): {class_name}.{method_name}{method_descriptor} {access_flags}"
        );
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

        let parameters = process_values(self, parameters).await?;
        let mut constructor_parameters = Vec::with_capacity(parameters.len() + 1);
        let object = Value::new_object(
            self.vm()?.garbage_collector(),
            Reference::Object(Object::new(class.clone())?),
        );
        constructor_parameters.push(object.clone());
        constructor_parameters.extend(parameters);
        Box::pin(self.execute(&class, &constructor, &constructor_parameters)).await?;
        Ok(object)
    }

    /// Print the stack trace. Used for debugging.
    pub(crate) async fn print_stack_trace(&self) {
        let name = self.name().await;
        eprintln!("Thread: {name}");
        let stack = self.stack.read();
        for entry in stack.entries.iter().rev() {
            let frame = &entry.frame;
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
    type Frame = Frame;

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

    fn frames(&self) -> ristretto_types::BoxFuture<'_, Result<Vec<Arc<Frame>>>> {
        Box::pin(std::future::ready(Thread::frames(self)))
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

    fn class_java_str<'a>(
        &'a self,
        class_name: &'a JavaStr,
    ) -> ristretto_types::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { Thread::class_java_str(self, class_name).await })
    }

    fn load_and_link_class<'a>(
        &'a self,
        class_name: &'a JavaStr,
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

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::ConfigurationBuilder;
    use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
    use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool};
    use ristretto_classloader::ClassPath;
    use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_instruction_yield_count_is_thread_based() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);

        for _ in 1..INSTRUCTION_YIELD_COUNT {
            assert!(!thread.record_synchronous_instruction());
        }
        assert!(thread.record_synchronous_instruction());

        assert!(!other_thread.record_synchronous_instruction());
        thread.reset_instruction_yield_count();
        assert!(!thread.record_synchronous_instruction());
        Ok(())
    }

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
        assert!(elapsed_time >= Duration::from_millis(100));
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
        assert!(elapsed_time < Duration::from_millis(1));
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

    async fn interpreted_test_thread() -> Result<(Arc<VM>, Arc<Thread>)> {
        let gc_configuration = GcConfigurationBuilder::new().threads(1).build();
        let garbage_collector = GarbageCollector::with_config(gc_configuration);
        let configuration = ConfigurationBuilder::new()
            .interpreted(true)
            .verify_mode(VerifyMode::None)
            .garbage_collector(garbage_collector)
            .build()?;
        let vm = VM::new(configuration).await?;
        let thread = Thread::new(&Arc::downgrade(&vm), 1);
        Ok((vm, thread))
    }

    #[expect(clippy::too_many_lines)]
    async fn recursive_test_class(thread: &Arc<Thread>) -> Result<Arc<Class>> {
        let mut constant_pool = ConstantPool::default();
        let super_class = constant_pool.add_class("java/lang/Object")?;
        let this_class = constant_pool.add_class("InterpreterRecursionTest")?;
        let stack_overflow_error = constant_pool.add_class("java/lang/StackOverflowError")?;
        let arithmetic_exception = constant_pool.add_class("java/lang/ArithmeticException")?;
        let code_index = constant_pool.add_utf8("Code")?;
        let recurse_name_index = constant_pool.add_utf8("recurse")?;
        let recurse_descriptor_index = constant_pool.add_utf8("(I)I")?;
        let recurse_ref = constant_pool.add_method_ref(this_class, "recurse", "(I)I")?;
        let catch_name_index = constant_pool.add_utf8("catchOverflow")?;
        let catch_descriptor_index = constant_pool.add_utf8("()I")?;
        let catch_arithmetic_name_index = constant_pool.add_utf8("catchArithmetic")?;
        let oversized_name_index = constant_pool.add_utf8("oversized")?;
        let void_descriptor_index = constant_pool.add_utf8("()V")?;

        let recurse = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: recurse_name_index,
            descriptor_index: recurse_descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_index,
                max_stack: 2,
                max_locals: 1,
                code: vec![
                    Instruction::Iload_0,
                    Instruction::Ifeq(9),
                    Instruction::Iload_0,
                    Instruction::Iconst_1,
                    Instruction::Isub,
                    Instruction::Invokestatic(recurse_ref),
                    Instruction::Iconst_1,
                    Instruction::Iadd,
                    Instruction::Ireturn,
                    Instruction::Iconst_0,
                    Instruction::Ireturn,
                ],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        };
        let catch_overflow = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: catch_name_index,
            descriptor_index: catch_descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_index,
                max_stack: 1,
                max_locals: 0,
                code: vec![
                    Instruction::Sipush(1_000),
                    Instruction::Invokestatic(recurse_ref),
                    Instruction::Ireturn,
                    Instruction::Pop,
                    Instruction::Iconst_m1,
                    Instruction::Ireturn,
                ],
                exception_table: vec![ExceptionTableEntry {
                    range_pc: 0..3,
                    handler_pc: 3,
                    catch_type: stack_overflow_error,
                }],
                attributes: Vec::new(),
            }],
        };
        let catch_arithmetic = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: catch_arithmetic_name_index,
            descriptor_index: catch_descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_index,
                max_stack: 2,
                max_locals: 0,
                code: vec![
                    Instruction::Iconst_1,
                    Instruction::Iconst_0,
                    Instruction::Idiv,
                    Instruction::Ireturn,
                    Instruction::Pop,
                    Instruction::Bipush(42),
                    Instruction::Ireturn,
                ],
                exception_table: vec![ExceptionTableEntry {
                    range_pc: 0..4,
                    handler_pc: 4,
                    catch_type: arithmetic_exception,
                }],
                attributes: Vec::new(),
            }],
        };
        let oversized = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: oversized_name_index,
            descriptor_index: void_descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_index,
                max_stack: 128,
                max_locals: 0,
                code: vec![Instruction::Return],
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        };

        let class = Class::from(
            None,
            ClassFile {
                access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
                constant_pool,
                this_class,
                super_class,
                methods: vec![recurse, catch_overflow, catch_arithmetic, oversized],
                ..Default::default()
            },
        )?;
        thread.register_class(class.clone()).await?;
        Ok(class)
    }

    #[tokio::test]
    async fn test_configured_java_stack_limit() -> Result<()> {
        let configuration = ConfigurationBuilder::new()
            .interpreted(true)
            .max_java_stack_size(16_384 * JAVA_STACK_SLOT_SIZE + 3)
            .build()?;
        let vm = VM::new(configuration).await?;
        let thread = Thread::new(&Arc::downgrade(&vm), 1);

        assert_eq!(16_384, thread.stack.read().max_slots);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_interpreted_recursion_uses_java_stack() -> Result<()> {
        let (_vm, thread) = interpreted_test_thread().await?;
        let class = recursive_test_class(&thread).await?;
        let method = class.try_get_method("recurse", "(I)I")?;
        let (stack_pointer, stack_capacity) = {
            let stack = thread.stack.read();
            (stack.entries.as_ptr(), stack.entries.capacity())
        };

        let result = thread
            .try_execute(&class, &method, &[Value::Int(4_000)])
            .await?;

        assert_eq!(Value::Int(4_000), result);
        assert!(thread.frames()?.is_empty());
        {
            let stack = thread.stack.read();
            assert_eq!(0, stack.used_slots);
            assert_eq!(stack_pointer, stack.entries.as_ptr());
            assert_eq!(stack_capacity, stack.entries.capacity());
        }

        let result = thread
            .try_execute(&class, &method, &[Value::Int(4_000)])
            .await?;

        assert_eq!(Value::Int(4_000), result);
        let stack = thread.stack.read();
        assert_eq!(0, stack.used_slots);
        assert_eq!(stack_pointer, stack.entries.as_ptr());
        assert_eq!(stack_capacity, stack.entries.capacity());
        Ok(())
    }

    #[tokio::test]
    async fn test_java_stack_overflow_is_catchable_and_cleans_up() -> Result<()> {
        let (_vm, thread) = interpreted_test_thread().await?;
        let class = recursive_test_class(&thread).await?;
        thread.stack.write().max_slots = 64;
        let catch_method = class.try_get_method("catchOverflow", "()I")?;

        let result = thread
            .try_execute(&class, &catch_method, &[] as &[Value])
            .await?;

        assert_eq!(Value::Int(-1), result);
        assert!(thread.frames()?.is_empty());
        assert_eq!(0, thread.stack.read().used_slots);

        thread.stack.write().max_slots = DEFAULT_MAX_JAVA_STACK_SIZE / JAVA_STACK_SLOT_SIZE;
        let recurse = class.try_get_method("recurse", "(I)I")?;
        let result = thread
            .try_execute(&class, &recurse, &[Value::Int(10)])
            .await?;
        assert_eq!(Value::Int(10), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_implicit_exception_is_catchable_at_java_stack_limit() -> Result<()> {
        let (_vm, thread) = interpreted_test_thread().await?;
        let class = recursive_test_class(&thread).await?;
        let method = class.try_get_method("catchArithmetic", "()I")?;
        thread.stack.write().max_slots = Frame::stack_slots_for(&method)?;

        let result = thread.try_execute(&class, &method, &[] as &[Value]).await?;

        assert_eq!(Value::Int(42), result);
        assert!(thread.frames()?.is_empty());
        let stack = thread.stack.read();
        assert_eq!(0, stack.used_slots);
        assert_eq!(0, stack.overflow_reserve_depth);
        Ok(())
    }

    #[tokio::test]
    async fn test_oversized_frame_is_rejected_with_stack_overflow() -> Result<()> {
        let (_vm, thread) = interpreted_test_thread().await?;
        let class = recursive_test_class(&thread).await?;
        let method = class.try_get_method("oversized", "()V")?;
        thread.stack.write().max_slots = 64;

        let result = thread.execute(&class, &method, &[] as &[Value]).await;

        assert!(matches!(
            result,
            Err(crate::Error::JavaError(StackOverflowError(_)))
        ));
        assert!(thread.frames()?.is_empty());
        assert_eq!(0, thread.stack.read().used_slots);
        Ok(())
    }

    #[tokio::test]
    async fn test_execution_boundary_releases_frame_monitor() -> Result<()> {
        let (_vm, thread, class) = crate::test::class().await?;
        let method = class.try_get_method("test", "()V")?;
        let frame = Arc::new(Frame::new(&Arc::downgrade(&thread), &class, &method));
        assert_eq!(1, frame.stack_slots()?);

        let monitor = Arc::new(ristretto_types::monitor::Monitor::new());
        monitor.acquire(thread.id()).await?;
        {
            let _boundary = ExecutionBoundary {
                thread: &thread,
                depth: 0,
            };
            thread.stack.write().push(
                frame,
                false,
                MonitorGuard::new(Some(monitor.clone()), thread.id()),
            )?;
        }

        assert!(thread.frames()?.is_empty());
        tokio::time::timeout(Duration::from_secs(1), monitor.acquire(thread.id() + 1))
            .await
            .map_err(|error| InternalError(error.to_string()))??;
        monitor.release(thread.id() + 1)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_hello_world_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("HelloWorld").await?;
        assert_eq!("HelloWorld", class.name());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_vm_drop_only() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        drop(thread);
        drop(vm);
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

    #[cfg(not(feature = "audio"))]
    #[tokio::test]
    async fn test_disabled_audio_returns_unsatisfied_link_error() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut constant_pool = ristretto_classfile::ConstantPool::default();
        let this_class =
            constant_pool.add_class("com/sun/media/sound/DirectAudioDeviceProvider")?;
        let name_index = constant_pool.add_utf8("nGetNumDevices")?;
        let descriptor_index = constant_pool.add_utf8("()I")?;
        let method = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC
                | MethodAccessFlags::STATIC
                | MethodAccessFlags::NATIVE,
            name_index,
            descriptor_index,
            ..Default::default()
        };
        let class_file = ristretto_classfile::ClassFile {
            constant_pool,
            this_class,
            methods: vec![method],
            ..Default::default()
        };
        let class = Class::from(None, class_file)?;
        let method = class.try_get_method("nGetNumDevices", "()I")?;

        let error = thread
            .execute(&class, &method, &[] as &[Value])
            .await
            .expect_err("disabled audio intrinsic should fail");
        assert!(matches!(
            error,
            crate::Error::JavaError(UnsatisfiedLinkError(message))
                if message == "'com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I'"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_print_stack_trace() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        thread.print_stack_trace().await;
        Ok(())
    }
}

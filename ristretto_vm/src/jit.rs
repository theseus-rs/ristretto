use crate::Error::InternalError;
use crate::jit_runtime_helpers::{self as runtime_helpers, RuntimeContext};
use crate::{Result, Thread, VM};
use dashmap::DashMap;
#[cfg(not(target_family = "wasm"))]
use rayon::ThreadPoolBuilder;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use ristretto_classloader::{Class, Method, Value};
use ristretto_gc::{GarbageCollector, Gc};
use ristretto_jit::Function;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use std::time::Duration;
#[cfg(not(target_family = "wasm"))]
use sysinfo::System;
use tokio::sync::{Mutex, mpsc};
#[cfg(not(target_family = "wasm"))]
use tokio::time::sleep;
#[cfg(not(target_family = "wasm"))]
use tracing::{debug, error, info};
#[cfg(target_family = "wasm")]
use tracing::{debug, info};

const BATCH_SIZE: usize = 5;
const BATCH_TIMEOUT_MS: u64 = 10;

/// Cache key derived from the `Arc<Method>` pointer, avoiding heap-allocated string keys.
type CacheKey = usize;

/// Derives a cache key from a method's Arc pointer.
fn cache_key(method: &Arc<Method>) -> CacheKey {
    Arc::as_ptr(method) as CacheKey
}

/// A compilation request for the background batch compiler
struct CompilationRequest {
    class: Arc<Class>,
    method: Arc<Method>,
}

/// A thread-safe per-VM JIT compiler that handles background compilation and caching.
///
/// # Overview
///
/// The `Compiler` struct manages JIT compilation for a single VM instance. It provides:
/// - A per-VM function cache for compiled native functions
/// - Background batch compilation for improved performance
/// - Per-VM isolation to prevent cross-contamination between parallel VM instances
///
/// # Per-VM Isolation
///
/// Each VM instance has its own `Compiler` with its own function cache and compilation queue.
/// This prevents issues where parallel VM instances loading classes with the same name but
/// different implementations could interfere with each other.
///
/// # Background Compilation
///
/// When batch compilation is enabled, methods are queued for background compilation. This allows
/// the VM to continue execution while compilation proceeds asynchronously. Compiled functions
/// are cached and returned on subsequent calls.
///
/// # Thread Safety
///
/// The compiler uses `DashMap` for the function cache and pending compilations tracking,
/// providing thread-safe concurrent access without locks in the common case.
pub struct Compiler {
    /// The underlying JIT compiler from `ristretto_jit`
    #[expect(clippy::struct_field_names)]
    jit_compiler: ristretto_jit::Compiler,
    /// Per-VM function cache for compiled functions (wrapped in Arc for sharing with background task)
    function_cache: Arc<DashMap<CacheKey, Option<Arc<Function>>>>,
    /// Per-VM compilation queue for background batch processing
    compilation_queue: Mutex<Option<mpsc::UnboundedSender<CompilationRequest>>>,
    /// Whether batch compilation is enabled
    batch_compilation_enabled: bool,
    /// Whether the compiler is in interpreted-only mode
    interpreted: bool,
}

impl std::fmt::Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compiler")
            .field("function_cache_size", &self.function_cache.len())
            .field("batch_compilation_enabled", &self.batch_compilation_enabled)
            .field("interpreted", &self.interpreted)
            .finish_non_exhaustive()
    }
}

impl Compiler {
    /// Creates a new JIT compiler for the VM.
    ///
    /// # Arguments
    ///
    /// * `batch_compilation` - Whether to enable background batch compilation
    /// * `interpreted` - Whether to run in interpreted-only mode (no JIT)
    ///
    /// # Returns
    ///
    /// Returns `Some(Compiler)` if the JIT compiler was successfully created,
    /// or `None` if the target ISA is not supported.
    #[must_use]
    pub fn new(batch_compilation: bool, interpreted: bool) -> Option<Self> {
        let jit_compiler = match ristretto_jit::Compiler::new() {
            Ok(compiler) => compiler,
            Err(error) => {
                debug!("JIT compiler not available: {error:?}");
                return None;
            }
        };

        Some(Self {
            jit_compiler,
            function_cache: Arc::new(DashMap::new()),
            compilation_queue: Mutex::new(None),
            batch_compilation_enabled: batch_compilation,
            interpreted,
        })
    }

    /// Gets a cached function by its cache key.
    /// Returns `None` if the key is not in the cache, `Some(None)` if the method
    /// couldn't be compiled, or `Some(Some(function))` if the method was compiled.
    #[expect(clippy::option_option)]
    pub fn get_cached(&self, key: CacheKey) -> Option<Option<Arc<Function>>> {
        self.function_cache.get(&key).map(|entry| entry.clone())
    }

    /// Inserts a function into the cache.
    pub fn insert_cached(&self, key: CacheKey, function: Option<Arc<Function>>) {
        self.function_cache.insert(key, function);
    }

    /// Checks if a key exists in the cache.
    pub fn contains_cached(&self, key: CacheKey) -> bool {
        self.function_cache.contains_key(&key)
    }

    /// Returns whether the compiler is in interpreted-only mode.
    #[must_use]
    pub fn is_interpreted(&self) -> bool {
        self.interpreted
    }

    /// Returns whether batch compilation is enabled.
    #[must_use]
    pub fn is_batch_compilation_enabled(&self) -> bool {
        self.batch_compilation_enabled
    }

    /// Attempts to compile a Java method to native code using the Just-In-Time compiler.
    ///
    /// This method first checks if the compiled function is already cached, returning it if
    /// available. Otherwise, it either compiles synchronously or queues for background compilation
    /// depending on the configuration.
    ///
    /// # Caching
    ///
    /// Successfully compiled functions are cached to avoid recompilation. Failed compilation
    /// attempts are also cached (as `None`) to avoid retrying incompatible methods.
    ///
    /// # Background Compilation
    ///
    /// When batch compilation is enabled, methods are queued for background compilation and this
    /// function returns `None` immediately. Subsequent calls return the compiled function once ready.
    #[cfg_attr(target_family = "wasm", expect(clippy::unused_async))]
    pub async fn compile(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
    ) -> Result<Option<Arc<Function>>> {
        if self.interpreted {
            return Ok(None);
        }

        // Skip methods that can never be JIT compiled (no Code attribute)
        if method.is_native() || method.is_abstract() {
            return Ok(None);
        }

        let key = cache_key(method);

        // Check cache first
        if let Some(function) = self.get_cached(key) {
            return Ok(function);
        }

        // If batch compilation is disabled, compile synchronously
        if !self.batch_compilation_enabled {
            return Ok(self.compile_method_sync(class, method, key));
        }

        // Batch compilation is not supported on wasm
        #[cfg(target_family = "wasm")]
        {
            Ok(self.compile_method_sync(class, method, key))
        }

        #[cfg(not(target_family = "wasm"))]
        {
            // Mark method as pending in cache (None = not yet compiled)
            // This prevents duplicate queueing on subsequent calls
            self.insert_cached(key, None);

            // Initialize batch compiler and send request in a single lock acquisition
            let request = CompilationRequest {
                class: class.clone(),
                method: method.clone(),
            };
            self.send_compilation_request(request).await;

            // Return None immediately to indicate the function is not ready yet
            Ok(None)
        }
    }

    /// Compiles a method synchronously.
    fn compile_method_sync(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        key: CacheKey,
    ) -> Option<Arc<Function>> {
        let definition = method.definition();

        // Fast-fail: skip methods that contain unsupported instructions
        let function = if ristretto_jit::Compiler::can_compile(definition) {
            let class_file = class.class_file();
            let symbols = runtime_helpers::symbols();
            match self.jit_compiler.compile(class_file, definition, &symbols) {
                Ok(function) => {
                    let function = Arc::new(function);
                    info!(
                        "compiled method {}.{}{}",
                        class.name(),
                        method.name(),
                        method.descriptor()
                    );
                    Some(function)
                }
                Err(error) => {
                    debug!(
                        "JIT compilation failed for {}.{}{}: {error}",
                        class.name(),
                        method.name(),
                        method.descriptor()
                    );
                    None
                }
            }
        } else {
            None
        };

        self.insert_cached(key, function.clone());
        function
    }

    /// Send a compilation request, initializing the batch compiler if needed.
    /// Uses a single Mutex acquisition for both initialization check and send.
    #[cfg(not(target_family = "wasm"))]
    async fn send_compilation_request(&self, request: CompilationRequest) {
        let mut queue_guard = self.compilation_queue.lock().await;

        if queue_guard.is_none() {
            // First-time initialization
            let (sender, mut receiver) = mpsc::unbounded_channel::<CompilationRequest>();
            *queue_guard = Some(sender);

            let cpus = System::physical_core_count().unwrap_or(1);
            let compiler_threads = ((cpus + 5) / 10).max(1);
            info!("JIT parallel compiler configured with {compiler_threads} threads");

            let jit_compiler = self.jit_compiler.clone();
            let function_cache = self.function_cache.clone();

            tokio::spawn(async move {
                let mut batch = Vec::new();

                loop {
                    let timeout = sleep(Duration::from_millis(BATCH_TIMEOUT_MS));
                    tokio::pin!(timeout);

                    let should_process;

                    tokio::select! {
                        request = receiver.recv() => {
                            match request {
                                Some(request) => {
                                    batch.push(request);
                                    should_process = batch.len() >= BATCH_SIZE;
                                }
                                None => break,
                            }
                        }
                        () = &mut timeout => {
                            should_process = !batch.is_empty();
                        }
                    }

                    if should_process {
                        process_compilation_batch(
                            compiler_threads,
                            &mut batch,
                            &jit_compiler,
                            &function_cache,
                        );
                    }
                }
            });
        }

        if let Some(sender) = queue_guard.as_ref() {
            let _ = sender.send(request);
        }
    }
}

/// Process a batch of compilation requests
#[cfg(not(target_family = "wasm"))]
fn process_compilation_batch(
    compiler_threads: usize,
    batch: &mut Vec<CompilationRequest>,
    jit_compiler: &ristretto_jit::Compiler,
    function_cache: &Arc<DashMap<CacheKey, Option<Arc<Function>>>>,
) {
    debug!("Processing compilation batch of {} methods", batch.len());
    let thread_pool = match ThreadPoolBuilder::new()
        .num_threads(compiler_threads)
        .thread_name(|thread_index| format!("jit-{thread_index}"))
        .build()
    {
        Ok(thread_pool) => thread_pool,
        Err(error) => {
            error!("Failed to create thread pool for JIT compilation: {error}");
            return;
        }
    };

    let batch_requests = std::mem::take(batch);
    thread_pool.install(|| {
        batch_requests.into_par_iter().for_each(|request| {
            let key = cache_key(&request.method);
            let definition = request.method.definition();

            // Fast-fail: skip methods that contain unsupported instructions
            let function = if ristretto_jit::Compiler::can_compile(definition) {
                let class_file = request.class.class_file();
                let symbols = runtime_helpers::symbols();
                match jit_compiler.compile(class_file, definition, &symbols) {
                    Ok(function) => {
                        let function = Arc::new(function);
                        info!(
                            "compiled method {}.{}{}",
                            request.class.name(),
                            request.method.name(),
                            request.method.descriptor()
                        );
                        Some(function)
                    }
                    Err(error) => {
                        debug!(
                            "JIT compilation failed for {}.{}{}: {error}",
                            request.class.name(),
                            request.method.name(),
                            request.method.descriptor()
                        );
                        None
                    }
                }
            } else {
                None
            };

            function_cache.insert(key, function);
        });
    });
}

/// Executes a previously JIT-compiled method with the given parameters. It handles the conversion
/// between VM values and JIT values, and manages the case of non-static methods where the first
/// parameter is the `this` reference.
///
/// # Value Conversion
///
/// Parameters are converted from VM value representation to JIT value representation before
/// execution, and the result is converted back from JIT value to VM value.
pub(crate) fn execute(
    function: &Arc<Function>,
    parameters: &[Value],
    gc: &GarbageCollector,
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    // Stack-allocate conversion buffer for typical method arities (0-8 params)
    let mut stack_buf: [ristretto_jit::Value; 8] = [const { ristretto_jit::Value::I32(0) }; 8];
    let mut count = 0;

    let arguments = if parameters.len() <= stack_buf.len() {
        for value in parameters {
            if matches!(value, Value::Unused) {
                continue;
            }
            stack_buf[count] = convert_to_jit(value)?;
            count += 1;
        }
        &stack_buf[..count]
    } else {
        // Fall back to Vec for unusually large parameter lists
        let vec = convert_parameters(parameters)?;
        let runtime_context = RuntimeContext::new(gc, vm, thread, class)?;
        let context = runtime_context.as_ptr();
        let result = function.execute(&vec, context)?;
        return finish_execute(&runtime_context, result);
    };

    let runtime_context = RuntimeContext::new(gc, vm, thread, class)?;
    let context = runtime_context.as_ptr();
    let result = function.execute(arguments, context)?;
    finish_execute(&runtime_context, result)
}

/// Converts the raw JIT result to a VM value, surfacing any pending exception
/// stored in the runtime context as `Err(Throwable(..))`.
///
/// Uses `take_pending_exception_into` so the throwable's GC root is held alive across the
/// `Gc::from_raw_i64` reconstruction and the construction of the `Err(Throwable(..))`
/// `Value`;only after this function's local exit does the helper release the root, by
/// which time the returned `Err` already owns a `Value::Object(Some(Gc))` referenced by
/// the surrounding stack frame.
fn finish_execute(
    runtime_context: &RuntimeContext,
    result: Option<ristretto_jit::Value>,
) -> Result<Option<Value>> {
    runtime_context.take_pending_exception_into(|pending| {
        if pending != 0 {
            let gc_ref: Gc<ristretto_gc::sync::RwLock<ristretto_classloader::Reference>> =
                Gc::from_raw_i64(pending);
            return Err(crate::Error::Throwable(Value::Object(Some(gc_ref))));
        }
        Ok(result.map(|v| convert_to_vm(&v)))
    })
}

/// Converts a vector of VM values to a vector of JIT values for passing to a JIT-compiled function.
/// Unused values (placeholders after Long/Double in the JVM two-slot representation) are skipped
/// because the JIT calling convention uses one slot per value regardless of width.
fn convert_parameters(parameters: &[Value]) -> Result<Vec<ristretto_jit::Value>> {
    let mut values = Vec::with_capacity(parameters.len());
    for value in parameters {
        if matches!(value, Value::Unused) {
            continue;
        }
        let value = convert_to_jit(value)?;
        values.push(value);
    }
    Ok(values)
}

/// Converts a single VM value to its corresponding JIT value representation.
fn convert_to_jit(value: &Value) -> Result<ristretto_jit::Value> {
    let jit_value = match value {
        Value::Int(value) => ristretto_jit::Value::I32(*value),
        Value::Long(value) => ristretto_jit::Value::I64(*value),
        Value::Float(value) => ristretto_jit::Value::F32(*value),
        Value::Double(value) => ristretto_jit::Value::F64(*value),
        Value::Object(None) => ristretto_jit::Value::Ptr(0),
        Value::Object(Some(gc_ref)) => ristretto_jit::Value::Ptr(gc_ref.as_ptr_i64()),
        Value::Unused => {
            return Err(InternalError(
                "Cannot convert Unused value to JIT".to_string(),
            ));
        }
    };
    Ok(jit_value)
}

/// Converts a JIT value returned by a compiled function back to a VM value.
///
/// # Safety
///
/// For `Ptr` values, the raw pointer must still be valid (i.e., the original `Gc` reference
/// must be kept alive by the caller for the duration of JIT execution).
fn convert_to_vm(jit_value: &ristretto_jit::Value) -> Value {
    match jit_value {
        ristretto_jit::Value::I32(value) => Value::from(*value),
        ristretto_jit::Value::I64(value) => Value::from(*value),
        ristretto_jit::Value::F32(value) => Value::from(*value),
        ristretto_jit::Value::F64(value) => Value::from(*value),
        ristretto_jit::Value::Ptr(0) => Value::Object(None),
        ristretto_jit::Value::Ptr(ptr) => {
            let gc_ref = ristretto_gc::Gc::from_raw_i64(*ptr);
            Value::Object(Some(gc_ref))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_parameters() -> Result<()> {
        let parameters = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.1),
            Value::Double(4.2),
        ];
        let values = convert_parameters(&parameters)?;
        assert_eq!(values.len(), 4);
        assert_eq!(values[0], ristretto_jit::Value::I32(1));
        assert_eq!(values[1], ristretto_jit::Value::I64(2));
        assert_eq!(values[2], ristretto_jit::Value::F32(3.1));
        assert_eq!(values[3], ristretto_jit::Value::F64(4.2));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_i32() -> Result<()> {
        let value = Value::Int(42);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::I32(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_i64() -> Result<()> {
        let value = Value::Long(42);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::I64(42));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_f32() -> Result<()> {
        let value = Value::Float(42.1);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::F32(42.1));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_f64() -> Result<()> {
        let value = Value::Double(42.1);
        let result = convert_to_jit(&value)?;
        assert_eq!(result, ristretto_jit::Value::F64(42.1));
        Ok(())
    }

    #[test]
    fn test_convert_to_jit_null_object() {
        let value = Value::Object(None);
        let result = convert_to_jit(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ristretto_jit::Value::Ptr(0));
    }

    #[test]
    fn test_convert_to_jit_unused() {
        let value = Value::Unused;
        let result = convert_to_jit(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_to_vm_i32() {
        let value = ristretto_jit::Value::I32(42);
        let result = convert_to_vm(&value);
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_convert_to_vm_i64() {
        let value = ristretto_jit::Value::I64(42);
        let result = convert_to_vm(&value);
        assert_eq!(result, Value::Long(42));
    }

    #[test]
    fn test_convert_to_vm_f32() {
        let value = ristretto_jit::Value::F32(42.1);
        let result = convert_to_vm(&value);
        assert_eq!(result, Value::Float(42.1));
    }

    #[test]
    fn test_convert_to_vm_f64() {
        let value = ristretto_jit::Value::F64(42.1);
        let result = convert_to_vm(&value);
        assert_eq!(result, Value::Double(42.1));
    }

    #[test]
    fn test_convert_to_vm_null_ptr() {
        let value = ristretto_jit::Value::Ptr(0);
        let result = convert_to_vm(&value);
        assert_eq!(result, Value::Object(None));
    }

    #[test]
    fn test_compiler_new() {
        let compiler = Compiler::new(false, false);
        assert!(compiler.is_some());
        let compiler = compiler.unwrap();
        assert!(!compiler.is_interpreted());
        assert!(!compiler.is_batch_compilation_enabled());
    }

    #[test]
    fn test_compiler_interpreted_mode() {
        let compiler = Compiler::new(false, true).unwrap();
        assert!(compiler.is_interpreted());
        assert!(!compiler.is_batch_compilation_enabled());
    }

    #[test]
    fn test_compiler_batch_compilation_mode() {
        let compiler = Compiler::new(true, false).unwrap();
        assert!(!compiler.is_interpreted());
        assert!(compiler.is_batch_compilation_enabled());
    }

    #[test]
    fn test_compiler_cache_operations() {
        let compiler = Compiler::new(false, false).unwrap();
        let test_key: CacheKey = 42;

        // Initially cache is empty
        assert!(!compiler.contains_cached(test_key));
        assert!(compiler.get_cached(test_key).is_none());

        // Insert a None (failed compilation)
        compiler.insert_cached(test_key, None);
        assert!(compiler.contains_cached(test_key));
        // Check that we get Some(None); meaning the method was cached but couldn't be compiled
        let cached = compiler.get_cached(test_key);
        assert!(cached.is_some());
        assert!(cached.unwrap().is_none());
    }

    #[test]
    fn test_compiler_debug() {
        let compiler = Compiler::new(true, false).unwrap();
        let debug_str = format!("{compiler:?}");
        assert!(debug_str.contains("Compiler"));
        assert!(debug_str.contains("batch_compilation_enabled"));
    }
}

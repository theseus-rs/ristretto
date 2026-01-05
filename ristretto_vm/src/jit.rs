use crate::Error::{InternalError, JitError};
use crate::Result;
use dashmap::DashMap;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classloader::{Class, Method, Value};
use ristretto_jit::Error::{
    UnsupportedInstruction, UnsupportedMethod, UnsupportedTargetISA, UnsupportedType,
};
use ristretto_jit::Function;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::sleep;
use tracing::{debug, error, info};

const BATCH_SIZE: usize = 10;
const BATCH_TIMEOUT_MS: u64 = 50;

/// Type alias for pending compilation senders map
type PendingCompilations = DashMap<String, Vec<oneshot::Sender<Result<Option<Arc<Function>>>>>>;

/// A compilation request for the background batch compiler
struct CompilationRequest {
    class: Arc<Class>,
    method: Method,
    response_sender: oneshot::Sender<Result<Option<Arc<Function>>>>,
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
    function_cache: Arc<DashMap<String, Option<Arc<Function>>>>,
    /// Per-VM compilation queue for background batch processing
    compilation_queue: Mutex<Option<mpsc::UnboundedSender<CompilationRequest>>>,
    /// Per-VM set to track methods currently being compiled (wrapped in Arc for sharing)
    pending_compilations: Arc<PendingCompilations>,
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
            pending_compilations: Arc::new(DashMap::new()),
            batch_compilation_enabled: batch_compilation,
            interpreted,
        })
    }

    /// Gets a cached function by its fully qualified method name.
    /// Returns `None` if the key is not in the cache, `Some(None)` if the method
    /// couldn't be compiled, or `Some(Some(function))` if the method was compiled.
    #[expect(clippy::option_option)]
    pub fn get_cached(&self, key: &str) -> Option<Option<Arc<Function>>> {
        self.function_cache.get(key).map(|entry| entry.clone())
    }

    /// Inserts a function into the cache.
    pub fn insert_cached(&self, key: String, function: Option<Arc<Function>>) {
        self.function_cache.insert(key, function);
    }

    /// Checks if a key exists in the cache.
    pub fn contains_cached(&self, key: &str) -> bool {
        self.function_cache.contains_key(key)
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
    pub async fn compile(
        &self,
        class: &Arc<Class>,
        method: &Method,
    ) -> Result<Option<Arc<Function>>> {
        if self.interpreted {
            return Ok(None);
        }

        let class_name = class.name();
        let method_name = method.name();
        let method_descriptor = method.descriptor();
        let fully_qualified_method_name = format!("{class_name}.{method_name}{method_descriptor}");

        // Check cache first
        if let Some(function) = self.get_cached(&fully_qualified_method_name) {
            debug!("Using cached function for {fully_qualified_method_name}");
            return Ok(function);
        }

        // If batch compilation is disabled, compile synchronously
        if !self.batch_compilation_enabled {
            return self.compile_method_sync(class, method, &fully_qualified_method_name, false);
        }

        // If the method is already being compiled, return None to indicate it's not ready yet
        if self
            .pending_compilations
            .contains_key(&fully_qualified_method_name)
        {
            debug!("Method {fully_qualified_method_name} is already being compiled in background");
            return Ok(None);
        }

        // Initialize the batch compiler if not already done
        self.initialize_batch_compiler().await;

        // Mark this method as pending compilation
        self.pending_compilations
            .insert(fully_qualified_method_name.clone(), Vec::new());

        // Send the compilation request to the background compiler
        let (response_sender, _response_receiver) = oneshot::channel();
        let request = CompilationRequest {
            class: class.clone(),
            method: method.clone(),
            response_sender,
        };

        if let Some(queue) = self.compilation_queue.lock().await.as_ref() {
            let _ = queue.send(request);
        }

        debug!("Queued method {fully_qualified_method_name} for background compilation");

        // Return None immediately to indicate the function is not ready yet
        Ok(None)
    }

    /// Compiles a method synchronously.
    fn compile_method_sync(
        &self,
        class: &Arc<Class>,
        method: &Method,
        fully_qualified_method_name: &str,
        notify_pending: bool,
    ) -> Result<Option<Arc<Function>>> {
        let class_file = class.class_file();
        let definition = method.definition();

        let function = match self.jit_compiler.compile(class_file, definition) {
            Ok(function) => {
                let function = Arc::new(function);
                info!("compiled method {fully_qualified_method_name}");
                Some(function)
            }
            Err(UnsupportedInstruction(instruction)) => {
                debug!("Unsupported instruction: {instruction:?}");
                None
            }
            Err(UnsupportedMethod(message)) => {
                debug!("Unsupported method: {message}");
                None
            }
            Err(UnsupportedTargetISA(message)) => {
                debug!("Unsupported target ISA: {message}");
                None
            }
            Err(UnsupportedType(vm_type)) => {
                debug!("Unsupported type: {vm_type}");
                None
            }
            Err(error) => {
                let constant_pool = class.constant_pool();
                error!(
                    "Error compiling instructions for {fully_qualified_method_name}:\n\
                    Error:\n\
                    {error:?}\n\
                    Constant Pool:\n\
                    {constant_pool}\n\
                    Method:\n\
                    {method:?}"
                );

                let err = JitError(error);
                if notify_pending
                    && let Some((_, pending_senders)) = self
                        .pending_compilations
                        .remove(fully_qualified_method_name)
                {
                    for sender in pending_senders {
                        let _ =
                            sender.send(Err(InternalError("JIT compilation failed".to_string())));
                    }
                }

                return Err(err);
            }
        };

        // Notify any pending waiters for this method if requested
        if notify_pending
            && let Some((_, pending_senders)) = self
                .pending_compilations
                .remove(fully_qualified_method_name)
        {
            for sender in pending_senders {
                let _ = sender.send(Ok(function.clone()));
            }
        }

        self.insert_cached(fully_qualified_method_name.to_string(), function.clone());
        Ok(function)
    }

    /// Initialize the background batch compilation system for this compiler instance.
    async fn initialize_batch_compiler(&self) {
        let mut queue = self.compilation_queue.lock().await;
        if queue.is_some() {
            // Already initialized
            return;
        }

        let (sender, mut receiver) = mpsc::unbounded_channel::<CompilationRequest>();
        *queue = Some(sender);
        drop(queue);

        let cpus = System::physical_core_count().unwrap_or(1);
        // cpus + 5 ensures that if there is a remainder it rounds to the nearest whole number
        let compiler_threads = ((cpus + 5) / 10).max(1);
        info!("JIT parallel compiler configured with {compiler_threads} threads");

        // Clone the JIT compiler for use in the background task
        let jit_compiler = self.jit_compiler.clone();
        let function_cache = self.function_cache.clone();
        let pending_compilations = self.pending_compilations.clone();

        // Spawn the background compilation task
        tokio::spawn(async move {
            let mut batch = Vec::new();

            loop {
                // Try to collect a batch of compilation requests
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
                            None => break, // Channel closed
                        }
                    }
                    () = &mut timeout => {
                        should_process = !batch.is_empty();
                    }
                }

                // Process the batch if we have requests and either hit batch size or timeout
                if should_process {
                    process_compilation_batch(
                        compiler_threads,
                        &mut batch,
                        &jit_compiler,
                        &function_cache,
                        &pending_compilations,
                    );
                }
            }
        });
    }
}

/// Process a batch of compilation requests
fn process_compilation_batch(
    compiler_threads: usize,
    batch: &mut Vec<CompilationRequest>,
    jit_compiler: &ristretto_jit::Compiler,
    function_cache: &Arc<DashMap<String, Option<Arc<Function>>>>,
    pending_compilations: &Arc<PendingCompilations>,
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
            let class_name = request.class.name();
            let method_name = request.method.name();
            let method_descriptor = request.method.descriptor();
            let fully_qualified_method_name =
                format!("{class_name}.{method_name}{method_descriptor}");

            let class_file = request.class.class_file();
            let definition = request.method.definition();

            let function = match jit_compiler.compile(class_file, definition) {
                Ok(function) => {
                    let function = Arc::new(function);
                    info!("compiled method {fully_qualified_method_name}");
                    Some(function)
                }
                Err(UnsupportedInstruction(instruction)) => {
                    debug!("Unsupported instruction: {instruction:?}");
                    None
                }
                Err(UnsupportedMethod(message)) => {
                    debug!("Unsupported method: {message}");
                    None
                }
                Err(UnsupportedTargetISA(message)) => {
                    debug!("Unsupported target ISA: {message}");
                    None
                }
                Err(UnsupportedType(vm_type)) => {
                    debug!("Unsupported type: {vm_type}");
                    None
                }
                Err(error) => {
                    let constant_pool = request.class.constant_pool();
                    error!(
                        "Error compiling instructions for {fully_qualified_method_name}:\n\
                        Error:\n\
                        {error:?}\n\
                        Constant Pool:\n\
                        {constant_pool}\n\
                        Method:\n\
                        {:?}",
                        request.method
                    );

                    if let Some((_, pending_senders)) =
                        pending_compilations.remove(&fully_qualified_method_name)
                    {
                        for sender in pending_senders {
                            let _ = sender
                                .send(Err(InternalError("JIT compilation failed".to_string())));
                        }
                    }

                    let _ = request.response_sender.send(Err(JitError(error)));
                    return;
                }
            };

            // Notify any pending waiters for this method
            if let Some((_, pending_senders)) =
                pending_compilations.remove(&fully_qualified_method_name)
            {
                for sender in pending_senders {
                    let _ = sender.send(Ok(function.clone()));
                }
            }

            function_cache.insert(fully_qualified_method_name, function.clone());
            let _ = request.response_sender.send(Ok(function));
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
    method: &Method,
    mut parameters: Vec<Value>,
) -> Result<Option<Value>> {
    if !method.access_flags().contains(MethodAccessFlags::STATIC) {
        // Remove the first parameter (the `this` reference) for non-static methods
        parameters.remove(0);
    }

    let arguments = convert_parameters(&parameters)?;
    let result = if let Some(value) = function.execute(arguments)? {
        let value = convert_to_vm(&value);
        Some(value)
    } else {
        None
    };
    Ok(result)
}

/// Converts a vector of VM values to a vector of JIT values for passing to a JIT-compiled function.
fn convert_parameters(parameters: &[Value]) -> Result<Vec<ristretto_jit::Value>> {
    let mut values = Vec::with_capacity(parameters.len());
    for value in parameters {
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
        _ => {
            eprintln!("Unsupported JIT value type: {value:?}");
            return Err(InternalError(format!(
                "Unsupported JIT value type: {value:?}"
            )));
        }
    };
    Ok(jit_value)
}

/// Converts a JIT value returned by a compiled function back to a VM value.
fn convert_to_vm(jit_value: &ristretto_jit::Value) -> Value {
    match jit_value {
        ristretto_jit::Value::I32(value) => Value::from(*value),
        ristretto_jit::Value::I64(value) => Value::from(*value),
        ristretto_jit::Value::F32(value) => Value::from(*value),
        ristretto_jit::Value::F64(value) => Value::from(*value),
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
    fn test_convert_to_jit_unsupported() {
        let value = Value::Object(None);
        let result = convert_to_jit(&value);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "Internal error: Unsupported JIT value type: Object(None)"
            );
        }
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

        // Initially cache is empty
        assert!(!compiler.contains_cached("test_method"));
        assert!(compiler.get_cached("test_method").is_none());

        // Insert a None (failed compilation)
        compiler.insert_cached("test_method".to_string(), None);
        assert!(compiler.contains_cached("test_method"));
        // Check that we get Some(None) - meaning the method was cached but couldn't be compiled
        let cached = compiler.get_cached("test_method");
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

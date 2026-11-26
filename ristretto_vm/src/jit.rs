use crate::Error::InternalError;
use crate::jit_runtime_helpers::{self, RuntimeContext};
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

/// Derives a cache key from the method pointer without allocating a string.
pub(crate) fn cache_key(method: &Arc<Method>) -> usize {
    Arc::as_ptr(method) as usize
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
    function_cache: Arc<DashMap<usize, Option<Arc<Function>>>>,
    /// Per-VM compilation queue for background batch processing
    compilation_queue: Mutex<Option<mpsc::UnboundedSender<CompilationRequest>>>,
    /// Whether batch compilation is enabled
    batch_compilation_enabled: bool,
    /// Whether the compiler is in interpreted-only mode
    interpreted: bool,
    #[cfg(feature = "execution-profile")]
    pub(crate) execution_recorder: Arc<crate::execution_profile::Recorder>,
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
            #[cfg(feature = "execution-profile")]
            execution_recorder: Arc::new(crate::execution_profile::Recorder::default()),
        })
    }

    /// Gets a cached function by its cache key.
    /// Returns `None` if the key is not in the cache, `Some(None)` if the method
    /// couldn't be compiled, or `Some(Some(function))` if the method was compiled.
    #[expect(clippy::option_option)]
    pub fn get_cached(&self, key: usize) -> Option<Option<Arc<Function>>> {
        self.function_cache.get(&key).map(|entry| entry.clone())
    }

    /// Inserts a function into the cache.
    pub fn insert_cached(&self, key: usize, function: Option<Arc<Function>>) {
        if function.is_some() {
            self.function_cache.insert(key, function);
        } else {
            // A concurrent production rejection must not erase a successfully
            // prepared function. Successful code remains available for dispatch.
            self.function_cache.entry(key).or_insert(None);
        }
    }

    /// Checks if a key exists in the cache.
    pub fn contains_cached(&self, key: usize) -> bool {
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
    #[cfg_attr(
        target_family = "wasm",
        expect(clippy::unused_async, clippy::unused_async_trait_impl)
    )]
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
            #[cfg(feature = "execution-profile")]
            self.execution_recorder.queued(class, method);

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
        key: usize,
    ) -> Option<Arc<Function>> {
        let definition = method.definition();

        // Fast-fail: skip methods that contain unsupported instructions
        let function = if ristretto_jit::Compiler::can_compile_in(class.class_file(), definition) {
            let class_file = class.class_file();
            let symbols = jit_runtime_helpers::symbols();
            #[cfg(feature = "execution-profile")]
            self.execution_recorder.starting(class, method);
            #[cfg(feature = "execution-profile")]
            let started = std::time::Instant::now();
            match self.jit_compiler.compile(class_file, definition, &symbols) {
                Ok(function) => {
                    let function = Arc::new(function);
                    #[cfg(feature = "execution-profile")]
                    self.execution_recorder.finished(
                        class,
                        method,
                        Ok(&function),
                        started.elapsed(),
                    );
                    info!(
                        "compiled method {}.{}{}",
                        class.name(),
                        method.name(),
                        method.descriptor()
                    );
                    Some(function)
                }
                Err(error) => {
                    #[cfg(feature = "execution-profile")]
                    self.execution_recorder.finished(
                        class,
                        method,
                        Err(error.to_string()),
                        started.elapsed(),
                    );
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
            #[cfg(feature = "execution-profile")]
            self.execution_recorder.rejected(class, method);
            None
        };

        self.insert_cached(key, function.clone());
        function
    }

    /// Prepare an explicitly requested experimental method for ordinary dispatch.
    /// Reject background mode so a queued production rejection cannot overwrite
    /// the prepared function. Failures are returned to the caller, never hidden as
    /// successful preparation followed by interpreter fallback.
    #[cfg(feature = "experimental-reference-dispatch")]
    pub(crate) fn prepare_experimental(
        &self,
        class: &Arc<Class>,
        method: &Arc<Method>,
    ) -> Result<()> {
        if self.interpreted || self.batch_compilation_enabled {
            return Err(InternalError(
                "Experimental preparation requires JIT enabled and batch compilation disabled"
                    .to_owned(),
            ));
        }
        if let Some(reason) = ristretto_jit::Compiler::experimental_rejection_reason(
            class.class_file(),
            method.definition(),
        ) {
            return Err(InternalError(format!(
                "Experimental JIT rejected {}.{}{}: {reason}",
                class.name(),
                method.name(),
                method.descriptor()
            )));
        }
        let key = cache_key(method);
        if self.get_cached(key).flatten().is_some() {
            return Ok(());
        }
        #[cfg(feature = "execution-profile")]
        self.execution_recorder.starting_experimental(class, method);
        #[cfg(feature = "execution-profile")]
        let started = std::time::Instant::now();
        let result = self
            .jit_compiler
            .compile(
                class.class_file(),
                method.definition(),
                &jit_runtime_helpers::symbols(),
            )
            .map(Arc::new);
        #[cfg(feature = "execution-profile")]
        self.execution_recorder.finished(
            class,
            method,
            result.as_ref().map_err(ToString::to_string),
            started.elapsed(),
        );
        let function = result?;
        self.insert_cached(key, Some(function));
        Ok(())
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
            #[cfg(feature = "execution-profile")]
            let execution_recorder = self.execution_recorder.clone();
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
                            #[cfg(feature = "execution-profile")]
                            &execution_recorder,
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
    function_cache: &Arc<DashMap<usize, Option<Arc<Function>>>>,
    #[cfg(feature = "execution-profile")] execution_recorder: &crate::execution_profile::Recorder,
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
            let function = if ristretto_jit::Compiler::can_compile_in(
                request.class.class_file(),
                definition,
            ) {
                let class_file = request.class.class_file();
                let symbols = jit_runtime_helpers::symbols();
                #[cfg(feature = "execution-profile")]
                execution_recorder.starting(&request.class, &request.method);
                #[cfg(feature = "execution-profile")]
                let started = std::time::Instant::now();
                match jit_compiler.compile(class_file, definition, &symbols) {
                    Ok(function) => {
                        let function = Arc::new(function);
                        #[cfg(feature = "execution-profile")]
                        execution_recorder.finished(
                            &request.class,
                            &request.method,
                            Ok(&function),
                            started.elapsed(),
                        );
                        info!(
                            "compiled method {}.{}{}",
                            request.class.name(),
                            request.method.name(),
                            request.method.descriptor()
                        );
                        Some(function)
                    }
                    Err(error) => {
                        #[cfg(feature = "execution-profile")]
                        execution_recorder.finished(
                            &request.class,
                            &request.method,
                            Err(error.to_string()),
                            started.elapsed(),
                        );
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
                #[cfg(feature = "execution-profile")]
                execution_recorder.rejected(&request.class, &request.method);
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
    gc: &Arc<GarbageCollector>,
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    crate::native_stack::with_compiled_stack(function.native_stack_bytes(), || {
        execute_on_stack(function, parameters, gc, vm, thread, class)
    })
}

/// Return the native frame before yielding so cancellation and executor migration
/// cannot race code still using the Java stack. Reference-bearing continuations
/// retain an owning context across slices; call requests await ordinary VM dispatch
/// after their generated caller has returned.
#[expect(
    clippy::too_many_lines,
    reason = "keep native slices, owning request dispatch and shared fuel accounting together"
)]
pub(crate) async fn execute_cooperatively(
    function: &Arc<Function>,
    parameters: &[Value],
    gc: &Arc<GarbageCollector>,
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    class: &Arc<Class>,
) -> Result<Option<Value>> {
    if !function.supports_cooperation() {
        return execute(function, parameters, gc, vm, thread, class);
    }
    let mut activation = function
        .activation(&convert_parameters(parameters)?)
        .map_err(execution_error)?;
    let runtime_context = if function.cooperation_requires_context() {
        Some(RuntimeContext::new(gc, vm, thread, class)?)
    } else {
        None
    };
    let frame = thread.current_frame()?;
    let mut budget = function.cooperation_budget();
    loop {
        let step = crate::native_stack::with_compiled_stack(function.native_stack_bytes(), || {
            activation
                .resume_with_context(
                    budget,
                    runtime_context
                        .as_ref()
                        .map_or(std::ptr::null(), RuntimeContext::as_ptr),
                )
                .map_err(execution_error)
        })?;
        match step {
            ristretto_jit::ExecutionStep::Call(request) => {
                frame.set_program_counter(request.bci);
                // A chain of finite callees also needs a cancellation boundary.
                // No generated frame or borrowed runtime pointer spans this yield.
                tokio::task::yield_now().await;
                budget = function.cooperation_budget();
                let context = runtime_context
                    .as_ref()
                    .ok_or_else(|| InternalError("call has no owning context".to_owned()))?;
                let arguments = activation.call_arguments().map_err(execution_error)?;
                let bits =
                    crate::native_stack::CallFuture::new(context.invoke_call(request, arguments))
                        .await?;
                activation.complete_call(bits).map_err(execution_error)?;
            }
            request @ (ristretto_jit::ExecutionStep::Monitor(_)
            | ristretto_jit::ExecutionStep::Allocation(_)
            | ristretto_jit::ExecutionStep::Field(_)
            | ristretto_jit::ExecutionStep::TypeCheck(_)
            | ristretto_jit::ExecutionStep::Constant(_)
            | ristretto_jit::ExecutionStep::ExceptionMatch(_)
            | ristretto_jit::ExecutionStep::ArrayStore(_)
            | ristretto_jit::ExecutionStep::ArrayAccess(_)) => {
                let remaining = activation.remaining_budget().map_err(execution_error)?;
                let context = runtime_context.as_ref().ok_or_else(|| {
                    InternalError("runtime request has no owning context".to_owned())
                })?;
                let arguments = activation.call_arguments().map_err(execution_error)?;
                let bits = match request {
                    ristretto_jit::ExecutionStep::Monitor(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_monitor(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::Allocation(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_allocation(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::Field(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_field(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::ExceptionMatch(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_exception_match(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::Constant(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_constant(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::TypeCheck(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_type_check(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::ArrayAccess(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_array_access(request, arguments),
                        )
                        .await?
                    }
                    ristretto_jit::ExecutionStep::ArrayStore(request) => {
                        frame.set_program_counter(request.bci);
                        crate::native_stack::CallFuture::new(
                            context.invoke_array_store(request, arguments),
                        )
                        .await?
                    }
                    _ => return Err(InternalError("invalid runtime request".to_owned())),
                };
                activation.complete_call(bits).map_err(execution_error)?;
                budget = if let Some(remaining) = remaining {
                    remaining
                } else {
                    tokio::task::yield_now().await;
                    function.cooperation_budget()
                };
            }
            ristretto_jit::ExecutionStep::Yielded => {
                let bci = activation
                    .suspended_bci()
                    .ok_or_else(|| InternalError("missing continuation BCI".to_owned()))?;
                frame.set_program_counter(bci);
                tokio::task::yield_now().await;
                budget = function.cooperation_budget();
            }
            ristretto_jit::ExecutionStep::Returned(value) => {
                return match runtime_context {
                    Some(context) => finish_execute(context, value),
                    None => value.map(|value| convert_to_vm(gc, &value)).transpose(),
                };
            }
        }
    }
}

fn execute_on_stack(
    function: &Arc<Function>,
    parameters: &[Value],
    gc: &Arc<GarbageCollector>,
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
            let slot = stack_buf
                .get_mut(count)
                .ok_or_else(|| InternalError(format!("Invalid JIT argument index {count}")))?;
            *slot = convert_to_jit(value)?;
            count += 1;
        }
        stack_buf
            .get(..count)
            .ok_or_else(|| InternalError(format!("Invalid JIT argument count {count}")))?
    } else {
        // Fall back to Vec for unusually large parameter lists
        let vec = convert_parameters(parameters)?;
        let runtime_context = RuntimeContext::new(gc, vm, thread, class)?;
        let context = runtime_context.as_ptr();
        let result = function.execute(&vec, context).map_err(execution_error)?;
        return finish_execute(runtime_context, result);
    };

    let runtime_context = RuntimeContext::new(gc, vm, thread, class)?;
    let context = runtime_context.as_ptr();
    let result = function
        .execute(arguments, context)
        .map_err(execution_error)?;
    finish_execute(runtime_context, result)
}

fn execution_error(error: ristretto_jit::Error) -> crate::Error {
    match error {
        ristretto_jit::Error::NativeStackOverflow { .. } => {
            crate::JavaError::StackOverflowError("Native stack exhausted".to_owned()).into()
        }
        error => error.into(),
    }
}

/// Converts the raw JIT result to a VM value, surfacing any pending exception
/// stored in the runtime context as `Err(Throwable(..))`.
///
/// Consumes the completed invocation. Exceptions and published reference results
/// transfer their owning handles into the returned error/value before context cleanup.
fn finish_execute(
    runtime_context: RuntimeContext,
    result: Option<ristretto_jit::Value>,
) -> Result<Option<Value>> {
    if let Some(exception) = runtime_context.take_pending_exception_owner() {
        return Err(crate::Error::Throwable(Value::Object(Some(exception))));
    }
    match result {
        Some(ristretto_jit::Value::Ptr(handle)) => {
            runtime_context.into_reference_result(handle).map(Some)
        }
        Some(value) => convert_to_vm(runtime_context.garbage_collector(), &value).map(Some),
        None => Ok(None),
    }
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
        Value::Object(Some(gc_ref)) => ristretto_jit::Value::Ptr(gc_ref.as_handle_i64()),
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
fn convert_to_vm(
    collector: &Arc<GarbageCollector>,
    jit_value: &ristretto_jit::Value,
) -> Result<Value> {
    let value = match jit_value {
        ristretto_jit::Value::I32(value) => Value::from(*value),
        ristretto_jit::Value::I64(value) => Value::from(*value),
        ristretto_jit::Value::F32(value) => Value::from(*value),
        ristretto_jit::Value::F64(value) => Value::from(*value),
        ristretto_jit::Value::Ptr(0) => Value::Object(None),
        ristretto_jit::Value::Ptr(ptr) => {
            let gc_ref = Gc::from_handle_rooted(collector, *ptr)?;
            Value::Object(Some(gc_ref))
        }
    };
    Ok(value)
}

#[cfg(all(
    test,
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
#[path = "jit_control_flow_tests.rs"]
mod control_flow_tests;

#[cfg(all(
    test,
    not(any(
        target_family = "wasm",
        target_endian = "big",
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
#[path = "jit_primitive_control_flow_tests.rs"]
mod primitive_control_flow_tests;

#[cfg(all(
    test,
    not(any(
        target_family = "wasm",
        target_endian = "big",
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
#[path = "jit_static_call_tests.rs"]
pub(crate) mod static_call_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(
        not(target_family = "wasm"),
        target_endian = "little",
        not(any(
            target_arch = "mips",
            target_arch = "mips64",
            target_os = "dragonfly",
            target_os = "solaris"
        ))
    ))]
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn mixed_width_duplication_executes_through_the_production_gate() -> Result<()> {
        use ristretto_classfile::attributes::{Attribute, Instruction};
        use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};

        let (vm, thread) = crate::test::thread().await?;
        let mut pool = ConstantPool::default();
        let this_class = pool.add_class("MixedWidthDuplication")?;
        let name_index = pool.add_utf8("run")?;
        let descriptor_index = pool.add_utf8("()I")?;
        let code_name = pool.add_utf8("Code")?;
        let class = Class::from(
            None,
            ClassFile {
                constant_pool: pool,
                this_class,
                methods: vec![ristretto_classfile::Method {
                    access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                    name_index,
                    descriptor_index,
                    attributes: vec![Attribute::Code {
                        name_index: code_name,
                        max_stack: 6,
                        max_locals: 0,
                        code: vec![
                            Instruction::Lconst_1,
                            Instruction::Iconst_2,
                            Instruction::Iconst_3,
                            Instruction::Dup2_x2,
                            Instruction::Isub,
                            Instruction::I2l,
                            Instruction::Lsub,
                            Instruction::L2i,
                            Instruction::Isub,
                            Instruction::Isub,
                            Instruction::Ireturn,
                        ],
                        exception_table: vec![],
                        attributes: vec![],
                    }],
                }],
                ..Default::default()
            },
        )?;
        let method = class.try_get_method("run", "()I")?;
        assert!(ristretto_jit::Compiler::can_compile(method.definition()));
        assert_eq!(
            thread.execute(&class, &method, &[] as &[Value]).await?,
            Some(Value::Int(1))
        );
        // Simulate a rejection finishing after successful code publication.
        vm.compiler()
            .expect("compiler")
            .insert_cached(cache_key(&method), None);
        assert!(
            vm.compiler()
                .expect("compiler")
                .get_cached(cache_key(&method))
                .flatten()
                .is_some()
        );
        #[cfg(feature = "execution-profile")]
        {
            let profile = vm.execution_profile();
            let method = profile
                .methods
                .iter()
                .find(|m| m.class == "MixedWidthDuplication")
                .expect("method record");
            assert_eq!(method.compiled_entries, 1);
            assert_eq!(method.interpreter_entries, 0);
        }
        Ok(())
    }

    #[test]
    fn test_convert_parameters() -> Result<()> {
        let parameters = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.1),
            Value::Double(4.2),
        ];
        let values = convert_parameters(&parameters)?;
        assert_eq!(
            values.as_slice(),
            &[
                ristretto_jit::Value::I32(1),
                ristretto_jit::Value::I64(2),
                ristretto_jit::Value::F32(3.1),
                ristretto_jit::Value::F64(4.2),
            ]
        );
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
        let result = convert_to_vm(&GarbageCollector::new(), &value).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_convert_to_vm_i64() {
        let value = ristretto_jit::Value::I64(42);
        let result = convert_to_vm(&GarbageCollector::new(), &value).unwrap();
        assert_eq!(result, Value::Long(42));
    }

    #[test]
    fn test_convert_to_vm_f32() {
        let value = ristretto_jit::Value::F32(42.1);
        let result = convert_to_vm(&GarbageCollector::new(), &value).unwrap();
        assert_eq!(result, Value::Float(42.1));
    }

    #[test]
    fn test_convert_to_vm_f64() {
        let value = ristretto_jit::Value::F64(42.1);
        let result = convert_to_vm(&GarbageCollector::new(), &value).unwrap();
        assert_eq!(result, Value::Double(42.1));
    }

    #[test]
    fn test_convert_to_vm_null_ptr() {
        let value = ristretto_jit::Value::Ptr(0);
        let result = convert_to_vm(&GarbageCollector::new(), &value).unwrap();
        assert_eq!(result, Value::Object(None));
    }

    #[test]
    fn reference_results_hold_an_owner_and_reject_foreign_or_retired_handles() -> Result<()> {
        let heap = GarbageCollector::new();
        let root = Gc::try_new(
            &heap,
            ristretto_gc::sync::RwLock::new(ristretto_classloader::Reference::from(vec![42_i8])),
        )?;
        let encoded = ristretto_jit::Value::Ptr(root.as_handle_i64());
        assert!(convert_to_vm(&GarbageCollector::new(), &encoded).is_err());
        let result = convert_to_vm(&heap, &encoded)?;
        drop(root);
        heap.collect_full()?;
        assert_eq!(&*result.as_byte_vec_ref()?, &[42]);
        drop(result);
        heap.collect_full()?;
        assert!(convert_to_vm(&heap, &encoded).is_err());
        Ok(())
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
        let test_key = 42;

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

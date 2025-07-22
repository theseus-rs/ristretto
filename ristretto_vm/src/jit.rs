use crate::Error::{InternalError, JitError};
use crate::{Result, VM};
use dashmap::DashMap;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classloader::{Class, Method, Value};
use ristretto_jit::Error::{
    UnsupportedInstruction, UnsupportedMethod, UnsupportedTargetISA, UnsupportedType,
};
use ristretto_jit::Function;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::sleep;
use tracing::{Instrument, debug, error, info, info_span};

/// A thread-safe global cache for JIT-compiled functions.
///
/// # Overview
///
/// The function cache stores compiled native functions indexed by their fully-qualified method
/// names. This allows the JVM to reuse previously compiled methods rather than recompiling them on
/// each invocation, significantly improving performance for frequently called methods.
///
/// # Structure
///
/// The cache maps fully qualified method names (in the format
/// `class_name.method_name(method_descriptor)`) to `Option<Arc<Function>>`. The `Option` allows us
/// to cache negative results (methods that failed to compile) to avoid repeated compilation
/// attempts for methods known to be incompatible with the JIT.
///
/// # Thread Safety
///
/// Uses `DashMap` to provide thread-safe concurrent access without locks in the common case.
/// This allows multiple threads to access the cache simultaneously without blocking each other.
///
/// # Lifetime
///
/// The cache persists for the entire duration of the program execution. Compiled functions are
/// not automatically evicted, which could potentially lead to high memory usage in long-running
/// applications that load many classes.
///
/// # Examples
///
/// ```text
/// "java/lang/String.length()I" => Some(Arc<Function>)
/// "java/util/HashMap.resize()V" => None  // Method was attempted but couldn't be compiled
/// ```
static FUNCTION_CACHE: LazyLock<DashMap<String, Option<Arc<Function>>>> =
    LazyLock::new(DashMap::new);

/// A compilation request for the background batch compiler
#[derive(Debug)]
struct CompilationRequest {
    vm: Arc<VM>,
    class: Arc<Class>,
    method: Method,
    response_sender: oneshot::Sender<Result<Option<Arc<Function>>>>,
}

/// A thread-safe global compilation queue for batch processing
static COMPILATION_QUEUE: LazyLock<Mutex<Option<mpsc::UnboundedSender<CompilationRequest>>>> =
    LazyLock::new(|| Mutex::new(None));

/// A thread-safe global set to track methods currently being compiled
#[expect(clippy::type_complexity)]
static PENDING_COMPILATIONS: LazyLock<
    DashMap<String, Vec<oneshot::Sender<Result<Option<Arc<Function>>>>>>,
> = LazyLock::new(DashMap::new);

/// Attempts to compile a Java method to native code using the Just-In-Time compiler. It first
/// checks if the method has already been compiled and cached, and returns the cached function if
/// available. Otherwise, it attempts to compile the method and caches the result.
///
/// # Caching
///
/// Successfully compiled functions are cached in order to avoid recompilation. Failed compilation
/// attempts are also cached (as `None`) to avoid retrying incompatible methods.
///
/// # Background Compilation
///
/// When batch compilation is enabled, methods are queued for background compilation and this
/// function returns `None` immediately while compilation proceeds asynchronously. Subsequent calls
/// for the same method will return the compiled function once ready.
pub(crate) async fn compile(
    vm: &Arc<VM>,
    class: &Arc<Class>,
    method: &Method,
) -> Result<Option<Arc<Function>>> {
    if vm.configuration().interpreted() {
        return Ok(None);
    }

    let class_name = class.name();
    let method_name = method.name();
    let method_descriptor = method.descriptor();
    let fully_qualified_method_name = format!("{class_name}.{method_name}{method_descriptor}");

    if let Some(function) = FUNCTION_CACHE.get(&fully_qualified_method_name) {
        debug!("Using cached function for {fully_qualified_method_name}");
        return Ok(function.clone());
    }

    // If batch compilation is disabled, compile synchronously (original behavior)
    if !vm.configuration().batch_compilation() {
        return compile_method(vm, class, method, &fully_qualified_method_name, false).await;
    }

    // If the method is already being compiled, return None to indicate it's not ready yet
    if PENDING_COMPILATIONS.contains_key(&fully_qualified_method_name) {
        debug!("Method {fully_qualified_method_name} is already being compiled in background");
        return Ok(None);
    }

    // Initialize the batch compiler if not already done
    initialize_batch_compiler().await;

    // Mark this method as pending compilation
    PENDING_COMPILATIONS.insert(fully_qualified_method_name.clone(), Vec::new());

    // Send the compilation request to the background compiler
    let (response_sender, _response_receiver) = oneshot::channel();
    let request = CompilationRequest {
        vm: vm.clone(),
        class: class.clone(),
        method: method.clone(),
        response_sender,
    };

    if let Some(queue) = COMPILATION_QUEUE.lock().await.as_ref() {
        let _ = queue.send(request);
    }

    debug!("Queued method {fully_qualified_method_name} for background compilation");

    // Return None immediately to indicate the function is not ready yet
    Ok(None)
}

/// Shared method compilation logic with optional pending compilation notification
async fn compile_method(
    vm: &Arc<VM>,
    class: &Arc<Class>,
    method: &Method,
    fully_qualified_method_name: &str,
    notify_pending: bool,
) -> Result<Option<Arc<Function>>> {
    let Some(compiler) = vm.compiler() else {
        return Ok(None);
    };

    let class_file = class.class_file();
    let definition = method.definition();

    let function = match compiler.compile(class_file, definition) {
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
            if notify_pending {
                if let Some((_, pending_senders)) =
                    PENDING_COMPILATIONS.remove(fully_qualified_method_name)
                {
                    for sender in pending_senders {
                        let _ =
                            sender.send(Err(InternalError("JIT compilation failed".to_string())));
                    }
                }
            }

            return Err(err);
        }
    };

    // Notify any pending waiters for this method if requested
    if notify_pending {
        if let Some((_, pending_senders)) = PENDING_COMPILATIONS.remove(fully_qualified_method_name)
        {
            for sender in pending_senders {
                let _ = sender.send(Ok(function.clone()));
            }
        }
    }

    FUNCTION_CACHE.insert(fully_qualified_method_name.to_string(), function.clone());
    Ok(function)
}

/// Initialize the background batch compilation system
async fn initialize_batch_compiler() {
    let mut queue = COMPILATION_QUEUE.lock().await;
    if queue.is_some() {
        // Already initialized
        return;
    }

    let (sender, mut receiver) = mpsc::unbounded_channel::<CompilationRequest>();
    *queue = Some(sender);
    drop(queue);

    // Spawn the background compilation task
    tokio::spawn(
        async move {
            let mut batch = Vec::new();
            const BATCH_SIZE: usize = 10;
            const BATCH_TIMEOUT_MS: u64 = 50;

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
                    _ = &mut timeout => {
                        should_process = !batch.is_empty();
                    }
                }

                // Process the batch if we have requests and either hit batch size or timeout
                if should_process {
                    process_compilation_batch(&mut batch).await;
                }
            }
        }
        .instrument(info_span!("jit")),
    );
}

/// Process a batch of compilation requests
async fn process_compilation_batch(batch: &mut Vec<CompilationRequest>) {
    debug!("Processing compilation batch of {} methods", batch.len());

    // Process all requests in the batch
    for request in batch.drain(..) {
        // Move the compilation logic to a blocking task since it may involve significant
        // computation
        let handle = tokio::task::spawn_blocking(async move || {
            let class_name = request.class.name();
            let method_name = request.method.name();
            let method_descriptor = request.method.descriptor();
            let fully_qualified_method_name =
                format!("{class_name}.{method_name}{method_descriptor}");
            let result = compile_method(
                &request.vm,
                &request.class,
                &request.method,
                &fully_qualified_method_name,
                true,
            )
            .await;
            let _ = request.response_sender.send(result);
        });
        if let Err(error) = handle.await {
            error!("Failed to join compilation task: {error}");
        }
    }
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
}

use crate::Error::InternalError;
use crate::{JitValue, Result, Value};
#[cfg(not(target_family = "wasm"))]
use cranelift::jit::JITModule;
use std::fmt::{Debug, Formatter};
#[cfg(not(target_family = "wasm"))]
use std::sync::{Arc, Mutex};

/// A structure representing a native function from the JIT compiler
#[derive(Clone)]
pub struct Function {
    function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
    #[cfg(not(target_family = "wasm"))]
    module: Arc<JitModuleOwner>,
}

#[cfg(not(target_family = "wasm"))]
struct JitModuleOwner {
    module: Mutex<Option<JITModule>>,
}

#[cfg(not(target_family = "wasm"))]
impl JitModuleOwner {
    fn new(module: Option<JITModule>) -> Self {
        Self {
            module: Mutex::new(module),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Drop for JitModuleOwner {
    fn drop(&mut self) {
        let module = match self.module.get_mut() {
            Ok(module) => module,
            Err(error) => error.into_inner(),
        };
        if let Some(module) = module.take() {
            // Safety: dropping the final owner means no function pointer from this
            // one-function module can still be executing or called again.
            unsafe { module.free_memory() };
        }
    }
}

impl Debug for Function {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Function")
            .field("function", &(self.function as *const ()))
            .finish_non_exhaustive()
    }
}

impl Function {
    /// Creates a new function instance
    #[cfg(test)]
    pub(crate) fn new(
        function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
    ) -> Self {
        Self {
            function,
            #[cfg(not(target_family = "wasm"))]
            module: Arc::new(JitModuleOwner::new(None)),
        }
    }

    /// Creates a function backed by an owning JIT module.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn with_module(
        function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
        module: JITModule,
    ) -> Self {
        Self {
            function,
            module: Arc::new(JitModuleOwner::new(Some(module))),
        }
    }

    /// Calls the function with the given arguments and a runtime context pointer.
    ///
    /// The `context` pointer is passed to the compiled function as the 4th argument.
    /// It typically points to a runtime context struct (e.g., the garbage collector)
    /// needed by runtime helper functions called from JIT-compiled code.
    ///
    /// # Errors
    ///
    /// if the function fails to execute
    ///
    /// The context pointer is treated as opaque by generated code and passed only to the runtime
    /// helpers registered by the VM.
    #[expect(
        clippy::not_unsafe_ptr_arg_deref,
        reason = "the JIT crate owns the validated generated-code ABI boundary"
    )]
    pub fn execute(&self, arguments: &[Value], context: *const u8) -> Result<Option<Value>> {
        // Use a stack-allocated buffer for small argument lists (most JVM methods have 0-5 args)
        let mut stack_buf = [JitValue::new(); 8];
        let mut result = JitValue::new();
        if arguments.len() <= stack_buf.len() {
            for (i, arg) in arguments.iter().enumerate() {
                let slot = stack_buf
                    .get_mut(i)
                    .ok_or_else(|| InternalError(format!("Invalid stack argument index {i}")))?;
                *slot = JitValue::from(arg.clone());
            }
            // Safety: the arguments and result use the ABI emitted by the owning JIT module,
            // and all pointers remain valid for the duration of the call.
            unsafe {
                (self.function)(
                    stack_buf.as_ptr(),
                    arguments.len(),
                    &raw mut result,
                    context,
                );
            }
        } else {
            let heap_args: Vec<JitValue> = arguments.iter().cloned().map(JitValue::from).collect();
            // Safety: the arguments and result use the ABI emitted by the owning JIT module,
            // and all pointers remain valid for the duration of the call.
            unsafe {
                (self.function)(
                    heap_args.as_ptr(),
                    heap_args.len(),
                    &raw mut result,
                    context,
                );
            }
        }
        result.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JitValue;

    unsafe extern "C" fn return_42(
        _arguments: *const JitValue,
        _arguments_len: usize,
        jit_value: *mut JitValue,
        _context: *const u8,
    ) {
        // Safety: the test passes a valid, aligned result pointer.
        unsafe {
            *jit_value = JitValue::from(42i64);
        }
    }

    #[test]
    fn test_function() -> Result<()> {
        let function = Function::new(return_42);
        assert!(format!("{function:?}").contains("Function"));
        let cloned_function = function.clone();
        drop(function);
        let value = cloned_function
            .execute(&[], std::ptr::null())?
            .expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }

    #[test]
    fn test_function_with_heap_arguments() -> Result<()> {
        let function = Function::new(return_42);
        let arguments = vec![Value::I32(1); 9];
        let value = function
            .execute(&arguments, std::ptr::null())?
            .expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }
}

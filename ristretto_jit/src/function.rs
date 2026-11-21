use crate::Error::InternalError;
use crate::{JitValue, Result, Value};
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
use cranelift::jit::JITModule;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
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
use std::sync::Mutex;

/// A structure representing a native function from the JIT compiler
#[derive(Clone)]
pub struct Function {
    argument_types: Arc<[i8]>,
    native_stack_bytes: usize,
    entry_point: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
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
    module: Arc<JitModuleOwner>,
}

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
struct JitModuleOwner {
    module: Mutex<Option<JITModule>>,
}

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
impl JitModuleOwner {
    fn new(module: Option<JITModule>) -> Self {
        Self {
            module: Mutex::new(module),
        }
    }
}

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
            .field("function", &(self.entry_point as *const ()))
            .finish_non_exhaustive()
    }
}

impl Function {
    /// Native space reserved beyond the generated frame for runtime transitions.
    pub const STACK_RESERVE_BYTES: usize = 256 * 1024;

    /// Backend frame span plus ABI setup, including spills and outgoing arguments.
    #[must_use]
    pub fn native_stack_bytes(&self) -> usize {
        self.native_stack_bytes
    }

    /// Creates a new function instance
    #[cfg(test)]
    pub(crate) fn new(
        function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
        argument_types: &[i8],
    ) -> Self {
        Self {
            argument_types: Arc::from(argument_types),
            native_stack_bytes: 0,
            entry_point: function,
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
            module: Arc::new(JitModuleOwner::new(None)),
        }
    }

    /// Creates a function backed by an owning JIT module.
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
    pub(crate) fn with_module(
        function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
        module: JITModule,
        native_stack_bytes: usize,
        argument_types: Vec<i8>,
    ) -> Self {
        Self {
            argument_types: Arc::from(argument_types),
            native_stack_bytes,
            entry_point: function,
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
        if arguments.len() != self.argument_types.len() {
            return Err(crate::Error::InvalidArgumentCount {
                expected: self.argument_types.len(),
                actual: arguments.len(),
            });
        }
        // Use a stack-allocated buffer for small argument lists (most JVM methods have 0-5 args)
        let mut stack_buf = [JitValue::new(); 8];
        let mut result = JitValue::new();
        if arguments.len() <= stack_buf.len() {
            for (i, (arg, expected)) in arguments.iter().zip(self.argument_types.iter()).enumerate()
            {
                let slot = stack_buf
                    .get_mut(i)
                    .ok_or_else(|| InternalError(format!("Invalid stack argument index {i}")))?;
                *slot = JitValue::argument(arg, *expected, i)?;
            }
            self.check_native_stack()?;
            // Safety: the arguments and result use the ABI emitted by the owning JIT module,
            // and all pointers remain valid for the duration of the call.
            unsafe {
                (self.entry_point)(
                    stack_buf.as_ptr(),
                    arguments.len(),
                    &raw mut result,
                    context,
                );
            }
        } else {
            let heap_args: Vec<JitValue> = arguments
                .iter()
                .zip(self.argument_types.iter())
                .enumerate()
                .map(|(index, (arg, expected))| JitValue::argument(arg, *expected, index))
                .collect::<Result<_>>()?;
            self.check_native_stack()?;
            // Safety: the arguments and result use the ABI emitted by the owning JIT module,
            // and all pointers remain valid for the duration of the call.
            unsafe {
                (self.entry_point)(
                    heap_args.as_ptr(),
                    heap_args.len(),
                    &raw mut result,
                    context,
                );
            }
        }
        result.try_into()
    }
    #[cfg_attr(
        any(
            target_family = "wasm",
            target_endian = "big",
            target_arch = "mips",
            target_arch = "mips64",
            target_os = "dragonfly",
            target_os = "solaris"
        ),
        expect(
            clippy::unused_self,
            clippy::unnecessary_wraps,
            reason = "these targets cannot construct native compiled functions; fixed Rust ABI test stubs still execute"
        )
    )]
    fn check_native_stack(&self) -> Result<()> {
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
        {
            let available =
                stacker::remaining_stack().ok_or(crate::Error::NativeStackUnavailable)?;
            let required = self
                .native_stack_bytes
                .checked_add(Self::STACK_RESERVE_BYTES)
                .ok_or_else(|| InternalError("native stack requirement overflow".to_owned()))?;
            if available < required {
                return Err(crate::Error::NativeStackOverflow {
                    required,
                    available,
                });
            }
        }
        Ok(())
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
        let cloned_function = {
            let function = Function::new(return_42, &[]);
            assert!(format!("{function:?}").contains("Function"));
            function.clone()
        };
        let value = cloned_function
            .execute(&[], std::ptr::null())?
            .expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }

    #[test]
    fn test_function_with_heap_arguments() -> Result<()> {
        let function = Function::new(return_42, &[crate::jit_value::I32; 9]);
        let arguments = vec![Value::I32(1); 9];
        let value = function
            .execute(&arguments, std::ptr::null())?
            .expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }
}

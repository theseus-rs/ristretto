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
#[path = "activation.rs"]
pub(crate) mod activation;
pub(crate) use activation::ContinuationLayout;
pub use activation::{
    Activation, AllocationKind, AllocationRequest, ArrayAccessKind, ArrayAccessRequest,
    ArrayStoreRequest, CallKind, CallRequest, ConstantRequest, ExceptionMatchRequest,
    ExecutionStep, FieldAccess, FieldRequest, MonitorKind, MonitorRequest, TypeCheckKind,
    TypeCheckRequest,
};
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
    entry_point: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
    native_stack_bytes: usize,
    argument_types: Arc<[i8]>,
    continuation: Option<ContinuationLayout>,
    #[cfg(feature = "execution-profile")]
    entries: Arc<portable_atomic::AtomicU64>,
    #[cfg(feature = "execution-profile")]
    resumes: Arc<portable_atomic::AtomicU64>,
    #[cfg(feature = "execution-profile")]
    backedges: Arc<portable_atomic::AtomicU64>,
    #[cfg(feature = "execution-profile")]
    code_bytes: usize,
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
            .field("native_stack_bytes", &self.native_stack_bytes)
            .field("argument_types", &self.argument_types)
            .finish_non_exhaustive()
    }
}

impl Function {
    const COOPERATIVE_INSTRUCTION_BUDGET: usize = 4096;
    pub(crate) const INLINE_ARGUMENT_CAPACITY: usize = 8;

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
            entry_point: function,
            native_stack_bytes: 0,
            argument_types: Arc::from(argument_types),
            continuation: None,
            #[cfg(feature = "execution-profile")]
            entries: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            resumes: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            backedges: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            code_bytes: 0,
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
            entry_point: function,
            native_stack_bytes,
            argument_types: Arc::from(argument_types),
            continuation: None,
            #[cfg(feature = "execution-profile")]
            entries: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            resumes: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            backedges: Arc::new(portable_atomic::AtomicU64::new(0)),
            #[cfg(feature = "execution-profile")]
            code_bytes: 0,
            module: Arc::new(JitModuleOwner::new(Some(module))),
        }
    }

    #[cfg(feature = "execution-profile")]
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
    pub(crate) fn with_profile(
        mut self,
        bytes: usize,
        backedges: Arc<portable_atomic::AtomicU64>,
    ) -> Self {
        self.code_bytes = bytes;
        self.backedges = backedges;
        self
    }

    /// Actual invocation entries, excluding continuation resumes; shared by clones.
    #[cfg(feature = "execution-profile")]
    #[must_use]
    pub fn execution_count(&self) -> u64 {
        self.entries.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Reentries of an invocation after a cooperative yield. Diagnostic builds only.
    #[cfg(feature = "execution-profile")]
    #[must_use]
    pub fn resume_count(&self) -> u64 {
        self.resumes.load(std::sync::atomic::Ordering::Relaxed)
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
    pub(crate) fn with_continuation(mut self, layout: Option<ContinuationLayout>) -> Self {
        self.continuation = layout;
        self
    }

    /// Whether this function can return and resume at a loop backedge, handler entry or Java call.
    #[must_use]
    pub fn supports_cooperation(&self) -> bool {
        self.continuation.is_some()
    }

    /// Whether all heap and Java call operations return owning runtime requests.
    /// Synchronous execution is unavailable for this ABI.
    #[must_use]
    pub fn requests_only(&self) -> bool {
        self.continuation.is_some_and(|layout| layout.requests_only)
    }

    /// Whether resumable generated code needs an owning runtime context and roots.
    #[must_use]
    pub fn cooperation_requires_context(&self) -> bool {
        self.continuation
            .is_some_and(|layout| layout.requires_context)
    }

    /// Conservative slice budget: at most 4096 + method instruction count bytecode
    /// visits between returns. Saving/restoring the declared locals is additional
    /// bounded work; this is not a wall-time guarantee.
    #[must_use]
    pub fn cooperation_budget(&self) -> std::num::NonZeroU32 {
        let instructions = self
            .continuation
            .map_or(1, |layout| layout.instructions.max(1));
        let budget = u32::try_from((Self::COOPERATIVE_INSTRUCTION_BUDGET / instructions).max(1))
            .unwrap_or(1);
        std::num::NonZeroU32::new(budget).unwrap_or(std::num::NonZeroU32::MIN)
    }

    /// Begin an owning native continuation, without entering generated code.
    /// # Errors
    /// Rejects functions without the continuation ABI and invalid arguments.
    pub fn activation(&self, arguments: &[Value]) -> Result<Activation> {
        Activation::new(self, arguments)
    }

    /// Taken explicit branches to the same or an earlier Java bytecode offset.
    /// Counts selected conditional/switch edges, not inferred loop iterations.
    #[cfg(feature = "execution-profile")]
    #[must_use]
    pub fn backedge_count(&self) -> u64 {
        self.backedges.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Generated code bytes, excluding module metadata and executable-page padding.
    #[cfg(feature = "execution-profile")]
    #[must_use]
    pub fn code_bytes(&self) -> usize {
        self.code_bytes
    }

    /// Calls the function with the given arguments and a runtime context pointer.
    ///
    /// Noncooperative code receives `context` as its fourth argument. Cooperative
    /// bodies receive private storage containing the context for that slice. This
    /// synchronous API runs every slice and invokes call helpers to completion;
    /// use [`Self::activation`]
    /// when the caller needs scheduling or cancellation boundaries.
    ///
    /// # Errors
    ///
    /// Returns an error before entry if the argument count or ABI value kinds do
    /// not match the descriptor, or native stack space is insufficient. Wide JVM
    /// arguments occupy one value; an instance receiver occupies the first value.
    /// Reference validity and runtime-context lifetime remain the embedding's contract.
    ///
    /// The context pointer is treated as opaque by generated code and passed only to the runtime
    /// helpers registered by the VM.
    #[expect(
        clippy::not_unsafe_ptr_arg_deref,
        reason = "the JIT crate owns the validated generated-code ABI boundary"
    )]
    pub fn execute(&self, arguments: &[Value], context: *const u8) -> Result<Option<Value>> {
        if self.requests_only() {
            return Err(InternalError(
                "request-only code requires activation dispatch".to_owned(),
            ));
        }
        if self.supports_cooperation() {
            // Synchronous embeddings explicitly run to completion. The VM uses
            // Activation directly and yields its executor between native slices.
            let mut activation = self.activation(arguments)?;
            activation.synchronous_calls()?;
            loop {
                match activation.resume_with_context(self.cooperation_budget(), context)? {
                    ExecutionStep::Returned(value) => return Ok(value),
                    ExecutionStep::Yielded => {}
                    ExecutionStep::Call(_)
                    | ExecutionStep::Monitor(_)
                    | ExecutionStep::Allocation(_)
                    | ExecutionStep::Field(_)
                    | ExecutionStep::TypeCheck(_)
                    | ExecutionStep::Constant(_)
                    | ExecutionStep::ExceptionMatch(_)
                    | ExecutionStep::ArrayStore(_)
                    | ExecutionStep::ArrayAccess(_) => {
                        return Err(InternalError(
                            "synchronous execution returned a call request".to_owned(),
                        ));
                    }
                }
            }
        }
        if arguments.len() != self.argument_types.len() {
            return Err(crate::Error::InvalidArgumentCount {
                expected: self.argument_types.len(),
                actual: arguments.len(),
            });
        }
        // Use a stack-allocated buffer for small argument lists (most JVM methods have 0-5 args)
        let mut stack_buf = [JitValue::new(); Self::INLINE_ARGUMENT_CAPACITY];
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
            #[cfg(feature = "execution-profile")]
            self.entries
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
            let mut heap_args = Vec::with_capacity(arguments.len());
            for (index, (argument, expected)) in
                arguments.iter().zip(self.argument_types.iter()).enumerate()
            {
                heap_args.push(JitValue::argument(argument, *expected, index)?);
            }
            self.check_native_stack()?;
            #[cfg(feature = "execution-profile")]
            self.entries
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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

    #[expect(
        clippy::cast_ptr_alignment,
        reason = "context comes from a live aligned AtomicUsize"
    )]
    unsafe extern "C" fn count_entry(
        arguments: *const JitValue,
        arguments_len: usize,
        result: *mut JitValue,
        context: *const u8,
    ) {
        // SAFETY: these tests keep their counter and result alive for the call.
        unsafe {
            let counter = &*context.cast::<std::sync::atomic::AtomicUsize>();
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return_42(arguments, arguments_len, result, context);
        }
    }

    #[test]
    fn argument_count_rejection_precedes_stack_check_and_native_entry() -> Result<()> {
        use crate::Error::InvalidArgumentCount;
        use crate::jit_value::I32;
        use std::sync::atomic::{AtomicUsize, Ordering};
        for count in [0_usize, 1, 8, 9, 10, 255] {
            let counter = AtomicUsize::new(0);
            let context = std::ptr::from_ref(&counter).cast::<u8>();
            let mut function = Function::new(count_entry, &vec![I32; count]);
            function.native_stack_bytes = usize::MAX;
            for actual in [0, count.saturating_sub(1), count + 1] {
                if actual == count {
                    continue;
                }
                assert!(matches!(
                    function.execute(&vec![Value::I32(1); actual], context),
                    Err(InvalidArgumentCount { expected, actual: found })
                        if expected == count && found == actual
                ));
            }
            assert_eq!(counter.load(Ordering::Relaxed), 0);
            #[cfg(feature = "execution-profile")]
            assert_eq!(function.execution_count(), 0);
            function.native_stack_bytes = 0;
            let cloned = function.clone();
            for function in [&function, &cloned] {
                assert_eq!(
                    function.execute(&vec![Value::I32(1); count], context)?,
                    Some(Value::I64(42))
                );
            }
            assert_eq!(counter.load(Ordering::Relaxed), 2);
            #[cfg(feature = "execution-profile")]
            assert_eq!(function.execution_count(), 2);
        }
        Ok(())
    }

    #[test]
    fn argument_kinds_are_checked_in_both_buffers_before_entry() -> Result<()> {
        use crate::Error::InvalidArgumentType;
        use crate::jit_value::{F32, F64, I32, I64, PTR};
        use std::sync::atomic::{AtomicUsize, Ordering};
        let values = [
            (I32, Value::I32(42)),
            (I64, Value::I64(42)),
            (F32, Value::F32(42.0)),
            (F64, Value::F64(42.0)),
            (PTR, Value::Ptr(0)),
        ];
        for prefix in [0, 8] {
            for (expected, valid) in &values {
                let counter = AtomicUsize::new(0);
                let context = std::ptr::from_ref(&counter).cast::<u8>();
                let types: Vec<_> = std::iter::repeat_n(I32, prefix)
                    .chain(std::iter::once(*expected))
                    .collect();
                let mut function = Function::new(count_entry, &types);
                function.native_stack_bytes = usize::MAX;
                for (actual, value) in &values {
                    if actual == expected {
                        continue;
                    }
                    let mut arguments = vec![Value::I32(1); prefix];
                    arguments.push(value.clone());
                    assert!(matches!(
                        function.execute(&arguments, context),
                        Err(InvalidArgumentType { index, expected: tag, actual: found })
                            if index == prefix && tag == *expected && found == *actual
                    ));
                }
                assert_eq!(counter.load(Ordering::Relaxed), 0);
                #[cfg(feature = "execution-profile")]
                assert_eq!(function.execution_count(), 0);
                function.native_stack_bytes = 0;
                let mut arguments = vec![Value::I32(1); prefix];
                arguments.push(valid.clone());
                assert_eq!(function.execute(&arguments, context)?, Some(Value::I64(42)));
                assert_eq!(counter.load(Ordering::Relaxed), 1);
            }
        }
        Ok(())
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

    #[cfg(feature = "execution-profile")]
    #[test]
    fn entry_counts_include_clones_and_both_argument_buffers() -> Result<()> {
        for count in [0, 9] {
            let function = Function::new(return_42, &vec![crate::jit_value::I32; count]);
            let cloned = function.clone();
            assert_eq!(function.execution_count(), 0);
            for _ in 0..2 {
                assert_eq!(
                    cloned.execute(&vec![Value::I32(1); count], std::ptr::null())?,
                    Some(Value::I64(42))
                );
            }
            assert_eq!(function.execution_count(), 2);
            assert_eq!(cloned.execution_count(), 2);
        }
        Ok(())
    }
}

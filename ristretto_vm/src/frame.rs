use crate::Error::{InternalError, InvalidProgramCounter};

use crate::field_ref_cache::FieldRefEntry;
use crate::instruction::{
    aaload, aastore, aconst_null, aload, aload_0, aload_1, aload_2, aload_3, aload_w, anewarray,
    areturn, arraylength, astore, astore_0, astore_1, astore_2, astore_3, astore_w, athrow, baload,
    bastore, bipush, breakpoint, caload, castore, checkcast, convert_error_to_throwable, d2f, d2i,
    d2l, dadd, daload, dastore, dcmpg, dcmpl, dconst_0, dconst_1, ddiv, dload, dload_0, dload_1,
    dload_2, dload_3, dload_w, dmul, dneg, drem, dreturn, dstore, dstore_0, dstore_1, dstore_2,
    dstore_3, dstore_w, dsub, dup, dup_x1, dup_x2, dup2, dup2_x1, dup2_x2, f2d, f2i, f2l, fadd,
    faload, fastore, fcmpg, fcmpl, fconst_0, fconst_1, fconst_2, fdiv, fload, fload_0, fload_1,
    fload_2, fload_3, fload_w, fmul, fneg, frem, freturn, fstore, fstore_0, fstore_1, fstore_2,
    fstore_3, fstore_w, fsub, getfield, getstatic, goto_w, i2b, i2c, i2d, i2f, i2l, i2s, iadd,
    iaload, iand, iastore, iconst_0, iconst_1, iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1,
    idiv, if_acmpeq, if_acmpne, if_icmpeq, if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne,
    ifeq, ifge, ifgt, ifle, iflt, ifne, ifnonnull, ifnull, iinc, iinc_w, iload, iload_0, iload_1,
    iload_2, iload_3, iload_w, impdep1, impdep2, imul, ineg, instanceof, invokedynamic,
    invokeinterface, invokespecial, invokestatic, invokevirtual, ior, irem, ireturn, ishl, ishr,
    istore, istore_0, istore_1, istore_2, istore_3, istore_w, isub, iushr, ixor, jsr, jsr_w, l2d,
    l2f, l2i, ladd, laload, land, lastore, lcmp, lconst_0, lconst_1, ldc, ldc_w, ldc2_w, ldiv,
    lload, lload_0, lload_1, lload_2, lload_3, lload_w, lmul, lneg, lookupswitch, lor, lrem,
    lreturn, lshl, lshr, lstore, lstore_0, lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr,
    lxor, monitorenter, monitorexit, multianewarray, new, newarray, nop, pop, pop2,
    process_throwable, putfield, putstatic, ret, ret_w, r#return, saload, sastore, sipush, swap,
    tableswitch, wide,
};
use crate::method_ref_cache::MethodRefEntry;
use crate::reference_cache::ClassReferences;
use crate::{LocalVariables, OperandStack, Result, Thread};
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_classloader::{Class, Method, Value};
use ristretto_types::JavaError::StackOverflowError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, Weak};
use tracing::{Level, debug, event_enabled};

/// Maximum number of bytecodes executed in one interpreter batch.
pub(crate) const INSTRUCTION_BATCH_SIZE: usize = 256;

/// Whether a call target has already been bound to its VM's intrinsic registry.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum IntrinsicBinding {
    /// Host calls and other uncached paths still require a lookup.
    #[default]
    Unresolved,
    /// Cached implementation or a cached miss for the actual target.
    Resolved(Option<crate::intrinsic_methods::IntrinsicMethod>),
}

/// A resolved Java method invocation that the thread trampoline must dispatch.
#[derive(Debug)]
pub(crate) struct MethodCall {
    pub(crate) class: Arc<Class>,
    pub(crate) method: Arc<Method>,
    /// Intrinsic selected for this VM and actual dispatch target.
    pub(crate) intrinsic: IntrinsicBinding,
    pub(crate) parameters: CallParameters,
    pub(crate) has_return_type: bool,
}

impl PartialEq for MethodCall {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.class, &other.class)
            && Arc::ptr_eq(&self.method, &other.method)
            && self.parameters == other.parameters
            && self.has_return_type == other.has_return_type
    }
}

/// Represents the result of executing a JVM bytecode instruction.
///
/// # Overview
///
/// Frame execution returns this enum at control-flow boundaries. Ordinary synchronous opcode
/// handlers return `Result<()>`; dispatch advances them directly without constructing this enum.
///
/// # Variants
///
/// - `Return(Option<Value>)`: Indicates that the method execution should terminate and return the
///   specified value (if any) to the caller. Methods with a void return type use `Return(None)`.
///
/// - `Continue`: Indicates that execution should continue with the next instruction (increments
///   the program counter by the encoded instruction length).
///
/// # Usage
///
/// The VM's execution loop uses this enum to determine whether to proceed to the next instruction,
/// jump to a different instruction, or return from the current method.
#[derive(Debug, PartialEq)]
pub(crate) enum ExecutionResult {
    Return(Option<Value>),
    Continue,
    /// The already-executed opcode selected this decoded cursor; do not advance it again.
    ContinueAtInstruction(usize),
    Call(MethodCall),
}

/// Async work is represented as bytecode data; no future is allocated by dispatch.
#[derive(Debug)]
pub(crate) enum InstructionResult {
    Sync(ExecutionResult),
    Async(Instruction),
}

/// Synchronous dispatch crosses into the async interpreter only at these boundaries.
#[derive(Debug)]
pub(crate) enum BatchExit {
    Complete(ExecutionResult),
    Async(Instruction),
    Yield,
    Fault(crate::Error),
}

/// A frame is created each time a method is invoked in the JVM.
///
/// # Overview
/// A frame stores data and partial results, performs dynamic linking, returns method values, and
/// dispatches exceptions. Each frame has its own array of local variables, its own operand stack,
/// and a reference to the runtime constant pool of the class of the current method.
///
/// # Fields
/// - `thread`: A weak reference to the thread that owns this frame
/// - `class`: Reference to the class that contains the method being executed
/// - `method`: Reference to the method being executed
/// - `instruction_index`: Internal execution cursor; public program counters are byte offsets
///
/// # Execution Model
/// When a method is invoked, a new frame is created and pushed onto the JVM stack of the invoking
/// thread. When the method completes (normally or abruptly), the frame is popped, and the invoker's
/// frame becomes the current frame.
///
/// # Stack Effects
/// The frame maintains two key data structures:
/// - Local Variables: Used to pass parameters to methods and store local variables
/// - Operand Stack: Used for storing intermediate computation results
///
/// # References
///
/// - [JVMS §2.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6)
#[derive(Debug)]
pub struct Frame {
    thread: Weak<Thread>,
    class: Arc<Class>,
    method: Arc<Method>,
    instruction_index: AtomicUsize,
    invalid_program_counter: AtomicUsize,
    method_refs: OnceLock<Arc<ClassReferences<MethodRefEntry>>>,
    field_refs: OnceLock<Arc<ClassReferences<FieldRefEntry>>>,
}

#[derive(Debug)]
pub(crate) struct FrameState {
    pub(crate) locals: LocalVariables,
    pub(crate) stack: OperandStack,
}

impl Default for FrameState {
    fn default() -> Self {
        Self {
            locals: LocalVariables::with_max_size(0),
            stack: OperandStack::with_max_size(0),
        }
    }
}

impl FrameState {
    #[inline]
    pub(crate) fn reset(
        &mut self,
        class: &Class,
        method: &Method,
        parameters: impl IntoIterator<Item = Value>,
    ) -> Result<()> {
        self.stack.reset(method.max_stack());
        let slots = self.locals.reset(parameters, method.max_locals());
        if slots > method.max_locals() {
            return Err(InternalError(format!(
                "Method parameters require {slots} local-variable slots, but {}.{}{} declares {}",
                class.name(),
                method.name(),
                method.descriptor(),
                method.max_locals()
            )));
        }
        Ok(())
    }

    pub(crate) fn clear(&mut self) {
        self.locals.clear();
        self.stack.clear();
    }

    pub(crate) fn capacity(&self) -> usize {
        self.locals.capacity() + self.stack.capacity()
    }
}

/// Cached calls leave arguments on the caller's stack until the dispatch boundary.
#[derive(Debug, PartialEq)]
pub(crate) enum CallParameters {
    Owned(Vec<Value>),
    Stack(usize),
}

impl From<Vec<Value>> for CallParameters {
    fn from(values: Vec<Value>) -> Self {
        Self::Owned(values)
    }
}

impl CallParameters {
    pub(crate) fn as_slice<'a>(&'a self, caller: Option<&'a FrameState>) -> Result<&'a [Value]> {
        match self {
            Self::Owned(values) => Ok(values),
            Self::Stack(count) => Self::caller(caller)?.stack.last_values(*count),
        }
    }

    fn caller(caller: Option<&FrameState>) -> Result<&FrameState> {
        caller.ok_or_else(|| InternalError("Missing caller for stack arguments".to_owned()))
    }

    pub(crate) fn into_values(self, caller: Option<&mut FrameState>) -> Result<Vec<Value>> {
        match self {
            Self::Owned(values) => Ok(values),
            Self::Stack(count) => {
                let caller = caller.ok_or_else(|| {
                    InternalError("Missing caller for stack arguments".to_owned())
                })?;
                Ok(caller.stack.drain_values(count)?.collect())
            }
        }
    }

    /// Transfer ownership directly into inline intrinsic argument storage.
    pub(crate) fn into_parameters(
        self,
        caller: Option<&mut FrameState>,
    ) -> Result<crate::Parameters> {
        match self {
            Self::Owned(values) => Ok(crate::Parameters::new(values)),
            Self::Stack(count) => {
                let caller = caller.ok_or_else(|| {
                    InternalError("Missing caller for stack arguments".to_owned())
                })?;
                Ok(caller.stack.drain_values(count)?.collect())
            }
        }
    }

    #[inline]
    pub(crate) fn initialize(
        self,
        state: &mut FrameState,
        class: &Class,
        method: &Method,
        caller: Option<&mut FrameState>,
    ) -> Result<()> {
        match self {
            Self::Owned(values) => state.reset(class, method, values),
            Self::Stack(count) => {
                let caller = caller.ok_or_else(|| {
                    InternalError("Missing caller for stack arguments".to_owned())
                })?;
                state.reset(class, method, caller.stack.drain_values(count)?)
            }
        }
    }
}

impl Frame {
    /// Create a metadata-only frame for the specified class and method.
    ///
    /// JIT execution uses this frame for stack walking without allocating interpreter locals or an
    /// operand stack. The thread owns interpreter storage separately.
    pub fn new(thread: &Weak<Thread>, class: &Arc<Class>, method: &Arc<Method>) -> Self {
        Frame {
            thread: thread.clone(),
            class: class.clone(),
            method: method.clone(),
            instruction_index: AtomicUsize::new(0),
            invalid_program_counter: AtomicUsize::new(0),
            method_refs: OnceLock::new(),
            field_refs: OnceLock::new(),
        }
    }

    /// Reuse metadata only while its Arc is unique, so stack-walker snapshots remain stable.
    pub(crate) fn reset(&mut self, class: &Arc<Class>, method: &Arc<Method>) {
        if !Arc::ptr_eq(&self.class, class) {
            self.method_refs.take();
            self.field_refs.take();
            self.class = class.clone();
        }
        if !Arc::ptr_eq(&self.method, method) {
            self.method = method.clone();
        }
        self.instruction_index.store(0, Ordering::Relaxed);
    }

    /// Get the thread that owns this frame.
    pub fn thread(&self) -> Result<Arc<Thread>> {
        match self.thread.upgrade() {
            Some(thread) => Ok(thread),
            None => Err(InternalError("Call stack is not available".to_string())),
        }
    }

    /// Get the class that owns this frame.
    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    /// Get a mutable reference to the class that owns this frame.
    pub fn class_mut(&mut self) -> &mut Arc<Class> {
        self.method_refs.take();
        self.field_refs.take();
        &mut self.class
    }

    #[inline]
    pub(crate) fn method_refs(&self) -> Result<&ClassReferences<MethodRefEntry>> {
        match self.method_refs.get() {
            Some(entries) => Ok(entries),
            None => self.initialize_method_refs(),
        }
    }

    #[cold]
    fn initialize_method_refs(&self) -> Result<&ClassReferences<MethodRefEntry>> {
        let entries = self
            .thread()?
            .vm()?
            .method_ref_cache()
            .for_class(&self.class);
        Ok(self.method_refs.get_or_init(|| entries))
    }

    #[inline]
    pub(crate) fn field_refs(&self) -> Result<&ClassReferences<FieldRefEntry>> {
        match self.field_refs.get() {
            Some(entries) => Ok(entries),
            None => self.initialize_field_refs(),
        }
    }

    #[cold]
    fn initialize_field_refs(&self) -> Result<&ClassReferences<FieldRefEntry>> {
        let entries = self
            .thread()?
            .vm()?
            .field_ref_cache()
            .for_class(&self.class);
        Ok(self.field_refs.get_or_init(|| entries))
    }

    /// Get the method in this frame.
    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    /// Get the current program counter in this frame.
    ///
    /// The program counter is the JVM byte offset in the code array of the method
    /// being executed. It's used to determine which instruction to execute next.
    #[inline]
    pub fn program_counter(&self) -> usize {
        self.method
            .instruction_offset(self.instruction_index.load(Ordering::Relaxed))
            .unwrap_or_else(|| self.invalid_program_counter.load(Ordering::Relaxed))
    }

    /// Set the current JVM byte offset in this frame. Invalid offsets are retained for
    /// diagnostics and rejected if interpreted execution resumes at them.
    #[inline]
    pub fn set_program_counter(&self, program_counter: usize) {
        let index = self
            .method
            .instruction_index(program_counter)
            .unwrap_or_else(|| {
                // Preserve an invalid requested PC for diagnostics without allowing execution
                // to mistake an operand byte for a decoded instruction index.
                self.invalid_program_counter
                    .store(program_counter, Ordering::Relaxed);
                usize::MAX
            });
        self.instruction_index.store(index, Ordering::Relaxed);
    }

    /// Execute the method in this frame
    ///
    /// # Overview
    ///
    /// This method runs the bytecode instructions of the current method in the frame.
    /// It sets up the local variables with the provided parameters, creates an operand
    /// stack, and executes each instruction in sequence until the method returns or
    /// an exception is thrown and not handled within the method.
    ///
    /// # Error Handling
    ///
    /// This method handles two types of errors:
    /// 1. Java exceptions: When a Java exception is thrown, the method attempts to find
    ///    an appropriate exception handler within the current method. If found, execution
    ///    continues at the handler. If not, the exception is propagated to the caller.
    /// 2. VM errors: Invalid program counter, stack overflow, etc. These are converted to
    ///    Java exceptions where possible.
    ///
    /// # Bytecode Execution Process
    ///
    /// 1. Initialize local variables with the provided parameters
    /// 2. Create an operand stack with the max stack size defined in the method
    /// 3. For each instruction:
    ///    - Load the instruction at the current program counter
    ///    - Execute the instruction, which may modify locals and stack
    ///    - Process the execution result (continue, jump, or return)
    pub async fn execute(&self, parameters: Vec<Value>) -> Result<Option<Value>> {
        let thread = self.thread()?;
        thread.execute(&self.class, &self.method, &parameters).await
    }

    /// Execute a bounded batch of bytecodes, returning early for method calls or returns.
    ///
    /// The interpreter owns mutable state across the batch, including across async handlers.
    /// The program counter remains current for exceptions and stack walking, and the thread's
    /// instruction budget preserves cooperative scheduling across batches and method calls.
    pub(crate) async fn execute_batch(
        &self,
        thread: &Thread,
        state: &mut FrameState,
    ) -> Result<ExecutionResult> {
        let debug = event_enabled!(Level::DEBUG);
        let mut remaining = INSTRUCTION_BATCH_SIZE;
        while remaining != 0 {
            let boundary = self.execute_sync_batch(thread, state, &mut remaining, debug)?;
            match self.resume_batch(state, boundary).await? {
                ExecutionResult::Continue => {}
                result => return Ok(result),
            }
        }
        Ok(ExecutionResult::Continue)
    }

    /// Resume a synchronous boundary with the stack unlocked, allowing Java reentry and
    /// cancellation to use the ordinary frame-state lease.
    pub(crate) async fn resume_batch(
        &self,
        state: &mut FrameState,
        boundary: BatchExit,
    ) -> Result<ExecutionResult> {
        let boundary = if matches!(boundary, BatchExit::Yield) {
            // This opcode already consumed budget before yielding. Resume through the same
            // loop for exactly one instruction without charging or logging it again.
            tokio::task::yield_now().await;
            let thread = self.thread()?;
            self.execute_sync_instructions::<false>(
                &mut thread.instruction_budget(),
                state,
                &mut 1,
                false,
            )?
        } else {
            boundary
        };
        let result = match boundary {
            BatchExit::Complete(ExecutionResult::ContinueAtInstruction(next)) => {
                self.instruction_index.store(next, Ordering::Relaxed);
                return Ok(ExecutionResult::Continue);
            }
            BatchExit::Complete(result) => return Ok(result),
            BatchExit::Async(instruction) => {
                // Loading/bootstrap work can re-enter Java execution. Keep the recursive
                // future boundary here, reached only after a synchronous cache miss.
                Box::pin(self.process_async(&mut state.stack, &instruction)).await
            }
            BatchExit::Fault(error) => Err(error),
            BatchExit::Yield => {
                return Err(InternalError("Unbudgeted dispatch yielded".to_owned()));
            }
        };

        match result {
            Ok(ExecutionResult::Continue) => {
                self.instruction_index.store(
                    self.instruction_index.load(Ordering::Relaxed) + 1,
                    Ordering::Relaxed,
                );
                Ok(ExecutionResult::Continue)
            }
            Ok(result) => Ok(result),
            Err(error) => {
                Self::handle_error_with_stack(self, &mut state.stack, error).await?;
                Ok(ExecutionResult::Continue)
            }
        }
    }

    pub(crate) fn execute_sync_batch(
        &self,
        thread: &Thread,
        state: &mut FrameState,
        remaining: &mut usize,
        debug: bool,
    ) -> Result<BatchExit> {
        self.execute_sync_batch_with_budget(
            &mut thread.instruction_budget(),
            state,
            remaining,
            debug,
        )
    }

    /// Keep ordinary bytecodes out of the async state machine. All opcodes use this dispatch
    /// loop; no instruction sequences or workload-specific patterns are recognized.
    #[inline]
    pub(crate) fn execute_sync_batch_with_budget(
        &self,
        budget: &mut crate::thread::InstructionBudget<'_>,
        state: &mut FrameState,
        remaining: &mut usize,
        debug: bool,
    ) -> Result<BatchExit> {
        self.execute_sync_instructions::<true>(budget, state, remaining, debug)
    }

    /// Share dispatch with yield resumption, which executes its already-budgeted instruction.
    #[expect(
        clippy::too_many_lines,
        reason = "One exhaustive dispatch loop covers every JVM opcode"
    )]
    #[inline(never)]
    fn execute_sync_instructions<const ACCOUNT: bool>(
        &self,
        budget: &mut crate::thread::InstructionBudget<'_>,
        state: &mut FrameState,
        remaining: &mut usize,
        debug: bool,
    ) -> Result<BatchExit> {
        let code = self.method.code();
        let FrameState { locals, stack } = state;
        let mut cursor = self.instruction_index.load(Ordering::Relaxed);
        // The thread owns the stack while this batch runs. Publish the exact cursor at
        // every boundary before a caller, handler, safepoint, or stack walker can observe it.
        let result = 'batch: {
            while *remaining != 0 {
                let Some(instruction) = code.get(cursor) else {
                    let pc = self
                        .method
                        .instruction_offset(cursor)
                        .unwrap_or_else(|| self.invalid_program_counter.load(Ordering::Relaxed));
                    break 'batch Err(InvalidProgramCounter(pc));
                };
                if debug {
                    self.instruction_index.store(cursor, Ordering::Relaxed);
                    if let Err(error) = self.debug_execute(locals, stack, instruction) {
                        break 'batch Err(error);
                    }
                }
                *remaining -= 1;
                if ACCOUNT && budget.record() {
                    break 'batch Ok(BatchExit::Yield);
                }
                let current_index = cursor;
                let program_counter = || {
                    self.method
                        .instruction_offset(current_index)
                        .unwrap_or_else(|| self.invalid_program_counter.load(Ordering::Relaxed))
                };
                // Keep success/error handling and cursor advancement explicitly inlined in each
                // opcode arm for performance: sharing this logic after the match reduces throughput.
                // Macros are deliberately avoided so IDE "Find References" can locate handler calls.
                // On failure, retain the faulting instruction for exception handling and stack walking.
                match instruction {
                    Instruction::Nop => match nop() {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aconst_null => match aconst_null(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_m1 => match iconst_m1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_0 => match iconst_0(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_1 => match iconst_1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_2 => match iconst_2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_3 => match iconst_3(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_4 => match iconst_4(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iconst_5 => match iconst_5(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lconst_0 => match lconst_0(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lconst_1 => match lconst_1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fconst_0 => match fconst_0(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fconst_1 => match fconst_1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fconst_2 => match fconst_2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dconst_0 => match dconst_0(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dconst_1 => match dconst_1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Bipush(value) => match bipush(stack, *value) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Sipush(value) => match sipush(stack, *value) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ldc(index) => {
                        let result = match ldc(self, stack, *index) {
                            Ok(InstructionResult::Sync(result)) => result,
                            Ok(InstructionResult::Async(instruction)) => {
                                break 'batch Ok(BatchExit::Async(instruction));
                            }
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        match result {
                            ExecutionResult::Continue => cursor += 1,
                            ExecutionResult::ContinueAtInstruction(next) => cursor = next,
                            result => break 'batch Ok(BatchExit::Complete(result)),
                        }
                    }
                    Instruction::Ldc_w(index) => {
                        let result = match ldc_w(self, stack, *index) {
                            Ok(InstructionResult::Sync(result)) => result,
                            Ok(InstructionResult::Async(instruction)) => {
                                break 'batch Ok(BatchExit::Async(instruction));
                            }
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        match result {
                            ExecutionResult::Continue => cursor += 1,
                            ExecutionResult::ContinueAtInstruction(next) => cursor = next,
                            result => break 'batch Ok(BatchExit::Complete(result)),
                        }
                    }
                    Instruction::Ldc2_w(index) => match ldc2_w(self, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload(index) => match iload(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload(index) => match lload(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload(index) => match fload(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload(index) => match dload(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload(index) => match aload(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload_0 => match iload_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload_1 => match iload_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload_2 => match iload_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload_3 => match iload_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload_0 => match lload_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload_1 => match lload_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload_2 => match lload_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload_3 => match lload_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload_0 => match fload_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload_1 => match fload_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload_2 => match fload_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload_3 => match fload_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload_0 => match dload_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload_1 => match dload_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload_2 => match dload_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload_3 => match dload_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload_0 => match aload_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload_1 => match aload_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload_2 => match aload_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload_3 => match aload_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iaload => match iaload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Laload => match laload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Faload => match faload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Daload => match daload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aaload => match aaload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Baload => match baload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Caload => match caload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Saload => match saload(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore(index) => match istore(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore(index) => match lstore(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore(index) => match fstore(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore(index) => match dstore(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore(index) => match astore(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore_0 => match istore_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore_1 => match istore_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore_2 => match istore_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore_3 => match istore_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore_0 => match lstore_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore_1 => match lstore_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore_2 => match lstore_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore_3 => match lstore_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore_0 => match fstore_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore_1 => match fstore_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore_2 => match fstore_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore_3 => match fstore_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore_0 => match dstore_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore_1 => match dstore_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore_2 => match dstore_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore_3 => match dstore_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore_0 => match astore_0(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore_1 => match astore_1(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore_2 => match astore_2(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore_3 => match astore_3(locals, stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iastore => match iastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lastore => match lastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fastore => match fastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dastore => match dastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aastore => match aastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Bastore => match bastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Castore => match castore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Sastore => match sastore(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Pop => match pop(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Pop2 => match pop2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup => match dup(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup_x1 => match dup_x1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup_x2 => match dup_x2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup2 => match dup2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup2_x1 => match dup2_x1(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dup2_x2 => match dup2_x2(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Swap => match swap(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iadd => match iadd(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ladd => match ladd(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fadd => match fadd(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dadd => match dadd(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Isub => match isub(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lsub => match lsub(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fsub => match fsub(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dsub => match dsub(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Imul => match imul(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lmul => match lmul(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fmul => match fmul(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dmul => match dmul(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Idiv => match idiv(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ldiv => match ldiv(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fdiv => match fdiv(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ddiv => match ddiv(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Irem => match irem(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lrem => match lrem(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Frem => match frem(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Drem => match drem(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ineg => match ineg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lneg => match lneg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fneg => match fneg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dneg => match dneg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ishl => match ishl(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lshl => match lshl(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ishr => match ishr(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lshr => match lshr(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iushr => match iushr(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lushr => match lushr(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iand => match iand(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Land => match land(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ior => match ior(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lor => match lor(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ixor => match ixor(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lxor => match lxor(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iinc(index, constant) => match iinc(locals, *index, *constant) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2l => match i2l(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2f => match i2f(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2d => match i2d(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::L2i => match l2i(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::L2f => match l2f(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::L2d => match l2d(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::F2i => match f2i(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::F2l => match f2l(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::F2d => match f2d(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::D2i => match d2i(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::D2l => match d2l(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::D2f => match d2f(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2b => match i2b(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2c => match i2c(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::I2s => match i2s(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lcmp => match lcmp(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fcmpl => match fcmpl(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fcmpg => match fcmpg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dcmpl => match dcmpl(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dcmpg => match dcmpg(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Ifeq(address) => {
                        let taken = match ifeq(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ifne(address) => {
                        let taken = match ifne(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Iflt(address) => {
                        let taken = match iflt(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ifge(address) => {
                        let taken = match ifge(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ifgt(address) => {
                        let taken = match ifgt(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ifle(address) => {
                        let taken = match ifle(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmpeq(address) => {
                        let taken = match if_icmpeq(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmpne(address) => {
                        let taken = match if_icmpne(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmplt(address) => {
                        let taken = match if_icmplt(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmpge(address) => {
                        let taken = match if_icmpge(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmpgt(address) => {
                        let taken = match if_icmpgt(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_icmple(address) => {
                        let taken = match if_icmple(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_acmpeq(address) => {
                        let taken = match if_acmpeq(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::If_acmpne(address) => {
                        let taken = match if_acmpne(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Goto(address) => {
                        let target = match self.branch_to(true, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Jsr(address) => {
                        let target = match self.branch_to_byte(
                            match jsr(stack, program_counter(), *address) {
                                Ok(pc) => pc,
                                Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                            },
                        ) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ret(index) => {
                        let target = match self.branch_to_byte(match ret(locals, *index) {
                            Ok(pc) => pc,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        }) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Tableswitch(table_switch) => {
                        let branch = {
                            let TableSwitch {
                                default,
                                low,
                                high,
                                offsets,
                            } = table_switch.as_ref();
                            let program_counter = program_counter();
                            self.branch_to_byte(
                                match tableswitch(
                                    stack,
                                    program_counter,
                                    *default,
                                    *low,
                                    *high,
                                    offsets,
                                ) {
                                    Ok(pc) => pc,
                                    Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                                },
                            )
                        };
                        let target = match branch {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Lookupswitch(lookup_switch) => {
                        let branch = {
                            let LookupSwitch { default, pairs } = lookup_switch.as_ref();
                            let program_counter = program_counter();
                            self.branch_to_byte(
                                match lookupswitch(stack, program_counter, *default, pairs) {
                                    Ok(pc) => pc,
                                    Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                                },
                            )
                        };
                        let target = match branch {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ireturn => {
                        break 'batch match ireturn(stack) {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Lreturn => {
                        break 'batch match lreturn(stack) {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Freturn => {
                        break 'batch match freturn(stack) {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Dreturn => {
                        break 'batch match dreturn(stack) {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Areturn => {
                        break 'batch match areturn(stack) {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Return => {
                        break 'batch match r#return() {
                            Ok(result) => Ok(BatchExit::Complete(result)),
                            Err(error) => Ok(BatchExit::Fault(error)),
                        };
                    }
                    Instruction::Getstatic(_)
                    | Instruction::Putstatic(_)
                    | Instruction::Getfield(_)
                    | Instruction::Putfield(_)
                    | Instruction::Invokevirtual(_)
                    | Instruction::Invokespecial(_)
                    | Instruction::Invokestatic(_)
                    | Instruction::Invokeinterface(_, _) => {
                        let result = match self.process_reference(stack, instruction) {
                            Ok(InstructionResult::Sync(result)) => result,
                            Ok(InstructionResult::Async(instruction)) => {
                                break 'batch Ok(BatchExit::Async(instruction));
                            }
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        match result {
                            ExecutionResult::Continue => cursor += 1,
                            ExecutionResult::ContinueAtInstruction(next) => cursor = next,
                            result => break 'batch Ok(BatchExit::Complete(result)),
                        }
                    }
                    Instruction::Newarray(array_type) => match newarray(self, stack, array_type) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Athrow => match athrow(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Arraylength => match arraylength(stack) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Wide => match wide() {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Invokedynamic(_)
                    | Instruction::New(_)
                    | Instruction::Anewarray(_)
                    | Instruction::Checkcast(_)
                    | Instruction::Instanceof(_)
                    | Instruction::Monitorenter
                    | Instruction::Monitorexit
                    | Instruction::Multianewarray(_, _) => {
                        break 'batch Ok(BatchExit::Async(instruction.clone()));
                    }
                    Instruction::Ifnull(address) => {
                        let taken = match ifnull(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Ifnonnull(address) => {
                        let taken = match ifnonnull(stack) {
                            Ok(taken) => taken,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        let target = match self.branch_to(taken, cursor, i32::from(*address)) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Goto_w(address) => {
                        let target = match self.branch_to(true, cursor, *address) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Jsr_w(address) => {
                        let target = match self.branch_to_byte(
                            match jsr_w(stack, program_counter(), *address) {
                                Ok(pc) => pc,
                                Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                            },
                        ) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                    Instruction::Breakpoint => match breakpoint() {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Impdep1 => match impdep1() {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Impdep2 => match impdep2() {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iload_w(index) => match iload_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lload_w(index) => match lload_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fload_w(index) => match fload_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dload_w(index) => match dload_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Aload_w(index) => match aload_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Istore_w(index) => match istore_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Lstore_w(index) => match lstore_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Fstore_w(index) => match fstore_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Dstore_w(index) => match dstore_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Astore_w(index) => match astore_w(locals, stack, *index) {
                        Ok(()) => cursor += 1,
                        Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                    },
                    Instruction::Iinc_w(index, constant) => {
                        match iinc_w(locals, *index, *constant) {
                            Ok(()) => cursor += 1,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        }
                    }
                    Instruction::Ret_w(index) => {
                        let target = match self.branch_to_byte(match ret_w(locals, *index) {
                            Ok(pc) => pc,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        }) {
                            Ok(target) => target,
                            Err(error) => break 'batch Ok(BatchExit::Fault(error)),
                        };
                        cursor = target;
                    }
                }
            }
            Ok(BatchExit::Complete(if ACCOUNT {
                ExecutionResult::Continue
            } else {
                ExecutionResult::ContinueAtInstruction(cursor)
            }))
        };
        self.instruction_index.store(cursor, Ordering::Relaxed);
        result
    }

    /// Resume this frame after a method call completes successfully.
    pub(crate) fn complete_call(
        &self,
        state: &mut FrameState,
        value: Option<Value>,
        has_return_type: bool,
    ) -> Result<()> {
        if has_return_type {
            // An intrinsic returning `None` for a non-void method denotes a null reference (for example, `TimeZone.getSystemGMTOffsetID`, whose native contract already documents null as its "no fixed offset" result); pushing it keeps the caller's operand stack balanced.
            let value = value.unwrap_or(Value::Object(None));
            state.stack.push(value)?;
        }
        self.instruction_index.store(
            self.instruction_index.load(Ordering::Relaxed) + 1,
            Ordering::Relaxed,
        );
        Ok(())
    }

    /// Deliver an error raised by a callee to this frame.
    pub(crate) async fn handle_error(
        &self,
        state: &mut FrameState,
        error: crate::Error,
    ) -> Result<()> {
        Self::handle_error_with_stack(self, &mut state.stack, error).await
    }

    async fn handle_error_with_stack(
        frame: &Frame,
        stack: &mut OperandStack,
        error: crate::Error,
    ) -> Result<()> {
        let throwable = if let crate::Error::Throwable(throwable) = error {
            throwable
        } else {
            let thread = frame.thread()?;
            // Implicit exceptions need constructor headroom even when the Java stack is full.
            let _overflow_reserve = thread.stack_overflow_reserve();
            convert_error_to_throwable(&thread, error).await?
        };
        let handler_program_counter = process_throwable(frame, stack, throwable).await?;
        frame.set_program_counter(handler_program_counter);
        Ok(())
    }

    /// Number of Java stack slots reserved by this frame.
    pub(crate) fn stack_slots(&self) -> Result<usize> {
        Self::stack_slots_for(&self.method)
    }

    /// Number of Java stack slots required by a method before its frame is allocated.
    pub(crate) fn stack_slots_for(method: &Method) -> Result<usize> {
        method
            .max_locals()
            .checked_add(method.max_stack())
            .and_then(|slots| slots.checked_add(1))
            .ok_or_else(|| StackOverflowError("Java stack slot count overflow".to_string()).into())
    }

    /// Debug the execution of an instruction in this frame
    ///
    /// # Overview
    ///
    /// This method logs detailed debug information about the current execution state
    /// of the frame, including local variables, operand stack, program counter, and
    /// the instruction about to be executed. It's only called when debug-level logging
    /// is enabled.
    ///
    /// # Debug Output
    ///
    /// The method logs the following information:
    /// - Class name, method name, method descriptor, and source file with line number (if available)
    /// - Contents of the local variables array
    /// - Current operand stack contents and remaining stack size
    /// - Current program counter and formatted instruction string
    ///
    /// # Implementation Note
    ///
    /// This is only invoked when the debug log level is enabled, minimizing performance impact.
    #[cold]
    #[inline(never)]
    fn debug_execute(
        &self,
        locals: &LocalVariables,
        stack: &OperandStack,
        instruction: &Instruction,
    ) -> Result<()> {
        let program_counter = self.program_counter();
        let class_name = self.class.name();
        let method_name = self.method.name();
        let method_descriptor = self.method.descriptor();
        let source = if let Some(source_file) = self.class.source_file() {
            let line_number = self.method.line_number(program_counter);
            format!(" ({source_file}:{line_number})")
        } else {
            String::new()
        };
        let constant_pool = self.class.constant_pool();
        let instruction = instruction.to_formatted_string(constant_pool)?;
        debug!("  frame: {class_name}.{method_name}{method_descriptor}{source}");
        debug!("    locals: {locals}");
        debug!("    stack: {stack}");
        debug!("    pc: {program_counter}; instruction: {instruction}");
        Ok(())
    }

    /// Construct concrete futures only for loading, initialization, or other async work.
    pub(crate) async fn process_async(
        &self,
        stack: &mut OperandStack,
        instruction: &Instruction,
    ) -> Result<ExecutionResult> {
        match instruction {
            Instruction::Getstatic(index) => getstatic(self, stack, *index).await,
            Instruction::Putstatic(index) => putstatic(self, stack, *index).await,
            Instruction::Getfield(index) => getfield(self, stack, *index).await,
            Instruction::Putfield(index) => putfield(self, stack, *index).await,
            Instruction::Invokevirtual(index) => invokevirtual(self, stack, *index).await,
            Instruction::Invokespecial(index) => invokespecial(self, stack, *index).await,
            Instruction::Invokestatic(index) => invokestatic(self, stack, *index).await,
            Instruction::Invokeinterface(index, count) => {
                invokeinterface(self, stack, *index, *count).await
            }
            Instruction::Invokedynamic(index) => invokedynamic(self, stack, *index).await,
            Instruction::New(index) => new(self, stack, *index).await,
            Instruction::Anewarray(index) => anewarray(self, stack, *index).await,
            Instruction::Athrow => athrow(stack).map(|()| ExecutionResult::Continue),
            Instruction::Checkcast(index) => checkcast(self, stack, *index).await,
            Instruction::Instanceof(index) => instanceof(self, stack, *index).await,
            Instruction::Monitorenter => monitorenter(self, stack).await,
            Instruction::Monitorexit => monitorexit(self, stack).await,
            Instruction::Multianewarray(index, dimensions) => {
                multianewarray(self, stack, *index, *dimensions).await
            }
            Instruction::Ldc(index) => {
                crate::instruction::load_constant_async(self, stack, u16::from(*index)).await
            }
            Instruction::Ldc_w(index) => {
                crate::instruction::load_constant_async(self, stack, *index).await
            }
            _ => Err(InternalError(format!(
                "Unexpected slow instruction: {instruction:?}"
            ))),
        }
    }

    #[expect(
        clippy::inline_always,
        reason = "Keep cached reference resolution inside dispatch"
    )]
    #[inline(always)]
    fn process_reference(
        &self,
        stack: &mut OperandStack,
        instruction: &Instruction,
    ) -> Result<InstructionResult> {
        match instruction {
            Instruction::Getstatic(index)
            | Instruction::Putstatic(index)
            | Instruction::Getfield(index)
            | Instruction::Putfield(index) => {
                let result = match crate::instruction::try_field(self, stack, *index, instruction)?
                {
                    Some(result) => InstructionResult::Sync(result),
                    None => InstructionResult::Async(instruction.clone()),
                };
                Ok(result)
            }
            Instruction::Invokevirtual(index)
            | Instruction::Invokespecial(index)
            | Instruction::Invokestatic(index)
            | Instruction::Invokeinterface(index, _) => {
                use crate::method_ref_cache::InvokeKind;
                let kind = match instruction {
                    Instruction::Invokevirtual(_) => InvokeKind::Virtual,
                    Instruction::Invokespecial(_) => InvokeKind::Special,
                    Instruction::Invokestatic(_) => InvokeKind::Static,
                    _ => InvokeKind::Interface,
                };
                let result = match crate::instruction::try_invoke(self, stack, *index, kind)? {
                    Some(result) => InstructionResult::Sync(result),
                    None => InstructionResult::Async(instruction.clone()),
                };
                Ok(result)
            }
            _ => Err(InternalError(format!(
                "Unexpected reference instruction: {instruction:?}"
            ))),
        }
    }

    #[expect(
        clippy::inline_always,
        reason = "Merge the synchronous fast path into dispatch to avoid per-instruction result copies"
    )]
    #[inline(always)]
    fn branch_to(&self, taken: bool, instruction_index: usize, displacement: i32) -> Result<usize> {
        if !taken {
            return Ok(instruction_index + 1);
        }
        if let Some(target) = self.method.branch_target_index(instruction_index) {
            return Ok(target);
        }
        // Unverified programmatic methods can contain invalid targets. Resolve them with
        // checked signed arithmetic so the ordinary boundary checks still report an error.
        let pc = self
            .method
            .instruction_offset(instruction_index)
            .unwrap_or_else(|| self.invalid_program_counter.load(Ordering::Relaxed));
        self.branch_to_byte(goto_w(pc, displacement)?)
    }

    /// Translate dynamic JVM byte targets (switches and return addresses) at the boundary.
    #[inline]
    fn branch_to_byte(&self, byte_offset: usize) -> Result<usize> {
        self.method
            .instruction_index(byte_offset)
            .ok_or(InvalidProgramCounter(byte_offset))
    }
}

impl ristretto_types::Frame for Frame {
    fn class(&self) -> &Arc<Class> {
        &self.class
    }

    fn method(&self) -> &Arc<Method> {
        &self.method
    }

    fn program_counter(&self) -> usize {
        Frame::program_counter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VM;
    use crate::configuration::ConfigurationBuilder;
    use crate::thread::Thread;
    use ristretto_classfile::attributes::Attribute;
    use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
    use ristretto_classloader::ClassPath;
    use std::path::PathBuf;
    use std::task::{Context, Waker};

    fn batch_test_frame(
        code: impl FnOnce(u16) -> Vec<Instruction>,
    ) -> Result<(Arc<Thread>, Frame, FrameState)> {
        let mut constant_pool = ConstantPool::default();
        let this_class = constant_pool.add_class("BatchTest")?;
        let name_index = constant_pool.add_utf8("test")?;
        let descriptor_index = constant_pool.add_utf8("(I)I")?;
        let code_index = constant_pool.add_utf8("Code")?;
        let class = Class::from(
            None,
            ClassFile {
                constant_pool,
                this_class,
                methods: vec![ristretto_classfile::Method {
                    access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                    name_index,
                    descriptor_index,
                    attributes: vec![Attribute::Code {
                        name_index: code_index,
                        max_stack: 2,
                        max_locals: 1,
                        code: code(this_class),
                        exception_table: Vec::new(),
                        attributes: Vec::new(),
                    }],
                }],
                ..Default::default()
            },
        )?;
        let thread = Thread::new(&Weak::new(), 1);
        let method = class.try_get_method("test", "(I)I")?;
        let frame = Frame::new(&Arc::downgrade(&thread), &class, &method);
        let mut state = FrameState::default();
        state.reset(&class, &method, [Value::Int(1_000)])?;
        Ok((thread, frame, state))
    }

    #[test]
    fn test_synchronous_batch_commits_budget_on_fault_and_invalid_pc() -> Result<()> {
        let (thread, frame, mut state) =
            batch_test_frame(|_| vec![Instruction::Iconst_1, Instruction::Pop, Instruction::Pop])?;
        let mut remaining = INSTRUCTION_BATCH_SIZE;
        assert!(matches!(
            frame.execute_sync_batch(&thread, &mut state, &mut remaining, false)?,
            BatchExit::Fault(crate::Error::OperandStackUnderflow)
        ));
        assert_eq!(3, thread.instruction_count());
        assert_eq!(2, frame.program_counter());
        assert_eq!(253, remaining);

        let (thread, frame, mut state) = batch_test_frame(|_| vec![Instruction::Nop])?;
        let mut remaining = INSTRUCTION_BATCH_SIZE;
        assert!(matches!(
            frame.execute_sync_batch(&thread, &mut state, &mut remaining, false),
            Err(InvalidProgramCounter(1))
        ));
        assert_eq!(1, thread.instruction_count());
        assert_eq!(1, frame.program_counter());
        assert_eq!(255, remaining);
        Ok(())
    }

    #[test]
    fn test_branch_faults_publish_the_opcode_byte_offset() -> Result<()> {
        for (branch, target) in [
            (Instruction::Ifeq(0), None),
            (Instruction::Goto(-2), Some(1)),
            (Instruction::Goto(3), Some(6)),
        ] {
            let (thread, frame, mut state) =
                batch_test_frame(|_| vec![Instruction::Bipush(42), Instruction::Pop, branch])?;
            let mut remaining = INSTRUCTION_BATCH_SIZE;
            let boundary = frame.execute_sync_batch(&thread, &mut state, &mut remaining, false)?;
            match target {
                None => assert!(matches!(
                    boundary,
                    BatchExit::Fault(crate::Error::OperandStackUnderflow)
                )),
                Some(target) => assert!(
                    matches!(boundary, BatchExit::Fault(InvalidProgramCounter(pc)) if pc == target)
                ),
            }
            assert_eq!(3, frame.program_counter());
            assert_eq!(3, thread.instruction_count());
            assert_eq!(INSTRUCTION_BATCH_SIZE - 3, remaining);
        }
        Ok(())
    }

    #[test]
    fn test_legacy_subroutines_return_to_the_next_bytecode() -> Result<()> {
        for branch in [Instruction::Jsr(-3), Instruction::Jsr_w(-3)] {
            let (thread, frame, mut state) = batch_test_frame(|_| {
                vec![
                    Instruction::Goto(6),
                    Instruction::Astore_0,
                    Instruction::Ret(0),
                    branch,
                    Instruction::Bipush(42),
                    Instruction::Ireturn,
                ]
            })?;
            let mut remaining = INSTRUCTION_BATCH_SIZE;
            assert!(matches!(
                frame.execute_sync_batch(&thread, &mut state, &mut remaining, false)?,
                BatchExit::Complete(ExecutionResult::Return(Some(Value::Int(42))))
            ));
        }
        Ok(())
    }

    #[test]
    fn test_program_counter_exposes_bytes_and_rejects_operand_bytes() -> Result<()> {
        let (thread, frame, mut state) = batch_test_frame(|_| {
            vec![
                Instruction::Bipush(42),
                Instruction::Pop,
                Instruction::Return,
            ]
        })?;
        frame.set_program_counter(2);
        assert_eq!(2, frame.program_counter());
        assert_eq!(2, ristretto_types::Frame::program_counter(&frame));

        frame.set_program_counter(1);
        assert_eq!(1, frame.program_counter());
        let mut remaining = INSTRUCTION_BATCH_SIZE;
        assert!(matches!(
            frame.execute_sync_batch(&thread, &mut state, &mut remaining, false),
            Err(InvalidProgramCounter(1))
        ));
        Ok(())
    }

    #[test]
    fn test_stack_arguments_fill_reused_wide_locals() -> Result<()> {
        let (_thread, frame, _) = batch_test_frame(|_| vec![Instruction::Return])?;
        let mut definition = frame.class().class_file().clone();
        let definition_method = definition.methods.first_mut().expect("test method");
        definition_method.descriptor_index = definition.constant_pool.add_utf8("(JD)D")?;
        for attribute in &mut definition_method.attributes {
            if let Attribute::Code { max_locals, .. } = attribute {
                *max_locals = 6;
            }
        }
        let class = Class::from(None, definition)?;
        let method = class.try_get_method("test", "(JD)D")?;
        let mut caller = FrameState::default();
        caller.stack.reset(3);
        caller.stack.push_int(19)?;
        caller.stack.push_long(42)?;
        caller.stack.push_double(2.0)?;
        let mut destination = FrameState::default();
        destination.reset(&class, &method, vec![Value::Int(99); 6])?;
        CallParameters::Stack(2).initialize(
            &mut destination,
            &class,
            &method,
            Some(&mut caller),
        )?;
        assert_eq!(19, caller.stack.pop_int()?);
        assert!(caller.stack.is_empty());
        assert_eq!(42, destination.locals.get_long(0)?);
        assert_eq!(Value::Unused, destination.locals.get(1)?);
        assert_eq!(Value::Double(2.0), destination.locals.get(2)?);
        for index in [3, 4, 5] {
            assert_eq!(Value::Unused, destination.locals.get(index)?);
        }
        assert!(destination.locals.get(6).is_err());
        assert!(destination.stack.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_preserves_locals_stack_and_branches() -> Result<()> {
        let (thread, frame, mut state) = batch_test_frame(|_| {
            vec![
                Instruction::Iconst_0,
                Instruction::Iload_0,
                Instruction::Ifeq(11),
                Instruction::Iinc(0, -1),
                Instruction::Iconst_1,
                Instruction::Iadd,
                Instruction::Goto(-9),
                Instruction::Ireturn,
            ]
        })?;

        // The loop must stop at a batch boundary before finishing the method.
        let mut result = frame.execute_batch(&thread, &mut state).await?;
        assert_eq!(ExecutionResult::Continue, result);
        for _ in 0..100 {
            result = frame.execute_batch(&thread, &mut state).await?;
            if result != ExecutionResult::Continue {
                break;
            }
        }
        assert_eq!(ExecutionResult::Return(Some(Value::Int(1_000))), result);
        assert_eq!(13, frame.program_counter());
        Ok(())
    }

    #[tokio::test]
    async fn test_yielded_conditional_resumes_at_the_correct_byte_offset() -> Result<()> {
        for (condition, next_pc, expected) in [(0, 5, 11), (1, 8, 22)] {
            let (thread, frame, mut state) = batch_test_frame(|_| {
                vec![
                    Instruction::Bipush(condition),
                    Instruction::Ifne(6),
                    Instruction::Bipush(11),
                    Instruction::Ireturn,
                    Instruction::Bipush(22),
                    Instruction::Ireturn,
                ]
            })?;
            // A yielded opcode has already consumed its budget, but has not popped operands.
            state.stack.push_int(i32::from(condition))?;
            frame.set_program_counter(2);
            let count = thread.instruction_count();
            assert_eq!(
                ExecutionResult::Continue,
                frame.resume_batch(&mut state, BatchExit::Yield).await?
            );
            assert_eq!(next_pc, frame.program_counter());
            assert_eq!(count, thread.instruction_count());
            assert_eq!(
                ExecutionResult::Return(Some(Value::Int(expected))),
                frame.execute_batch(&thread, &mut state).await?
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_stops_at_return() -> Result<()> {
        let (thread, frame, mut state) =
            batch_test_frame(|_| vec![Instruction::Iconst_1, Instruction::Ireturn])?;
        assert_eq!(
            ExecutionResult::Return(Some(Value::Int(1))),
            frame.execute_batch(&thread, &mut state).await?
        );
        assert_eq!(1, frame.program_counter());
        Ok(())
    }

    #[tokio::test]
    async fn test_batches_yield_with_sync_and_ready_async_handlers() -> Result<()> {
        for ready_async in [false, true] {
            let (thread, frame, mut state) = batch_test_frame(|class_index| {
                if ready_async {
                    // instanceof null uses an async handler that completes without awaiting.
                    vec![
                        Instruction::Aconst_null,
                        Instruction::Instanceof(class_index),
                        Instruction::Pop,
                        Instruction::Goto(-5),
                    ]
                } else {
                    vec![Instruction::Goto(0)]
                }
            })?;

            {
                // Disable Tokio's implicit mutex budget to check the interpreter's own budget.
                // Use a finite number of batches so a missing explicit yield fails, not hangs.
                let mut execution = std::pin::pin!(tokio::task::unconstrained(async {
                    for _ in 0..64 {
                        assert_eq!(
                            ExecutionResult::Continue,
                            frame.execute_batch(&thread, &mut state).await?
                        );
                    }
                    Ok::<(), crate::Error>(())
                }));
                let mut context = Context::from_waker(Waker::noop());
                assert!(execution.as_mut().poll(&mut context).is_pending());
            }

            // Cancelling a yielded batch ends the mutable borrow, and the frame can resume.
            assert_eq!(
                ExecutionResult::Continue,
                frame.execute_batch(&thread, &mut state).await?
            );
        }
        Ok(())
    }

    async fn get_class(class_name: &str) -> Result<(Arc<VM>, Arc<Thread>, Arc<Class>)> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_path = cargo_manifest.join("..").join("classes");
        let class_path = ClassPath::from(&[classes_path]);
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .build()?;
        let vm = VM::new(configuration).await?;
        let weak_vm = Arc::downgrade(&vm);
        let thread = Thread::new(&weak_vm, 3);
        let class = thread.class(class_name).await?;
        Ok((vm, thread, class))
    }

    #[tokio::test]
    async fn test_execute() -> Result<()> {
        let (_vm, thread, class) = get_class("java.lang.Math").await?;
        let method = class.method("addExact", "(II)I").expect("method not found");
        let parameters = vec![Value::Int(1), Value::Int(2)];
        let frame = Frame::new(&Arc::downgrade(&thread), &class, &method);
        let result = frame.execute(parameters).await?;
        assert!(matches!(result, Some(Value::Int(3))));
        Ok(())
    }

    #[tokio::test]
    async fn test_reused_metadata_resets_pc_and_class_references() -> Result<()> {
        let (vm, _thread, mut frame) = crate::test::frame().await?;
        let original_class = frame.class().clone();
        let original_method = frame.method().clone();
        let methods = vm.method_ref_cache().for_class(&original_class);
        let fields = vm.field_ref_cache().for_class(&original_class);
        assert!(std::ptr::eq(frame.method_refs()?, methods.as_ref()));
        assert!(std::ptr::eq(frame.field_refs()?, fields.as_ref()));
        frame.instruction_index.store(17, Ordering::Relaxed);
        frame.reset(&original_class, &original_method);
        assert_eq!(0, frame.program_counter());
        assert!(std::ptr::eq(frame.method_refs()?, methods.as_ref()));
        let mut definition = original_class.class_file().clone();
        definition.this_class = definition.constant_pool.add_class("OtherFrameClass")?;
        let other = Class::from(None, definition)?;
        let method = other.try_get_method("test", "()V")?;
        frame.reset(&other, &method);
        assert!(!std::ptr::eq(frame.method_refs()?, methods.as_ref()));
        assert!(!std::ptr::eq(frame.field_refs()?, fields.as_ref()));
        assert!(Arc::ptr_eq(frame.method(), &method));
        Ok(())
    }

    #[tokio::test]
    async fn test_method_call_equality() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let call = MethodCall {
            intrinsic: IntrinsicBinding::Unresolved,
            class: frame.class().clone(),
            method: frame.method().clone(),
            parameters: vec![Value::Int(42)].into(),
            has_return_type: true,
        };
        let same_call = MethodCall {
            intrinsic: IntrinsicBinding::Unresolved,
            class: frame.class().clone(),
            method: frame.method().clone(),
            parameters: vec![Value::Int(42)].into(),
            has_return_type: true,
        };

        assert_eq!(call, same_call);
        Ok(())
    }

    #[tokio::test]
    async fn test_with_parameters_rejects_too_many_local_slots() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let result = FrameState::default().reset(frame.class(), frame.method(), [Value::Int(1)]);

        assert!(matches!(result, Err(InternalError(message)) if message ==
            "Method parameters require 1 local-variable slots, but Test.test()V declares 0"));
        Ok(())
    }

    #[test]
    fn test_adjust_parameters() {
        let parameters = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.0),
            Value::Double(4.0),
        ];
        let mut locals = LocalVariables::with_max_size(0);
        assert_eq!(6, locals.reset(parameters, 8));
        assert_eq!(
            (0..8)
                .map(|index| locals.get(index).unwrap())
                .collect::<Vec<_>>(),
            vec![
                Value::Int(1),
                Value::Long(2),
                Value::Unused,
                Value::Float(3.0),
                Value::Double(4.0),
                Value::Unused,
                Value::Unused,
                Value::Unused,
            ]
        );
    }
}

#[cfg(test)]
mod resolution_tests;

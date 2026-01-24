use crate::Error::{InternalError, InvalidProgramCounter};
use crate::frame::ExecutionResult::{Continue, ContinueAtPosition, Return};
use crate::instruction::{
    aaload, aastore, aconst_null, aload, aload_0, aload_1, aload_2, aload_3, aload_w, anewarray,
    areturn, arraylength, astore, astore_0, astore_1, astore_2, astore_3, astore_w, athrow, baload,
    bastore, bipush, breakpoint, caload, castore, checkcast, convert_error_to_throwable, d2f, d2i,
    d2l, dadd, daload, dastore, dcmpg, dcmpl, dconst_0, dconst_1, ddiv, dload, dload_0, dload_1,
    dload_2, dload_3, dload_w, dmul, dneg, drem, dreturn, dstore, dstore_0, dstore_1, dstore_2,
    dstore_3, dstore_w, dsub, dup, dup_x1, dup_x2, dup2, dup2_x1, dup2_x2, f2d, f2i, f2l, fadd,
    faload, fastore, fcmpg, fcmpl, fconst_0, fconst_1, fconst_2, fdiv, fload, fload_0, fload_1,
    fload_2, fload_3, fload_w, fmul, fneg, frem, freturn, fstore, fstore_0, fstore_1, fstore_2,
    fstore_3, fstore_w, fsub, getfield, getstatic, goto, goto_w, i2b, i2c, i2d, i2f, i2l, i2s,
    iadd, iaload, iand, iastore, iconst_0, iconst_1, iconst_2, iconst_3, iconst_4, iconst_5,
    iconst_m1, idiv, if_acmpeq, if_acmpne, if_icmpeq, if_icmpge, if_icmpgt, if_icmple, if_icmplt,
    if_icmpne, ifeq, ifge, ifgt, ifle, iflt, ifne, ifnonnull, ifnull, iinc, iinc_w, iload, iload_0,
    iload_1, iload_2, iload_3, iload_w, impdep1, impdep2, imul, ineg, instanceof, invokedynamic,
    invokeinterface, invokespecial, invokestatic, invokevirtual, ior, irem, ireturn, ishl, ishr,
    istore, istore_0, istore_1, istore_2, istore_3, istore_w, isub, iushr, ixor, jsr, jsr_w, l2d,
    l2f, l2i, ladd, laload, land, lastore, lcmp, lconst_0, lconst_1, ldc, ldc_w, ldc2_w, ldiv,
    lload, lload_0, lload_1, lload_2, lload_3, lload_w, lmul, lneg, lookupswitch, lor, lrem,
    lreturn, lshl, lshr, lstore, lstore_0, lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr,
    lxor, monitorenter, monitorexit, multianewarray, new, newarray, nop, pop, pop2,
    process_throwable, putfield, putstatic, ret, ret_w, r#return, saload, sastore, sipush, swap,
    tableswitch, wide,
};
use crate::{LocalVariables, OperandStack, Result, Thread};
use byte_unit::{Byte, UnitType};
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_classloader::{Class, Method, Value};
use ristretto_macros::async_method;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use tracing::{Level, debug, event_enabled};

/// Represents the result of executing a JVM bytecode instruction.
///
/// # Overview
///
/// This enum is used to control the flow of execution within a frame. After each
/// instruction is processed, an `ExecutionResult` is returned to indicate how the
/// virtual machine should proceed.
///
/// # Variants
///
/// - `Return(Option<Value>)`: Indicates that the method execution should terminate and
///   return the specified value (if any) to the caller. Methods with a void return type
///   use `Return(None)`.
///
/// - `Continue`: Indicates that execution should continue with the next instruction
///   (increments the program counter by 1).
///
/// - `ContinueAtPosition(usize)`: Indicates that execution should continue at the specified
///   bytecode offset. Used for branch instructions like `goto`, `if_*`, and exception handlers.
///
/// # Usage
///
/// The VM's execution loop uses this enum to determine whether to proceed to the next
/// instruction, jump to a different instruction, or return from the current method.
#[derive(Debug, PartialEq)]
pub(crate) enum ExecutionResult {
    Return(Option<Value>),
    Continue,
    ContinueAtPosition(usize),
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
/// - `program_counter`: Current position in the bytecode
///
/// # Execution Model
/// When a method is invoked, a new frame is created and pushed onto the JVM stack of the invoking thread.
/// When the method completes (normally or abruptly), the frame is popped, and the invoker's frame becomes
/// the current frame.
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
    program_counter: AtomicUsize,
}

impl Frame {
    /// Create a new frame for the specified class. To invoke a method on an object reference, the
    /// object reference must be the first parameter in the parameters vector.
    pub fn new(thread: &Weak<Thread>, class: &Arc<Class>, method: &Arc<Method>) -> Self {
        Frame {
            thread: thread.clone(),
            class: class.clone(),
            method: method.clone(),
            program_counter: AtomicUsize::new(0),
        }
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
        &mut self.class
    }

    /// Get the method in this frame.
    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    /// Get the current program counter in this frame.
    ///
    /// The program counter represents the current position in the bytecode of the method
    /// being executed. It's used to determine which instruction to execute next.
    #[inline]
    pub fn program_counter(&self) -> usize {
        self.program_counter.load(Ordering::Relaxed)
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
    #[async_method]
    pub async fn execute(&self, mut parameters: Vec<Value>) -> Result<Option<Value>> {
        let max_locals = self.method.max_locals();
        Frame::adjust_parameters(&mut parameters, max_locals);
        let locals = &mut LocalVariables::new(parameters);
        let max_stack = self.method.max_stack();
        let stack = &mut OperandStack::with_max_size(max_stack);
        let code = self.method.code();
        let mut instruction_count: u32 = 0;

        loop {
            // Yield periodically to allow tokio to process cancellation and other tasks
            instruction_count = instruction_count.wrapping_add(1);
            if instruction_count.is_multiple_of(1024) {
                tokio::task::yield_now().await;
            }

            let program_counter = self.program_counter.load(Ordering::Relaxed);
            let Some(instruction) = code.get(program_counter) else {
                return Err(InvalidProgramCounter(program_counter));
            };

            if event_enabled!(Level::DEBUG) {
                self.debug_execute(locals, stack, instruction)?;
            }

            let result = Box::pin(self.process(locals, stack, instruction)).await;
            match result {
                Ok(Continue) => {
                    self.program_counter
                        .store(program_counter + 1, Ordering::Relaxed);
                }
                Ok(ContinueAtPosition(program_counter)) => {
                    self.program_counter
                        .store(program_counter, Ordering::Relaxed);
                }
                Ok(Return(value)) => return Ok(value),
                Err(error) => {
                    let thread = self.thread()?;
                    let throwable = convert_error_to_throwable(&thread, error).await?;
                    let handler_program_counter = process_throwable(self, stack, throwable).await?;
                    self.program_counter
                        .store(handler_program_counter, Ordering::Relaxed);
                }
            }
        }
    }

    /// Adjusts the parameters vector to conform to JVM local variable layout rules.
    ///
    /// # Overview
    ///
    /// According to the JVM specification, `long` and `double` values occupy two consecutive
    /// local variable slots. This method inserts `Value::Unused` placeholders after
    /// each `Long` and `Double` value in the parameters vector to ensure proper layout
    /// in the local variables array. It also ensures the total size matches `max_size` by
    /// padding with additional `Value::Unused` entries if necessary.
    ///
    /// # JVM Specification
    ///
    /// The JVM uses indices to 32-bit address for local variables. Local variables that are `long`
    /// or `double` occupy two consecutive slots due to their 64-bit width. However, this JVM
    /// implementation is not constrained by the 32-bit limit, so second slot is reserved and should
    /// not be used for accessing variables.
    ///
    /// # Examples
    ///
    /// ```text
    /// // Before adjustment: [Int(1), Long(2), Float(3.0)]
    /// // After adjustment:  [Int(1), Long(2), Unused, Float(3.0)]
    /// ```
    ///
    /// # References
    ///
    /// - [JVMS §2.6.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1)
    fn adjust_parameters(parameters: &mut Vec<Value>, max_size: usize) {
        let mut index = parameters.len();
        while index > 0 {
            index -= 1;
            match &parameters[index] {
                Value::Long(_) | Value::Double(_) => {
                    parameters.insert(index + 1, Value::Unused);
                }
                _ => {}
            }
        }
        if parameters.len() < max_size {
            parameters.resize(max_size, Value::Unused);
        }
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
        let stack_size = u64::try_from(stacker::remaining_stack().unwrap_or(0))?;
        let stack_size = Byte::from_u64(stack_size).get_appropriate_unit(UnitType::Decimal);
        debug!("  frame: {class_name}.{method_name}{method_descriptor}{source}");
        debug!("    locals: {locals}");
        debug!("    stack ({stack_size:#.3}): {stack}");
        debug!("    pc: {program_counter}; instruction: {instruction}");
        Ok(())
    }

    /// Process an instruction in this frame
    ///
    /// # Overview
    ///
    /// This method is responsible for executing a single JVM bytecode instruction within the current frame.
    /// It dispatches the instruction to the appropriate handler function based on its opcode and
    /// manages the modification of the local variables and operand stack.
    ///
    /// # Error Handling
    ///
    /// If an instruction throws an exception (either explicitly via `athrow` or due to an error
    /// condition), this method returns an `Err` that will be handled by the caller (`execute`
    /// method).
    ///
    /// # Implementation Note
    ///
    /// This method uses a large match statement to dispatch each instruction to its
    /// specialized handler function. The JVM specification defines over 200 bytecode
    /// instructions, and this method handles all of them.
    ///
    /// # References
    ///
    /// - [JVMS §6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html)
    #[expect(clippy::too_many_lines)]
    async fn process(
        &self,
        locals: &mut LocalVariables,
        stack: &mut OperandStack,
        instruction: &Instruction,
    ) -> Result<ExecutionResult> {
        match instruction {
            Instruction::Nop => nop(),
            Instruction::Aconst_null => aconst_null(stack),
            Instruction::Iconst_m1 => iconst_m1(stack),
            Instruction::Iconst_0 => iconst_0(stack),
            Instruction::Iconst_1 => iconst_1(stack),
            Instruction::Iconst_2 => iconst_2(stack),
            Instruction::Iconst_3 => iconst_3(stack),
            Instruction::Iconst_4 => iconst_4(stack),
            Instruction::Iconst_5 => iconst_5(stack),
            Instruction::Lconst_0 => lconst_0(stack),
            Instruction::Lconst_1 => lconst_1(stack),
            Instruction::Fconst_0 => fconst_0(stack),
            Instruction::Fconst_1 => fconst_1(stack),
            Instruction::Fconst_2 => fconst_2(stack),
            Instruction::Dconst_0 => dconst_0(stack),
            Instruction::Dconst_1 => dconst_1(stack),
            Instruction::Bipush(value) => bipush(stack, *value),
            Instruction::Sipush(value) => sipush(stack, *value),
            Instruction::Ldc(index) => Box::pin(ldc(self, stack, *index)).await,
            Instruction::Ldc_w(index) => Box::pin(ldc_w(self, stack, *index)).await,
            Instruction::Ldc2_w(index) => ldc2_w(self, stack, *index),
            Instruction::Iload(index) => iload(locals, stack, *index),
            Instruction::Lload(index) => lload(locals, stack, *index),
            Instruction::Fload(index) => fload(locals, stack, *index),
            Instruction::Dload(index) => dload(locals, stack, *index),
            Instruction::Aload(index) => aload(locals, stack, *index),
            Instruction::Iload_0 => iload_0(locals, stack),
            Instruction::Iload_1 => iload_1(locals, stack),
            Instruction::Iload_2 => iload_2(locals, stack),
            Instruction::Iload_3 => iload_3(locals, stack),
            Instruction::Lload_0 => lload_0(locals, stack),
            Instruction::Lload_1 => lload_1(locals, stack),
            Instruction::Lload_2 => lload_2(locals, stack),
            Instruction::Lload_3 => lload_3(locals, stack),
            Instruction::Fload_0 => fload_0(locals, stack),
            Instruction::Fload_1 => fload_1(locals, stack),
            Instruction::Fload_2 => fload_2(locals, stack),
            Instruction::Fload_3 => fload_3(locals, stack),
            Instruction::Dload_0 => dload_0(locals, stack),
            Instruction::Dload_1 => dload_1(locals, stack),
            Instruction::Dload_2 => dload_2(locals, stack),
            Instruction::Dload_3 => dload_3(locals, stack),
            Instruction::Aload_0 => aload_0(locals, stack),
            Instruction::Aload_1 => aload_1(locals, stack),
            Instruction::Aload_2 => aload_2(locals, stack),
            Instruction::Aload_3 => aload_3(locals, stack),
            Instruction::Iaload => iaload(stack),
            Instruction::Laload => laload(stack),
            Instruction::Faload => faload(stack),
            Instruction::Daload => daload(stack),
            Instruction::Aaload => aaload(stack),
            Instruction::Baload => baload(stack),
            Instruction::Caload => caload(stack),
            Instruction::Saload => saload(stack),
            Instruction::Istore(index) => istore(locals, stack, *index),
            Instruction::Lstore(index) => lstore(locals, stack, *index),
            Instruction::Fstore(index) => fstore(locals, stack, *index),
            Instruction::Dstore(index) => dstore(locals, stack, *index),
            Instruction::Astore(index) => astore(locals, stack, *index),
            Instruction::Istore_0 => istore_0(locals, stack),
            Instruction::Istore_1 => istore_1(locals, stack),
            Instruction::Istore_2 => istore_2(locals, stack),
            Instruction::Istore_3 => istore_3(locals, stack),
            Instruction::Lstore_0 => lstore_0(locals, stack),
            Instruction::Lstore_1 => lstore_1(locals, stack),
            Instruction::Lstore_2 => lstore_2(locals, stack),
            Instruction::Lstore_3 => lstore_3(locals, stack),
            Instruction::Fstore_0 => fstore_0(locals, stack),
            Instruction::Fstore_1 => fstore_1(locals, stack),
            Instruction::Fstore_2 => fstore_2(locals, stack),
            Instruction::Fstore_3 => fstore_3(locals, stack),
            Instruction::Dstore_0 => dstore_0(locals, stack),
            Instruction::Dstore_1 => dstore_1(locals, stack),
            Instruction::Dstore_2 => dstore_2(locals, stack),
            Instruction::Dstore_3 => dstore_3(locals, stack),
            Instruction::Astore_0 => astore_0(locals, stack),
            Instruction::Astore_1 => astore_1(locals, stack),
            Instruction::Astore_2 => astore_2(locals, stack),
            Instruction::Astore_3 => astore_3(locals, stack),
            Instruction::Iastore => iastore(stack),
            Instruction::Lastore => lastore(stack),
            Instruction::Fastore => fastore(stack),
            Instruction::Dastore => dastore(stack),
            Instruction::Aastore => aastore(stack),
            Instruction::Bastore => bastore(stack),
            Instruction::Castore => castore(stack),
            Instruction::Sastore => sastore(stack),
            Instruction::Pop => pop(stack),
            Instruction::Pop2 => pop2(stack),
            Instruction::Dup => dup(stack),
            Instruction::Dup_x1 => dup_x1(stack),
            Instruction::Dup_x2 => dup_x2(stack),
            Instruction::Dup2 => dup2(stack),
            Instruction::Dup2_x1 => dup2_x1(stack),
            Instruction::Dup2_x2 => dup2_x2(stack),
            Instruction::Swap => swap(stack),
            Instruction::Iadd => iadd(stack),
            Instruction::Ladd => ladd(stack),
            Instruction::Fadd => fadd(stack),
            Instruction::Dadd => dadd(stack),
            Instruction::Isub => isub(stack),
            Instruction::Lsub => lsub(stack),
            Instruction::Fsub => fsub(stack),
            Instruction::Dsub => dsub(stack),
            Instruction::Imul => imul(stack),
            Instruction::Lmul => lmul(stack),
            Instruction::Fmul => fmul(stack),
            Instruction::Dmul => dmul(stack),
            Instruction::Idiv => idiv(stack),
            Instruction::Ldiv => ldiv(stack),
            Instruction::Fdiv => fdiv(stack),
            Instruction::Ddiv => ddiv(stack),
            Instruction::Irem => irem(stack),
            Instruction::Lrem => lrem(stack),
            Instruction::Frem => frem(stack),
            Instruction::Drem => drem(stack),
            Instruction::Ineg => ineg(stack),
            Instruction::Lneg => lneg(stack),
            Instruction::Fneg => fneg(stack),
            Instruction::Dneg => dneg(stack),
            Instruction::Ishl => ishl(stack),
            Instruction::Lshl => lshl(stack),
            Instruction::Ishr => ishr(stack),
            Instruction::Lshr => lshr(stack),
            Instruction::Iushr => iushr(stack),
            Instruction::Lushr => lushr(stack),
            Instruction::Iand => iand(stack),
            Instruction::Land => land(stack),
            Instruction::Ior => ior(stack),
            Instruction::Lor => lor(stack),
            Instruction::Ixor => ixor(stack),
            Instruction::Lxor => lxor(stack),
            Instruction::Iinc(index, constant) => iinc(locals, *index, *constant),
            Instruction::I2l => i2l(stack),
            Instruction::I2f => i2f(stack),
            Instruction::I2d => i2d(stack),
            Instruction::L2i => l2i(stack),
            Instruction::L2f => l2f(stack),
            Instruction::L2d => l2d(stack),
            Instruction::F2i => f2i(stack),
            Instruction::F2l => f2l(stack),
            Instruction::F2d => f2d(stack),
            Instruction::D2i => d2i(stack),
            Instruction::D2l => d2l(stack),
            Instruction::D2f => d2f(stack),
            Instruction::I2b => i2b(stack),
            Instruction::I2c => i2c(stack),
            Instruction::I2s => i2s(stack),
            Instruction::Lcmp => lcmp(stack),
            Instruction::Fcmpl => fcmpl(stack),
            Instruction::Fcmpg => fcmpg(stack),
            Instruction::Dcmpl => dcmpl(stack),
            Instruction::Dcmpg => dcmpg(stack),
            Instruction::Ifeq(address) => ifeq(stack, *address),
            Instruction::Ifne(address) => ifne(stack, *address),
            Instruction::Iflt(address) => iflt(stack, *address),
            Instruction::Ifge(address) => ifge(stack, *address),
            Instruction::Ifgt(address) => ifgt(stack, *address),
            Instruction::Ifle(address) => ifle(stack, *address),
            Instruction::If_icmpeq(address) => if_icmpeq(stack, *address),
            Instruction::If_icmpne(address) => if_icmpne(stack, *address),
            Instruction::If_icmplt(address) => if_icmplt(stack, *address),
            Instruction::If_icmpge(address) => if_icmpge(stack, *address),
            Instruction::If_icmpgt(address) => if_icmpgt(stack, *address),
            Instruction::If_icmple(address) => if_icmple(stack, *address),
            Instruction::If_acmpeq(address) => if_acmpeq(stack, *address),
            Instruction::If_acmpne(address) => if_acmpne(stack, *address),
            Instruction::Goto(address) => goto(*address),
            Instruction::Jsr(address) => jsr(stack, *address),
            Instruction::Ret(index) => ret(locals, *index),
            Instruction::Tableswitch(TableSwitch {
                default,
                low,
                high,
                offsets,
            }) => {
                let program_counter = self.program_counter.load(Ordering::Relaxed);
                tableswitch(stack, program_counter, *default, *low, *high, offsets)
            }
            Instruction::Lookupswitch(LookupSwitch { default, pairs }) => {
                let program_counter = self.program_counter.load(Ordering::Relaxed);
                lookupswitch(stack, program_counter, *default, pairs)
            }
            Instruction::Ireturn => ireturn(stack),
            Instruction::Lreturn => lreturn(stack),
            Instruction::Freturn => freturn(stack),
            Instruction::Dreturn => dreturn(stack),
            Instruction::Areturn => areturn(stack),
            Instruction::Return => r#return(),
            Instruction::Getstatic(index) => Box::pin(getstatic(self, stack, *index)).await,
            Instruction::Putstatic(index) => Box::pin(putstatic(self, stack, *index)).await,
            Instruction::Getfield(index) => {
                Box::pin(getfield(self, stack, &self.class, *index)).await
            }
            Instruction::Putfield(index) => {
                Box::pin(putfield(self, stack, &self.class, *index)).await
            }
            Instruction::Invokevirtual(index) => Box::pin(invokevirtual(self, stack, *index)).await,
            Instruction::Invokespecial(index) => Box::pin(invokespecial(self, stack, *index)).await,
            Instruction::Invokestatic(index) => Box::pin(invokestatic(self, stack, *index)).await,
            Instruction::Invokeinterface(index, count) => {
                Box::pin(invokeinterface(self, stack, *index, *count)).await
            }
            Instruction::Invokedynamic(index) => Box::pin(invokedynamic(self, stack, *index)).await,
            Instruction::New(index) => Box::pin(new(self, stack, *index)).await,
            Instruction::Newarray(array_type) => newarray(stack, array_type),
            Instruction::Anewarray(index) => Box::pin(anewarray(self, stack, *index)).await,
            Instruction::Arraylength => arraylength(stack),
            Instruction::Athrow => Box::pin(athrow(stack)).await,
            Instruction::Checkcast(class_index) => {
                Box::pin(checkcast(self, stack, *class_index)).await
            }
            Instruction::Instanceof(class_index) => {
                Box::pin(instanceof(self, stack, *class_index)).await
            }
            Instruction::Monitorenter => monitorenter(stack),
            Instruction::Monitorexit => monitorexit(stack),
            Instruction::Wide => wide(),
            Instruction::Multianewarray(index, dimensions) => {
                Box::pin(multianewarray(self, stack, *index, *dimensions)).await
            }
            Instruction::Ifnull(address) => ifnull(stack, *address),
            Instruction::Ifnonnull(address) => ifnonnull(stack, *address),
            Instruction::Goto_w(address) => goto_w(*address),
            Instruction::Jsr_w(address) => jsr_w(stack, *address),
            Instruction::Breakpoint => breakpoint(),
            Instruction::Impdep1 => impdep1(),
            Instruction::Impdep2 => impdep2(),
            // Wide instructions
            Instruction::Iload_w(index) => iload_w(locals, stack, *index),
            Instruction::Lload_w(index) => lload_w(locals, stack, *index),
            Instruction::Fload_w(index) => fload_w(locals, stack, *index),
            Instruction::Dload_w(index) => dload_w(locals, stack, *index),
            Instruction::Aload_w(index) => aload_w(locals, stack, *index),
            Instruction::Istore_w(index) => istore_w(locals, stack, *index),
            Instruction::Lstore_w(index) => lstore_w(locals, stack, *index),
            Instruction::Fstore_w(index) => fstore_w(locals, stack, *index),
            Instruction::Dstore_w(index) => dstore_w(locals, stack, *index),
            Instruction::Astore_w(index) => astore_w(locals, stack, *index),
            Instruction::Iinc_w(index, constant) => iinc_w(locals, *index, *constant),
            Instruction::Ret_w(index) => ret_w(locals, *index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VM;
    use crate::configuration::ConfigurationBuilder;
    use crate::thread::Thread;
    use ristretto_classloader::ClassPath;
    use std::path::PathBuf;

    async fn get_class(class_name: &str) -> Result<(Arc<Thread>, Arc<Class>)> {
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
        Ok((thread, class))
    }

    #[tokio::test]
    async fn test_execute() -> Result<()> {
        let (thread, class) = get_class("java.lang.Math").await?;
        let method = class.method("addExact", "(II)I").expect("method not found");
        let parameters = vec![Value::Int(1), Value::Int(2)];
        let frame = Frame::new(&Arc::downgrade(&thread), &class, &method);
        let result = frame.execute(parameters).await?;
        assert!(matches!(result, Some(Value::Int(3))));
        Ok(())
    }

    #[test]
    fn test_adjust_parameters() {
        let mut parameters = vec![
            Value::Int(1),
            Value::Long(2),
            Value::Float(3.0),
            Value::Double(4.0),
        ];
        Frame::adjust_parameters(&mut parameters, 8);
        assert_eq!(
            parameters,
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

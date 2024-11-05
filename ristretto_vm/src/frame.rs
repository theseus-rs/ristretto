use crate::frame::ExecutionResult::{Continue, ContinueAtPosition, Return};
use crate::instruction::{
    aaload, aastore, aconst_null, aload, aload_0, aload_1, aload_2, aload_3, aload_w, anewarray,
    areturn, arraylength, astore, astore_0, astore_1, astore_2, astore_3, astore_w, athrow, baload,
    bastore, bipush, caload, castore, checkcast, convert_error_to_throwable, d2f, d2i, d2l, dadd,
    daload, dastore, dcmpg, dcmpl, dconst_0, dconst_1, ddiv, dload, dload_0, dload_1, dload_2,
    dload_3, dload_w, dmul, dneg, drem, dreturn, dstore, dstore_0, dstore_1, dstore_2, dstore_3,
    dstore_w, dsub, dup, dup2, dup2_x1, dup2_x2, dup_x1, dup_x2, f2d, f2i, f2l, fadd, faload,
    fastore, fcmpg, fcmpl, fconst_0, fconst_1, fconst_2, fdiv, fload, fload_0, fload_1, fload_2,
    fload_3, fload_w, fmul, fneg, frem, freturn, fstore, fstore_0, fstore_1, fstore_2, fstore_3,
    fstore_w, fsub, getfield, getstatic, goto, goto_w, i2b, i2c, i2d, i2f, i2l, i2s, iadd, iaload,
    iand, iastore, iconst_0, iconst_1, iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, idiv,
    if_acmpeq, if_acmpne, if_icmpeq, if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne, ifeq,
    ifge, ifgt, ifle, iflt, ifne, ifnonnull, ifnull, iinc, iinc_w, iload, iload_0, iload_1,
    iload_2, iload_3, iload_w, imul, ineg, instanceof, invokedynamic, invokeinterface,
    invokespecial, invokestatic, invokevirtual, ior, irem, ireturn, ishl, ishr, istore, istore_0,
    istore_1, istore_2, istore_3, istore_w, isub, iushr, ixor, jsr, jsr_w, l2d, l2f, l2i, ladd,
    laload, land, lastore, lcmp, lconst_0, lconst_1, ldc, ldc2_w, ldc_w, ldiv, lload, lload_0,
    lload_1, lload_2, lload_3, lload_w, lmul, lneg, lookupswitch, lor, lrem, lreturn, lshl, lshr,
    lstore, lstore_0, lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr, lxor, multianewarray,
    new, newarray, pop, pop2, process_throwable, putfield, putstatic, r#return, ret, ret_w, saload,
    sastore, sipush, swap, tableswitch,
};
use crate::Error::{InternalError, InvalidOperand, InvalidProgramCounter};
use crate::{LocalVariables, OperandStack, Result, Thread};
use async_recursion::async_recursion;
use byte_unit::{Byte, UnitType};
use ristretto_classfile::attributes::Instruction;
use ristretto_classloader::{Class, Method, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use tracing::{debug, event_enabled, Level};

#[derive(Debug, PartialEq)]
pub(crate) enum ExecutionResult {
    Return(Option<Value>),
    Continue,
    ContinueAtPosition(usize),
}

/// A frame stores data and partial results, performs dynamic linking, returns method values, and
/// dispatches exceptions.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6>
#[derive(Debug)]
pub(crate) struct Frame {
    thread: Weak<Thread>,
    class: Arc<Class>,
    method: Arc<Method>,
    locals: Arc<LocalVariables>,
    stack: Arc<OperandStack>,
    program_counter: AtomicUsize,
}

impl Frame {
    /// Create a new frame for the specified class. To invoke a method on an object reference, the
    /// object reference must be the first argument in the arguments vector.
    pub fn new(
        thread: &Weak<Thread>,
        class: &Arc<Class>,
        method: &Arc<Method>,
        arguments: Vec<Value>,
    ) -> Result<Self> {
        let max_locals = method.max_locals();
        let locals = LocalVariables::with_max_size(max_locals);
        for (index, argument) in arguments.into_iter().enumerate() {
            locals.set(index, argument)?;
        }
        let max_stack = method.max_stack();
        let stack = OperandStack::with_max_size(max_stack);
        Ok(Frame {
            thread: thread.clone(),
            class: class.clone(),
            method: method.clone(),
            locals: Arc::new(locals),
            stack: Arc::new(stack),
            program_counter: AtomicUsize::new(0),
        })
    }

    /// Get the thread that owns this frame.
    ///
    /// # Errors
    /// if the thread is not available.
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

    /// Get the method in this frame
    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    /// Get the local variables in this frame
    pub fn locals(&self) -> &LocalVariables {
        &self.locals
    }

    /// Get the operand stack in this frame
    pub fn stack(&self) -> &OperandStack {
        &self.stack
    }

    /// Get the program counter in this frame
    #[inline]
    pub fn program_counter(&self) -> usize {
        self.program_counter.load(Ordering::Relaxed)
    }

    /// Execute the method in this frame
    ///
    /// # Errors
    /// * if the program counter is invalid
    /// * if an invalid instruction is encountered
    #[async_recursion(?Send)]
    pub async fn execute(&self) -> Result<Option<Value>> {
        let code = self.method.code();

        loop {
            let program_counter = self.program_counter.load(Ordering::Relaxed);
            let Some(instruction) = code.get(program_counter) else {
                return Err(InvalidProgramCounter(program_counter));
            };

            if event_enabled!(Level::DEBUG) {
                self.debug_execute(instruction)?;
            }

            let result = self.process(instruction).await;
            match result {
                Ok(Continue) => {
                    self.program_counter
                        .store(program_counter + 1, Ordering::Relaxed);
                }
                Ok(ContinueAtPosition(program_counter)) => {
                    self.program_counter
                        .store(program_counter, Ordering::Relaxed);
                }
                Ok(Return(value)) => return Ok(value.clone()),
                Err(error) => {
                    let vm = self.thread()?.vm()?;
                    let throwable = convert_error_to_throwable(vm, error).await?;
                    let handler_program_counter = process_throwable(self, throwable).await?;
                    self.program_counter
                        .store(handler_program_counter, Ordering::Relaxed);
                }
            }
        }
    }

    /// Debug the execution of an instruction in this frame
    fn debug_execute(&self, instruction: &Instruction) -> Result<()> {
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
        debug!("    locals: {}", self.locals);
        debug!("    stack ({stack_size:#.3}): {}", self.stack);
        debug!("    pc: {program_counter}; instruction: {instruction}");
        Ok(())
    }

    /// Process an instruction in this frame
    #[expect(clippy::too_many_lines)]
    async fn process(&self, instruction: &Instruction) -> Result<ExecutionResult> {
        match instruction {
            Instruction::Nop => Ok(Continue), // Do nothing
            Instruction::Aconst_null => aconst_null(&self.stack),
            Instruction::Iconst_m1 => iconst_m1(&self.stack),
            Instruction::Iconst_0 => iconst_0(&self.stack),
            Instruction::Iconst_1 => iconst_1(&self.stack),
            Instruction::Iconst_2 => iconst_2(&self.stack),
            Instruction::Iconst_3 => iconst_3(&self.stack),
            Instruction::Iconst_4 => iconst_4(&self.stack),
            Instruction::Iconst_5 => iconst_5(&self.stack),
            Instruction::Lconst_0 => lconst_0(&self.stack),
            Instruction::Lconst_1 => lconst_1(&self.stack),
            Instruction::Fconst_0 => fconst_0(&self.stack),
            Instruction::Fconst_1 => fconst_1(&self.stack),
            Instruction::Fconst_2 => fconst_2(&self.stack),
            Instruction::Dconst_0 => dconst_0(&self.stack),
            Instruction::Dconst_1 => dconst_1(&self.stack),
            Instruction::Bipush(value) => bipush(&self.stack, *value),
            Instruction::Sipush(value) => sipush(&self.stack, *value),
            Instruction::Ldc(index) => ldc(self, *index).await,
            Instruction::Ldc_w(index) => ldc_w(self, *index).await,
            Instruction::Ldc2_w(index) => ldc2_w(self, *index),
            Instruction::Iload(index) => iload(&self.locals, &self.stack, *index),
            Instruction::Lload(index) => lload(&self.locals, &self.stack, *index),
            Instruction::Fload(index) => fload(&self.locals, &self.stack, *index),
            Instruction::Dload(index) => dload(&self.locals, &self.stack, *index),
            Instruction::Aload(index) => aload(&self.locals, &self.stack, *index),
            Instruction::Iload_0 => iload_0(&self.locals, &self.stack),
            Instruction::Iload_1 => iload_1(&self.locals, &self.stack),
            Instruction::Iload_2 => iload_2(&self.locals, &self.stack),
            Instruction::Iload_3 => iload_3(&self.locals, &self.stack),
            Instruction::Lload_0 => lload_0(&self.locals, &self.stack),
            Instruction::Lload_1 => lload_1(&self.locals, &self.stack),
            Instruction::Lload_2 => lload_2(&self.locals, &self.stack),
            Instruction::Lload_3 => lload_3(&self.locals, &self.stack),
            Instruction::Fload_0 => fload_0(&self.locals, &self.stack),
            Instruction::Fload_1 => fload_1(&self.locals, &self.stack),
            Instruction::Fload_2 => fload_2(&self.locals, &self.stack),
            Instruction::Fload_3 => fload_3(&self.locals, &self.stack),
            Instruction::Dload_0 => dload_0(&self.locals, &self.stack),
            Instruction::Dload_1 => dload_1(&self.locals, &self.stack),
            Instruction::Dload_2 => dload_2(&self.locals, &self.stack),
            Instruction::Dload_3 => dload_3(&self.locals, &self.stack),
            Instruction::Aload_0 => aload_0(&self.locals, &self.stack),
            Instruction::Aload_1 => aload_1(&self.locals, &self.stack),
            Instruction::Aload_2 => aload_2(&self.locals, &self.stack),
            Instruction::Aload_3 => aload_3(&self.locals, &self.stack),
            Instruction::Iaload => iaload(&self.stack),
            Instruction::Laload => laload(&self.stack),
            Instruction::Faload => faload(&self.stack),
            Instruction::Daload => daload(&self.stack),
            Instruction::Aaload => aaload(&self.stack),
            Instruction::Baload => baload(&self.stack),
            Instruction::Caload => caload(&self.stack),
            Instruction::Saload => saload(&self.stack),
            Instruction::Istore(index) => istore(&self.locals, &self.stack, *index),
            Instruction::Lstore(index) => lstore(&self.locals, &self.stack, *index),
            Instruction::Fstore(index) => fstore(&self.locals, &self.stack, *index),
            Instruction::Dstore(index) => dstore(&self.locals, &self.stack, *index),
            Instruction::Astore(index) => astore(&self.locals, &self.stack, *index),
            Instruction::Istore_0 => istore_0(&self.locals, &self.stack),
            Instruction::Istore_1 => istore_1(&self.locals, &self.stack),
            Instruction::Istore_2 => istore_2(&self.locals, &self.stack),
            Instruction::Istore_3 => istore_3(&self.locals, &self.stack),
            Instruction::Lstore_0 => lstore_0(&self.locals, &self.stack),
            Instruction::Lstore_1 => lstore_1(&self.locals, &self.stack),
            Instruction::Lstore_2 => lstore_2(&self.locals, &self.stack),
            Instruction::Lstore_3 => lstore_3(&self.locals, &self.stack),
            Instruction::Fstore_0 => fstore_0(&self.locals, &self.stack),
            Instruction::Fstore_1 => fstore_1(&self.locals, &self.stack),
            Instruction::Fstore_2 => fstore_2(&self.locals, &self.stack),
            Instruction::Fstore_3 => fstore_3(&self.locals, &self.stack),
            Instruction::Dstore_0 => dstore_0(&self.locals, &self.stack),
            Instruction::Dstore_1 => dstore_1(&self.locals, &self.stack),
            Instruction::Dstore_2 => dstore_2(&self.locals, &self.stack),
            Instruction::Dstore_3 => dstore_3(&self.locals, &self.stack),
            Instruction::Astore_0 => astore_0(&self.locals, &self.stack),
            Instruction::Astore_1 => astore_1(&self.locals, &self.stack),
            Instruction::Astore_2 => astore_2(&self.locals, &self.stack),
            Instruction::Astore_3 => astore_3(&self.locals, &self.stack),
            Instruction::Iastore => iastore(&self.stack),
            Instruction::Lastore => lastore(&self.stack),
            Instruction::Fastore => fastore(&self.stack),
            Instruction::Dastore => dastore(&self.stack),
            Instruction::Aastore => aastore(&self.stack),
            Instruction::Bastore => bastore(&self.stack),
            Instruction::Castore => castore(&self.stack),
            Instruction::Sastore => sastore(&self.stack),
            Instruction::Pop => pop(&self.stack),
            Instruction::Pop2 => pop2(&self.stack),
            Instruction::Dup => dup(&self.stack),
            Instruction::Dup_x1 => dup_x1(&self.stack),
            Instruction::Dup_x2 => dup_x2(&self.stack),
            Instruction::Dup2 => dup2(&self.stack),
            Instruction::Dup2_x1 => dup2_x1(&self.stack),
            Instruction::Dup2_x2 => dup2_x2(&self.stack),
            Instruction::Swap => swap(&self.stack),
            Instruction::Iadd => iadd(&self.stack),
            Instruction::Ladd => ladd(&self.stack),
            Instruction::Fadd => fadd(&self.stack),
            Instruction::Dadd => dadd(&self.stack),
            Instruction::Isub => isub(&self.stack),
            Instruction::Lsub => lsub(&self.stack),
            Instruction::Fsub => fsub(&self.stack),
            Instruction::Dsub => dsub(&self.stack),
            Instruction::Imul => imul(&self.stack),
            Instruction::Lmul => lmul(&self.stack),
            Instruction::Fmul => fmul(&self.stack),
            Instruction::Dmul => dmul(&self.stack),
            Instruction::Idiv => idiv(&self.stack),
            Instruction::Ldiv => ldiv(&self.stack),
            Instruction::Fdiv => fdiv(&self.stack),
            Instruction::Ddiv => ddiv(&self.stack),
            Instruction::Irem => irem(&self.stack),
            Instruction::Lrem => lrem(&self.stack),
            Instruction::Frem => frem(&self.stack),
            Instruction::Drem => drem(&self.stack),
            Instruction::Ineg => ineg(&self.stack),
            Instruction::Lneg => lneg(&self.stack),
            Instruction::Fneg => fneg(&self.stack),
            Instruction::Dneg => dneg(&self.stack),
            Instruction::Ishl => ishl(&self.stack),
            Instruction::Lshl => lshl(&self.stack),
            Instruction::Ishr => ishr(&self.stack),
            Instruction::Lshr => lshr(&self.stack),
            Instruction::Iushr => iushr(&self.stack),
            Instruction::Lushr => lushr(&self.stack),
            Instruction::Iand => iand(&self.stack),
            Instruction::Land => land(&self.stack),
            Instruction::Ior => ior(&self.stack),
            Instruction::Lor => lor(&self.stack),
            Instruction::Ixor => ixor(&self.stack),
            Instruction::Lxor => lxor(&self.stack),
            Instruction::Iinc(index, constant) => iinc(&self.locals, *index, *constant),
            Instruction::I2l => i2l(&self.stack),
            Instruction::I2f => i2f(&self.stack),
            Instruction::I2d => i2d(&self.stack),
            Instruction::L2i => l2i(&self.stack),
            Instruction::L2f => l2f(&self.stack),
            Instruction::L2d => l2d(&self.stack),
            Instruction::F2i => f2i(&self.stack),
            Instruction::F2l => f2l(&self.stack),
            Instruction::F2d => f2d(&self.stack),
            Instruction::D2i => d2i(&self.stack),
            Instruction::D2l => d2l(&self.stack),
            Instruction::D2f => d2f(&self.stack),
            Instruction::I2b => i2b(&self.stack),
            Instruction::I2c => i2c(&self.stack),
            Instruction::I2s => i2s(&self.stack),
            Instruction::Lcmp => lcmp(&self.stack),
            Instruction::Fcmpl => fcmpl(&self.stack),
            Instruction::Fcmpg => fcmpg(&self.stack),
            Instruction::Dcmpl => dcmpl(&self.stack),
            Instruction::Dcmpg => dcmpg(&self.stack),
            Instruction::Ifeq(address) => ifeq(&self.stack, *address),
            Instruction::Ifne(address) => ifne(&self.stack, *address),
            Instruction::Iflt(address) => iflt(&self.stack, *address),
            Instruction::Ifge(address) => ifge(&self.stack, *address),
            Instruction::Ifgt(address) => ifgt(&self.stack, *address),
            Instruction::Ifle(address) => ifle(&self.stack, *address),
            Instruction::If_icmpeq(address) => if_icmpeq(&self.stack, *address),
            Instruction::If_icmpne(address) => if_icmpne(&self.stack, *address),
            Instruction::If_icmplt(address) => if_icmplt(&self.stack, *address),
            Instruction::If_icmpge(address) => if_icmpge(&self.stack, *address),
            Instruction::If_icmpgt(address) => if_icmpgt(&self.stack, *address),
            Instruction::If_icmple(address) => if_icmple(&self.stack, *address),
            Instruction::If_acmpeq(address) => if_acmpeq(&self.stack, *address),
            Instruction::If_acmpne(address) => if_acmpne(&self.stack, *address),
            Instruction::Goto(address) => goto(*address),
            Instruction::Jsr(address) => jsr(&self.stack, *address),
            Instruction::Ret(index) => ret(&self.locals, *index),
            Instruction::Tableswitch {
                default,
                low,
                high,
                offsets,
            } => {
                let program_counter = self.program_counter.load(Ordering::Relaxed);
                tableswitch(&self.stack, program_counter, *default, *low, *high, offsets)
            }
            Instruction::Lookupswitch { default, pairs } => {
                let program_counter = self.program_counter.load(Ordering::Relaxed);
                lookupswitch(&self.stack, program_counter, *default, pairs)
            }
            Instruction::Ireturn => ireturn(&self.stack),
            Instruction::Lreturn => lreturn(&self.stack),
            Instruction::Freturn => freturn(&self.stack),
            Instruction::Dreturn => dreturn(&self.stack),
            Instruction::Areturn => areturn(&self.stack),
            Instruction::Return => r#return(),
            Instruction::Getstatic(index) => getstatic(self, *index).await,
            Instruction::Putstatic(index) => putstatic(self, *index).await,
            Instruction::Getfield(index) => getfield(&self.stack, &self.class, *index),
            Instruction::Putfield(index) => putfield(&self.stack, &self.class, *index),
            Instruction::Invokevirtual(index) => invokevirtual(self, *index).await,
            Instruction::Invokespecial(index) => invokespecial(self, *index).await,
            Instruction::Invokestatic(index) => invokestatic(self, *index).await,
            Instruction::Invokeinterface(index, count) => {
                invokeinterface(self, *index, *count).await
            }
            Instruction::Invokedynamic(index) => invokedynamic(self, *index).await,
            Instruction::New(index) => new(self, *index).await,
            Instruction::Newarray(array_type) => newarray(&self.stack, array_type),
            Instruction::Anewarray(index) => anewarray(self, *index).await,
            Instruction::Arraylength => arraylength(&self.stack),
            Instruction::Athrow => athrow(self).await,
            Instruction::Checkcast(class_index) => checkcast(self, *class_index).await,
            Instruction::Instanceof(class_index) => instanceof(self, *class_index).await,
            Instruction::Monitorenter | Instruction::Monitorexit => {
                // The monitorenter and monitorexit instructions are not currently used by this
                // implementation.
                // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.monitorenter
                // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.monitorexit
                let _ = self.stack.pop_object()?;
                Ok(Continue)
            }
            Instruction::Wide => {
                // The wide instruction is not directly used by this implementation.  The wide
                // versions of instructions are specifically enumerated in the instruction set.
                // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide
                Err(InvalidOperand {
                    expected: "*_w instruction".to_string(),
                    actual: "Wide".to_string(),
                })
            }
            Instruction::Multianewarray(index, dimensions) => {
                multianewarray(self, *index, *dimensions).await
            }
            Instruction::Ifnull(address) => ifnull(&self.stack, *address),
            Instruction::Ifnonnull(address) => ifnonnull(&self.stack, *address),
            Instruction::Goto_w(address) => goto_w(*address),
            Instruction::Jsr_w(address) => jsr_w(&self.stack, *address),
            Instruction::Breakpoint | Instruction::Impdep1 | Instruction::Impdep2 => {
                // Breakpoint, Impdep1 and Impdep2 instructions are reserved for debugging and implementation
                // dependent operations.
                // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.2
                Ok(Continue)
            }
            // Wide instructions
            Instruction::Iload_w(index) => iload_w(&self.locals, &self.stack, *index),
            Instruction::Lload_w(index) => lload_w(&self.locals, &self.stack, *index),
            Instruction::Fload_w(index) => fload_w(&self.locals, &self.stack, *index),
            Instruction::Dload_w(index) => dload_w(&self.locals, &self.stack, *index),
            Instruction::Aload_w(index) => aload_w(&self.locals, &self.stack, *index),
            Instruction::Istore_w(index) => istore_w(&self.locals, &self.stack, *index),
            Instruction::Lstore_w(index) => lstore_w(&self.locals, &self.stack, *index),
            Instruction::Fstore_w(index) => fstore_w(&self.locals, &self.stack, *index),
            Instruction::Dstore_w(index) => dstore_w(&self.locals, &self.stack, *index),
            Instruction::Astore_w(index) => astore_w(&self.locals, &self.stack, *index),
            Instruction::Iinc_w(index, constant) => iinc_w(&self.locals, *index, *constant),
            Instruction::Ret_w(index) => ret_w(&self.locals, *index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::ConfigurationBuilder;
    use crate::thread::Thread;
    use crate::VM;
    use ristretto_classloader::ClassPath;
    use std::path::PathBuf;

    async fn get_class(class_name: &str) -> Result<(Arc<Thread>, Arc<Class>)> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_path = cargo_manifest.join("../classes");
        let class_path = ClassPath::from(classes_path.to_string_lossy());
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .build()?;
        let vm = VM::new(configuration).await?;
        let thread = Thread::new(&Arc::downgrade(&vm));
        let class = vm.load_class(&thread, class_name).await?;
        Ok((thread, class))
    }

    #[tokio::test]
    async fn test_execute() -> Result<()> {
        let (thread, class) = get_class("Expressions").await?;
        let method = class.method("add", "(II)I").expect("method not found");
        let arguments = vec![Value::Int(1), Value::Int(2)];
        let frame = Frame::new(&Arc::downgrade(&thread), &class, &method, arguments)?;
        let result = frame.execute().await?;
        assert!(matches!(result, Some(Value::Int(3))));
        Ok(())
    }

    #[tokio::test]
    async fn test_initial_frame() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        assert!(frame.locals.is_empty()?);
        assert!(frame.stack.is_empty()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_nop() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let process_result = frame.process(&Instruction::Nop).await?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty()?);
        assert!(frame.stack.is_empty()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_monitorenter() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        frame.stack.push_object(None)?;
        let process_result = frame.process(&Instruction::Monitorenter).await?;
        assert_eq!(Continue, process_result);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_monitorexit() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        frame.stack.push_object(None)?;
        let process_result = frame.process(&Instruction::Monitorexit).await?;
        assert_eq!(Continue, process_result);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_wide() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let result = frame.process(&Instruction::Wide).await;
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "*_w instruction" && actual == "Wide"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_process_breakpoint() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let process_result = frame.process(&Instruction::Breakpoint).await?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty()?);
        assert!(frame.stack.is_empty()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_impdep1() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let process_result = frame.process(&Instruction::Impdep1).await?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty()?);
        assert!(frame.stack.is_empty()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_impdep2() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let process_result = frame.process(&Instruction::Impdep2).await?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty()?);
        assert!(frame.stack.is_empty()?);
        Ok(())
    }
}

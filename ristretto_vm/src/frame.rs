use crate::frame::ExecutionResult::{Continue, ContinueAtPosition, Return};
use crate::instruction::{
    aaload, aastore, aconst_null, aload, aload_0, aload_1, aload_2, aload_3, aload_w, anewarray,
    areturn, arraylength, astore, astore_0, astore_1, astore_2, astore_3, astore_w, baload,
    bastore, bipush, caload, castore, checkcast, d2f, d2i, d2l, dadd, daload, dastore, dcmpg,
    dcmpl, dconst_0, dconst_1, ddiv, dload, dload_0, dload_1, dload_2, dload_3, dload_w, dmul,
    dneg, drem, dreturn, dstore, dstore_0, dstore_1, dstore_2, dstore_3, dstore_w, dsub, dup, dup2,
    dup2_x1, dup2_x2, dup_x1, dup_x2, f2d, f2i, f2l, fadd, faload, fastore, fcmpg, fcmpl, fconst_0,
    fconst_1, fconst_2, fdiv, fload, fload_0, fload_1, fload_2, fload_3, fload_w, fmul, fneg, frem,
    freturn, fstore, fstore_0, fstore_1, fstore_2, fstore_3, fstore_w, fsub, getfield, getstatic,
    goto, goto_w, i2b, i2c, i2d, i2f, i2l, i2s, iadd, iaload, iand, iastore, iconst_0, iconst_1,
    iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, idiv, if_acmpeq, if_acmpne, if_icmpeq,
    if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne, ifeq, ifge, ifgt, ifle, iflt, ifne,
    ifnonnull, ifnull, iinc, iinc_w, iload, iload_0, iload_1, iload_2, iload_3, iload_w, imul,
    ineg, instanceof, invokedynamic, invokeinterface, invokespecial, invokestatic, invokevirtual,
    ior, irem, ireturn, ishl, ishr, istore, istore_0, istore_1, istore_2, istore_3, istore_w, isub,
    iushr, ixor, jsr, jsr_w, l2d, l2f, l2i, ladd, laload, land, lastore, lcmp, lconst_0, lconst_1,
    ldc, ldc2_w, ldc_w, ldiv, lload, lload_0, lload_1, lload_2, lload_3, lload_w, lmul, lneg,
    lookupswitch, lor, lrem, lreturn, lshl, lshr, lstore, lstore_0, lstore_1, lstore_2, lstore_3,
    lstore_w, lsub, lushr, lxor, multianewarray, new, newarray, pop, pop2, putfield, putstatic,
    ret, ret_w, saload, sastore, sipush, swap, tableswitch,
};
use crate::Error::{InvalidOperand, InvalidProgramCounter};
use crate::{CallStack, LocalVariables, OperandStack, Result, VM};
use ristretto_classfile::attributes::Instruction;
use ristretto_classloader::{Class, Method, Value};
use std::sync::Arc;
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
#[derive(Clone, Debug)]
pub(crate) struct Frame {
    pub(crate) class: Arc<Class>,
    pub(crate) method: Arc<Method>,
    pub(crate) locals: LocalVariables,
    pub(crate) stack: OperandStack,
    pub(crate) program_counter: usize,
}

impl Frame {
    /// Create a new frame for the specified class. To invoke a method on an object reference, the
    /// object reference must be the first argument in the arguments vector.
    pub fn new(class: &Arc<Class>, method: &Arc<Method>, arguments: Vec<Value>) -> Result<Self> {
        let max_locals = method.max_locals();
        let mut locals = LocalVariables::with_max_size(max_locals);
        for (index, argument) in arguments.into_iter().enumerate() {
            locals.set(index, argument)?;
        }
        let max_stack = method.max_stack();
        let stack = OperandStack::with_max_size(max_stack);
        Ok(Frame {
            class: class.clone(),
            method: method.clone(),
            locals,
            stack,
            program_counter: 0,
        })
    }

    /// Execute the method in this frame
    ///
    /// # Errors
    /// * if the program counter is invalid
    /// * if an invalid instruction is encountered
    pub fn execute(&mut self, vm: &VM, call_stack: &mut CallStack) -> Result<Option<Value>> {
        // TODO: avoid cloning code
        let code = self.method.code().clone();

        loop {
            let Some(instruction) = code.get(self.program_counter) else {
                return Err(InvalidProgramCounter(self.program_counter));
            };

            if event_enabled!(Level::DEBUG) {
                self.debug_execute(instruction)?;
            }

            let result = self.process(vm, call_stack, instruction);
            match result {
                Ok(Continue) => self.program_counter += 1,
                Ok(ContinueAtPosition(pc)) => self.program_counter = pc,
                Ok(Return(value)) => return Ok(value.clone()),
                Err(error) => {
                    // TODO: implement exception handling
                    return Err(error);
                }
            }
        }
    }

    /// Debug the execution of an instruction in this frame
    #[inline]
    fn debug_execute(&self, instruction: &Instruction) -> Result<()> {
        let class_name = self.class.name();
        let method_name = self.method.name();
        let method_descriptor = self.method.descriptor();
        let source = if let Some(source_file) = self.class.source_file() {
            let line_number = self.method.line_number(self.program_counter);
            format!(" ({source_file}:{line_number})")
        } else {
            String::new()
        };
        let constant_pool = self.class.constant_pool();
        let instruction = instruction.to_formatted_string(constant_pool)?;
        debug!("  frame: {class_name}.{method_name}{method_descriptor}{source}");
        debug!("    locals: {}", self.locals);
        debug!("    stack: {}", self.stack);
        debug!(
            "    pc: {}; instruction: {instruction}",
            self.program_counter
        );
        Ok(())
    }

    /// Process an instruction in this frame
    #[expect(clippy::too_many_lines)]
    fn process(
        &mut self,
        vm: &VM,
        call_stack: &mut CallStack,
        instruction: &Instruction,
    ) -> Result<ExecutionResult> {
        match instruction {
            Instruction::Nop => Ok(Continue), // Do nothing
            Instruction::Aconst_null => aconst_null(&mut self.stack),
            Instruction::Iconst_m1 => iconst_m1(&mut self.stack),
            Instruction::Iconst_0 => iconst_0(&mut self.stack),
            Instruction::Iconst_1 => iconst_1(&mut self.stack),
            Instruction::Iconst_2 => iconst_2(&mut self.stack),
            Instruction::Iconst_3 => iconst_3(&mut self.stack),
            Instruction::Iconst_4 => iconst_4(&mut self.stack),
            Instruction::Iconst_5 => iconst_5(&mut self.stack),
            Instruction::Lconst_0 => lconst_0(&mut self.stack),
            Instruction::Lconst_1 => lconst_1(&mut self.stack),
            Instruction::Fconst_0 => fconst_0(&mut self.stack),
            Instruction::Fconst_1 => fconst_1(&mut self.stack),
            Instruction::Fconst_2 => fconst_2(&mut self.stack),
            Instruction::Dconst_0 => dconst_0(&mut self.stack),
            Instruction::Dconst_1 => dconst_1(&mut self.stack),
            Instruction::Bipush(value) => bipush(&mut self.stack, *value),
            Instruction::Sipush(value) => sipush(&mut self.stack, *value),
            Instruction::Ldc(index) => ldc(vm, call_stack, self, *index),
            Instruction::Ldc_w(index) => ldc_w(vm, call_stack, self, *index),
            Instruction::Ldc2_w(index) => ldc2_w(self, *index),
            Instruction::Iload(index) => iload(&self.locals, &mut self.stack, *index),
            Instruction::Lload(index) => lload(&self.locals, &mut self.stack, *index),
            Instruction::Fload(index) => fload(&self.locals, &mut self.stack, *index),
            Instruction::Dload(index) => dload(&self.locals, &mut self.stack, *index),
            Instruction::Aload(index) => aload(&self.locals, &mut self.stack, *index),
            Instruction::Iload_0 => iload_0(&self.locals, &mut self.stack),
            Instruction::Iload_1 => iload_1(&self.locals, &mut self.stack),
            Instruction::Iload_2 => iload_2(&self.locals, &mut self.stack),
            Instruction::Iload_3 => iload_3(&self.locals, &mut self.stack),
            Instruction::Lload_0 => lload_0(&self.locals, &mut self.stack),
            Instruction::Lload_1 => lload_1(&self.locals, &mut self.stack),
            Instruction::Lload_2 => lload_2(&self.locals, &mut self.stack),
            Instruction::Lload_3 => lload_3(&self.locals, &mut self.stack),
            Instruction::Fload_0 => fload_0(&self.locals, &mut self.stack),
            Instruction::Fload_1 => fload_1(&self.locals, &mut self.stack),
            Instruction::Fload_2 => fload_2(&self.locals, &mut self.stack),
            Instruction::Fload_3 => fload_3(&self.locals, &mut self.stack),
            Instruction::Dload_0 => dload_0(&self.locals, &mut self.stack),
            Instruction::Dload_1 => dload_1(&self.locals, &mut self.stack),
            Instruction::Dload_2 => dload_2(&self.locals, &mut self.stack),
            Instruction::Dload_3 => dload_3(&self.locals, &mut self.stack),
            Instruction::Aload_0 => aload_0(&self.locals, &mut self.stack),
            Instruction::Aload_1 => aload_1(&self.locals, &mut self.stack),
            Instruction::Aload_2 => aload_2(&self.locals, &mut self.stack),
            Instruction::Aload_3 => aload_3(&self.locals, &mut self.stack),
            Instruction::Iaload => iaload(&mut self.stack),
            Instruction::Laload => laload(&mut self.stack),
            Instruction::Faload => faload(&mut self.stack),
            Instruction::Daload => daload(&mut self.stack),
            Instruction::Aaload => aaload(&mut self.stack),
            Instruction::Baload => baload(&mut self.stack),
            Instruction::Caload => caload(&mut self.stack),
            Instruction::Saload => saload(&mut self.stack),
            Instruction::Istore(index) => istore(&mut self.locals, &mut self.stack, *index),
            Instruction::Lstore(index) => lstore(&mut self.locals, &mut self.stack, *index),
            Instruction::Fstore(index) => fstore(&mut self.locals, &mut self.stack, *index),
            Instruction::Dstore(index) => dstore(&mut self.locals, &mut self.stack, *index),
            Instruction::Astore(index) => astore(&mut self.locals, &mut self.stack, *index),
            Instruction::Istore_0 => istore_0(&mut self.locals, &mut self.stack),
            Instruction::Istore_1 => istore_1(&mut self.locals, &mut self.stack),
            Instruction::Istore_2 => istore_2(&mut self.locals, &mut self.stack),
            Instruction::Istore_3 => istore_3(&mut self.locals, &mut self.stack),
            Instruction::Lstore_0 => lstore_0(&mut self.locals, &mut self.stack),
            Instruction::Lstore_1 => lstore_1(&mut self.locals, &mut self.stack),
            Instruction::Lstore_2 => lstore_2(&mut self.locals, &mut self.stack),
            Instruction::Lstore_3 => lstore_3(&mut self.locals, &mut self.stack),
            Instruction::Fstore_0 => fstore_0(&mut self.locals, &mut self.stack),
            Instruction::Fstore_1 => fstore_1(&mut self.locals, &mut self.stack),
            Instruction::Fstore_2 => fstore_2(&mut self.locals, &mut self.stack),
            Instruction::Fstore_3 => fstore_3(&mut self.locals, &mut self.stack),
            Instruction::Dstore_0 => dstore_0(&mut self.locals, &mut self.stack),
            Instruction::Dstore_1 => dstore_1(&mut self.locals, &mut self.stack),
            Instruction::Dstore_2 => dstore_2(&mut self.locals, &mut self.stack),
            Instruction::Dstore_3 => dstore_3(&mut self.locals, &mut self.stack),
            Instruction::Astore_0 => astore_0(&mut self.locals, &mut self.stack),
            Instruction::Astore_1 => astore_1(&mut self.locals, &mut self.stack),
            Instruction::Astore_2 => astore_2(&mut self.locals, &mut self.stack),
            Instruction::Astore_3 => astore_3(&mut self.locals, &mut self.stack),
            Instruction::Iastore => iastore(&mut self.stack),
            Instruction::Lastore => lastore(&mut self.stack),
            Instruction::Fastore => fastore(&mut self.stack),
            Instruction::Dastore => dastore(&mut self.stack),
            Instruction::Aastore => aastore(&mut self.stack),
            Instruction::Bastore => bastore(&mut self.stack),
            Instruction::Castore => castore(&mut self.stack),
            Instruction::Sastore => sastore(&mut self.stack),
            Instruction::Pop => pop(&mut self.stack),
            Instruction::Pop2 => pop2(&mut self.stack),
            Instruction::Dup => dup(&mut self.stack),
            Instruction::Dup_x1 => dup_x1(&mut self.stack),
            Instruction::Dup_x2 => dup_x2(&mut self.stack),
            Instruction::Dup2 => dup2(&mut self.stack),
            Instruction::Dup2_x1 => dup2_x1(&mut self.stack),
            Instruction::Dup2_x2 => dup2_x2(&mut self.stack),
            Instruction::Swap => swap(&mut self.stack),
            Instruction::Iadd => iadd(&mut self.stack),
            Instruction::Ladd => ladd(&mut self.stack),
            Instruction::Fadd => fadd(&mut self.stack),
            Instruction::Dadd => dadd(&mut self.stack),
            Instruction::Isub => isub(&mut self.stack),
            Instruction::Lsub => lsub(&mut self.stack),
            Instruction::Fsub => fsub(&mut self.stack),
            Instruction::Dsub => dsub(&mut self.stack),
            Instruction::Imul => imul(&mut self.stack),
            Instruction::Lmul => lmul(&mut self.stack),
            Instruction::Fmul => fmul(&mut self.stack),
            Instruction::Dmul => dmul(&mut self.stack),
            Instruction::Idiv => idiv(&mut self.stack),
            Instruction::Ldiv => ldiv(&mut self.stack),
            Instruction::Fdiv => fdiv(&mut self.stack),
            Instruction::Ddiv => ddiv(&mut self.stack),
            Instruction::Irem => irem(&mut self.stack),
            Instruction::Lrem => lrem(&mut self.stack),
            Instruction::Frem => frem(&mut self.stack),
            Instruction::Drem => drem(&mut self.stack),
            Instruction::Ineg => ineg(&mut self.stack),
            Instruction::Lneg => lneg(&mut self.stack),
            Instruction::Fneg => fneg(&mut self.stack),
            Instruction::Dneg => dneg(&mut self.stack),
            Instruction::Ishl => ishl(&mut self.stack),
            Instruction::Lshl => lshl(&mut self.stack),
            Instruction::Ishr => ishr(&mut self.stack),
            Instruction::Lshr => lshr(&mut self.stack),
            Instruction::Iushr => iushr(&mut self.stack),
            Instruction::Lushr => lushr(&mut self.stack),
            Instruction::Iand => iand(&mut self.stack),
            Instruction::Land => land(&mut self.stack),
            Instruction::Ior => ior(&mut self.stack),
            Instruction::Lor => lor(&mut self.stack),
            Instruction::Ixor => ixor(&mut self.stack),
            Instruction::Lxor => lxor(&mut self.stack),
            Instruction::Iinc(index, constant) => iinc(&mut self.locals, *index, *constant),
            Instruction::I2l => i2l(&mut self.stack),
            Instruction::I2f => i2f(&mut self.stack),
            Instruction::I2d => i2d(&mut self.stack),
            Instruction::L2i => l2i(&mut self.stack),
            Instruction::L2f => l2f(&mut self.stack),
            Instruction::L2d => l2d(&mut self.stack),
            Instruction::F2i => f2i(&mut self.stack),
            Instruction::F2l => f2l(&mut self.stack),
            Instruction::F2d => f2d(&mut self.stack),
            Instruction::D2i => d2i(&mut self.stack),
            Instruction::D2l => d2l(&mut self.stack),
            Instruction::D2f => d2f(&mut self.stack),
            Instruction::I2b => i2b(&mut self.stack),
            Instruction::I2c => i2c(&mut self.stack),
            Instruction::I2s => i2s(&mut self.stack),
            Instruction::Lcmp => lcmp(&mut self.stack),
            Instruction::Fcmpl => fcmpl(&mut self.stack),
            Instruction::Fcmpg => fcmpg(&mut self.stack),
            Instruction::Dcmpl => dcmpl(&mut self.stack),
            Instruction::Dcmpg => dcmpg(&mut self.stack),
            Instruction::Ifeq(address) => ifeq(&mut self.stack, *address),
            Instruction::Ifne(address) => ifne(&mut self.stack, *address),
            Instruction::Iflt(address) => iflt(&mut self.stack, *address),
            Instruction::Ifge(address) => ifge(&mut self.stack, *address),
            Instruction::Ifgt(address) => ifgt(&mut self.stack, *address),
            Instruction::Ifle(address) => ifle(&mut self.stack, *address),
            Instruction::If_icmpeq(address) => if_icmpeq(&mut self.stack, *address),
            Instruction::If_icmpne(address) => if_icmpne(&mut self.stack, *address),
            Instruction::If_icmplt(address) => if_icmplt(&mut self.stack, *address),
            Instruction::If_icmpge(address) => if_icmpge(&mut self.stack, *address),
            Instruction::If_icmpgt(address) => if_icmpgt(&mut self.stack, *address),
            Instruction::If_icmple(address) => if_icmple(&mut self.stack, *address),
            Instruction::If_acmpeq(address) => if_acmpeq(&mut self.stack, *address),
            Instruction::If_acmpne(address) => if_acmpne(&mut self.stack, *address),
            Instruction::Goto(address) => goto(*address),
            Instruction::Jsr(address) => jsr(&mut self.stack, *address),
            Instruction::Ret(index) => ret(&self.locals, *index),
            Instruction::Tableswitch {
                default,
                low,
                high,
                offsets,
            } => tableswitch(
                &mut self.stack,
                self.program_counter,
                *default,
                *low,
                *high,
                offsets,
            ),
            Instruction::Lookupswitch { default, pairs } => {
                lookupswitch(&mut self.stack, self.program_counter, *default, pairs)
            }
            Instruction::Ireturn => ireturn(&mut self.stack),
            Instruction::Lreturn => lreturn(&mut self.stack),
            Instruction::Freturn => freturn(&mut self.stack),
            Instruction::Dreturn => dreturn(&mut self.stack),
            Instruction::Areturn => areturn(&mut self.stack),
            Instruction::Return => Ok(Return(None)),
            Instruction::Getstatic(index) => getstatic(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Putstatic(index) => putstatic(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Getfield(index) => {
                getfield(&mut self.stack, self.class.constant_pool(), *index)
            }
            Instruction::Putfield(index) => {
                putfield(&mut self.stack, self.class.constant_pool(), *index)
            }
            Instruction::Invokevirtual(index) => invokevirtual(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Invokespecial(index) => invokespecial(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Invokestatic(index) => invokestatic(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Invokeinterface(index, _count) => invokeinterface(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Invokedynamic(index) => invokedynamic(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::New(index) => new(
                vm,
                call_stack,
                &mut self.stack,
                self.class.constant_pool(),
                *index,
            ),
            Instruction::Newarray(array_type) => newarray(&mut self.stack, array_type),
            Instruction::Anewarray(index) => {
                anewarray(vm, call_stack, &mut self.stack, &self.class, *index)
            }
            Instruction::Arraylength => arraylength(&mut self.stack),
            Instruction::Athrow => todo!(),
            Instruction::Checkcast(class_index) => {
                let constant_pool = self.class.constant_pool();
                let class_name = constant_pool.try_get_class(*class_index)?;
                checkcast(&mut self.stack, class_name)
            }
            Instruction::Instanceof(class_index) => {
                let constant_pool = self.class.constant_pool();
                let class_name = constant_pool.try_get_class(*class_index)?;
                instanceof(&mut self.stack, class_name)
            }
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
                Err(InvalidOperand {
                    expected: "*_w instruction".to_string(),
                    actual: "Wide".to_string(),
                })
            }
            Instruction::Multianewarray(index, dimensions) => multianewarray(
                vm,
                call_stack,
                &mut self.stack,
                &self.class,
                *index,
                *dimensions,
            ),
            Instruction::Ifnull(address) => ifnull(&mut self.stack, *address),
            Instruction::Ifnonnull(address) => ifnonnull(&mut self.stack, *address),
            Instruction::Goto_w(address) => goto_w(*address),
            Instruction::Jsr_w(address) => jsr_w(&mut self.stack, *address),
            Instruction::Breakpoint | Instruction::Impdep1 | Instruction::Impdep2 => {
                // Breakpoint, Impdep1 and Impdep2 instructions are reserved for debugging and implementation
                // dependent operations.
                // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.2
                Ok(Continue)
            }
            // Wide instructions
            Instruction::Iload_w(index) => iload_w(&self.locals, &mut self.stack, *index),
            Instruction::Lload_w(index) => lload_w(&self.locals, &mut self.stack, *index),
            Instruction::Fload_w(index) => fload_w(&self.locals, &mut self.stack, *index),
            Instruction::Dload_w(index) => dload_w(&self.locals, &mut self.stack, *index),
            Instruction::Aload_w(index) => aload_w(&self.locals, &mut self.stack, *index),
            Instruction::Istore_w(index) => istore_w(&mut self.locals, &mut self.stack, *index),
            Instruction::Lstore_w(index) => lstore_w(&mut self.locals, &mut self.stack, *index),
            Instruction::Fstore_w(index) => fstore_w(&mut self.locals, &mut self.stack, *index),
            Instruction::Dstore_w(index) => dstore_w(&mut self.locals, &mut self.stack, *index),
            Instruction::Astore_w(index) => astore_w(&mut self.locals, &mut self.stack, *index),
            Instruction::Iinc_w(index, constant) => iinc_w(&mut self.locals, *index, *constant),
            Instruction::Ret_w(index) => ret_w(&self.locals, *index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call_stack::CallStack;
    use crate::configuration::ConfigurationBuilder;
    use ristretto_classloader::ClassPath;
    use std::path::PathBuf;

    fn get_class(class_name: &str) -> Result<(VM, CallStack, Arc<Class>)> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_path = cargo_manifest.join("../classes");
        let class_path = ClassPath::from(classes_path.to_string_lossy());
        let configuration = ConfigurationBuilder::new()
            .class_path(class_path.clone())
            .build();
        let vm = VM::new(configuration)?;
        let mut call_stack = CallStack::new();
        let class = vm.class(&mut call_stack, class_name)?;
        Ok((vm, call_stack, class))
    }

    #[test]
    fn test_execute() -> Result<()> {
        let (vm, mut call_stack, class) = get_class("Expressions")?;
        let method = class.method("add", "(II)I").expect("method not found");
        let arguments = vec![Value::Int(1), Value::Int(2)];
        let mut frame = Frame::new(&class, &method, arguments)?;
        let result = frame.execute(&vm, &mut call_stack)?;
        assert!(matches!(result, Some(Value::Int(3))));
        Ok(())
    }

    #[test]
    fn test_initial_frame() -> Result<()> {
        let (_vm, _call_stack, frame) = crate::test::frame()?;
        assert!(frame.locals.is_empty());
        assert!(frame.stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_process_nop() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Nop)?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty());
        assert!(frame.stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_process_return() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Return)?;
        assert!(matches!(process_result, Return(None)));
        Ok(())
    }

    // #[test]
    // fn test_process_athrow() -> Result<()> {
    //     todo!()
    // }

    #[test]
    fn test_process_monitorenter() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        frame.stack.push_object(None)?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Monitorenter)?;
        assert_eq!(Continue, process_result);
        Ok(())
    }

    #[test]
    fn test_process_monitorexit() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        frame.stack.push_object(None)?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Monitorexit)?;
        assert_eq!(Continue, process_result);
        Ok(())
    }

    #[test]
    fn test_process_wide() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        assert!(matches!(
            frame.process(&vm, &mut call_stack, &Instruction::Wide),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "*_w instruction" && actual == "Wide"
        ));
        Ok(())
    }

    #[test]
    fn test_process_breakpoint() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Breakpoint)?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty());
        assert!(frame.stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_process_impdep1() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Impdep1)?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty());
        assert!(frame.stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_process_impdep2() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let process_result = frame.process(&vm, &mut call_stack, &Instruction::Impdep2)?;
        assert_eq!(Continue, process_result);
        assert!(frame.locals.is_empty());
        assert!(frame.stack.is_empty());
        Ok(())
    }
}

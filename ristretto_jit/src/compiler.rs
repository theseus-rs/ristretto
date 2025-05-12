use crate::Error::{
    InternalError, UnsupportedInstruction, UnsupportedMethod, UnsupportedTargetISA, UnsupportedType,
};
use crate::function::Function;
use crate::instruction::{
    bipush, breakpoint, d2f, d2i, d2l, dadd, dcmpg, dcmpl, dconst_0, dconst_1, ddiv, dload,
    dload_0, dload_1, dload_2, dload_3, dload_w, dmul, dneg, drem, dreturn, dstore, dstore_0,
    dstore_1, dstore_2, dstore_3, dstore_w, dsub, dup, dup_x1, dup_x2, dup2, dup2_x1, dup2_x2, f2d,
    f2i, f2l, fadd, fcmpg, fcmpl, fconst_0, fconst_1, fconst_2, fdiv, fload, fload_0, fload_1,
    fload_2, fload_3, fload_w, fmul, fneg, frem, freturn, fstore, fstore_0, fstore_1, fstore_2,
    fstore_3, fstore_w, fsub, i2b, i2c, i2d, i2f, i2l, i2s, iadd, iand, iconst_0, iconst_1,
    iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, idiv, iinc, iinc_w, iload, iload_0, iload_1,
    iload_2, iload_3, iload_w, impdep1, impdep2, imul, ineg, ior, irem, ireturn, ishl, ishr,
    istore, istore_0, istore_1, istore_2, istore_3, istore_w, isub, iushr, ixor, l2d, l2f, l2i,
    ladd, land, lcmp, lconst_0, lconst_1, ldc, ldc_w, ldc2_w, ldiv, lload, lload_0, lload_1,
    lload_2, lload_3, lload_w, lmul, lneg, lor, lrem, lreturn, lshl, lshr, lstore, lstore_0,
    lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr, lxor, monitorenter, monitorexit, nop, pop,
    pop2, r#return, sipush, swap, wide,
};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{JitValue, Result};
use cranelift::codegen::ir::UserFuncName;
use cranelift::codegen::settings::Flags;
use cranelift::jit::{JITBuilder, JITModule};
use cranelift::module::{Linkage, Module, default_libcall_names};
use cranelift::prelude::*;
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::{
    BaseType, ClassFile, ConstantPool, FieldType, Method, MethodAccessFlags,
};
use std::fmt::Debug;
use std::mem;

#[cfg(debug_assertions)]
const ENABLE_VERIFIER: &str = "true";
#[cfg(not(debug_assertions))]
const ENABLE_VERIFIER: &str = "false";

/// Java Virtual Machine (JVM) bytecode to native code compiler.
pub struct Compiler {
    jit_module: JITModule,
}

impl Compiler {
    /// Creates a new instance of the compiler for the host machine.
    ///
    /// # Errors
    /// * If the target ISA is not supported
    /// * If the target ISA cannot be created
    pub fn new() -> Result<Self> {
        let isa_builder = cranelift::native::builder().map_err(UnsupportedTargetISA)?;
        let mut flag_builder = settings::builder();
        let settings = [("opt_level", "speed"), ("enable_verifier", ENABLE_VERIFIER)];
        for (name, value) in &settings {
            if flag_builder.set(name, value).is_err() {
                return Err(InternalError(format!(
                    "unable to set jit compiler flag '{name}'={value}'"
                )));
            }
        }
        let flags = Flags::new(flag_builder);

        let target_isa = isa_builder.finish(flags)?;
        let jit_builder = JITBuilder::with_isa(target_isa, default_libcall_names());
        let jit_module = JITModule::new(jit_builder);
        let compiler = Compiler { jit_module };
        Ok(compiler)
    }

    /// Compiles the given bytecode into native code.
    ///
    /// # Errors
    /// if the Java byte code cannot be compiled to native code
    pub fn compile(&mut self, class_file: &ClassFile, method: &Method) -> Result<Function> {
        let constant_pool = &class_file.constant_pool;
        let class_name = class_file.class_name()?;
        let method_name = constant_pool.try_get_utf8(method.name_index)?;
        let method_descriptor = constant_pool.try_get_utf8(method.descriptor_index)?;
        if !method.access_flags.contains(MethodAccessFlags::STATIC) && method_name != "<init>" {
            return Err(UnsupportedMethod(format!(
                "Unable to compile method that is not <init> or static: {class_name}.{method_name}{method_descriptor}"
            )));
        }
        let Some((max_stack, instructions)) = method.attributes.iter().find_map(|attribute| {
            if let Attribute::Code {
                max_stack, code, ..
            } = attribute
            {
                Some((*max_stack, code))
            } else {
                None
            }
        }) else {
            return Err(InternalError("No Code attribute found".to_string()));
        };

        let function_name = Self::function_name(class_name, method_name);
        let signature = self.signature();
        let function =
            self.jit_module
                .declare_function(function_name.as_str(), Linkage::Local, &signature)?;
        let mut module_context = self.jit_module.make_context();
        module_context.func.signature = signature;
        module_context.func.name = UserFuncName::user(0, function.as_u32());

        let mut function_context = FunctionBuilderContext::new();
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);

        let block = function_builder.create_block();
        function_builder.switch_to_block(block);
        function_builder.append_block_params_for_function_params(block);
        let (arguments_pointer, _arguments_length_pointer, return_pointer) =
            Self::function_pointers(&mut function_builder, block)?;

        let mut locals = Self::locals(method_descriptor, &mut function_builder, arguments_pointer)?;

        let mut stack = OperandStack::with_capacity(max_stack);
        for instruction in instructions {
            Self::process_instruction(
                constant_pool,
                &mut function_builder,
                &mut locals,
                &mut stack,
                return_pointer,
                instruction,
            )?;
        }

        function_builder.seal_all_blocks();
        function_builder.finalize();

        self.jit_module
            .define_function(function, &mut module_context)?;
        self.jit_module.clear_context(&mut module_context);
        self.jit_module.finalize_definitions()?;

        let code = self.jit_module.get_finalized_function(function);
        let function = unsafe {
            let function: fn(*const JitValue, usize, *mut JitValue) = mem::transmute(code);
            Function::new(function)
        };
        Ok(function)
    }

    /// Creates a new function name from the class and method names.
    fn function_name(class_name: &str, method_name: &str) -> String {
        let class_name = class_name.replace('/', "_");
        let method_name = method_name
            .strip_prefix("<")
            .unwrap_or(method_name)
            .strip_suffix(">")
            .unwrap_or(method_name);
        format!("{class_name}__{method_name}")
    }

    /// Creates a new signature from the method descriptor.
    fn signature(&mut self) -> Signature {
        let mut signature = self.jit_module.make_signature();
        let arguments_type = self.jit_module.target_config().pointer_type();
        signature.params.push(AbiParam::new(arguments_type)); // pointer to array
        signature.params.push(AbiParam::new(types::I64)); // length of array
        let return_type = self.jit_module.target_config().pointer_type();
        signature.params.push(AbiParam::new(return_type));
        signature
    }

    /// Retrieves the function pointers from the function builder.
    ///
    /// # Errors
    /// If the function pointers cannot be retrieved
    fn function_pointers(
        function_builder: &mut FunctionBuilder,
        block: Block,
    ) -> Result<(Value, Value, Value)> {
        let mut params = function_builder.block_params(block).to_vec();
        let Some(return_pointer) = params.pop() else {
            return Err(InternalError("undefined return pointer".to_string()));
        };
        let Some(arguments_length) = params.pop() else {
            return Err(InternalError(
                "undefined arguments length pointer".to_string(),
            ));
        };
        let Some(arguments_pointer) = params.pop() else {
            return Err(InternalError("undefined arguments pointer".to_string()));
        };
        if !params.is_empty() {
            return Err(InternalError("unexpected function parameters".to_string()));
        }
        Ok((arguments_pointer, arguments_length, return_pointer))
    }

    /// Creates a new locals array for the function.
    ///
    /// # Errors
    /// If the locals array cannot be created
    fn locals(
        descriptor: &str,
        function_builder: &mut FunctionBuilder,
        arguments_pointer: Value,
    ) -> Result<LocalVariables> {
        let size_of = i64::try_from(size_of::<JitValue>())
            .map_err(|error| InternalError(format!("{error:?}")))?;
        let struct_size = function_builder.ins().iconst(types::I64, size_of);

        let mut variable_index = 0;
        let (parameter_types, _return_type) = FieldType::parse_method_descriptor(descriptor)?;
        for (index, parameter_type) in parameter_types.iter().enumerate() {
            let index =
                i64::try_from(index).map_err(|error| InternalError(format!("{error:?}")))?;
            let index = function_builder.ins().iconst(types::I64, index);
            let offset = function_builder.ins().imul(index, struct_size);
            let address = function_builder.ins().iadd(arguments_pointer, offset);

            // Ignore the discriminant
            let variable = Variable::new(variable_index);
            variable_index = variable_index
                .checked_add(1)
                .ok_or_else(|| InternalError("variable index overflow".to_string()))?;
            let native_type = Self::native_type(parameter_type)?;
            function_builder.declare_var(variable, native_type);
            let value = function_builder
                .ins()
                .load(native_type, MemFlags::trusted(), address, 8);
            function_builder.def_var(variable, value);
            if let FieldType::Base(BaseType::Long | BaseType::Double) = parameter_type {
                // The JVM specification requires that Long and Double take two places in the
                // locals list when passed to a method. This method adjusts the variables index
                // to account for this.
                //
                // See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6.1>
                variable_index = variable_index
                    .checked_add(1)
                    .ok_or_else(|| InternalError("variable index overflow".to_string()))?;
            }
        }
        Ok(LocalVariables::new())
    }

    /// Creates a new native type from the given field type.
    ///
    /// # Errors
    /// If the field type is not supported
    fn native_type(field_type: &FieldType) -> Result<Type> {
        match field_type {
            FieldType::Base(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Int
                | BaseType::Short,
            ) => Ok(types::I32),
            FieldType::Base(BaseType::Double) => Ok(types::F64),
            FieldType::Base(BaseType::Float) => Ok(types::F32),
            FieldType::Base(BaseType::Long) => Ok(types::I64),
            _ => Err(UnsupportedType(field_type.to_string())),
        }
    }

    #[expect(clippy::too_many_lines)]
    fn process_instruction(
        constant_pool: &ConstantPool,
        function_builder: &mut FunctionBuilder,
        locals: &mut LocalVariables,
        stack: &mut OperandStack,
        return_pointer: Value,
        instruction: &Instruction,
    ) -> Result<()> {
        match instruction {
            Instruction::Nop => nop(),
            // Instruction::Aconst_null => aconst_null(stack),
            Instruction::Iconst_m1 => iconst_m1(function_builder, stack)?,
            Instruction::Iconst_0 => iconst_0(function_builder, stack)?,
            Instruction::Iconst_1 => iconst_1(function_builder, stack)?,
            Instruction::Iconst_2 => iconst_2(function_builder, stack)?,
            Instruction::Iconst_3 => iconst_3(function_builder, stack)?,
            Instruction::Iconst_4 => iconst_4(function_builder, stack)?,
            Instruction::Iconst_5 => iconst_5(function_builder, stack)?,
            Instruction::Lconst_0 => lconst_0(function_builder, stack)?,
            Instruction::Lconst_1 => lconst_1(function_builder, stack)?,
            Instruction::Fconst_0 => fconst_0(function_builder, stack)?,
            Instruction::Fconst_1 => fconst_1(function_builder, stack)?,
            Instruction::Fconst_2 => fconst_2(function_builder, stack)?,
            Instruction::Dconst_0 => dconst_0(function_builder, stack)?,
            Instruction::Dconst_1 => dconst_1(function_builder, stack)?,
            Instruction::Bipush(value) => bipush(function_builder, stack, *value)?,
            Instruction::Sipush(value) => sipush(function_builder, stack, *value)?,
            Instruction::Ldc(index) => ldc(constant_pool, function_builder, stack, *index)?,
            Instruction::Ldc_w(index) => {
                ldc_w(constant_pool, function_builder, stack, *index)?;
            }
            Instruction::Ldc2_w(index) => {
                ldc2_w(constant_pool, function_builder, stack, *index)?;
            }
            Instruction::Iload(index) => iload(function_builder, locals, stack, *index)?,
            Instruction::Lload(index) => lload(function_builder, locals, stack, *index)?,
            Instruction::Fload(index) => fload(function_builder, locals, stack, *index)?,
            Instruction::Dload(index) => dload(function_builder, locals, stack, *index)?,
            // Instruction::Aload(index) => aload(locals, stack, *index),
            Instruction::Iload_0 => iload_0(function_builder, locals, stack)?,
            Instruction::Iload_1 => iload_1(function_builder, locals, stack)?,
            Instruction::Iload_2 => iload_2(function_builder, locals, stack)?,
            Instruction::Iload_3 => iload_3(function_builder, locals, stack)?,
            Instruction::Lload_0 => lload_0(function_builder, locals, stack)?,
            Instruction::Lload_1 => lload_1(function_builder, locals, stack)?,
            Instruction::Lload_2 => lload_2(function_builder, locals, stack)?,
            Instruction::Lload_3 => lload_3(function_builder, locals, stack)?,
            Instruction::Fload_0 => fload_0(function_builder, locals, stack)?,
            Instruction::Fload_1 => fload_1(function_builder, locals, stack)?,
            Instruction::Fload_2 => fload_2(function_builder, locals, stack)?,
            Instruction::Fload_3 => fload_3(function_builder, locals, stack)?,
            Instruction::Dload_0 => dload_0(function_builder, locals, stack)?,
            Instruction::Dload_1 => dload_1(function_builder, locals, stack)?,
            Instruction::Dload_2 => dload_2(function_builder, locals, stack)?,
            Instruction::Dload_3 => dload_3(function_builder, locals, stack)?,
            // Instruction::Aload_0 => aload_0(locals, stack),
            // Instruction::Aload_1 => aload_1(locals, stack),
            // Instruction::Aload_2 => aload_2(locals, stack),
            // Instruction::Aload_3 => aload_3(locals, stack),
            // Instruction::Iaload => iaload(stack),
            // Instruction::Laload => laload(stack),
            // Instruction::Faload => faload(stack),
            // Instruction::Daload => daload(stack),
            // Instruction::Aaload => aaload(stack),
            // Instruction::Baload => baload(stack),
            // Instruction::Caload => caload(stack),
            // Instruction::Saload => saload(stack),
            Instruction::Istore(index) => istore(function_builder, locals, stack, *index)?,
            Instruction::Lstore(index) => lstore(function_builder, locals, stack, *index)?,
            Instruction::Fstore(index) => fstore(function_builder, locals, stack, *index)?,
            Instruction::Dstore(index) => dstore(function_builder, locals, stack, *index)?,
            // Instruction::Astore(index) => astore(locals, locals, stack, *index)?,
            Instruction::Istore_0 => istore_0(function_builder, locals, stack)?,
            Instruction::Istore_1 => istore_1(function_builder, locals, stack)?,
            Instruction::Istore_2 => istore_2(function_builder, locals, stack)?,
            Instruction::Istore_3 => istore_3(function_builder, locals, stack)?,
            Instruction::Lstore_0 => lstore_0(function_builder, locals, stack)?,
            Instruction::Lstore_1 => lstore_1(function_builder, locals, stack)?,
            Instruction::Lstore_2 => lstore_2(function_builder, locals, stack)?,
            Instruction::Lstore_3 => lstore_3(function_builder, locals, stack)?,
            Instruction::Fstore_0 => fstore_0(function_builder, locals, stack)?,
            Instruction::Fstore_1 => fstore_1(function_builder, locals, stack)?,
            Instruction::Fstore_2 => fstore_2(function_builder, locals, stack)?,
            Instruction::Fstore_3 => fstore_3(function_builder, locals, stack)?,
            Instruction::Dstore_0 => dstore_0(function_builder, locals, stack)?,
            Instruction::Dstore_1 => dstore_1(function_builder, locals, stack)?,
            Instruction::Dstore_2 => dstore_2(function_builder, locals, stack)?,
            Instruction::Dstore_3 => dstore_3(function_builder, locals, stack)?,
            // Instruction::Astore_0 => astore_0(locals, locals, locals, stack),
            // Instruction::Astore_1 => astore_1(locals, locals, locals, stack),
            // Instruction::Astore_2 => astore_2(locals, locals, locals, stack),
            // Instruction::Astore_3 => astore_3(locals, locals, locals, stack),
            // Instruction::Iastore => iastore(stack),
            // Instruction::Lastore => lastore(stack),
            // Instruction::Fastore => fastore(stack),
            // Instruction::Dastore => dastore(stack),
            // Instruction::Aastore => aastore(stack),
            // Instruction::Bastore => bastore(stack),
            // Instruction::Castore => castore(stack),
            // Instruction::Sastore => sastore(stack),
            Instruction::Pop => pop(stack)?,
            Instruction::Pop2 => pop2(function_builder, stack)?,
            Instruction::Dup => dup(stack)?,
            Instruction::Dup_x1 => dup_x1(stack)?,
            Instruction::Dup_x2 => dup_x2(function_builder, stack)?,
            Instruction::Dup2 => dup2(function_builder, stack)?,
            Instruction::Dup2_x1 => dup2_x1(function_builder, stack)?,
            Instruction::Dup2_x2 => dup2_x2(function_builder, stack)?,
            Instruction::Swap => swap(stack)?,
            Instruction::Iadd => iadd(function_builder, stack)?,
            Instruction::Ladd => ladd(function_builder, stack)?,
            Instruction::Fadd => fadd(function_builder, stack)?,
            Instruction::Dadd => dadd(function_builder, stack)?,
            Instruction::Isub => isub(function_builder, stack)?,
            Instruction::Lsub => lsub(function_builder, stack)?,
            Instruction::Fsub => fsub(function_builder, stack)?,
            Instruction::Dsub => dsub(function_builder, stack)?,
            Instruction::Imul => imul(function_builder, stack)?,
            Instruction::Lmul => lmul(function_builder, stack)?,
            Instruction::Fmul => fmul(function_builder, stack)?,
            Instruction::Dmul => dmul(function_builder, stack)?,
            Instruction::Idiv => idiv(function_builder, stack)?,
            Instruction::Ldiv => ldiv(function_builder, stack)?,
            Instruction::Fdiv => fdiv(function_builder, stack)?,
            Instruction::Ddiv => ddiv(function_builder, stack)?,
            Instruction::Irem => irem(function_builder, stack)?,
            Instruction::Lrem => lrem(function_builder, stack)?,
            Instruction::Frem => frem(function_builder, stack)?,
            Instruction::Drem => drem(function_builder, stack)?,
            Instruction::Ineg => ineg(function_builder, stack)?,
            Instruction::Lneg => lneg(function_builder, stack)?,
            Instruction::Fneg => fneg(function_builder, stack)?,
            Instruction::Dneg => dneg(function_builder, stack)?,
            Instruction::Ishl => ishl(function_builder, stack)?,
            Instruction::Lshl => lshl(function_builder, stack)?,
            Instruction::Ishr => ishr(function_builder, stack)?,
            Instruction::Lshr => lshr(function_builder, stack)?,
            Instruction::Iushr => iushr(function_builder, stack)?,
            Instruction::Lushr => lushr(function_builder, stack)?,
            Instruction::Iand => iand(function_builder, stack)?,
            Instruction::Land => land(function_builder, stack)?,
            Instruction::Ior => ior(function_builder, stack)?,
            Instruction::Lor => lor(function_builder, stack)?,
            Instruction::Ixor => ixor(function_builder, stack)?,
            Instruction::Lxor => lxor(function_builder, stack)?,
            Instruction::Iinc(index, constant) => {
                iinc(function_builder, locals, *index, *constant)?;
            }
            Instruction::I2l => i2l(function_builder, stack)?,
            Instruction::I2f => i2f(function_builder, stack)?,
            Instruction::I2d => i2d(function_builder, stack)?,
            Instruction::L2i => l2i(function_builder, stack)?,
            Instruction::L2f => l2f(function_builder, stack)?,
            Instruction::L2d => l2d(function_builder, stack)?,
            Instruction::F2i => f2i(function_builder, stack)?,
            Instruction::F2l => f2l(function_builder, stack)?,
            Instruction::F2d => f2d(function_builder, stack)?,
            Instruction::D2i => d2i(function_builder, stack)?,
            Instruction::D2l => d2l(function_builder, stack)?,
            Instruction::D2f => d2f(function_builder, stack)?,
            Instruction::I2b => i2b(function_builder, stack)?,
            Instruction::I2c => i2c(function_builder, stack)?,
            Instruction::I2s => i2s(function_builder, stack)?,
            Instruction::Lcmp => lcmp(function_builder, stack)?,
            Instruction::Fcmpl => fcmpl(function_builder, stack)?,
            Instruction::Fcmpg => fcmpg(function_builder, stack)?,
            Instruction::Dcmpl => dcmpl(function_builder, stack)?,
            Instruction::Dcmpg => dcmpg(function_builder, stack)?,
            // Instruction::Ifeq(address) => ifeq(stack, *address),
            // Instruction::Ifne(address) => ifne(stack, *address),
            // Instruction::Iflt(address) => iflt(stack, *address),
            // Instruction::Ifge(address) => ifge(stack, *address),
            // Instruction::Ifgt(address) => ifgt(stack, *address),
            // Instruction::Ifle(address) => ifle(stack, *address),
            // Instruction::If_icmpeq(address) => if_icmpeq(stack, *address),
            // Instruction::If_icmpne(address) => if_icmpne(stack, *address),
            // Instruction::If_icmplt(address) => if_icmplt(stack, *address),
            // Instruction::If_icmpge(address) => if_icmpge(stack, *address),
            // Instruction::If_icmpgt(address) => if_icmpgt(stack, *address),
            // Instruction::If_icmple(address) => if_icmple(stack, *address),
            // Instruction::If_acmpeq(address) => if_acmpeq(stack, *address),
            // Instruction::If_acmpne(address) => if_acmpne(stack, *address),
            // Instruction::Goto(address) => goto(*address),
            // Instruction::Jsr(address) => jsr(stack, *address),
            // Instruction::Ret(index) => ret(locals, *index),
            // Instruction::Tableswitch {
            //     default,
            //     low,
            //     high,
            //     offsets,
            // } => {
            //     let program_counter = self.program_counter.load(Ordering::Relaxed);
            //     tableswitch(stack, program_counter, *default, *low, *high, offsets)
            // }
            // Instruction::Lookupswitch { default, pairs } => {
            //     let program_counter = self.program_counter.load(Ordering::Relaxed);
            //     lookupswitch(stack, program_counter, *default, pairs)
            // }
            Instruction::Ireturn => ireturn(function_builder, stack, return_pointer)?,
            Instruction::Lreturn => lreturn(function_builder, stack, return_pointer)?,
            Instruction::Freturn => freturn(function_builder, stack, return_pointer)?,
            Instruction::Dreturn => dreturn(function_builder, stack, return_pointer)?,
            // Instruction::Areturn => areturn(stack),
            Instruction::Return => r#return(function_builder, return_pointer),
            // Instruction::Getstatic(index) => getstatic(self, stack, *index).await,
            // Instruction::Putstatic(index) => putstatic(self, stack, *index).await,
            // Instruction::Getfield(index) => getfield(stack, &self.class, *index),
            // Instruction::Putfield(index) => putfield(stack, &self.class, *index),
            // Instruction::Invokevirtual(index) => invokevirtual(self, stack, *index).await,
            // Instruction::Invokespecial(index) => invokespecial(self, stack, *index).await,
            // Instruction::Invokestatic(index) => invokestatic(self, stack, *index).await,
            // Instruction::Invokeinterface(index, count) => {
            //     invokeinterface(self, stack, *index, *count).await
            // }
            // Instruction::Invokedynamic(index) => invokedynamic(self, stack, *index).await,
            // Instruction::New(index) => new(self, stack, *index).await,
            // Instruction::Newarray(array_type) => newarray(stack, array_type),
            // Instruction::Anewarray(index) => anewarray(self, stack, *index).await,
            // Instruction::Arraylength => arraylength(stack),
            // Instruction::Athrow => athrow(stack).await,
            // Instruction::Checkcast(class_index) => checkcast(self, stack, *class_index).await,
            // Instruction::Instanceof(class_index) => instanceof(self, stack, *class_index).await,
            Instruction::Monitorenter => monitorenter(stack)?,
            Instruction::Monitorexit => monitorexit(stack)?,
            Instruction::Wide => wide()?,
            // Instruction::Multianewarray(index, dimensions) => {
            //     multianewarray(self, stack, *index, *dimensions).await
            // }
            // Instruction::Ifnull(address) => ifnull(stack, *address),
            // Instruction::Ifnonnull(address) => ifnonnull(stack, *address),
            // Instruction::Goto_w(address) => goto_w(*address),
            // Instruction::Jsr_w(address) => jsr_w(stack, *address),
            Instruction::Breakpoint => breakpoint(function_builder, stack),
            Instruction::Impdep1 => impdep1(function_builder, stack),
            Instruction::Impdep2 => impdep2(function_builder, stack),
            // Wide instructions
            Instruction::Iload_w(index) => iload_w(function_builder, locals, stack, *index)?,
            Instruction::Lload_w(index) => lload_w(function_builder, locals, stack, *index)?,
            Instruction::Fload_w(index) => fload_w(function_builder, locals, stack, *index)?,
            Instruction::Dload_w(index) => dload_w(function_builder, locals, stack, *index)?,
            // Instruction::Aload_w(index) => aload_w(locals, stack, *index),
            Instruction::Istore_w(index) => istore_w(function_builder, locals, stack, *index)?,
            Instruction::Lstore_w(index) => lstore_w(function_builder, locals, stack, *index)?,
            Instruction::Fstore_w(index) => fstore_w(function_builder, locals, stack, *index)?,
            Instruction::Dstore_w(index) => dstore_w(function_builder, locals, stack, *index)?,
            // Instruction::Astore_w(index) => astore_w(locals, stack, *index),
            Instruction::Iinc_w(index, constant) => {
                iinc_w(function_builder, locals, *index, *constant)?;
            }
            // Instruction::Ret_w(index) => ret_w(locals, *index),
            _ => {
                return Err(UnsupportedInstruction(instruction.clone()));
            }
        }
        Ok(())
    }
}

impl Debug for Compiler {
    /// Formats the compiler for debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compiler")
            .field("module", &self.jit_module.isa().triple())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_new() {
        let result = Compiler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_compiler_debug() -> Result<()> {
        let compiler = Compiler::new()?;
        let debug_string = format!("{compiler:?}");
        assert!(debug_string.contains("Compiler"));
        assert!(debug_string.contains("module"));
        Ok(())
    }

    #[test]
    fn test_function_name() {
        let class_name = "java/lang/Object";
        let method_name = "<clinit>";
        let function_name = Compiler::function_name(class_name, method_name);
        assert_eq!("java_lang_Object__clinit", function_name);
    }

    #[test]
    fn test_make_signature_no_parameters_no_return() -> Result<()> {
        let mut compiler = Compiler::new()?;
        let signature = compiler.signature();
        assert_eq!(signature.params.len(), 3);
        assert_eq!(signature.returns, vec![]);
        Ok(())
    }

    #[test]
    fn test_native_type_int() -> Result<()> {
        for base_type in [
            BaseType::Boolean,
            BaseType::Byte,
            BaseType::Char,
            BaseType::Int,
            BaseType::Short,
        ] {
            assert_eq!(
                Compiler::native_type(&FieldType::Base(base_type))?,
                types::I32
            );
        }
        Ok(())
    }

    #[test]
    fn test_native_type_double() -> Result<()> {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Double))?,
            types::F64
        );
        Ok(())
    }

    #[test]
    fn test_native_type_float() -> Result<()> {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Float))?,
            types::F32
        );
        Ok(())
    }

    #[test]
    fn test_native_type_long() -> Result<()> {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Long))?,
            types::I64
        );
        Ok(())
    }

    #[test]
    fn test_native_type_object() {
        let class_name = "java/lang/Object".to_string();
        let result = Compiler::native_type(&FieldType::Object(class_name));
        assert!(matches!(result, Err(UnsupportedType(_))));
    }

    #[test]
    fn test_native_type_array_int() {
        for base_type in [
            BaseType::Boolean,
            BaseType::Byte,
            BaseType::Char,
            BaseType::Int,
            BaseType::Short,
        ] {
            let field_type = FieldType::Base(base_type);
            let result = Compiler::native_type(&FieldType::Array(Box::new(field_type)));
            assert!(matches!(result, Err(UnsupportedType(_))));
        }
    }

    #[test]
    fn test_native_type_array_double() {
        let field_type = FieldType::Base(BaseType::Double);
        let result = Compiler::native_type(&FieldType::Array(Box::new(field_type)));
        assert!(matches!(result, Err(UnsupportedType(_))));
    }

    #[test]
    fn test_native_type_array_float() {
        let field_type = FieldType::Base(BaseType::Float);
        let result = Compiler::native_type(&FieldType::Array(Box::new(field_type)));
        assert!(matches!(result, Err(UnsupportedType(_))));
    }

    #[test]
    fn test_native_type_array_long() {
        let field_type = FieldType::Base(BaseType::Long);
        let result = Compiler::native_type(&FieldType::Array(Box::new(field_type)));
        assert!(matches!(result, Err(UnsupportedType(_))));
    }

    #[test]
    fn test_native_type_array_object() {
        let class_name = "java/lang/Object".to_string();
        let field_type = FieldType::Object(class_name);
        let result = Compiler::native_type(&FieldType::Array(Box::new(field_type)));
        assert!(matches!(result, Err(UnsupportedType(_))));
    }
}

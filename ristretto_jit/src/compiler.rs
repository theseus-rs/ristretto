use crate::Error::{
    InternalError, InvalidBlockAddress, UnsupportedInstruction, UnsupportedTargetISA,
    UnsupportedType,
};
use crate::control_flow_graph::InstructionControlFlow;
use crate::function::Function;
use crate::instruction::ThrowContext;
use crate::instruction::{
    aaload, aastore, aconst_null, aload_ref, aload_ref_0, aload_ref_1, aload_ref_2, aload_ref_3,
    aload_ref_w, anewarray, areturn, arraylength, astore_ref, astore_ref_0, astore_ref_1,
    astore_ref_2, astore_ref_3, astore_ref_w, athrow, baload, bastore, bipush, breakpoint, caload,
    castore, checkcast, d2f, d2i, d2l, dadd, daload, dastore, dcmpg, dcmpl, dconst_0, dconst_1,
    ddiv, dload, dload_0, dload_1, dload_2, dload_3, dload_w, dmul, dneg, drem, dreturn, dstore,
    dstore_0, dstore_1, dstore_2, dstore_3, dstore_w, dsub, dup, dup_x1, dup_x2, dup2, dup2_x1,
    dup2_x2, emit_exception_return, f2d, f2i, f2l, fadd, faload, fastore, fcmpg, fcmpl, fconst_0,
    fconst_1, fconst_2, fdiv, fload, fload_0, fload_1, fload_2, fload_3, fload_w, fmul, fneg, frem,
    freturn, fstore, fstore_0, fstore_1, fstore_2, fstore_3, fstore_w, fsub, getfield, getstatic,
    goto, goto_w, i2b, i2c, i2d, i2f, i2l, i2s, iadd, iaload, iand, iastore, iconst_0, iconst_1,
    iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, idiv, if_acmpeq, if_acmpne, if_icmpeq,
    if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne, ifeq, ifge, ifgt, ifle, iflt, ifne,
    ifnonnull, ifnull, iinc, iinc_w, iload, iload_0, iload_1, iload_2, iload_3, iload_w, impdep1,
    impdep2, imul, ineg, instanceof, ior, irem, ireturn, ishl, ishr, istore, istore_0, istore_1,
    istore_2, istore_3, istore_w, isub, iushr, ixor, jsr, jsr_w, l2d, l2f, l2i, ladd, laload, land,
    lastore, lcmp, lconst_0, lconst_1, ldc, ldc_w, ldc2_w, ldiv, lload, lload_0, lload_1, lload_2,
    lload_3, lload_w, lmul, lneg, lookupswitch, lor, lrem, lreturn, lshl, lshr, lstore, lstore_0,
    lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr, lxor, monitorenter, monitorexit,
    multianewarray, new_object, newarray, nop, pop, pop2, putfield, putstatic, ret, ret_w,
    r#return, saload, sastore, sipush, swap, tableswitch, wide,
};
use crate::local_type::LocalType;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{JitValue, Result, control_flow_graph};
use ahash::AHashMap;
use cranelift::codegen::ir::UserFuncName;
use cranelift::codegen::isa::OwnedTargetIsa;
use cranelift::codegen::settings::Flags;
use cranelift::jit::{JITBuilder, JITModule};
use cranelift::module::{Linkage, Module, default_libcall_names};
use cranelift::prelude::*;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_classfile::{
    BaseType, ClassFile, ConstantPool, FieldType, JavaStr, Method, MethodAccessFlags,
};
use std::fmt::Debug;
use std::mem;
use std::sync::Arc;

#[cfg(debug_assertions)]
const ENABLE_VERIFIER: &str = "true";
#[cfg(not(debug_assertions))]
const ENABLE_VERIFIER: &str = "false";

/// Java Virtual Machine (JVM) bytecode to native code compiler.
#[derive(Clone)]
pub struct Compiler {
    target_isa: OwnedTargetIsa,
}

impl Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compiler")
            .field("target_isa", &self.target_isa.to_string())
            .finish()
    }
}

impl Compiler {
    /// Creates a new instance of the compiler for the host machine.
    ///
    /// # Errors
    ///
    /// - If the target ISA is not supported
    /// - If the target ISA cannot be created
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
        Ok(Compiler { target_isa })
    }

    /// Creates a new JIT module for the compiler using the cached target ISA.
    pub(crate) fn jit_module(&self, symbols: &[(&str, *const u8)]) -> JITModule {
        let mut jit_builder =
            JITBuilder::with_isa(Arc::clone(&self.target_isa), default_libcall_names());
        for &(name, ptr) in symbols {
            jit_builder.symbol(name, ptr);
        }
        JITModule::new(jit_builder)
    }

    /// Returns true if the method can potentially be JIT compiled.
    /// This performs a fast scan of the method's bytecode instructions to check
    /// for unsupported opcodes without doing any expensive compilation work.
    ///
    /// Methods with non-empty exception tables are currently rejected. The
    /// exception-handler dispatch codegen (in `instruction::exception` and
    /// `populate_dispatch_block`) is implemented and unit-tested via the synthetic
    /// `athrow_caught_by_catch_all_handler` test, but enabling it for production
    /// methods exposes unrelated JIT gaps:
    ///
    /// - `monitorenter`/`monitorexit` are no-ops in the JIT (see
    ///   `instruction/monitor.rs`); compiler-generated try/finally blocks for
    ///   `synchronized` methods would silently lose synchronization semantics.
    /// - Array `iaload`/`aaload`/`arraylength` NPEs from JIT-compiled methods
    ///   currently report the wrong extended message (the receiver name is
    ///   substituted instead of the array name).
    ///
    /// Once those gaps are closed, this guard can be removed in favor of the
    /// dispatch path being exercised by integration tests.
    #[must_use]
    pub fn can_compile(method: &Method) -> bool {
        let Some((instructions, exception_table)) =
            method.attributes.iter().find_map(|attribute| {
                if let Attribute::Code {
                    code,
                    exception_table,
                    ..
                } = attribute
                {
                    Some((code, exception_table))
                } else {
                    None
                }
            })
        else {
            return false;
        };
        if !exception_table.is_empty() {
            return false;
        }
        Self::first_unsupported_instruction(instructions).is_none()
    }

    /// Compiles the given bytecode into native code.
    ///
    /// # Errors
    ///
    /// if the Java byte code cannot be compiled to native code
    #[expect(clippy::too_many_lines)]
    pub fn compile(
        &self,
        class_file: &ClassFile,
        method: &Method,
        symbols: &[(&str, *const u8)],
    ) -> Result<Function> {
        let Some((max_stack, instructions, exception_table)) =
            method.attributes.iter().find_map(|attribute| {
                if let Attribute::Code {
                    max_stack,
                    code,
                    exception_table,
                    ..
                } = attribute
                {
                    Some((*max_stack, code, exception_table))
                } else {
                    None
                }
            })
        else {
            return Err(InternalError("No Code attribute found".to_string()));
        };

        if let Some(instruction) = Self::first_unsupported_instruction(instructions) {
            return Err(UnsupportedInstruction(instruction.clone()));
        }

        let constant_pool = &class_file.constant_pool;
        let method_descriptor = constant_pool.try_get_utf8(method.descriptor_index)?;
        let is_static = method.access_flags.contains(MethodAccessFlags::STATIC);
        let mut jit_module = self.jit_module(symbols);
        let class_name = class_file.class_name()?;
        let method_name = constant_pool.try_get_utf8(method.name_index)?;

        let class_name_str = class_name.to_str_lossy();
        let method_name_str = method_name.to_str_lossy();
        let function_name = Self::function_name(&class_name_str, &method_name_str);
        let signature = Self::signature(&jit_module);
        let function =
            jit_module.declare_function(function_name.as_str(), Linkage::Local, &signature)?;
        let mut module_context = jit_module.make_context();
        module_context.func.signature = signature;
        module_context.func.name = UserFuncName::user(0, function.as_u32());

        let mut function_context = FunctionBuilderContext::new();
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);

        let helpers = RuntimeHelpers::declare(&mut jit_module, &mut function_builder)?;

        let (entry_block, blocks) = control_flow_graph::get_blocks(
            &mut function_builder,
            constant_pool,
            instructions,
            exception_table,
        )?;
        // Use the entry block for function setup (entry_block cannot be jumped to)
        function_builder.switch_to_block(entry_block);
        function_builder.append_block_params_for_function_params(entry_block);
        let (arguments_pointer, _arguments_length_pointer, return_pointer, context_pointer) =
            Self::function_pointers(&mut function_builder, entry_block)?;

        let mut locals = Self::locals(
            &mut function_builder,
            method_descriptor,
            instructions,
            arguments_pointer,
            is_static,
        )?;

        // Get the block for address 0 (may be entry_block or a separate loop body block)
        let block_zero = *blocks.get(&0).ok_or_else(|| InvalidBlockAddress(0))?;

        // Shared exception return block. All instructions that can throw branch here on a
        // pending exception. The block body writes a NONE discriminant to the return pointer
        // and returns.
        let exception_block = function_builder.create_block();

        // If entry_block and block_zero are different, we need to jump from entry to block_zero
        // This happens when there are backward jumps to address 0
        if entry_block != block_zero {
            function_builder.ins().jump(block_zero, &[]);
            function_builder.switch_to_block(block_zero);
        }

        let mut stack = OperandStack::with_capacity(max_stack);
        let mut block_is_terminated = false;
        for (program_counter, instruction) in instructions.iter().enumerate() {
            if let Some(block) = blocks.get(&program_counter) {
                // The block needs to be filled before switching
                if program_counter != 0 {
                    // Only add a jump if the current block is not already terminated
                    if !block_is_terminated {
                        let block_arguments = stack.as_block_arguments();
                        function_builder.ins().jump(*block, &block_arguments);
                    }

                    function_builder.switch_to_block(*block);
                    stack.reset(&mut function_builder)?;
                }
                // We've switched to a new block, so it's not terminated yet
                block_is_terminated = false;
            } else if block_is_terminated {
                // Skip this instruction; it's unreachable dead code after a control flow change
                // and not a target of any jump
                continue;
            }

            Self::process_instruction(
                constant_pool,
                &mut function_builder,
                &blocks,
                &mut locals,
                &mut stack,
                program_counter,
                return_pointer,
                context_pointer,
                &helpers,
                exception_block,
                exception_table,
                instruction,
            )?;

            // Check if this instruction terminates the block
            if instruction.changes_control_flow() {
                block_is_terminated = true;
            }
        }

        // Emit the shared exception return block body: write NONE to the return pointer
        // and return from the function. This block has no parameters.
        function_builder.switch_to_block(exception_block);
        emit_exception_return(&mut function_builder, return_pointer);

        function_builder.seal_all_blocks();
        function_builder.finalize();

        jit_module.define_function(function, &mut module_context)?;
        jit_module.clear_context(&mut module_context);
        jit_module.finalize_definitions()?;

        let code = jit_module.get_finalized_function(function);
        let function = unsafe {
            let function: fn(*const JitValue, usize, *mut JitValue, *const u8) =
                mem::transmute(code);
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
    fn signature(jit_module: &JITModule) -> Signature {
        let mut signature = jit_module.make_signature();
        let pointer_type = jit_module.target_config().pointer_type();
        signature.params.push(AbiParam::new(pointer_type)); // pointer to JitValue array
        signature.params.push(AbiParam::new(types::I64)); // length of array
        signature.params.push(AbiParam::new(pointer_type)); // return value pointer
        signature.params.push(AbiParam::new(pointer_type)); // runtime context pointer
        signature
    }

    /// Retrieves the function pointers from the function builder.
    ///
    /// # Errors
    ///
    /// If the function pointers cannot be retrieved
    fn function_pointers(
        function_builder: &mut FunctionBuilder,
        block: Block,
    ) -> Result<(Value, Value, Value, Value)> {
        let mut params = function_builder.block_params(block).to_vec();
        let Some(context_pointer) = params.pop() else {
            return Err(InternalError("undefined context pointer".to_string()));
        };
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
        Ok((
            arguments_pointer,
            arguments_length,
            return_pointer,
            context_pointer,
        ))
    }

    /// Creates a new locals array for the function.
    ///
    /// # Errors
    ///
    /// If the locals array cannot be created
    fn locals(
        function_builder: &mut FunctionBuilder,
        descriptor: &JavaStr,
        instructions: &[Instruction],
        arguments_pointer: Value,
        is_static: bool,
    ) -> Result<LocalVariables> {
        let size_of = i64::try_from(size_of::<JitValue>())
            .map_err(|error| InternalError(format!("{error:?}")))?;
        let struct_size = function_builder.ins().iconst(types::I64, size_of);
        let mut local_types = Vec::new();

        // For non-static methods, local variable 0 is `this` (an object reference).
        // It is passed as the first JIT argument but is NOT part of the method descriptor.
        let arg_offset = usize::from(!is_static);
        if !is_static {
            let index = function_builder.ins().iconst(types::I64, 0);
            let offset = function_builder.ins().imul(index, struct_size);
            let address = function_builder.ins().iadd(arguments_pointer, offset);
            let native_type = Self::native_object_type();
            let variable = function_builder.declare_var(native_type);
            local_types.push(native_type);
            let value = function_builder
                .ins()
                .load(native_type, MemFlags::trusted(), address, 8);
            function_builder.def_var(variable, value);
        }

        let (parameter_types, _return_type) = FieldType::parse_method_descriptor(descriptor)?;
        for (index, parameter_type) in parameter_types.iter().enumerate() {
            let arg_index = i64::try_from(index + arg_offset)
                .map_err(|error| InternalError(format!("{error:?}")))?;
            let index = function_builder.ins().iconst(types::I64, arg_index);
            let offset = function_builder.ins().imul(index, struct_size);
            let address = function_builder.ins().iadd(arguments_pointer, offset);

            // Ignore the discriminant
            let native_type = Self::native_type(parameter_type);
            let variable = function_builder.declare_var(native_type);
            local_types.push(native_type);
            let value = function_builder
                .ins()
                .load(native_type, MemFlags::trusted(), address, 8);
            function_builder.def_var(variable, value);

            // The JVM specification requires that Long and Double take two places in the
            // locals list when passed to a method. This method adjusts the variables index
            // to account for this.
            //
            // See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1>
            //
            // NOTE: if the jit compiler is ever updated to re-write the original instructions, this
            //       logic should probably be removed and the instructions should be rewritten to
            //       use sequential variable indices instead.
            match parameter_type {
                FieldType::Base(BaseType::Double) => {
                    local_types.push(native_type);
                    let variable = function_builder.declare_var(native_type);
                    let value = function_builder.ins().f64const(0.0);
                    function_builder.def_var(variable, value);
                }
                FieldType::Base(BaseType::Long) => {
                    local_types.push(native_type);
                    let variable = function_builder.declare_var(native_type);
                    let value = function_builder.ins().iconst(types::I64, 0);
                    function_builder.def_var(variable, value);
                }
                _ => {}
            }
        }

        let non_method_argument_locals_start_index = local_types.len();

        // Add the locals for the method body
        for instruction in instructions {
            let Some((index, field_type)) = instruction.local_type()? else {
                continue;
            };
            let native_type = Self::native_type(&field_type);
            // If the index is greater than the current length of the types, we need to
            // extend the vector with default values until we reach the index.
            while index >= local_types.len() {
                // Default to I8 to indicate an uninitialized local; this is a placeholder type and
                // should be replaced with the correct type later.
                local_types.push(types::I8);
            }

            let existing_type = local_types[index];
            if existing_type == types::I8 {
                local_types[index] = native_type;
            } else if existing_type != native_type {
                // TODO: the jit compiler should handle this case gracefully and rewrite the
                //       instructions to use unique local variables with types that do not conflict.
                return Err(UnsupportedType(format!(
                    "Incompatible local variable type at index {index}: expected {native_type}, found {existing_type}"
                )));
            }
        }

        for mut native_type in local_types
            .into_iter()
            .skip(non_method_argument_locals_start_index)
        {
            if native_type == types::I8 {
                // Default to I32 for uninitialized locals
                native_type = types::I32;
            }

            // Create a variable for the local
            let variable = function_builder.declare_var(native_type);
            let value = match native_type {
                types::F32 => function_builder.ins().f32const(0.0),
                types::F64 => function_builder.ins().f64const(0.0),
                _ => function_builder.ins().iconst(native_type, 0),
            };
            function_builder.def_var(variable, value);
        }

        Ok(LocalVariables::new())
    }

    /// Creates a new native type from the given field type.
    fn native_type(field_type: &FieldType) -> Type {
        match field_type {
            FieldType::Base(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Int
                | BaseType::Short,
            ) => types::I32,
            FieldType::Base(BaseType::Double) => types::F64,
            FieldType::Base(BaseType::Float) => types::F32,
            FieldType::Base(BaseType::Long) => types::I64,
            FieldType::Object(_) | FieldType::Array(_) => Self::native_object_type(),
        }
    }

    /// Returns the native type for object and array field types.
    /// This is used for local variables that hold object references.
    fn native_object_type() -> Type {
        types::I64
    }

    /// Returns the first unsupported instruction in the list, or `None` if all are supported.
    /// This is used for fast-fail before creating the expensive JIT module.
    ///
    /// # Scope of the JIT today
    ///
    /// All `Invoke*` opcodes are rejected here. Combined with the `exception_table.is_empty()`
    /// guard in [`Self::can_compile`], this means production JIT-eligible methods are limited
    /// to leaf, exception-table-free arithmetic / control-flow / array / field-access code.
    ///
    /// A consequence is that the exception-handler dispatch infrastructure built on top of
    /// `pending_exception` (in `instruction::exception` and `populate_dispatch_block`) is
    /// reachable from production methods only through `athrow`, NPE, `ArrayStoreException`,
    /// and `<clinit>` failures from field/array helpers;never through propagation across a
    /// JIT call site or into a JIT catch handler. The dispatch logic is therefore exercised
    /// primarily by the synthetic `athrow_caught_by_catch_all_handler` codegen test until
    /// `Invoke*` lowering and the gating items listed in [`Self::can_compile`] land.
    fn first_unsupported_instruction(instructions: &[Instruction]) -> Option<&Instruction> {
        instructions.iter().find(|instruction| {
            matches!(
                instruction,
                Instruction::Invokevirtual(..)
                    | Instruction::Invokespecial(..)
                    | Instruction::Invokestatic(..)
                    | Instruction::Invokeinterface(..)
                    | Instruction::Invokedynamic(..)
            )
        })
    }

    #[expect(clippy::too_many_arguments)]
    #[expect(clippy::too_many_lines)]
    fn process_instruction(
        constant_pool: &ConstantPool,
        function_builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        locals: &mut LocalVariables,
        stack: &mut OperandStack,
        program_counter: usize,
        return_pointer: Value,
        context_pointer: Value,
        helpers: &RuntimeHelpers,
        exception_block: Block,
        exception_table: &[ExceptionTableEntry],
        instruction: &Instruction,
    ) -> Result<()> {
        let throw_context = ThrowContext {
            exception_block,
            blocks,
            exception_table,
            program_counter,
        };
        let throw_context = &throw_context;
        match instruction {
            Instruction::Nop => nop(),
            Instruction::Aconst_null => aconst_null(function_builder, stack)?,
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
            Instruction::Aload(index) => aload_ref(function_builder, locals, stack, *index)?,
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
            Instruction::Aload_0 => aload_ref_0(function_builder, locals, stack)?,
            Instruction::Aload_1 => aload_ref_1(function_builder, locals, stack)?,
            Instruction::Aload_2 => aload_ref_2(function_builder, locals, stack)?,
            Instruction::Aload_3 => aload_ref_3(function_builder, locals, stack)?,
            Instruction::Iaload => iaload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Laload => laload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Faload => faload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Daload => daload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Aaload => aaload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Baload => baload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Caload => caload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Saload => saload(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Istore(index) => istore(function_builder, locals, stack, *index)?,
            Instruction::Lstore(index) => lstore(function_builder, locals, stack, *index)?,
            Instruction::Fstore(index) => fstore(function_builder, locals, stack, *index)?,
            Instruction::Dstore(index) => dstore(function_builder, locals, stack, *index)?,
            Instruction::Astore(index) => astore_ref(function_builder, locals, stack, *index)?,
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
            Instruction::Astore_0 => astore_ref_0(function_builder, locals, stack)?,
            Instruction::Astore_1 => astore_ref_1(function_builder, locals, stack)?,
            Instruction::Astore_2 => astore_ref_2(function_builder, locals, stack)?,
            Instruction::Astore_3 => astore_ref_3(function_builder, locals, stack)?,
            Instruction::Iastore => iastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Lastore => lastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Fastore => fastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Dastore => dastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Aastore => aastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Bastore => bastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Castore => castore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Sastore => sastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
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
            Instruction::Ifeq(address) => {
                ifeq(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifne(address) => {
                ifne(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Iflt(address) => {
                iflt(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifge(address) => {
                ifge(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifgt(address) => {
                ifgt(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifle(address) => {
                ifle(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmpeq(address) => {
                if_icmpeq(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmpne(address) => {
                if_icmpne(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmplt(address) => {
                if_icmplt(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmpge(address) => {
                if_icmpge(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmpgt(address) => {
                if_icmpgt(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_icmple(address) => {
                if_icmple(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_acmpeq(address) => {
                if_acmpeq(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::If_acmpne(address) => {
                if_acmpne(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Goto(address) => goto(function_builder, blocks, stack, *address)?,
            Instruction::Jsr(address) => {
                jsr(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ret(index) => ret(function_builder, blocks, locals, stack, *index)?,
            Instruction::Tableswitch(table_switch) => tableswitch(
                function_builder,
                blocks,
                stack,
                program_counter,
                table_switch,
            )?,
            Instruction::Lookupswitch(lookup_switch) => lookupswitch(
                function_builder,
                blocks,
                stack,
                program_counter,
                lookup_switch,
            )?,
            Instruction::Ireturn => ireturn(function_builder, stack, return_pointer)?,
            Instruction::Lreturn => lreturn(function_builder, stack, return_pointer)?,
            Instruction::Freturn => freturn(function_builder, stack, return_pointer)?,
            Instruction::Dreturn => dreturn(function_builder, stack, return_pointer)?,
            Instruction::Areturn => areturn(function_builder, stack, return_pointer)?,
            Instruction::Return => r#return(function_builder, stack, return_pointer),
            Instruction::Getstatic(index) => getstatic(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::Putstatic(index) => putstatic(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::Getfield(index) => getfield(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::Putfield(index) => putfield(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::New(index) => new_object(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::Newarray(array_type) => {
                newarray(
                    function_builder,
                    stack,
                    array_type,
                    context_pointer,
                    helpers,
                )?;
            }
            Instruction::Anewarray(index) => anewarray(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
            )?,
            Instruction::Arraylength => arraylength(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Athrow => athrow(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
            )?,
            Instruction::Checkcast(class_index) => checkcast(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *class_index,
            )?,
            Instruction::Instanceof(class_index) => instanceof(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *class_index,
            )?,
            Instruction::Monitorenter => monitorenter(stack)?,
            Instruction::Monitorexit => monitorexit(stack)?,
            Instruction::Wide => wide()?,
            Instruction::Multianewarray(index, dimensions) => multianewarray(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                *dimensions,
            )?,
            Instruction::Ifnull(address) => {
                ifnull(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifnonnull(address) => {
                ifnonnull(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Goto_w(address) => goto_w(function_builder, blocks, stack, *address)?,
            Instruction::Jsr_w(address) => {
                jsr_w(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Breakpoint => breakpoint(function_builder, stack),
            Instruction::Impdep1 => impdep1(function_builder, stack),
            Instruction::Impdep2 => impdep2(function_builder, stack),
            // Wide instructions
            Instruction::Iload_w(index) => iload_w(function_builder, locals, stack, *index)?,
            Instruction::Lload_w(index) => lload_w(function_builder, locals, stack, *index)?,
            Instruction::Fload_w(index) => fload_w(function_builder, locals, stack, *index)?,
            Instruction::Dload_w(index) => dload_w(function_builder, locals, stack, *index)?,
            Instruction::Aload_w(index) => aload_ref_w(function_builder, locals, stack, *index)?,
            Instruction::Istore_w(index) => istore_w(function_builder, locals, stack, *index)?,
            Instruction::Lstore_w(index) => lstore_w(function_builder, locals, stack, *index)?,
            Instruction::Fstore_w(index) => fstore_w(function_builder, locals, stack, *index)?,
            Instruction::Dstore_w(index) => dstore_w(function_builder, locals, stack, *index)?,
            Instruction::Astore_w(index) => astore_ref_w(function_builder, locals, stack, *index)?,
            Instruction::Iinc_w(index, constant) => {
                iinc_w(function_builder, locals, *index, *constant)?;
            }
            Instruction::Ret_w(index) => ret_w(function_builder, blocks, locals, stack, *index)?,
            _ => {
                return Err(UnsupportedInstruction(instruction.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::JavaString;

    #[test]
    fn test_compiler_new() {
        let result = Compiler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_name() {
        let class_name = "java/lang/Object";
        let method_name = "<clinit>";
        let function_name = Compiler::function_name(class_name, method_name);
        assert_eq!("java_lang_Object__clinit", function_name);
    }

    #[test]
    fn test_native_type_int() {
        for base_type in [
            BaseType::Boolean,
            BaseType::Byte,
            BaseType::Char,
            BaseType::Int,
            BaseType::Short,
        ] {
            assert_eq!(
                Compiler::native_type(&FieldType::Base(base_type)),
                types::I32
            );
        }
    }

    #[test]
    fn test_native_type_double() {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Double)),
            types::F64
        );
    }

    #[test]
    fn test_native_type_float() {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Float)),
            types::F32
        );
    }

    #[test]
    fn test_native_type_long() {
        assert_eq!(
            Compiler::native_type(&FieldType::Base(BaseType::Long)),
            types::I64
        );
    }

    #[test]
    fn test_native_type_object() {
        let class_name = JavaString::from("java/lang/Object");
        assert_eq!(
            Compiler::native_type(&FieldType::Object(class_name)),
            types::I64
        );
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
            assert_eq!(
                Compiler::native_type(&FieldType::Array(Box::new(field_type))),
                types::I64
            );
        }
    }

    #[test]
    fn test_native_type_array_double() {
        let field_type = FieldType::Base(BaseType::Double);
        assert_eq!(
            Compiler::native_type(&FieldType::Array(Box::new(field_type))),
            types::I64
        );
    }

    #[test]
    fn test_native_type_array_float() {
        let field_type = FieldType::Base(BaseType::Float);
        assert_eq!(
            Compiler::native_type(&FieldType::Array(Box::new(field_type))),
            types::I64
        );
    }

    #[test]
    fn test_native_type_array_long() {
        let field_type = FieldType::Base(BaseType::Long);
        assert_eq!(
            Compiler::native_type(&FieldType::Array(Box::new(field_type))),
            types::I64
        );
    }

    #[test]
    fn test_native_type_array_object() {
        let class_name = JavaString::from("java/lang/Object");
        let field_type = FieldType::Object(class_name);
        assert_eq!(
            Compiler::native_type(&FieldType::Array(Box::new(field_type))),
            types::I64
        );
    }
}

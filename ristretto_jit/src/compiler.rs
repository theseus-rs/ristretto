use crate::Error::{
    InternalError, InvalidBlockAddress, UnsupportedInstruction, UnsupportedTargetISA,
};
use crate::control_flow_graph::InstructionControlFlow;
use crate::control_flow_graph::StackType;
use crate::function::Function;
use crate::instruction::{CallTarget, ThrowContext, array_access, array_access_metadata};
use crate::instruction::{
    aastore, aconst_null, aload_ref, aload_ref_0, aload_ref_1, aload_ref_2, aload_ref_3,
    aload_ref_w, anewarray, areturn, astore_ref, astore_ref_0, astore_ref_1, astore_ref_2,
    astore_ref_3, astore_ref_w, athrow, bipush, breakpoint, checkcast, d2f, d2i, d2l, dadd, dcmpg,
    dcmpl, dconst_0, dconst_1, ddiv, dload, dload_0, dload_1, dload_2, dload_3, dload_w, dmul,
    dneg, drem, dreturn, dstore, dstore_0, dstore_1, dstore_2, dstore_3, dstore_w, dsub, dup,
    dup_x1, dup_x2, dup2, dup2_x1, dup2_x2, emit_exception_return, f2d, f2i, f2l, fadd, fcmpg,
    fcmpl, fconst_0, fconst_1, fconst_2, fdiv, fload, fload_0, fload_1, fload_2, fload_3, fload_w,
    fmul, fneg, frem, freturn, fstore, fstore_0, fstore_1, fstore_2, fstore_3, fstore_w, fsub,
    getfield, getstatic, goto, goto_w, i2b, i2c, i2d, i2f, i2l, i2s, iadd, iand, iconst_0,
    iconst_1, iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, idiv, if_acmpeq, if_acmpne,
    if_icmpeq, if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne, ifeq, ifge, ifgt, ifle, iflt,
    ifne, ifnonnull, ifnull, iinc, iinc_w, iload, iload_0, iload_1, iload_2, iload_3, iload_w,
    impdep1, impdep2, imul, ineg, instanceof, invoke, ior, irem, ireturn, ishl, ishr, istore,
    istore_0, istore_1, istore_2, istore_3, istore_w, isub, iushr, ixor, jsr, jsr_w, l2d, l2f, l2i,
    ladd, land, lcmp, lconst_0, lconst_1, ldc, ldc2_w, ldiv, lload, lload_0, lload_1, lload_2,
    lload_3, lload_w, lmul, lneg, lookupswitch, lor, lrem, lreturn, lshl, lshr, lstore, lstore_0,
    lstore_1, lstore_2, lstore_3, lstore_w, lsub, lushr, lxor, monitor, multianewarray, new_object,
    newarray, nop, pop, pop2, putfield, putstatic, ret, ret_w, r#return, sipush, swap, tableswitch,
    wide,
};
use crate::local_type::LocalType;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{JitValue, Result, control_flow_graph};
use ahash::AHashMap;
use cranelift::codegen::ir::{MemFlagsData, StackSlot, UserFuncName};
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

#[cfg(feature = "execution-profile")]
#[path = "backedge_profile.rs"]
mod backedge_profile;
#[path = "continuation.rs"]
pub(crate) mod continuation;
#[path = "frame_roots.rs"]
mod frame_roots;
#[path = "native_frame.rs"]
mod native_frame;
#[path = "reference_liveness.rs"]
mod reference_liveness;

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

enum Rejection<'a> {
    Method(&'static str),
    Instruction(&'static str, &'a [Instruction], &'a Instruction),
}

impl Rejection<'_> {
    fn reason(&self) -> &'static str {
        match self {
            Self::Method(reason) | Self::Instruction(reason, ..) => reason,
        }
    }

    fn bytecode(&self) -> Option<(usize, u8)> {
        let Self::Instruction(_, code, rejected) = self else {
            return None;
        };
        Instruction::with_offsets(code).find_map(|(offset, instruction)| {
            std::ptr::eq(instruction, *rejected).then(|| (offset, instruction.code()))
        })
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
                return Err(InternalError(
                    ["unable to set jit compiler flag '", name, "'=", value, "'"].concat(),
                ));
            }
        }
        let flags = Flags::new(flag_builder);
        let target_isa = isa_builder.finish(flags)?;
        if !matches!(target_isa.name(), "x64" | "aarch64" | "riscv64") {
            return Err(UnsupportedTargetISA(
                "native frame sizing is not supported for this ISA",
            ));
        }
        Ok(Compiler { target_isa })
    }

    /// Creates a new JIT module for the compiler using the cached target ISA.
    pub(crate) fn jit_module(&self, symbols: &[(&str, *const u8)]) -> JITModule {
        let mut jit_builder =
            JITBuilder::with_isa(Arc::clone(&self.target_isa), default_libcall_names());
        for &(name, ptr) in symbols {
            jit_builder.symbol(name, ptr);
        }
        jit_builder.symbol(native_frame::PROBE_SYMBOL, native_frame::probe as *const u8);
        #[cfg(feature = "execution-profile")]
        jit_builder.symbol(
            backedge_profile::SYMBOL,
            backedge_profile::record as *const u8,
        );
        JITModule::new(jit_builder)
    }

    /// Returns true if the method can potentially be JIT compiled.
    /// This performs a fast scan of the method's bytecode instructions to check
    /// for unsupported opcodes without doing any expensive compilation work.
    ///
    /// Production execution is limited to static primitive methods. Backward/self edges
    /// use an owning continuation ABI so the VM can yield and cancel between bounded slices.
    /// Reference operations and runtime-helper calls remain available to focused
    /// compiler tests while their frame-root and semantic qualification is incomplete.
    ///
    /// Methods with non-empty exception tables are also rejected. The
    /// exception-handler dispatch codegen (in `instruction::exception` and
    /// `populate_dispatch_block`) is implemented and unit-tested via the synthetic
    /// `athrow_caught_by_catch_all_handler` test, but enabling it for production
    /// methods exposes unrelated JIT gaps:
    ///
    /// - Monitor runtime integration remains experimental until complete synchronization,
    ///   cancellation and Java memory-ordering qualification.
    /// - Array `iaload`/`aaload`/`arraylength` NPEs from JIT-compiled methods
    ///   currently report the wrong extended message (the receiver name is
    ///   substituted instead of the array name).
    ///
    /// Once those gaps are closed, this guard can be removed in favor of the
    /// dispatch path being exercised by integration tests.
    #[must_use]
    pub fn can_compile(method: &Method) -> bool {
        Self::rejection_reason(method).is_none()
    }

    /// Reports the first production eligibility restriction, without compiling.
    /// This uses the same gate as [`Self::can_compile`].
    #[must_use]
    pub fn rejection_reason(method: &Method) -> Option<&'static str> {
        Self::production_rejection(None, method).map(|rejection| rejection.reason())
    }

    /// JVM byte offset and opcode for a restriction identifiable without the declaring class.
    #[must_use]
    pub fn rejected_bytecode(method: &Method) -> Option<(usize, u8)> {
        Self::production_rejection(None, method)?.bytecode()
    }

    /// Check ordinary dispatch with the declaring constant pool available.
    #[must_use]
    pub fn can_compile_in(class: &ClassFile, method: &Method) -> bool {
        Self::production_rejection(Some(class), method).is_none()
    }

    /// Diagnose the same classfile-aware policy used by ordinary VM dispatch.
    #[must_use]
    pub fn rejection_in(class: &ClassFile, method: &Method) -> Option<crate::CompilationRejection> {
        Self::production_rejection(Some(class), method).map(|rejection| {
            crate::CompilationRejection {
                reason: rejection.reason(),
                bytecode: rejection.bytecode(),
            }
        })
    }

    fn production_rejection<'m>(
        class: Option<&ClassFile>,
        method: &'m Method,
    ) -> Option<Rejection<'m>> {
        if !method.access_flags.contains(MethodAccessFlags::STATIC) {
            return Some(Rejection::Method("non_static_method"));
        }
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
            return Some(Rejection::Method("missing_code"));
        };
        if !exception_table.is_empty() {
            return Some(Rejection::Method("exception_handlers"));
        }
        if let Some(instruction) = Self::first_unsupported_instruction(instructions) {
            return Some(Rejection::Instruction(
                "unsupported_invocation",
                instructions,
                instruction,
            ));
        }
        instructions
            .iter()
            .find(|instruction| {
                if Self::is_production_safe_instruction(instruction) {
                    return false;
                }
                // A long store carries only primitive bits through its checked
                // field request. Keep the ordinary root handoff at this boundary.
                !matches!(instruction, Instruction::Putstatic(index) if class
                    .and_then(|class| Self::static_field_descriptor(class, *index))
                    .is_some_and(|descriptor| descriptor.as_bytes() == b"J"))
            })
            .map(|instruction| {
                Rejection::Instruction("unqualified_instruction", instructions, instruction)
            })
    }

    /// Reports restrictions for explicit experimental reference-method preparation.
    ///
    /// This additive policy does not change production eligibility. The first slice
    /// supports object graphs, fields and static/constructor calls, using owning
    /// runtime requests and reference-bearing continuations. Monitors, exception
    /// handlers, arrays and virtual/interface dispatch remain outside this slice.
    #[cfg(feature = "experimental-reference-dispatch")]
    #[must_use]
    pub fn experimental_rejection_reason(
        class: &ClassFile,
        method: &Method,
    ) -> Option<&'static str> {
        Self::reference_rejection_reason(class, method, false)
    }

    /// Capability policy for request-only code, additionally admitting primitive
    /// arrays, reference arrays with class components, and `checkcast` requests for
    /// classes or one-dimensional arrays. Multidimensional allocation/cast targets
    /// and `instanceof` remain excluded. The embedding must separately
    /// admit descriptors/component layouts, validate token types and perform Java
    /// null, bounds, reference-store and cast compatibility checks after each request.
    /// A successful cast must return the original owning token (or null).
    /// Static fields use the existing zero/one-cell requests. Their constant-pool
    /// metadata must be well formed; field access rules, declaring-class
    /// initialization and supported layouts remain the embedding's responsibility.
    #[cfg(feature = "experimental-reference-dispatch")]
    #[must_use]
    pub fn request_rejection_reason(class: &ClassFile, method: &Method) -> Option<&'static str> {
        let Ok(descriptor) = class.constant_pool.try_get_utf8(method.descriptor_index) else {
            return Some("experimental_descriptor");
        };
        if FieldType::parse_method_descriptor(descriptor).is_err() {
            return Some("experimental_descriptor");
        }
        Self::reference_rejection_reason(class, method, true)
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    fn is_request_extension_instruction(class: &ClassFile, instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Checkcast(index) => {
                let Ok(name) = class.constant_pool.try_get_class(*index) else {
                    return false;
                };
                if name.as_bytes().first() == Some(&b'[') {
                    return matches!(FieldType::parse_java_str(name), Ok(FieldType::Array(component))
                        if !matches!(*component, FieldType::Array(_)));
                }
                name.as_bytes().split(|byte| *byte == b'/').all(|part| {
                    !part.is_empty() && part.iter().all(|byte| !matches!(byte, b'.' | b';' | b'['))
                })
            }
            Instruction::Anewarray(index) => class
                .constant_pool
                .try_get_class(*index)
                .is_ok_and(|name| !name.to_str_lossy().starts_with('[')),
            Instruction::Invokevirtual(_)
            | Instruction::Invokeinterface(..)
            | Instruction::Newarray(_)
            | Instruction::Arraylength
            | Instruction::Aaload
            | Instruction::Aastore
            | Instruction::Iaload
            | Instruction::Laload
            | Instruction::Faload
            | Instruction::Daload
            | Instruction::Baload
            | Instruction::Caload
            | Instruction::Saload
            | Instruction::Iastore
            | Instruction::Lastore
            | Instruction::Fastore
            | Instruction::Dastore
            | Instruction::Bastore
            | Instruction::Castore
            | Instruction::Sastore => true,
            _ => false,
        }
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    fn reference_rejection_reason(
        class: &ClassFile,
        method: &Method,
        request_extensions: bool,
    ) -> Option<&'static str> {
        if method
            .access_flags
            .contains(MethodAccessFlags::SYNCHRONIZED)
        {
            return Some("experimental_synchronized_method");
        }
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
            return Some("missing_code");
        };
        if !exception_table.is_empty() {
            return Some("experimental_exception_handlers");
        }
        if request_extensions
            && instructions.iter().any(|instruction| {
                matches!(instruction, Instruction::Getstatic(index) | Instruction::Putstatic(index)
                    if !Self::request_static_metadata_valid(class, *index))
            })
        {
            return Some("experimental_static_field_metadata");
        }
        if instructions.iter().all(|instruction| {
            Self::is_production_safe_instruction(instruction)
                || (request_extensions
                    && Self::is_request_extension_instruction(class, instruction))
                || match instruction {
                    Instruction::Ldc(index) => matches!(
                        class.constant_pool.get(u16::from(*index)),
                        Some(
                            ristretto_classfile::Constant::Integer(_)
                                | ristretto_classfile::Constant::Float(_)
                        )
                    ),
                    Instruction::Ldc_w(index) => matches!(
                        class.constant_pool.get(*index),
                        Some(
                            ristretto_classfile::Constant::Integer(_)
                                | ristretto_classfile::Constant::Float(_)
                        )
                    ),
                    _ => false,
                }
                || matches!(
                    instruction,
                    Instruction::Aconst_null
                        | Instruction::Aload(_)
                        | Instruction::Aload_w(_)
                        | Instruction::Aload_0
                        | Instruction::Aload_1
                        | Instruction::Aload_2
                        | Instruction::Aload_3
                        | Instruction::Astore(_)
                        | Instruction::Astore_w(_)
                        | Instruction::Astore_0
                        | Instruction::Astore_1
                        | Instruction::Astore_2
                        | Instruction::Astore_3
                        | Instruction::Areturn
                        | Instruction::Ifnull(_)
                        | Instruction::Ifnonnull(_)
                        | Instruction::If_acmpeq(_)
                        | Instruction::If_acmpne(_)
                        | Instruction::New(_)
                        | Instruction::Getfield(_)
                        | Instruction::Putfield(_)
                        | Instruction::Getstatic(_)
                        | Instruction::Putstatic(_)
                        | Instruction::Invokestatic(_)
                        | Instruction::Invokespecial(_)
                )
        }) {
            None
        } else {
            Some("experimental_unqualified_instruction")
        }
    }

    fn static_field_descriptor<'a>(class: &'a ClassFile<'_>, index: u16) -> Option<&'a JavaStr> {
        let pool = &class.constant_pool;
        let (owner, member) = pool.try_get_field_ref(index).ok()?;
        pool.try_get_class(*owner).ok()?;
        let (name, descriptor) = pool.try_get_name_and_type(*member).ok()?;
        if pool.try_get_utf8(*name).ok()?.is_empty() {
            return None;
        }
        pool.try_get_utf8(*descriptor).ok()
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    fn request_static_metadata_valid(class: &ClassFile, index: u16) -> bool {
        Self::static_field_descriptor(class, index)
            .is_some_and(|descriptor| FieldType::parse_java_str(descriptor).is_ok())
    }

    /// Compiles the given bytecode into native code.
    ///
    /// # Errors
    ///
    /// if the Java byte code cannot be compiled to native code
    pub fn compile(
        &self,
        class_file: &ClassFile,
        method: &Method,
        symbols: &[(&str, *const u8)],
    ) -> Result<Function> {
        self.compile_mode(class_file, method, symbols, false)
    }

    /// Compile the request-only subset with exclusively resumable runtime requests.
    ///
    /// Generated code never calls synchronous allocation, field, array, cast or invocation
    /// helpers. Only frame publication and pending-error callbacks are needed by
    /// reference-bearing methods. The embedding owns all reference tokens and must
    /// complete each request after the native frame has returned. Synchronous
    /// [`Function::execute`] is disabled for the returned function.
    ///
    /// # Errors
    /// Rejects methods outside the explicit experimental capability policy or
    /// methods that fail native compilation and bounded frame/state limits.
    #[cfg(feature = "experimental-reference-dispatch")]
    pub fn compile_requests(
        &self,
        class_file: &ClassFile,
        method: &Method,
        symbols: &[(&str, *const u8)],
    ) -> Result<Function> {
        if let Some(reason) = Self::request_rejection_reason(class_file, method) {
            return Err(crate::Error::UnsupportedMethod(reason.to_owned()));
        }
        self.compile_mode(class_file, method, symbols, true)
    }

    #[expect(clippy::too_many_lines)]
    fn compile_mode(
        &self,
        class_file: &ClassFile,
        method: &Method,
        symbols: &[(&str, *const u8)],
        requests_only: bool,
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
        let (_, return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
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

        let (entry_block, blocks, reference_parameters) = control_flow_graph::get_blocks(
            &mut function_builder,
            constant_pool,
            instructions,
            exception_table,
        )?;
        #[cfg(feature = "execution-profile")]
        let backedge_profile = backedge_profile::BackedgeProfile::new(
            &mut jit_module,
            &mut function_builder,
            &blocks,
        )?;
        // Use the entry block for function setup (entry_block cannot be jumped to)
        function_builder.switch_to_block(entry_block);
        function_builder.append_block_params_for_function_params(entry_block);
        let (arguments_pointer, arguments_length, return_pointer, context_pointer) =
            Self::function_pointers(&mut function_builder, entry_block)?;
        // One shared outgoing buffer per activation, rather than one stack slot
        // per call site. Nested calls have their own native activation buffers.
        let mut request_capacity = exception_table
            .iter()
            .any(|entry| entry.catch_type != 0)
            .then_some(0);
        for instruction in instructions {
            if let Some(count) = continuation::argument_count(instruction, constant_pool)? {
                request_capacity = Some(request_capacity.unwrap_or(0).max(count));
            }
        }
        let call_arguments = if let Some(capacity) = request_capacity {
            Some(native_frame::word_stack_slot(
                &mut function_builder,
                capacity.max(1),
            )?)
        } else {
            None
        };

        let (mut locals, argument_types) = Self::locals(
            &mut function_builder,
            method_descriptor,
            instructions,
            arguments_pointer,
            is_static,
        )?;
        // This initial ABI is exercised by direct backend/VM tests. Qualified
        // primitive methods need no runtime-helper calls or frame-owned references.
        // Keep root requirements independent of the production eligibility gate:
        // qualifying another opcode must never silently remove its root handoff.
        let rootless = is_static
            && exception_table.is_empty()
            && instructions.iter().all(|instruction| {
                Self::is_rootless_primitive_instruction(instruction)
                    || (requests_only
                        && match instruction {
                            Instruction::Ldc(index) => matches!(
                                constant_pool.get(u16::from(*index)),
                                Some(
                                    ristretto_classfile::Constant::Integer(_)
                                        | ristretto_classfile::Constant::Float(_)
                                )
                            ),
                            Instruction::Ldc_w(index) => matches!(
                                constant_pool.get(*index),
                                Some(
                                    ristretto_classfile::Constant::Integer(_)
                                        | ristretto_classfile::Constant::Float(_)
                                )
                            ),
                            _ => false,
                        })
            });
        // Calls return requests to the async VM. Handler entries also consume
        // local fuel: exception edges can form loops without a backward branch.
        let continuation = if instructions.iter().all(|instruction| {
            !matches!(
                instruction,
                Instruction::Invokedynamic(_)
                    | Instruction::Jsr(_)
                    | Instruction::Jsr_w(_)
                    | Instruction::Ret(_)
                    | Instruction::Ret_w(_)
            )
        }) && (requests_only
            || !exception_table.is_empty()
            || instructions.iter().any(|instruction| {
                instruction.branch_offsets().any(|offset| offset <= 0)
                    || CallTarget::from_instruction(instruction).is_some()
                    || crate::instruction::allocation_metadata(instruction).is_some()
                    || crate::instruction::field_metadata(instruction).is_some()
                    || array_access_metadata(instruction).is_some()
                    || crate::instruction::reference_constant_index(instruction, constant_pool)
                        .is_some()
                    || matches!(
                        instruction,
                        Instruction::Monitorenter
                            | Instruction::Monitorexit
                            | Instruction::Checkcast(_)
                            | Instruction::Instanceof(_)
                            | Instruction::Aastore
                    )
            })) {
            Some(continuation::Continuation::new(
                &mut function_builder,
                &blocks,
                instructions,
                &locals,
                &reference_parameters,
                !rootless,
                max_stack,
                constant_pool,
                exception_table,
                requests_only,
            )?)
        } else {
            None
        };
        let continuation_pointer = context_pointer;
        let context_pointer = if continuation
            .as_ref()
            .is_some_and(|state| state.layout.requires_context)
        {
            let pointer_type = function_builder.func.dfg.value_type(context_pointer);
            function_builder.ins().load(
                pointer_type,
                MemFlagsData::trusted(),
                continuation_pointer,
                native_frame::word_offset(crate::function::ContinuationLayout::CONTEXT)?,
            )
        } else {
            context_pointer
        };

        // Get the block for address 0 (may be entry_block or a separate loop body block)
        let block_zero = *blocks.get(&0).ok_or_else(|| InvalidBlockAddress(0))?;

        // Shared exception return block. All instructions that can throw branch here on a
        // pending exception. The block body writes a NONE discriminant to the return pointer
        // and returns.
        let exception_block = function_builder.create_block();
        let size_probe = native_frame::SizeProbe::new(&mut function_builder);
        let mut frame_roots = if rootless {
            None
        } else {
            size_probe.initialize(&mut function_builder);
            Some(frame_roots::FrameRoots::new(
                &mut function_builder,
                &locals,
                max_stack,
                instructions,
                exception_table,
                exception_block,
                size_probe.slot(),
            )?)
        };

        if rootless {
            let body = if continuation.is_some() {
                function_builder.create_block()
            } else {
                block_zero
            };
            size_probe.emit_cold_path(
                &mut jit_module,
                &mut function_builder,
                arguments_length,
                body,
                exception_block,
            )?;
            if let Some(continuation) = &continuation {
                function_builder.switch_to_block(body);
                continuation.entry(&mut function_builder, continuation_pointer, block_zero)?;
            }
        } else if let Some(continuation) = &continuation {
            continuation.entry(&mut function_builder, continuation_pointer, block_zero)?;
        } else {
            function_builder.ins().jump(block_zero, &[]);
        }
        function_builder.switch_to_block(block_zero);

        let mut stack = OperandStack::with_capacity(max_stack);
        stack.mark_references(reference_parameters);
        let mut continued_blocks = continuation.as_ref().map(|_| blocks.clone());
        let handler_blocks = continuation
            .as_ref()
            .filter(|_| !exception_table.is_empty())
            .map(|continuation| continuation.handler_targets(&blocks, exception_table))
            .transpose()?;
        let mut block_is_terminated = false;
        for (program_counter, instruction) in Instruction::with_offsets(instructions) {
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

            if let Some(frame_roots) = &mut frame_roots {
                frame_roots.publish(
                    &mut function_builder,
                    &locals,
                    stack
                        .as_slice()
                        .iter()
                        .copied()
                        .filter(|value| stack.is_reference(*value)),
                    &helpers,
                    context_pointer,
                    program_counter,
                )?;
            }
            if let (Some(continuation), Some(targets)) = (&continuation, &mut continued_blocks) {
                continuation.redirect(targets, program_counter, instruction)?;
            }
            let instruction_blocks = continued_blocks.as_ref().unwrap_or(&blocks);
            #[cfg(feature = "execution-profile")]
            let backedges = backedge_profile.edges(
                &mut function_builder,
                instruction_blocks,
                program_counter,
                instruction,
            )?;
            #[cfg(feature = "execution-profile")]
            let instruction_blocks = backedges
                .as_ref()
                .map_or(instruction_blocks, |edges| &edges.blocks);
            Self::process_instruction(
                constant_pool,
                &mut function_builder,
                instruction_blocks,
                handler_blocks.as_ref().unwrap_or(&blocks),
                &mut locals,
                &mut stack,
                program_counter,
                return_pointer,
                return_type.as_ref(),
                context_pointer,
                &helpers,
                call_arguments,
                continuation
                    .as_ref()
                    .map(|continuation| (continuation, continuation_pointer)),
                exception_block,
                exception_table,
                instruction,
            )?;
            #[cfg(feature = "execution-profile")]
            if let Some(edges) = backedges {
                backedge_profile.emit(&mut function_builder, edges);
            }
            if let Some(targets) = &mut continued_blocks {
                continuation::Continuation::restore_targets(
                    targets,
                    &blocks,
                    program_counter,
                    instruction,
                )?;
            }

            // Check if this instruction terminates the block
            if instruction.changes_control_flow() {
                block_is_terminated = true;
            }
        }

        if let Some(continuation) = &continuation {
            continuation.finish(
                &mut function_builder,
                continuation_pointer,
                &locals,
                frame_roots.as_mut(),
                &helpers,
                context_pointer,
            )?;
        }

        // Emit the shared exception return block body: write NONE to the return pointer
        // and return from the function. This block has no parameters.
        function_builder.switch_to_block(exception_block);
        emit_exception_return(&mut function_builder, return_pointer)?;

        function_builder.seal_all_blocks();
        function_builder.finalize(jit_module.target_config());

        jit_module.define_function(function, &mut module_context)?;
        let native_stack_bytes = match native_frame::stack_bytes(&module_context) {
            Ok(bytes) => bytes,
            Err(error) => {
                // No entry pointer has escaped; release code allocated before metadata rejection.
                unsafe { jit_module.free_memory() };
                return Err(error);
            }
        };
        #[cfg(feature = "execution-profile")]
        let code_bytes = module_context
            .compiled_code()
            .map_or(0, |code| code.code_buffer().len());
        jit_module.clear_context(&mut module_context);
        jit_module.finalize_definitions()?;

        let code = jit_module.get_finalized_function(function);
        let function = unsafe {
            let function: unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8) =
                mem::transmute(code);
            Function::with_module(function, jit_module, native_stack_bytes, argument_types)
                .with_continuation(continuation.map(|continuation| continuation.layout))
        };
        #[cfg(feature = "execution-profile")]
        let function = function.with_profile(code_bytes, backedge_profile.counter);
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
        [&class_name, "__", method_name].concat()
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
    ) -> Result<(LocalVariables, Vec<i8>)> {
        let size_of = i64::try_from(size_of::<JitValue>())?;
        let struct_size = function_builder.ins().iconst(types::I64, size_of);
        let mut locals = LocalVariables::new();
        let (parameter_types, _) = FieldType::parse_method_descriptor(descriptor)?;
        let parameters = std::iter::once(StackType::Reference)
            .take(usize::from(!is_static))
            .chain(parameter_types.iter().map(Self::local_kind));
        let mut local_index = 0;
        let mut argument_types =
            Vec::with_capacity(parameter_types.len() + usize::from(!is_static));
        for (argument_index, kind) in parameters.enumerate() {
            let tag = match kind {
                StackType::Reference => crate::jit_value::PTR,
                StackType::Primitive(types::I32) => crate::jit_value::I32,
                StackType::Primitive(types::I64) => crate::jit_value::I64,
                StackType::Primitive(types::F32) => crate::jit_value::F32,
                StackType::Primitive(types::F64) => crate::jit_value::F64,
                StackType::Primitive(_) => {
                    return Err(InternalError(
                        "unsupported argument representation".to_owned(),
                    ));
                }
            };
            argument_types.push(tag);
            let index = function_builder
                .ins()
                .iconst(types::I64, i64::try_from(argument_index)?);
            let offset = function_builder.ins().imul(index, struct_size);
            let address = function_builder.ins().iadd(arguments_pointer, offset);
            let variable = locals.declare(function_builder, local_index, kind);
            // Function::execute validates count and discriminants from this same
            // parameter sequence before entering these untagged loads.
            let value = function_builder.ins().load(
                kind.machine_type(),
                MemFlagsData::trusted(),
                address,
                i32::try_from(JitValue::VALUE_OFFSET)?,
            );
            function_builder.def_var(variable, value);
            // A wide argument consumes two JVM slots but has one SSA value. Its
            // second slot may later be reused by a different typed local.
            local_index += if kind.is_category_two() { 2 } else { 1 };
        }
        for instruction in instructions {
            if let Some((index, field_type)) = instruction.local_type()? {
                locals.declare(function_builder, index, Self::local_kind(&field_type));
            }
        }
        Ok((locals, argument_types))
    }

    fn local_kind(field_type: &FieldType) -> StackType {
        match field_type {
            FieldType::Object(_) | FieldType::Array(_) => StackType::Reference,
            FieldType::Base(_) => StackType::Primitive(Self::native_type(field_type)),
        }
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
    /// Dynamic calls remain rejected here. Static/special/virtual/interface
    /// calls use the experimental runtime bridge. This checks backend support only;
    /// [`Self::can_compile`] additionally restricts production execution to static,
    /// primitive instructions without exception tables. Loops use resumable execution.
    /// Reference operations and helper calls remain excluded.
    ///
    /// Direct compiler tests exercise the wider lowering surface, including
    /// `pending_exception` dispatch and `athrow_caught_by_catch_all_handler`.
    /// Those tests do not qualify live-VM frame rooting or control-flow behavior.
    /// The production gate must remain until the conditions in [`Self::can_compile`]
    /// are resolved and covered by VM integration tests.
    fn first_unsupported_instruction(instructions: &[Instruction]) -> Option<&Instruction> {
        instructions
            .iter()
            .find(|instruction| matches!(instruction, Instruction::Invokedynamic(..)))
    }

    /// Production eligibility can expand only after qualification. Root handoff
    /// selection separately uses the narrower rootless classification below.
    fn is_production_safe_instruction(instruction: &Instruction) -> bool {
        Self::is_rootless_primitive_instruction(instruction)
    }

    fn is_primitive_branch(instruction: &Instruction) -> bool {
        matches!(
            instruction,
            Instruction::Ifeq(_)
                | Instruction::Ifne(_)
                | Instruction::Iflt(_)
                | Instruction::Ifge(_)
                | Instruction::Ifgt(_)
                | Instruction::Ifle(_)
                | Instruction::If_icmpeq(_)
                | Instruction::If_icmpne(_)
                | Instruction::If_icmplt(_)
                | Instruction::If_icmpge(_)
                | Instruction::If_icmpgt(_)
                | Instruction::If_icmple(_)
                | Instruction::Goto(_)
                | Instruction::Goto_w(_)
                | Instruction::Tableswitch(_)
                | Instruction::Lookupswitch(_)
        )
    }

    /// A conservative finite-control-flow subset: every explicit destination must
    /// advance the bytecode offset. Including switch defaults is essential; a
    /// single self/backward edge could prevent the executor from regaining control.
    #[cfg(test)]
    fn is_forward_primitive_branch(instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Ifeq(offset)
            | Instruction::Ifne(offset)
            | Instruction::Iflt(offset)
            | Instruction::Ifge(offset)
            | Instruction::Ifgt(offset)
            | Instruction::Ifle(offset)
            | Instruction::If_icmpeq(offset)
            | Instruction::If_icmpne(offset)
            | Instruction::If_icmplt(offset)
            | Instruction::If_icmpge(offset)
            | Instruction::If_icmpgt(offset)
            | Instruction::If_icmple(offset)
            | Instruction::Goto(offset) => *offset > 0,
            Instruction::Goto_w(offset) => *offset > 0,
            Instruction::Tableswitch(table) => {
                table.default > 0 && table.offsets.iter().all(|offset| *offset > 0)
            }
            Instruction::Lookupswitch(table) => {
                table.default > 0 && table.pairs.values().all(|offset| *offset > 0)
            }
            _ => false,
        }
    }

    /// Instructions that do not produce frame-owned heap references or require
    /// helper root handoff. Do not add heap/reference operations here to enable
    /// production execution; eligibility and root requirements are separate gates.
    #[expect(
        clippy::too_many_lines,
        reason = "the explicit bytecode allowlist is easier to audit as one exhaustive policy"
    )]
    fn is_rootless_primitive_instruction(instruction: &Instruction) -> bool {
        if Self::is_primitive_branch(instruction) {
            return true;
        }
        matches!(
            instruction,
            Instruction::Nop
                | Instruction::Iconst_m1
                | Instruction::Iconst_0
                | Instruction::Iconst_1
                | Instruction::Iconst_2
                | Instruction::Iconst_3
                | Instruction::Iconst_4
                | Instruction::Iconst_5
                | Instruction::Lconst_0
                | Instruction::Lconst_1
                | Instruction::Fconst_0
                | Instruction::Fconst_1
                | Instruction::Fconst_2
                | Instruction::Dconst_0
                | Instruction::Dconst_1
                | Instruction::Bipush(_)
                | Instruction::Sipush(_)
                | Instruction::Ldc2_w(_)
                | Instruction::Iload(_)
                | Instruction::Lload(_)
                | Instruction::Fload(_)
                | Instruction::Dload(_)
                | Instruction::Iload_0
                | Instruction::Iload_1
                | Instruction::Iload_2
                | Instruction::Iload_3
                | Instruction::Lload_0
                | Instruction::Lload_1
                | Instruction::Lload_2
                | Instruction::Lload_3
                | Instruction::Fload_0
                | Instruction::Fload_1
                | Instruction::Fload_2
                | Instruction::Fload_3
                | Instruction::Dload_0
                | Instruction::Dload_1
                | Instruction::Dload_2
                | Instruction::Dload_3
                | Instruction::Istore(_)
                | Instruction::Lstore(_)
                | Instruction::Fstore(_)
                | Instruction::Dstore(_)
                | Instruction::Istore_0
                | Instruction::Istore_1
                | Instruction::Istore_2
                | Instruction::Istore_3
                | Instruction::Lstore_0
                | Instruction::Lstore_1
                | Instruction::Lstore_2
                | Instruction::Lstore_3
                | Instruction::Fstore_0
                | Instruction::Fstore_1
                | Instruction::Fstore_2
                | Instruction::Fstore_3
                | Instruction::Dstore_0
                | Instruction::Dstore_1
                | Instruction::Dstore_2
                | Instruction::Dstore_3
                | Instruction::Pop
                | Instruction::Pop2
                | Instruction::Dup
                | Instruction::Dup_x1
                | Instruction::Dup_x2
                | Instruction::Dup2
                | Instruction::Dup2_x1
                | Instruction::Dup2_x2
                | Instruction::Swap
                | Instruction::Iadd
                | Instruction::Ladd
                | Instruction::Fadd
                | Instruction::Dadd
                | Instruction::Isub
                | Instruction::Lsub
                | Instruction::Fsub
                | Instruction::Dsub
                | Instruction::Imul
                | Instruction::Lmul
                | Instruction::Fmul
                | Instruction::Dmul
                | Instruction::Fdiv
                | Instruction::Ddiv
                | Instruction::Frem
                | Instruction::Drem
                | Instruction::Ineg
                | Instruction::Lneg
                | Instruction::Fneg
                | Instruction::Dneg
                | Instruction::Ishl
                | Instruction::Lshl
                | Instruction::Ishr
                | Instruction::Lshr
                | Instruction::Iushr
                | Instruction::Lushr
                | Instruction::Iand
                | Instruction::Land
                | Instruction::Ior
                | Instruction::Lor
                | Instruction::Ixor
                | Instruction::Lxor
                | Instruction::Iinc(_, _)
                | Instruction::I2l
                | Instruction::I2f
                | Instruction::I2d
                | Instruction::L2i
                | Instruction::L2f
                | Instruction::L2d
                | Instruction::F2i
                | Instruction::F2l
                | Instruction::F2d
                | Instruction::D2i
                | Instruction::D2l
                | Instruction::D2f
                | Instruction::I2b
                | Instruction::I2c
                | Instruction::I2s
                | Instruction::Lcmp
                | Instruction::Fcmpl
                | Instruction::Fcmpg
                | Instruction::Dcmpl
                | Instruction::Dcmpg
                | Instruction::Ireturn
                | Instruction::Lreturn
                | Instruction::Freturn
                | Instruction::Dreturn
                | Instruction::Return
                | Instruction::Iload_w(_)
                | Instruction::Lload_w(_)
                | Instruction::Fload_w(_)
                | Instruction::Dload_w(_)
                | Instruction::Istore_w(_)
                | Instruction::Lstore_w(_)
                | Instruction::Fstore_w(_)
                | Instruction::Dstore_w(_)
                | Instruction::Iinc_w(_, _)
        )
    }

    #[expect(clippy::too_many_arguments)]
    #[expect(clippy::too_many_lines)]
    fn process_instruction(
        constant_pool: &ConstantPool,
        function_builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        exception_blocks: &AHashMap<usize, Block>,
        locals: &mut LocalVariables,
        stack: &mut OperandStack,
        program_counter: usize,
        return_pointer: Value,
        return_type: Option<&FieldType>,
        context_pointer: Value,
        helpers: &RuntimeHelpers,
        call_arguments: Option<StackSlot>,
        continuation: Option<(&continuation::Continuation, Value)>,
        exception_block: Block,
        exception_table: &[ExceptionTableEntry],
        instruction: &Instruction,
    ) -> Result<()> {
        let throw_context = ThrowContext {
            exception_block,
            blocks: exception_blocks,
            exception_table,
            program_counter,
            continuation: continuation.and_then(|(continuation, state)| {
                call_arguments.map(|arguments| (continuation, state, arguments))
            }),
        };
        let throw_context = &throw_context;
        let request_continuation = continuation.and_then(|(continuation, state)| {
            call_arguments.map(|arguments| crate::instruction::RequestContinuation {
                call: continuation::CallContinuation {
                    continuation,
                    state,
                },
                arguments,
                bci: program_counter,
            })
        });
        match instruction {
            Instruction::Iaload
            | Instruction::Laload
            | Instruction::Faload
            | Instruction::Daload
            | Instruction::Aaload
            | Instruction::Baload
            | Instruction::Caload
            | Instruction::Saload
            | Instruction::Iastore
            | Instruction::Lastore
            | Instruction::Fastore
            | Instruction::Dastore
            | Instruction::Bastore
            | Instruction::Castore
            | Instruction::Sastore
            | Instruction::Arraylength => {
                let access = array_access_metadata(instruction)
                    .ok_or_else(|| InternalError("missing array access metadata".to_owned()))?;
                array_access(
                    function_builder,
                    stack,
                    helpers,
                    context_pointer,
                    throw_context,
                    access,
                    request_continuation,
                )?;
            }
            Instruction::Invokestatic(index)
            | Instruction::Invokespecial(index)
            | Instruction::Invokevirtual(index)
            | Instruction::Invokeinterface(index, _) => invoke(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                call_arguments
                    .ok_or_else(|| InternalError("missing outgoing argument buffer".to_owned()))?,
                if matches!(instruction, Instruction::Invokestatic(_)) {
                    CallTarget::Static(*index)
                } else if matches!(instruction, Instruction::Invokespecial(_)) {
                    CallTarget::Special(*index)
                } else if let Instruction::Invokeinterface(_, count) = instruction {
                    CallTarget::Interface(*index, *count)
                } else {
                    CallTarget::Virtual(*index)
                },
                continuation.map(|(continuation, state)| continuation::CallContinuation {
                    continuation,
                    state,
                }),
            )?,
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
            Instruction::Ldc(index) => ldc(
                constant_pool,
                function_builder,
                stack,
                u16::from(*index),
                helpers,
                context_pointer,
                throw_context,
                request_continuation,
            )?,
            Instruction::Ldc_w(index) => {
                ldc(
                    constant_pool,
                    function_builder,
                    stack,
                    *index,
                    helpers,
                    context_pointer,
                    throw_context,
                    request_continuation,
                )?;
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
            Instruction::Aastore => aastore(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                request_continuation,
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
            Instruction::Goto(address) => {
                goto(function_builder, blocks, stack, program_counter, *address)?;
            }
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
            Instruction::Ireturn => {
                ireturn(function_builder, stack, return_pointer, return_type)?;
            }
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
                request_continuation,
            )?,
            Instruction::Putstatic(index) => putstatic(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                request_continuation,
            )?,
            Instruction::Getfield(index) => getfield(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                request_continuation,
            )?,
            Instruction::Putfield(index) => putfield(
                constant_pool,
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                request_continuation,
            )?,
            Instruction::New(index) => new_object(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                request_continuation,
            )?,
            Instruction::Newarray(array_type) => {
                newarray(
                    function_builder,
                    stack,
                    array_type,
                    context_pointer,
                    helpers,
                    throw_context,
                    request_continuation,
                )?;
            }
            Instruction::Anewarray(index) => anewarray(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                request_continuation,
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
                request_continuation,
            )?,
            Instruction::Instanceof(class_index) => instanceof(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *class_index,
                request_continuation,
            )?,
            Instruction::Monitorenter | Instruction::Monitorexit => monitor(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                call_arguments
                    .ok_or_else(|| InternalError("missing monitor argument buffer".to_owned()))?,
                matches!(instruction, Instruction::Monitorenter),
                continuation.map(|(continuation, state)| continuation::CallContinuation {
                    continuation,
                    state,
                }),
            )?,
            Instruction::Wide => wide()?,
            Instruction::Multianewarray(index, dimensions) => multianewarray(
                function_builder,
                stack,
                helpers,
                context_pointer,
                throw_context,
                *index,
                *dimensions,
                request_continuation,
            )?,
            Instruction::Ifnull(address) => {
                ifnull(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Ifnonnull(address) => {
                ifnonnull(function_builder, blocks, stack, program_counter, *address)?;
            }
            Instruction::Goto_w(address) => {
                goto_w(function_builder, blocks, stack, program_counter, *address)?;
            }
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
            Instruction::Invokedynamic(_) => {
                return Err(UnsupportedInstruction(instruction.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::{CLASS_INITIALIZER_NAME, JavaString};

    #[test]
    fn test_compiler_new() {
        let result = Compiler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn ordinary_long_store_admission_uses_the_declaring_constant_pool() -> Result<()> {
        let mut class = ClassFile::default();
        let pool = &mut class.constant_pool;
        let owner = pool.add_class("Fields")?;
        let long = pool.add_field_ref(owner, "bits", "J")?;
        let invalid = [
            pool.add_field_ref(owner, "count", "I")?,
            pool.add_field_ref(owner, "reference", "Ljava/lang/Object;")?,
            pool.add_field_ref(owner, "array", "[J")?,
            pool.add_field_ref(owner, "", "J")?,
            pool.add_field_ref(0, "bits", "J")?,
            pool.add_method_ref(owner, "run", "()J")?,
            0,
        ];
        let method = |index, tail| Method {
            access_flags: MethodAccessFlags::STATIC,
            attributes: vec![Attribute::Code {
                name_index: 0,
                max_stack: 2,
                max_locals: 0,
                code: vec![Instruction::Lconst_0, Instruction::Putstatic(index), tail],
                exception_table: vec![],
                attributes: vec![],
            }],
            ..Default::default()
        };
        let valid = method(long, Instruction::Return);
        assert!(Compiler::can_compile_in(&class, &valid));
        assert_eq!(Compiler::rejection_in(&class, &valid), None);
        assert!(!Compiler::can_compile(&valid));
        for index in invalid {
            let body = method(index, Instruction::Return);
            assert!(!Compiler::can_compile_in(&class, &body));
            assert_eq!(
                Compiler::rejection_in(&class, &body),
                Some(crate::CompilationRejection {
                    reason: "unqualified_instruction",
                    bytecode: Some((1, Instruction::Putstatic(index).code())),
                })
            );
        }
        let next = method(long, Instruction::Aconst_null);
        assert_eq!(
            Compiler::rejection_in(&class, &next).and_then(|r| r.bytecode),
            Some((4, Instruction::Aconst_null.code()))
        );
        Ok(())
    }

    #[test]
    fn rejected_bytecode_reports_the_gate_that_denied_the_whole_method() {
        let method = |access_flags, code| Method {
            access_flags,
            attributes: vec![Attribute::Code {
                name_index: 0,
                max_stack: 2,
                max_locals: 1,
                code,
                exception_table: vec![],
                attributes: vec![],
            }],
            ..Default::default()
        };
        // The first branch jumps over an unreachable putstatic at byte offset 3.
        let cold = vec![
            Instruction::Goto(6),
            Instruction::Putstatic(1),
            Instruction::Return,
        ];
        let body = method(MethodAccessFlags::STATIC, cold.clone());
        assert_eq!(
            Compiler::rejected_bytecode(&body),
            Some((3, Instruction::Putstatic(1).code()))
        );
        assert_eq!(
            Compiler::rejected_bytecode(&method(MethodAccessFlags::PUBLIC, cold)),
            None
        );
        let invocation = method(
            MethodAccessFlags::STATIC,
            vec![
                Instruction::Aconst_null,
                Instruction::Sipush(12),
                Instruction::Invokedynamic(1),
            ],
        );
        assert_eq!(
            Compiler::rejection_reason(&invocation),
            Some("unsupported_invocation")
        );
        assert_eq!(
            Compiler::rejected_bytecode(&invocation),
            Some((4, Instruction::Invokedynamic(1).code()))
        );
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    #[test]
    fn experimental_gate_preserves_production_and_rejects_unqualified_capabilities() {
        let class = ClassFile::default();
        let mut method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            attributes: vec![Attribute::Code {
                name_index: 0,
                max_stack: 2,
                max_locals: 2,
                code: vec![
                    Instruction::Aload_0,
                    Instruction::Getfield(1),
                    Instruction::Areturn,
                ],
                exception_table: vec![],
                attributes: vec![],
            }],
            ..Default::default()
        };
        assert_eq!(
            Compiler::experimental_rejection_reason(&class, &method),
            None
        );
        assert_eq!(
            Compiler::rejection_reason(&method),
            Some("non_static_method")
        );
        method.access_flags.insert(MethodAccessFlags::STATIC);
        assert_eq!(
            Compiler::rejection_reason(&method),
            Some("unqualified_instruction")
        );
        for instruction in [
            Instruction::Invokedynamic(1),
            Instruction::Invokevirtual(1),
            Instruction::Invokeinterface(1, 1),
            Instruction::Monitorenter,
            Instruction::Monitorexit,
            Instruction::Jsr(1),
            Instruction::Ret(1),
            Instruction::Athrow,
            Instruction::Aaload,
            Instruction::Idiv,
        ] {
            let Attribute::Code { code, .. } =
                method.attributes.first_mut().expect("code attribute")
            else {
                panic!("expected code attribute")
            };
            *code = vec![instruction];
            assert_eq!(
                Compiler::experimental_rejection_reason(&class, &method),
                Some("experimental_unqualified_instruction")
            );
        }
        method.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
        assert_eq!(
            Compiler::experimental_rejection_reason(&class, &method),
            Some("experimental_synchronized_method")
        );
    }

    #[cfg(feature = "experimental-reference-dispatch")]
    #[test]
    fn experimental_numeric_constants_do_not_admit_reference_constants() -> Result<()> {
        let mut class = ClassFile::default();
        let integer = class.constant_pool.add_integer(-1_000_000)?;
        let float = class.constant_pool.add_float(1.5)?;
        let string = class.constant_pool.add_string("not a numeric constant")?;
        let reference = class.constant_pool.add_class("java/lang/Object")?;
        for (index, admitted) in [
            (integer, true),
            (float, true),
            (string, false),
            (reference, false),
            (u16::MAX, false),
        ] {
            let method = Method {
                access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                attributes: vec![Attribute::Code {
                    name_index: 0,
                    max_stack: 1,
                    max_locals: 0,
                    code: vec![Instruction::Ldc_w(index), Instruction::Ireturn],
                    exception_table: vec![],
                    attributes: vec![],
                }],
                ..Default::default()
            };
            assert_eq!(
                Compiler::experimental_rejection_reason(&class, &method).is_none(),
                admitted
            );
            assert!(!Compiler::can_compile(&method));
        }
        Ok(())
    }

    #[test]
    fn test_production_safe_instruction_policy() {
        for instruction in [
            Instruction::Iload_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ifeq(3),
            Instruction::Goto(3),
            Instruction::Goto_w(5),
            Instruction::Ifeq(0),
            Instruction::If_icmpge(-1),
            Instruction::Goto(-3),
            Instruction::Goto_w(0),
            Instruction::Ireturn,
        ] {
            assert!(Compiler::is_production_safe_instruction(&instruction));
        }

        for instruction in [
            Instruction::Aload_0,
            Instruction::Getfield(1),
            Instruction::Iaload,
            Instruction::Ifnull(3),
            Instruction::If_acmpeq(3),
            Instruction::Jsr(3),
            Instruction::Monitorenter,
            Instruction::Idiv,
            Instruction::Invokestatic(1),
            Instruction::Invokespecial(1),
            Instruction::Invokevirtual(1),
            Instruction::Invokeinterface(1, 1),
        ] {
            assert!(!Compiler::is_production_safe_instruction(&instruction));
        }
    }

    #[test]
    fn test_forward_control_flow_requires_every_edge_to_advance() {
        use indexmap::IndexMap;
        use ristretto_classfile::attributes::{LookupSwitch, TableSwitch};

        let branches: [fn(i16) -> Instruction; 13] = [
            Instruction::Ifeq,
            Instruction::Ifne,
            Instruction::Iflt,
            Instruction::Ifge,
            Instruction::Ifgt,
            Instruction::Ifle,
            Instruction::If_icmpeq,
            Instruction::If_icmpne,
            Instruction::If_icmplt,
            Instruction::If_icmpge,
            Instruction::If_icmpgt,
            Instruction::If_icmple,
            Instruction::Goto,
        ];
        for offset in [i16::MIN, -1, 0, 3, i16::MAX] {
            for branch in branches {
                assert_eq!(
                    Compiler::is_forward_primitive_branch(&branch(offset)),
                    offset > 0
                );
            }
        }
        for offset in [i32::MIN, -1, 0, 5, i32::MAX] {
            assert_eq!(
                Compiler::is_forward_primitive_branch(&Instruction::Goto_w(offset)),
                offset > 0
            );
            // Place the candidate edge in the default and each explicit case.
            for position in 0..3 {
                let mut offsets = [32, 36, 40];
                *offsets.get_mut(position).expect("edge position") = offset;
                for instruction in [
                    Instruction::Tableswitch(Box::new(TableSwitch {
                        default: offsets[0],
                        low: 0,
                        high: 1,
                        offsets: offsets[1..].to_vec(),
                    })),
                    Instruction::Lookupswitch(Box::new(LookupSwitch {
                        default: offsets[0],
                        pairs: IndexMap::from([(-1, offsets[1]), (1, offsets[2])]),
                    })),
                ] {
                    assert_eq!(
                        Compiler::is_forward_primitive_branch(&instruction),
                        offset > 0
                    );
                }
            }
        }
    }

    #[test]
    fn test_function_name() {
        let class_name = "java/lang/Object";
        let method_name = CLASS_INITIALIZER_NAME;
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

#[cfg(all(test, feature = "experimental-reference-dispatch"))]
#[path = "request_only_tests.rs"]
mod request_only_tests;
